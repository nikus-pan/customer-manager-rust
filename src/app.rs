use eframe::egui;
use crate::models::Customer;
use crate::db::Database;
use std::fs::File;
use std::io::Write;

pub struct CustomerApp {
    db: Database,
    customers: Vec<Customer>,
    new_name: String,
    new_email: String,
    new_phone: String,
    new_address: String,
    search_text: String,
    selected_customer_id: Option<i32>,
    is_editing: bool,
    status_msg: String,
    status_timer: f64,
    show_delete_confirm: bool,
}

impl CustomerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::setup_custom_fonts(&cc.egui_ctx);
        
        let db = Database::new();
        let customers = db.load_customers().unwrap_or_default();
        
        Self {
            db,
            customers,
            new_name: String::new(),
            new_email: String::new(),
            new_phone: String::new(),
            new_address: String::new(),
            search_text: String::new(),
            selected_customer_id: None,
            is_editing: false,
            status_msg: "系統就緒".to_string(),
            status_timer: 2.0,
            show_delete_confirm: false,
        }
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
        self.status_timer = 3.0;
    }

    fn refresh_data(&mut self) {
        if let Ok(data) = self.db.load_customers() {
            self.customers = data;
        }
    }

    fn reset_form(&mut self) {
        self.new_name.clear();
        self.new_email.clear();
        self.new_phone.clear();
        self.new_address.clear();
        self.is_editing = false;
        self.selected_customer_id = None;
    }

    fn export_to_csv(&mut self) {
        let mut file = match File::create("customers_export.csv") {
            Ok(f) => f,
            Err(e) => { self.show_status(&format!("❌ 無法建立檔案: {}", e)); return; }
        };

        // 寫入 UTF-8 BOM 以便 Excel 正確顯示中文
        file.write_all(&[0xEF, 0xBB, 0xBF]).ok();
        writeln!(file, "ID,姓名,電子郵件,聯絡電話,地址").ok();
        for c in &self.customers {
            writeln!(file, "{},{},{},{},{}", c.id, c.name, c.email, c.phone, c.address).ok();
        }
        self.show_status("✅ 資料已成功匯出至 customers_export.csv");
    }

    fn add_customer(&mut self) {
        if self.new_name.is_empty() { self.show_status("⚠️ 錯誤：姓名不能為空"); return; }
        let customer = Customer { id: 0, name: self.new_name.clone(), email: self.new_email.clone(), phone: self.new_phone.clone(), address: self.new_address.clone() };
        if self.db.add_customer(&customer).is_ok() {
            self.refresh_data(); self.reset_form(); self.show_status("✅ 成功新增客戶");
        }
    }

    fn update_customer(&mut self) {
        if let Some(id) = self.selected_customer_id {
            let customer = Customer { id, name: self.new_name.clone(), email: self.new_email.clone(), phone: self.new_phone.clone(), address: self.new_address.clone() };
            if self.db.update_customer(&customer).is_ok() {
                self.refresh_data(); self.reset_form(); self.show_status("✅ 成功更新客戶資料");
            }
        }
    }

    fn delete_customer(&mut self) {
        if let Some(id) = self.selected_customer_id {
            if self.db.delete_customer(id).is_ok() {
                self.refresh_data(); self.reset_form(); self.show_status("🗑️ 已刪除客戶資料");
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

        // 刪除確認視窗
        if self.show_delete_confirm {
            egui::Window::new("確認刪除")
                .collapsible(false).resizable(false).anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("您確定要永久刪除此客戶的所有資料嗎？此操作不可撤銷。");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("🗑️ 確認刪除").clicked() { self.delete_customer(); }
                        if ui.button("🚫 取消").clicked() { self.show_delete_confirm = false; }
                    });
                });
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading("📋 客戶關係管理系統");
                ui.add_space(20.0);
                if !self.status_msg.is_empty() {
                    ui.label(egui::RichText::new(&self.status_msg).color(egui::Color32::from_rgb(255, 0, 0)).strong());
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("📤 匯出 CSV").clicked() { self.export_to_csv(); }
                    if ui.button("🔄 刷新").clicked() { self.refresh_data(); self.show_status("🔄 資料已刷新"); }
                });
            });
            ui.add_space(8.0);
        });

        egui::SidePanel::left("form_panel").resizable(false).default_width(280.0).show(ctx, |ui| {
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.vertical_centered(|ui| { ui.heading(if self.is_editing { "📝 編輯客戶" } else { "➕ 新增客戶" }); });
                ui.add_space(10.0);
                ui.label("姓名："); ui.text_edit_singleline(&mut self.new_name);
                ui.label("電子郵件："); ui.text_edit_singleline(&mut self.new_email);
                ui.label("聯絡電話："); ui.text_edit_singleline(&mut self.new_phone);
                ui.label("通訊地址："); ui.add(egui::TextEdit::multiline(&mut self.new_address).desired_rows(3).desired_width(ui.available_width()));
                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    if self.is_editing {
                        if ui.button("💾 儲存修改").clicked() { self.update_customer(); }
                        if ui.button("🚫 取消").clicked() { self.reset_form(); self.show_status("已取消編輯"); }
                    } else {
                        if ui.button("✨ 點擊新增").clicked() { self.add_customer(); }
                    }
                });
            });
            ui.add_space(20.0); ui.separator(); ui.add_space(10.0);
            ui.label("🔍 快速搜尋："); ui.text_edit_singleline(&mut self.search_text);
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(format!("總計：{} 位客戶", self.customers.len()));
                if let Some(_id) = self.selected_customer_id {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🗑️ 刪除選取項目").clicked() { self.show_delete_confirm = true; }
                    });
                }
            });
            ui.add_space(5.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("👥 客戶清單");
            ui.add_space(5.0);
            
            let search = self.search_text.to_lowercase();
            let filtered_data: Vec<Customer> = self.customers.iter()
                .filter(|c| search.is_empty() || c.name.to_lowercase().contains(&search) || c.email.to_lowercase().contains(&search))
                .cloned().collect();

            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                for customer in filtered_data {
                    let id = customer.id;
                    let is_selected = self.selected_customer_id == Some(id);
                    let frame = egui::Frame::group(ui.style())
                        .fill(if is_selected { ui.visuals().selection.bg_fill.linear_multiply(0.1) } else { ui.visuals().widgets.noninteractive.bg_fill })
                        .stroke(if is_selected { ui.visuals().selection.stroke } else { ui.visuals().widgets.noninteractive.bg_stroke })
                        .rounding(5.0).inner_margin(10.0);

                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.strong(format!("👤 {}", customer.name));
                                ui.label(format!("📧 {}", customer.email));
                                ui.label(format!("📞 {}", customer.phone));
                                ui.label(format!("📍 {}", customer.address));
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("✏️ 編輯").clicked() {
                                    self.selected_customer_id = Some(id);
                                    self.new_name = customer.name.clone(); self.new_email = customer.email.clone();
                                    self.new_phone = customer.phone.clone(); self.new_address = customer.address.clone();
                                    self.is_editing = true; self.show_status(&format!("正在編輯：{}", customer.name)); ctx.request_repaint();
                                }
                                let card_rect = ui.available_rect_before_wrap();
                                if ui.interact(card_rect, ui.id().with(id), egui::Sense::click()).clicked() {
                                    if !self.is_editing { self.selected_customer_id = Some(id); }
                                }
                            });
                        });
                    });
                    ui.add_space(8.0);
                }
            });
        });
    }
}
