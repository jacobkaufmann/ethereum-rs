use ethereum_types::{Address, U256};

/// An Ethereum transaction.
pub struct Transaction {
    nonce: U256,
    gas_price: U256,
    gas_limit: U256,
    to: Option<Address>,
    value: U256,
    v: U256,
    r: U256,
    s: U256,
    init: Option<Vec<u8>>,
    data: Option<Vec<u8>>,
}
