use ethereum_types::{H256, U256};
use keccak_hash::KECCAK_EMPTY;

/// An Ethereum account.
pub struct Account {
    /// The number of transactions sent from this account (non-contract) or the number
    /// of contract creations made by this account.
    nonce: U256,
    /// The number of Wei owned by this account.
    balance: U256,
    /// Hash of the root node of a Merkle Patricia tree that encodes the storages contents
    /// of this account.
    storage_root: H256,
    /// Hash of the EVM code of this account.
    code_hash: H256,
}

impl Account {
    /// Returns whether this account is a contract account.
    fn is_contract(&self) -> bool {
        self.code_hash != KECCAK_EMPTY
    }

    /// Returns whether this account is empty.
    fn is_empty(&self) -> bool {
        self.nonce.is_zero() && self.balance.is_zero() && self.code_hash == KECCAK_EMPTY
    }
}
