use revm_primitives::{ Address, BlockEnv, TxEnv, TxKind, U256 };

use crate::ByteSliceView;

#[derive(serde::Deserialize)]
pub struct TempBlockEnv {
    pub number: U256,
    pub coinbase: Address,
    pub timestamp: U256,
    pub gas_limit: U256,
    pub basefee: U256,
}
#[derive(serde::Deserialize)]
pub struct TempTxEnv {
    pub caller: Address,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub transact_to: Address,
    pub value: U256,
    pub nonce: u64,
    pub chain_id: u64,
    pub gas_priority_fee: U256,
}
pub trait EnvDecoder<T> {
    fn decode(env: ByteSliceView) -> Self;
    fn from(temp: T) -> Self;
}

impl EnvDecoder<TempBlockEnv> for BlockEnv {
    fn decode(block: ByteSliceView) -> BlockEnv {
        let block_temp: TempBlockEnv = serde_json
            ::from_str(
                &String::from_utf8(
                    block
                        .read()
                        .unwrap()
                        //.ok_or_else(|| Error::unset_arg(BLOCK))?
                        .to_vec()
                ).expect("String Decoding Failed")
            )
            .expect("JSON Decoding Failed");

        return <Self as EnvDecoder<TempBlockEnv>>::from(block_temp).into();
    }

    fn from(temp: TempBlockEnv) -> Self {
        BlockEnv {
            number: temp.number,
            coinbase: temp.coinbase,
            timestamp: temp.timestamp,
            gas_limit: temp.gas_limit,
            basefee: temp.basefee,
            ..Default::default()
        }
    }
}

impl EnvDecoder<TempTxEnv> for TxEnv {
    fn decode(tx: ByteSliceView) -> TxEnv {
        let tx_temp: TempTxEnv = serde_json
            ::from_str(
                &String::from_utf8(
                    tx
                        .read()
                        .unwrap()
                        //.ok_or_else(|| Error::unset_arg(tx))?
                        .to_vec()
                ).expect("String Decoding Failed")
            )
            .expect("JSON Decoding Failed");

        return <Self as EnvDecoder<TempTxEnv>>::from(tx_temp).into();
    }

    fn from(temp: TempTxEnv) -> Self {
        TxEnv {
            caller: temp.caller,
            chain_id: Some(temp.chain_id),
            gas_limit: temp.gas_limit,
            gas_price: temp.gas_price,
            transact_to: match temp.transact_to {
                Address::ZERO => TxKind::Create,
                _ => TxKind::Call(temp.transact_to),
            },
            value: temp.value,
            nonce: temp.nonce,
            gas_priority_fee: match temp.gas_priority_fee {
                U256::ZERO => None,
                _ => Some(temp.gas_priority_fee),
            },
            ..Default::default()
        }
    }
}
