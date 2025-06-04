use rocksdb::{DB, Options, ColumnFamilyDescriptor, ColumnFamily, BoundColumnFamily};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

const DATA_COLUMN_FAMILY_V1: &str = "version1";
const DATA_COLUMN_FAMILY_V2: &str = "version2";


pub struct Database {
    pub db: Arc<DB>,
    pub cf1: String,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        
        let cfs = vec![
            ColumnFamilyDescriptor::new(DATA_COLUMN_FAMILY_V1, Options::default()),
            ColumnFamilyDescriptor::new(DATA_COLUMN_FAMILY_V2, Options::default()),
        ];
        
        let db = Arc::new(DB::open_cf_descriptors(&opts, path, cfs)?);
        let cf1 = DATA_COLUMN_FAMILY_V1.to_string();

        Ok(Self {
            db,
            cf1,
        })
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), rocksdb::Error> {
        let cf1 = self.db.cf_handle(self.cf1.as_str()).unwrap();
        self.db.put_cf(&cf1, key, value)
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, rocksdb::Error> {
        let cf1 = self.db.cf_handle(self.cf1.as_str()).unwrap();

        self.db.get_cf(&cf1, key)
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use tempfile::TempDir;

    #[test]
    fn test_put_get() {
        let tmp_dir = TempDir::new().unwrap();
        let db_path = tmp_dir.path().to_str().unwrap();
        let db = Database::new(db_path).unwrap();

        let key = b"my_key";
        let value = b"my_value";

        // Test put
        db.put(key, value).unwrap();

        // Test get
        let retrieved_value = db.get(key).unwrap().unwrap();
        let retrieved_value_string = String::from_utf8(retrieved_value.clone()).unwrap();
        println!("Retrieved value: {}", retrieved_value_string);
        assert_eq!(retrieved_value, value);

        // Test get non-existent key
        let non_existent_key = b"non_existent_key";
        let retrieved_none = db.get(non_existent_key).unwrap();
        println!("Retrieved value for non-existent key: {:?}", retrieved_none);
        assert!(retrieved_none.is_none());
    }
}