use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum TransactionError {
    #[error("A write conflict occured")]
    Conflict,
    #[error("The transaction with id `{0}` does not exist")]
    NotFound(usize),
    #[error("The transaction with id `{0}` already exists")]
    AlreadyExists(usize),
}

#[derive(Default)]
struct Transaction {
    updated_pairs: HashMap<String, String>,
    new_pairs: HashMap<String, String>,
}

impl Transaction {
    fn new() -> Self {
        Self::default()
    }

    fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> Option<String> {
        self.new_pairs.insert(key.into(), value.into())
    }

    fn get(&self, key: &String) -> Option<&String> {
        self.new_pairs.get(key).or(self.updated_pairs.get(key))
    }
}

#[derive(Default)]
pub struct Database {
    store: HashMap<String, String>,
    transactions: HashMap<usize, Transaction>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: String) {
        if let Some(prev_value) = self.store.insert(key.clone(), value) {
            for (_, t) in self.transactions.iter_mut() {
                t.updated_pairs.insert(key.clone(), prev_value.clone());
            }
        }
    }

    pub fn get(&self, key: &String) -> Option<String> {
        self.store.get(key.into()).cloned()
    }

    pub fn begin_tx_with_id(&mut self, tx_id: usize) -> Result<usize, TransactionError> {
        if self.transactions.contains_key(&tx_id) {
            return Err(TransactionError::AlreadyExists(tx_id));
        }

        self.transactions.insert(tx_id, Transaction::new());
        Ok(tx_id)
    }

    pub fn set_tx(
        &mut self,
        key: String,
        value: String,
        tx_id: usize,
    ) -> Result<Option<String>, TransactionError> {
        Ok(self
            .transactions
            .get_mut(&tx_id)
            .ok_or(TransactionError::NotFound(tx_id))?
            .set(key, value))
    }

    pub fn get_tx(&self, key: &String, tx_id: usize) -> Result<Option<String>, TransactionError> {
        Ok(self
            .transactions
            .get(&tx_id)
            .ok_or(TransactionError::NotFound(tx_id))?
            .get(key)
            .or(self.get(key).as_ref())
            .cloned())
    }

    pub fn commit_tx(&mut self, tx_id: usize) -> Result<bool, TransactionError> {
        let tx = self
            .transactions
            .get(&tx_id)
            .ok_or(TransactionError::NotFound(tx_id))?;

        // Check if any of the keys we want to update have been changed
        if tx.new_pairs.iter().any(|(key, value)| {
            tx.updated_pairs.contains_key(key) && tx.updated_pairs.get(key).unwrap() != value
        }) {
            // Do not care about the result of the abort
            let _ = self.abort_tx(tx_id);
            return Err(TransactionError::Conflict)?;
        }

        let updates = tx
            .new_pairs
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<Vec<_>>();

        for (key, value) in updates {
            self.set(key, value);
        }

        let _ = self.transactions.remove(&tx_id);
        Ok(true)
    }

    pub fn abort_tx(&mut self, tx_id: usize) -> Result<bool, TransactionError> {
        Ok(self
            .transactions
            .remove(&tx_id)
            .ok_or(TransactionError::NotFound(tx_id))
            .map(|_| true)
            .unwrap())
    }
}
