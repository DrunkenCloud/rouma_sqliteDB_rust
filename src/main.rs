use std::fs::File;
use std::io::{self, Write};
use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Table {
    name: String,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[derive(Debug)]
struct Database {
    tables: Vec<Table>,
}

impl Database {
    fn new() -> Self {
        Database { tables: Vec::new() }
    }

    fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Write the header
        bytes.extend_from_slice(b".rouma");

        // Placeholder for the hash
        bytes.extend_from_slice(&[0u8; 10]);

        for table in &self.tables {
            // Write number of rows (2 bytes)
            let num_rows = table.rows.len() as u16;
            bytes.extend_from_slice(&num_rows.to_be_bytes());

            // Write table structure
            bytes.extend_from_slice(table.name.as_bytes());
            bytes.push(0); // Null terminator for the table name

            for column in &table.columns {
                bytes.extend_from_slice(column.as_bytes());
                bytes.push(0); // Null terminator for each column name
            }

            // End of table structure marker
            bytes.extend_from_slice(&[0x00, 0x01, 0x02]);

            // Write table rows
            for row in &table.rows {
                for cell in row {
                    bytes.extend_from_slice(cell.as_bytes());
                    bytes.push(0); // Null terminator for each cell
                }
            }
        }

        // Calculate the hash of the data (excluding the header)
        let mut hasher = Sha256::new();
        hasher.update(&bytes[16..]); // Skip the header and hash placeholder
        let hash_result = hasher.finalize();

        // Insert the hash into the placeholder
        let hash_bytes = &hash_result[..10];
        bytes.splice(6..16, hash_bytes.iter().cloned());

        bytes
    }

    fn write_to_file(&self, filename: &str) -> io::Result<()> {
        let bytes = self.to_bytes();
        let mut file = File::create(filename)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}
fn main() -> io::Result<()> {
    // Create a sample database
    let mut db = Database::new();

    // Create a sample table
    let table = Table {
        name: "users".to_string(),
        columns: vec!["id".to_string(), "name".to_string(), "email".to_string()],
        rows: vec![
            vec!["1".to_string(), "user1".to_string(), "user1@example.com".to_string()],
            vec!["2".to_string(), "user2".to_string(), "user2@example.com".to_string()],
        ],
    };

    db.add_table(table);

    // Write the database to a file
    db.write_to_file("database.db")?;

    Ok(())
}
