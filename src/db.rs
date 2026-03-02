use rusqlite::{Connection, Result, params};
use std::path::PathBuf;
use crate::models::Customer;

pub struct Database {
    db_path: PathBuf,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Self {
        let db = Self { db_path };
        db.init().expect("無法初始化資料庫");
        db
    }

    fn init(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS customers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                phone TEXT,
                address TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn load_customers(&self) -> Result<Vec<Customer>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT id, name, email, phone, address FROM customers")?;
        let customer_iter = stmt.query_map([], |row| {
            Ok(Customer {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                phone: row.get(3)?,
                address: row.get(4)?,
            })
        })?;

        let mut customers = Vec::new();
        for customer in customer_iter {
            customers.push(customer?);
        }
        Ok(customers)
    }

    pub fn add_customer(&self, customer: &Customer) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO customers (name, email, phone, address) VALUES (?, ?, ?, ?)",
            params![customer.name, customer.email, customer.phone, customer.address],
        )?;
        Ok(())
    }

    pub fn delete_customer(&self, id: i32) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM customers WHERE id = ?", params![id])?;
        Ok(())
    }
    
    pub fn update_customer(&self, customer: &Customer) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "UPDATE customers SET name = ?, email = ?, phone = ?, address = ? WHERE id = ?",
            params![customer.name, customer.email, customer.phone, customer.address, customer.id],
        )?;
        Ok(())
    }
}
