use std::{ collections::HashMap, str::FromStr };

use alloy_primitives::{ Address, B256 };
use revm::primitives::{ Account, ResultAndState };

// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/d11ed2fb0a0130363b1e82d5742ad7516df0e749/contracts/token/ERC20/IERC20.sol#L16
const TRANSFER_EVENT_TOPICS: B256 = B256::from_str(
    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
).unwrap();
// MINT : Transfer event with zero address in from field.
// BURN : Transfer event with zero address in to field.
// summerize erc20 contract events(transfer,burn,mint)to sync with comsos bank modulefn
pub fn bank_end_handler(results: ResultAndState) -> Vec<BackRecord> {
    let mut records = Vec::new();
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(10);
    // handle native token
    results.state.iter().for_each(|(owner, account)| {
        if account.is_touched() {
            records.push(BackRecord {
                coin: Address::ZERO, // native coin, just commit
                owner,
                value: account.info.balance,
                action: BackAction::COMMIT,
            })
        }
    });
    // handle erc20 token
    results.result
        .logs()
        .iter()
        .for_each(|log| {
            if log.topics().eq(&TRANSFER_EVENT_TOPICS) {
                return;
            }
            let coin = log.address;
            let (topics, data) = log.split();
            let from = Address::from_slice(&topics[1].as_bytes());
            let to = Address::from_slice(&topics[2].as_bytes());
            let value = B256::from_slice(&data);

            if from == Address::ZERO {
                // Mint - to_address: +value
                records.push(BackRecord {
                    coin,
                    owner: to,
                    value,
                    action: BackAction::PLUS,
                });
            } else if to == Address::ZERO {
                // Burn - from_address: -value
                records.push(BackRecord {
                    coin,
                    owner: from,
                    value,
                    action: BackAction::MINUS,
                });
            } else {
                // Transfer - from_address: -value, to_address: +value
                records.push(BackRecord {
                    coin,
                    owner: from,
                    value,
                    action: BackAction::MINUS,
                });
                records.push(BackRecord {
                    coin,
                    owner: to,
                    value,
                    action: BackAction::PLUS,
                });
            }
        });
    records
}
