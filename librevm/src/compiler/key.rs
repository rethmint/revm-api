use alloy_primitives::B256;

pub type SledDBKeySlice = [u8; 32];

#[derive(Debug, Clone, Copy)]
pub struct SledDbKey(SledDBKeySlice);

impl SledDbKey {
    pub fn with_b256(b256: B256) -> Self {
        Self(*b256)
    }

    pub fn as_inner(&self) -> &[u8; 32] {
        &self.0
    }
}
