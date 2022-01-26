use ethereum_types::{Address, U256};
use std::collections::HashSet;

use crate::core::types::log::Log;

/// Accrued transaction substate.
///
/// Information acted upon immediately following a transaction.
pub struct AccruedSubstate {
    /// Self-destruct set. A set of accounts that will be discarded following the
    /// completion of the associated transaction.
    self_destruct_accounts: HashSet<Address>,
    /// Log series.
    log_series: Vec<Log>,
    /// Touched account set.
    ///
    /// Empty accounts in this set are deleted at the end of the transaction.
    touched_accounts: HashSet<Address>,
    /// Refund balance.
    refund_balance: U256,
    /// Accessed account set.
    accessed_accounts: HashSet<Address>,
    /// Accessed storage set.
    accessed_storage: HashSet<(Address, U256)>,
}

impl AccruedSubstate {
    /// Returns an empty accrued transaction substate.
    // TODO: Add precompile accounts to accessed account set.
    pub fn empty() -> Self {
        AccruedSubstate {
            self_destruct_accounts: HashSet::new(),
            log_series: Vec::new(),
            touched_accounts: HashSet::new(),
            refund_balance: U256::zero(),
            accessed_accounts: HashSet::new(),
            accessed_storage: HashSet::new(),
        }
    }

    /// Returns the self-destruct set.
    pub fn self_destruct_accounts(&self) -> &HashSet<Address> {
        &self.self_destruct_accounts
    }

    /// Returns the log series.
    pub fn log_series(&self) -> &Vec<Log> {
        &self.log_series
    }

    /// Returns the touched account set.
    pub fn touched_accounts(&self) -> &HashSet<Address> {
        &self.touched_accounts
    }

    /// Returns the refund balance.
    pub fn refund_balance(&self) -> U256 {
        self.refund_balance
    }

    /// Returns the accessed account set.
    pub fn accessed_accounts(&self) -> &HashSet<Address> {
        &self.accessed_accounts
    }

    /// Returns the accessed storage set.
    pub fn accessed_storage(&self) -> &HashSet<(Address, U256)> {
        &self.accessed_storage
    }
}
