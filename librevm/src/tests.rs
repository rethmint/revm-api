use std::collections::HashMap;

use alloy_primitives::Bytes;
use revm_interpreter::analysis::to_analysed;
use revm_primitives::{
    address, keccak256, Address, BlockEnv, Bytecode, Env, FixedBytes, TxEnv, TxKind, U256,
};

//use crate::{ initialize, initialize_native, ByteSliceView, CallRequest, CallTransaction };

//#[test]
//fn test_ffi_revm_initialize_function() {
//    let call_request = CallRequest {
//        transaction: CallTransaction {
//            sender: Some(Address::from_word(
//                address!("d8da6bf26964af9d7eed9e03e53415d37aa96045").into_word(),
//            )),
//            gas_price: Some(U256::from(11)),
//            max_fee_per_gas: None,
//            max_priority_fee_per_gas: None,
//            blob_versioned_hashes: vec![],
//            max_fee_per_blob_gas: None,
//            data: "".to_string().into(),
//            nonce: U256::from(1),
//            value: U256::from(1000),
//            ..Default::default()
//        },
//        ..Default::default()
//    };
//    let call_request_json = serde_json::to_string(&call_request).unwrap();
//    let call_request_bytes = call_request_json.as_bytes();
//
//    let result = initialize(ByteSliceView::new(call_request_bytes));
//
//    assert!(result.is_some());
//}
//
//#[test]
//fn test_revm_initialize_function() {
//    let mut env = Env::<BlockEnv, TxEnv>::default();
//
//    // Tx env
//    env.tx.chain_id = Some(1);
//    env.tx.caller =
//        Address::from_word(address!("e100713fc15400d1e94096a545879e7c6407001e").into_word());
//    env.tx.gas_limit = 0xf4240;
//    env.tx.gas_price = U256::from(0x3e8);
//    env.tx.transact_to = TxKind::Create;
//    env.tx.value = U256::ZERO;
//    env.tx.data = "0x".to_string().into();
//    env.tx.nonce = 0x1 as u64;
//    env.tx.access_list = Vec::new();
//    env.tx.gas_priority_fee = None;
//    env.tx.blob_hashes = Vec::new();
//    env.tx.max_fee_per_blob_gas = None;
//    env.tx.authorization_list = None;
//
//    // Block env
//    env.block.number = U256::from(1);
//    env.block.coinbase =
//        Address::from_word(address!("00000000000000000000000000000000c014bace").into_word());
//    env.block.timestamp = U256::ZERO;
//    env.block.gas_limit = U256::from(0xf4240);
//    env.block.basefee = U256::from(0x3e7);
//    //env.block.difficulty= U256;
//    //env.block.prevrandao= Option<B256>;
//    //env.block.blob_excess_gas_and_price= Option<BlobExcessGasAndPrice>;
//
//    // Cfg env
//    env.cfg.chain_id = 1;
//    //assert_eq!(
//    //    env.validate_tx::<crate::LatestSpec>(),
//    //    Err(InvalidTransaction::InvalidChainId)
//    //);
//
//    // pre
//    let mut pre = HashMap::new();
//    pre.insert(
//        Address::from_word(address!("e100713fc15400d1e94096a545879e7c6407001e").into_word()),
//        TestAccountInfo {
//            balance: U256::from(0x3b9aca01),
//            code: "0x".to_string().into(),
//            nonce: 1 as u64,
//            storage: HashMap::new(),
//        },
//    );
//
//    let mut cache_state = revm::CacheState::new(false);
//    for (address, info) in pre {
//        let code_hash = keccak256(&info.code);
//        let bytecode = to_analysed(Bytecode::new_raw(info.code));
//        let acc_info = revm::primitives::AccountInfo {
//            balance: info.balance,
//            code_hash,
//            code: None,
//            nonce: info.nonce,
//        };
//        cache_state.insert_account_with_storage(address, acc_info, info.storage);
//    }
//
//    // let result = evm_call(env, cache_state);
//
//    // assert!(result.is_some());
//}
//
////{
////  "create_tx_empty": {
////    "env": {
////      "currentBaseFee": "0x3e7",
////      "currentCoinbase": "0x00000000000000000000000000000000c014bace",
////      "currentGasLimit": "0xf4240",
////      "currentNumber": "0x1",
////      "currentTimestamp": "0x0"
////    },
////    "post": {
////      "Shanghai": [
////        {
////          "hash": "0x8ae438f7a4a14dbc25410dfaa12e95e7b36f311ab904b4358c3b544e06df4c50",
////          "indexes": {
////            "data": 0,
////            "gas": 0,
////            "value": 0
////          },
////          "logs": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
////        }
////      ]
////    },
////    "pre": {
////      "0xe100713fc15400d1e94096a545879e7c6407001e": {
////        "balance": "0x3b9aca01",
////        "code": "0x",
////        "nonce": "0x1",
////        "storage": {}
////      }
////    },
////    "transaction": {
////      "data": [
////        "0x"
////      ],
////      "gasLimit": [
////        "0xf4240"
////      ],
////      "maxFeePerGas": "0x3e8",
////      "maxPriorityFeePerGas": "0x3e8",
////      "nonce": "0x1",
////      "secretKey": "0x00000000000000000000000000000000000000000000000000000002b1263d2b",
////      "sender": "0xe100713fc15400d1e94096a545879e7c6407001e",
////      "value": [
////        "0x0"
////      ]
////    }
////  }
////}
//
//pub struct TestAccountInfo {
//    pub balance: U256,
//    pub code: Bytes,
//    pub nonce: u64,
//    pub storage: HashMap<U256, U256>,
//}
//
////{
////  "create2_factory": {
////    "env": {
////      "currentBaseFee": "0x3e7",
////      "currentCoinbase": "0x00000000000000000000000000000000c014bace",
////      "currentGasLimit": "0xf4240",
////      "currentNumber": "0x1",
////      "currentTimestamp": "0x0"
////    },
////    "post": {
////      "Shanghai": [
////        {
////          "hash": "0xefaad235d39701d77e9395ed68e4394112130868d26b72ad8fcaf1813db46761",
////          "indexes": {
////            "data": 0,
////            "gas": 0,
////            "value": 0
////          },
////          "logs": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
////        }
////      ]
////    },
////    "pre": {
////      "0x000000000000000000000000000000000000c0de": {
////        "balance": "0x0",
////        "code": "0x36600060003760003660006000f5",
////        "nonce": "0x1",
////        "storage": {}
////      },
////      "0xe100713fc15400d1e94096a545879e7c6407001e": {
////        "balance": "0x3b9aca01",
////        "code": "0x",
////        "nonce": "0x1",
////        "storage": {}
////      }
////    },
////    "transaction": {
////      "data": [
////        "0x60fe60005360016000f3"
////      ],
////      "gasLimit": [
////        "0xf4240"
////      ],
////      "maxFeePerGas": "0x3e8",
////      "maxPriorityFeePerGas": "0x3e8",
////      "nonce": "0x1",
////      "secretKey": "0x00000000000000000000000000000000000000000000000000000002b1263d2b",
////      "sender": "0xe100713fc15400d1e94096a545879e7c6407001e",
////      "to": "0x000000000000000000000000000000000000c0de",
////      "value": [
////        "0x0"
////      ]
////    }
////  }
////}
