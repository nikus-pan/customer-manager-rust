use rusqlite::{Connection, Result, params};
use std::path::PathBuf;
use std::fs;
use crate::models::Customer;
use directories::ProjectDirs;
use chrono::Local;

pub struct Database {
    db_path: PathBuf,
}

impl Database {
    pub fn new() -> Self {
        let db_dir = if let Some(proj_dirs) = ProjectDirs::from("com", "nikuswork", "customermanager") {
            let config_dir = proj_dirs.config_dir();
            fs::create_dir_all(config_dir).ok();
            config_dir.to_path_buf()
        } else {
            PathBuf::from(".")
        };

        let db_path = db_dir.join("customer.db");
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
                email TEXT NOT NULL UNIQUE,
                phone TEXT,
                address TEXT,
                tags TEXT DEFAULT '',
                created_at TEXT,
                updated_at TEXT
            )",
            [],
        )?;
        
        // 確保舊版本資料庫升級 (Migration)
        // 嘗試加入新欄位，若已存在會報錯，我們忽略它
        let _ = conn.execute("ALTER TABLE customers ADD COLUMN tags TEXT DEFAULT ''", []);
        let _ = conn.execute("ALTER TABLE customers ADD COLUMN created_at TEXT", []);
        let _ = conn.execute("ALTER TABLE customers ADD COLUMN updated_at TEXT", []);
        
        Ok(())
    }

    pub fn load_customers(&self) -> Result<Vec<Customer>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT id, name, email, phone, address, tags, created_at, updated_at FROM customers")?;
        let customer_iter = stmt.query_map([], |row| {
            Ok(Customer {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                phone: row.get(3)?,
                address: row.get(4)?,
                tags: row.get(5).unwrap_or_default(),
                created_at: row.get(6).unwrap_or_default(),
                updated_at: row.get(7).unwrap_or_default(),
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
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO customers (name, email, phone, address, tags, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![customer.name, customer.email, customer.phone, customer.address, customer.tags, now, now],
        )?;
        Ok(())
    }

    pub fn update_customer(&self, customer: &Customer) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "UPDATE customers SET name = ?, email = ?, phone = ?, address = ?, tags = ?, updated_at = ? WHERE id = ?",
            params![customer.name, customer.email, customer.phone, customer.address, customer.tags, now, customer.id],
        )?;
        Ok(())
    }

    pub fn delete_customer(&self, id: i32) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM customers WHERE id = ?", params![id])?;
        Ok(())
    }
}
