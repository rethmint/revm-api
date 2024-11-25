use std::{ collections::HashMap, str::FromStr };

use alloy_primitives::{ Address, B256 };
use revm::primitives::{ Account, ExecutionResult };
// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/d11ed2fb0a0130363b1e82d5742ad7516df0e749/contracts/token/ERC20/IERC20.sol#L16
const TRANSFER_EVENT_TOPICS: B256 = B256::from_str(
    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
);
// MINT : Transfer event with zero address in from field.
// BURN : Transfer event with zero address in to field.
// summerize erc20 contract events(transfer,burn,mint)to sync with comsos bank modulefn
pub fn record_bank(results: ExecutionResult) {
    results
        .logs()
        .iter()
        .filter(|log| log.topics().eq(&TRANSFER_EVENT_TOPICS))
        .map(|log| {
            let from = log.topics()[1];
            let to = log.topics()[2];
            let token = log.address;
            let value = B256::from(log.data);
            if from == Address::zero() {
                // Mint event
                println!("Mint event: to = {:?}, value = {:?}", to, value);
            } else if to == Address::zero() {
                // Burn event
                println!("Burn event: from = {:?}, value = {:?}", from, value);
            } else {
                // Transfer event
                println!("Transfer event: from = {:?}, to = {:?}, value = {:?}", from, to, value);
            }
        })
}

fn parse_transfer_event(log);
