use alloy_primitives::B256;
use db_key::Key;

#[derive(Debug, Clone, Copy)]
pub struct QueryKey([u8; 33]);

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

    pub fn to_b256(&self) -> B256 {
        B256::from_slice(&self.0[1..])
    }
}

impl Key for QueryKey {
    fn from_u8(key: &[u8]) -> Self {
        assert!(
            key.len() == 33,
            "Expected 33 bytes (1 byte prefix + 32 byte key)"
        );

        let mut array = [0u8; 33];
        array.copy_from_slice(key);
        QueryKey(array)
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(&self.0)
    }
}

pub enum KeyPrefix {
    Count,
    Label,
    Bytecode,
}

impl KeyPrefix {
    fn as_byte(&self) -> u8 {
        match self {
            KeyPrefix::Count => 0x01,
            KeyPrefix::Label => 0x02,
            KeyPrefix::Bytecode => 0x03,
        }
    }
}
