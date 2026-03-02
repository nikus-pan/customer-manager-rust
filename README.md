# 📋 客戶關係管理系統 (Customer Manager Rust)

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://www.microsoft.com/windows)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

這是一個使用 Rust 語言開發的高性能、輕量級桌面客戶管理系統 (CRM)。採用 `egui` 即時模式 GUI 框架開發，具備簡約直觀的繁體中文介面，並使用 SQLite 作為嵌入式資料庫。

## ✨ 功能亮點

- **🚀 高性能渲染**: 基於 `egui` 的即時模式渲染，介面流暢無卡頓。
- **📦 零配置依賴**: 使用 SQLite 嵌入式資料庫，程式啟動即可使用，無需安裝額外資料庫服務。
- **🖋️ 完美中文顯示**: 自動載入系統微軟正黑體，解決 Rust 桌面程式常見的中文字型問題。
- **🛡️ 資料安全**: 
    - 刪除操作二次確認彈窗。
    - 資料自動儲存於使用者 AppData 目錄，更新程式不丟失資料。
- **📊 資料交互**: 支援一鍵匯出符合 Excel 標準的 UTF-8 CSV 報表。
- **🔍 快速搜尋**: 即時過濾姓名與電子郵件。

## 🛠️ 技術棧

- **GUI**: [eframe/egui](https://github.com/emilk/egui)
- **Database**: [rusqlite](https://github.com/rusqlite/rusqlite) (SQLite)
- **Storage**: `directories` (符合作業系統規範的資料路徑)
- **CI/CD**: `cargo-dist` (自動化打包與 GitHub Release)

## 📥 安裝與使用

### 下載安裝
請前往 [Releases](https://github.com/nikus-pan/customer-manager-rust/releases) 頁面下載最新版本的 `CustomerManager_Setup.exe` 一鍵安裝包。

### 開發者編譯
如果您想自行編譯，請確保已安裝 [Rust](https://www.rust-lang.org/learn/get-started)：

```bash
# 複製專案
git clone https://github.com/nikus-pan/customer-manager-rust.git
cd customer-manager-rust

# 執行程式
cargo run --release
```

## 📖 操作指引

1. **新增客戶**: 在左側面板填寫資料，點擊「✨ 點擊新增」。
2. **編輯資料**: 點擊清單卡片右側的「✏️ 編輯」，修改後點擊「💾 儲存修改」。
3. **刪除客戶**: 選取卡片後，點擊右下角「🗑️ 刪除選取項目」，並在彈窗中確認。
4. **資料匯出**: 點擊右上角「📤 匯出 CSV」即可在程式目錄產生報表。

## 📄 授權協議

本專案採用 [MIT License](LICENSE) 授權。
