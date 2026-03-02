use eframe::egui;
use crate::models::Customer;
use crate::db::Database;
use std::fs::File;
use std::io::Write;

#[derive(PartialEq)]
enum SortOrder { Name, Time, Tags }

pub struct CustomerApp {
    db: Database,
    customers: Vec<Customer>,
    new_name: String,
    new_email: String,
    new_phone: String,
    new_address: String,
    new_tags: String,
    search_text: String,
    selected_customer_id: Option<i32>,
    is_editing: bool,
    status_msg: String,
    status_timer: f64,
    show_delete_confirm: bool,
    sort_order: SortOrder,
}

impl CustomerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::setup_custom_fonts(&cc.egui_ctx);
        Self::setup_theme(&cc.egui_ctx);
        
        let db = Database::new();
        let customers = db.load_customers().unwrap_or_default();
        
        Self {
            db,
            customers,
            new_name: String::new(),
            new_email: String::new(),
            new_phone: String::new(),
            new_address: String::new(),
            new_tags: String::new(),
            search_text: String::new(),
            selected_customer_id: None,
            is_editing: false,
            status_msg: "✨ 系統已就緒".to_string(),
            status_timer: 3.0,
            show_delete_confirm: false,
            sort_order: SortOrder::Time,
        }
    }

    fn setup_theme(ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 35);
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 70));
        visuals.selection.bg_fill = egui::Color32::from_rgb(0, 120, 215).linear_multiply(0.4);
        visuals.window_fill = egui::Color32::from_rgb(20, 20, 25);
        ctx.set_visuals(visuals);
    }

    fn setup_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        let font_path = "C:\\Windows\\Fonts\\msjh.ttc";
        if let Ok(font_data) = std::fs::read(font_path) {
            fonts.font_data.insert("my_font".to_owned(), egui::FontData::from_owned(font_data));
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("my_font".to_owned());
            ctx.set_fonts(fonts);
        }
    }

    fn show_status(&mut self, msg: &str) {
        self.status_msg = msg.to_string();
        self.status_timer = 5.0;
    }

    fn refresh_data(&mut self) {
        if let Ok(data) = self.db.load_customers() {
            self.customers = data;
        }
    }

    fn reset_form(&mut self) {
        self.new_name.clear(); self.new_email.clear(); self.new_phone.clear(); self.new_address.clear(); self.new_tags.clear();
        self.is_editing = false; self.selected_customer_id = None;
    }

    fn export_to_csv(&mut self) {
        let mut file = match File::create("customers_export.csv") {
            Ok(f) => f,
            Err(e) => { self.show_status(&format!("❌ 匯出失敗: {}", e)); return; }
        };
        file.write_all(&[0xEF, 0xBB, 0xBF]).ok();
        writeln!(file, "ID,姓名,電子郵件,聯絡電話,地址,標籤,建立時間").ok();
        for c in &self.customers {
            writeln!(file, "{},{},{},{},{},{},{}", c.id, c.name, c.email, c.phone, c.address, c.tags, c.created_at).ok();
        }
        self.show_status("📂 已匯出至 customers_export.csv");
    }

    fn add_customer(&mut self) {
        if self.new_name.is_empty() { self.show_status("⚠️ 姓名不能為空"); return; }
        let customer = Customer { id: 0, name: self.new_name.clone(), email: self.new_email.clone(), phone: self.new_phone.clone(), address: self.new_address.clone(), tags: self.new_tags.clone(), created_at: "".to_string(), updated_at: "".to_string() };
        match self.db.add_customer(&customer) {
            Ok(_) => { self.refresh_data(); self.reset_form(); self.show_status("✅ 成功新增客戶"); }
            Err(_) => self.show_status("❌ 錯誤：Email 可能重複"),
        }
    }

    fn update_customer(&mut self) {
        if let Some(id) = self.selected_customer_id {
            let customer = Customer { id, name: self.new_name.clone(), email: self.new_email.clone(), phone: self.new_phone.clone(), address: self.new_address.clone(), tags: self.new_tags.clone(), created_at: "".to_string(), updated_at: "".to_string() };
            if self.db.update_customer(&customer).is_ok() {
                self.refresh_data(); self.reset_form(); self.show_status("✅ 資料更新成功");
            }
        }
    }

    fn delete_customer(&mut self) {
        if let Some(id) = self.selected_customer_id {
            if self.db.delete_customer(id).is_ok() {
                self.refresh_data(); self.reset_form(); self.show_status("🗑️ 客戶資料已刪除");
            }
        }
        self.show_delete_confirm = false;
    }
}

impl eframe::App for CustomerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.status_timer > 0.0 {
            self.status_timer -= ctx.input(|i| i.stable_dt) as f64;
            if self.status_timer <= 0.0 { self.status_msg.clear(); }
            ctx.request_repaint();
        }

        if self.show_delete_confirm {
            egui::Window::new("🛑 確認永久刪除").collapsible(false).resizable(false).anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("您確定要永久刪除此客戶資料嗎？");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        // 刪除按鈕：背景黃色，文字紅色
                        let confirm_delete = egui::Button::new(egui::RichText::new("🗑️ 確定刪除").color(egui::Color32::RED).strong())
                            .fill(egui::Color32::YELLOW);
                        if ui.add(confirm_delete).clicked() { self.delete_customer(); }
                        if ui.button("🚫 取消").clicked() { self.show_delete_confirm = false; }
                    });
                });
        }

        egui::TopBottomPanel::top("header").frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 20)).inner_margin(10.0)).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("💎 企業客戶管理系統").color(egui::Color32::WHITE).strong());
                ui.add_space(20.0);
                if !self.status_msg.is_empty() {
                    ui.label(egui::RichText::new(&self.status_msg).color(egui::Color32::from_rgb(0, 255, 150)).strong());
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("📤 匯出 CSV").clicked() { self.export_to_csv(); }
                    if ui.button("🔄 刷新").clicked() { self.refresh_data(); self.show_status("🔄 資料已刷新"); }
                });
            });
        });

        egui::SidePanel::left("sidebar").resizable(false).default_width(300.0).show(ctx, |ui| {
            ui.add_space(15.0);
            ui.vertical_centered(|ui| { ui.heading(if self.is_editing { "📝 編輯客戶" } else { "➕ 新增客戶" }); });
            ui.add_space(15.0);
            ui.group(|ui| {
                ui.label("姓名："); ui.text_edit_singleline(&mut self.new_name);
                ui.label("電子郵件："); ui.text_edit_singleline(&mut self.new_email);
                ui.label("聯絡電話："); ui.text_edit_singleline(&mut self.new_phone);
                ui.label("標籤 (逗號分隔)："); ui.text_edit_singleline(&mut self.new_tags);
                ui.label("通訊地址："); ui.add(egui::TextEdit::multiline(&mut self.new_address).desired_rows(2).desired_width(ui.available_width()));
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    if self.is_editing {
                        if ui.button(egui::RichText::new("💾 儲存修改").color(egui::Color32::LIGHT_BLUE)).clicked() { self.update_customer(); }
                        if ui.button("🚫 取消").clicked() { self.reset_form(); }
                    } else {
                        if ui.button("✨ 立即新增").clicked() { self.add_customer(); }
                    }
                });
            });
            ui.add_space(30.0); ui.separator();
            ui.label("🔍 搜尋："); ui.text_edit_singleline(&mut self.search_text);
            ui.add_space(15.0);
            ui.label("🔃 排序：");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.sort_order, SortOrder::Time, "時間");
                ui.selectable_value(&mut self.sort_order, SortOrder::Name, "姓名");
                ui.selectable_value(&mut self.sort_order, SortOrder::Tags, "標籤");
            });
        });

        egui::TopBottomPanel::bottom("footer").frame(egui::Frame::none().fill(egui::Color32::WHITE).inner_margin(5.0)).show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::BLUE);
            ui.horizontal(|ui| {
                ui.label(format!("👥 總計：{} 位客戶", self.customers.len()));
                if let Some(_) = self.selected_customer_id {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 刪除按鈕：背景黃色，文字紅色
                        let del_btn = egui::Button::new(egui::RichText::new("🗑️ 刪除選取").color(egui::Color32::RED).strong())
                            .fill(egui::Color32::YELLOW);
                        if ui.add(del_btn).clicked() { self.show_delete_confirm = true; }
                    });
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let search = self.search_text.to_lowercase();
            let mut filtered: Vec<Customer> = self.customers.iter()
                .filter(|c| search.is_empty() || c.name.to_lowercase().contains(&search) || c.email.to_lowercase().contains(&search) || c.tags.to_lowercase().contains(&search))
                .cloned().collect();
            match self.sort_order {
                SortOrder::Name => filtered.sort_by(|a, b| a.name.cmp(&b.name)),
                SortOrder::Time => filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
                SortOrder::Tags => filtered.sort_by(|a, b| a.tags.cmp(&b.tags)),
            }
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                for customer in filtered {
                    let id = customer.id;
                    let is_selected = self.selected_customer_id == Some(id);
                    let frame = egui::Frame::none()
                        .fill(if is_selected { egui::Color32::from_rgb(0, 80, 150) } else { egui::Color32::from_rgb(30, 30, 35) })
                        .stroke(egui::Stroke::new(if is_selected { 2.5 } else { 1.0 }, if is_selected { egui::Color32::from_rgb(0, 191, 255) } else { egui::Color32::from_rgb(50, 50, 60) }))
                        .rounding(10.0).inner_margin(15.0);
                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.strong(egui::RichText::new(format!("👤 {}", customer.name)).size(18.0).color(egui::Color32::WHITE));
                                    for tag in customer.tags.split(',').filter(|s| !s.trim().is_empty()) {
                                        egui::Frame::none().fill(egui::Color32::from_rgb(0, 60, 120)).rounding(4.0).inner_margin(egui::Margin::symmetric(6.0, 2.0)).show(ui, |ui| {
                                            ui.label(egui::RichText::new(tag.trim()).size(12.0).color(egui::Color32::LIGHT_BLUE).strong());
                                        });
                                        ui.add_space(4.0);
                                    }
                                });
                                ui.label(egui::RichText::new(format!("📧 {}  •  📞 {}", customer.email, customer.phone)).color(egui::Color32::LIGHT_GRAY));
                                ui.label(egui::RichText::new(format!("📍 {}", customer.address)).color(egui::Color32::GRAY));
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // 編輯按鈕：背景淺黃色，文字亮藍色
                                let edit_btn = egui::Button::new(egui::RichText::new("✏️ 編輯").color(egui::Color32::from_rgb(0, 191, 255)).strong())
                                    .fill(egui::Color32::from_rgb(255, 255, 224));
                                if ui.add(edit_btn).clicked() {
                                    self.selected_customer_id = Some(id);
                                    self.new_name = customer.name.clone(); self.new_email = customer.email.clone();
                                    self.new_phone = customer.phone.clone(); self.new_address = customer.address.clone();
                                    self.new_tags = customer.tags.clone();
                                    self.is_editing = true; self.show_status(&format!("正在編輯：{}", customer.name));
                                    ctx.request_repaint();
                                }
                                let card_rect = ui.available_rect_before_wrap();
                                if ui.interact(card_rect, ui.id().with(id), egui::Sense::click()).clicked() {
                                    if !self.is_editing { self.selected_customer_id = Some(id); self.show_status(&format!("已選取：{}", customer.name)); }
                                }
                            });
                        });
                    });
                    ui.add_space(10.0);
                }
            });
        });
    }
}
