use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub timestamp: DateTime<Utc>,
    pub status: TransactionStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

impl Transaction {
    pub fn new(amount: f64, currency: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            amount,
            currency,
            timestamp: Utc::now(),
            status: TransactionStatus::Pending,
        }
    }

    pub fn validate(&self) -> bool {
        if self.amount <= 0.0 {
            return false;
        }
        matches!(self.currency.as_str(), "USD" | "THB")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction() {
        let transaction = Transaction::new(100.0, "THB".to_string());
        assert_eq!(transaction.amount, 100.0);
        assert_eq!(transaction.currency, "THB");
        matches!(transaction.status, TransactionStatus::Pending);
    }

    #[test]
    fn test_validate_valid_transaction() {
        let transaction = Transaction::new(100.0, "THB".to_string());
        assert!(transaction.validate());

        let transaction_eur = Transaction::new(5.0, "USD".to_string());
        assert!(transaction_eur.validate());
    }

    #[test]
    fn test_validate_invalid_amount() {
        let transaction = Transaction::new(0.0, "THB".to_string());
        assert!(!transaction.validate());

        let transaction_negative = Transaction::new(-10.0, "THB".to_string());
        assert!(!transaction_negative.validate());
    }

    #[test]
    fn test_validate_invalid_currency() {
        let transaction = Transaction::new(100.0, "JPY".to_string());
        assert!(!transaction.validate());

        let transaction_invalid = Transaction::new(100.0, "INVALID".to_string());
        assert!(!transaction_invalid.validate());
    }
}