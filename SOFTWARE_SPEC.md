# 客戶關係管理系統 (Customer Manager Rust) - 軟體說明書

## 1. 系統概述
本系統是一款基於 Rust 語言開發的輕量級桌面應用程式，旨在提供高效、穩定的客戶資料管理功能。系統採用即時模式 GUI 框架，並配合嵌入式資料庫，確保流暢的使用體驗與資料安全性。

## 2. 技術架構
*   **程式語言**: Rust (Edition 2021)
*   **GUI 框架**: `eframe` / `egui` (Immediate Mode GUI)
*   **資料庫**: `SQLite` (透過 `rusqlite` 驅動，使用 `bundled` 特性封裝)
*   **序列化**: `serde`
*   **日期處理**: `chrono`

## 3. 模組化設計
系統程式碼分為以下核心模組：
*   **`models.rs`**: 定義 `Customer` 資料模型與欄位。
*   **`db.rs`**: 資料庫持久層封裝，處理 SQL 語句與連線管理。
*   **`app.rs`**: UI 邏輯層，包含狀態管理、事件處理與組件佈局。
*   **`main.rs`**: 應用程式入口，配置視窗參數與初始化字體。

## 4. 資料庫設計
資料庫檔案名稱：`customer.db`
資料表結構：`customers`
| 欄位名 | 類型 | 說明 |
| :--- | :--- | :--- |
| id | INTEGER | 主鍵，自動遞增 |
| name | TEXT | 客戶姓名 (必填) |
| email | TEXT | 電子郵件 |
| phone | TEXT | 聯絡電話 |
| address | TEXT | 通訊地址 |

## 5. 關鍵技術實現
*   **中文字體載入**: 系統啟動時會自動偵測並載入 Windows 系統中的 `msjh.ttc` (微軟正黑體)，解決 Rust GUI 常見的中文亂碼問題。
*   **借用檢查優化**: 在 UI 渲染過程中，透過資料分離技術避免了 Rust 的 `Borrow Checker` 衝突，確保狀態更新的安全。
