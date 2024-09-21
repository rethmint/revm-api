use ethers_core::types::H256;
use revm_primitives::Address;

use crate::Op;
use std::collections::{btree_map, BTreeMap};

pub type WriteOp = Op<Vec<u8>>;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct WriteSet(BTreeMap<(Address, H256), WriteOp>);

impl WriteSet {
    pub fn new(change: Vec<((Address, H256), WriteOp)>) -> Self {
        let mut write_set: BTreeMap<(Address, H256), WriteOp> = BTreeMap::new();
        for (key, write_op) in change {
            write_set.insert(key, write_op);
        }
        Self(write_set)
    }
}

impl ::std::iter::FromIterator<((Address, H256), WriteOp)> for WriteSet {
    fn from_iter<I: IntoIterator<Item = ((Address, H256), WriteOp)>>(iter: I) -> Self {
        let mut ws = WriteSet::default();
        for write in iter {
            ws.0.insert(write.0, write.1);
        }
        ws
    }
}

impl<'a> IntoIterator for &'a WriteSet {
    type Item = (&'a (Address, H256), &'a WriteOp);
    type IntoIter = btree_map::Iter<'a, (Address, H256), WriteOp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl ::std::iter::IntoIterator for WriteSet {
    type Item = ((Address, H256), WriteOp);
    type IntoIter = btree_map::IntoIter<(Address, H256), WriteOp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
