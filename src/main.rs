mod models;
mod db;
mod app;

use app::CustomerApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("客戶關係管理系統"),
        ..Default::default()
    };

    eframe::run_native(
        "customer_manager",
        options,
        Box::new(|cc| {
            // 在這裡可以進行更多的環境設定，例如字體載入
            Ok(Box::new(CustomerApp::new(cc)))
        }),
    )
}
