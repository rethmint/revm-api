use revm_primitives::{address, Address, Bytes, U256};

use crate::{initialize, ByteSliceView, CallRequest, CallTransaction};

#[test]
fn test_revm_initialize_function() {
    let call_request = CallRequest {
        transaction: CallTransaction {
            sender: Some(Address::from_word(
                address!("d8da6bf26964af9d7eed9e03e53415d37aa96045").into_word(),
            )),
            gas_price: Some(U256::from(100)),
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            blob_versioned_hashes: vec![],
            max_fee_per_blob_gas: None,
            data: "".to_string().into(),
            nonce: U256::from(1),
            value: U256::from(1000),
            ..Default::default()
        },
        ..Default::default()
    };
    let call_request_json = serde_json::to_string(&call_request).unwrap();
    let call_request_bytes = call_request_json.as_bytes();

    let result = initialize(ByteSliceView::new(call_request_bytes));

    assert!(result.is_some());
}
