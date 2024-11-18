use alloy_primitives::B256;
use sled::IVec;

pub type QueryKeySlice = [u8; 33];

#[derive(Debug, Clone, Copy)]
pub struct QueryKey(QueryKeySlice);

/*
* Prefix
* 0x0...[u8;32] -> Count
* 0x1...[u8;32] -> Shared object file
*/

impl QueryKey {
    pub fn with_prefix(key: B256, prefix: KeyPrefix) -> Self {
        let mut prefixed = [0u8; 33];
        prefixed[0] = prefix.as_byte();
        prefixed[1..].copy_from_slice(key.as_slice());
        QueryKey(prefixed)
    }

    pub fn as_inner(&self) -> &[u8; 33] {
        &self.0
    }

    pub fn match_prefix(&self, prefix: KeyPrefix) -> bool {
        self.0[0] == prefix.as_byte()
    }

    pub fn update_prefix(&mut self, new_prefix: KeyPrefix) {
        self.0[0] = new_prefix.as_byte();
    }

    pub fn to_b256(self) -> B256 {
        B256::from_slice(&self.0[1..])
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0[1..33]
    }

    pub fn from_ivec(ivec: IVec) -> Self {
        assert!(ivec.len() == 33);

        let mut arr = [0u8; 33];
        arr.copy_from_slice(&ivec);
        QueryKey(arr)
    }
}

pub enum KeyPrefix {
    Count,
    SO,
}

impl KeyPrefix {
    fn as_byte(&self) -> u8 {
        match self {
            KeyPrefix::Count => 0x01,
            KeyPrefix::SO => 0x02,
        }
    }
}
