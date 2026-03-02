use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub tags: String,       // 以逗號分隔的標籤
    pub created_at: String, // 建立時間
    pub updated_at: String, // 最後修改時間
}
