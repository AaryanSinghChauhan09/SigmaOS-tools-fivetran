#![forbid(unsafe_code)]
#![allow(non_snake_case)]
//! Fivetran Connector Sync Engine for SigmaOS
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct FivetranRecord {
    pub id: String,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncBatchResult {
    pub synced_count: usize,
    pub rejected_count: usize,
}

pub struct FivetranConnector {
    pub destination_table: String,
    pub required_fields: Vec<String>,
}

impl FivetranConnector {
    pub fn new(destination_table: &str, required_fields: Vec<&str>) -> Self {
        Self {
            destination_table: destination_table.to_string(),
            required_fields: required_fields.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn sync_batch(&self, records: Vec<FivetranRecord>) -> SyncBatchResult {
        let mut synced = 0;
        let mut rejected = 0;

        for r in records {
            let mut valid = true;
            for req in &self.required_fields {
                if !r.fields.contains_key(req) {
                    valid = false;
                    break;
                }
            }
            if valid {
                synced += 1;
            } else {
                rejected += 1;
            }
        }

        SyncBatchResult {
            synced_count: synced,
            rejected_count: rejected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fivetran_schema_sync_batch_aaa() {
        // Arrange
        let connector = FivetranConnector::new("customers", vec!["id", "email"]);
        let mut r1_fields = HashMap::new();
        r1_fields.insert("id".to_string(), "C001".to_string());
        r1_fields.insert("email".to_string(), "c@sigmaos.org".to_string());

        let mut r2_fields = HashMap::new();
        r2_fields.insert("id".to_string(), "C002".to_string()); // Missing email!

        let records = vec![
            FivetranRecord {
                id: "1".into(),
                fields: r1_fields,
            },
            FivetranRecord {
                id: "2".into(),
                fields: r2_fields,
            },
        ];

        // Act
        let result = connector.sync_batch(records);

        // Assert
        assert_eq!(result.synced_count, 1);
        assert_eq!(result.rejected_count, 1);
    }
}

#[cfg(test)]
mod fivetran_extra_tests {
    use super::*;

    #[test]
    fn test_fivetran_all_valid_records_batch_aaa() {
        // Arrange
        let connector = FivetranConnector::new("events", vec!["id", "type"]);
        let mut f1 = HashMap::new();
        f1.insert("id".into(), "1".into());
        f1.insert("type".into(), "click".into());
        let records = vec![FivetranRecord { id: "e1".into(), fields: f1 }];
        // Act
        let result = connector.sync_batch(records);
        // Assert
        assert_eq!(result.synced_count, 1);
        assert_eq!(result.rejected_count, 0);
    }

    #[test]
    fn test_fivetran_empty_batch_aaa() {
        // Arrange
        let connector = FivetranConnector::new("events", vec!["id"]);
        // Act
        let result = connector.sync_batch(Vec::new());
        // Assert
        assert_eq!(result.synced_count, 0);
        assert_eq!(result.rejected_count, 0);
    }

    #[test]
    fn test_fivetran_destination_table_name_aaa() {
        // Arrange & Act
        let connector = FivetranConnector::new("analytics_users", vec![]);
        // Assert
        assert_eq!(connector.destination_table, "analytics_users");
    }
}
