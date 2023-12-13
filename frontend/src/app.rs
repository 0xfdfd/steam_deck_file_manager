#[derive(Debug, Clone)]
pub struct WebUiConfig {
    pub host: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum WebUiMessage {
    /// Set font.
    ActFont((String, Vec<u8>)),
    /// Set the home directory and refresh.
    ActCWD(String),
    /// Set the home directory.
    SetHomeDir(String),
    /// Set current working directory.
    SetCWD(String),
    /// Set file list.
    SetFileList(crate::protocol::ReaddirResponse),
}

pub struct WebUI {
    inited: bool,
    homedir: Option<String>,
    cwd: Option<String>,
    filelist: Option<crate::protocol::ReaddirResponse>,

    client: crate::http_client::HttpClient,
    tx: std::sync::Arc<std::sync::mpsc::Sender<WebUiMessage>>,
    rx: std::sync::mpsc::Receiver<WebUiMessage>,
}

impl WebUI {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: WebUiConfig) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let (tx, rx) = std::sync::mpsc::channel::<WebUiMessage>();

        let client = crate::http_client::new(config.host.as_str());
        let ui = WebUI {
            inited: false,
            homedir: None,
            cwd: None,
            filelist: None,

            client: client,
            tx: std::sync::Arc::new(tx),
            rx: rx,
        };

        return ui;
    }
}

impl eframe::App for WebUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Initialize data.
        if self.inited == false {
            self.init(ctx);
            self.inited = true;
        }

        // Handle data update.
        // Due to update is trigger at redraw, you need to call [`egui::Context::request_repaint`] to
        // ensure the update function is executed in time.
        if let Ok(msg) = self.rx.try_recv() {
            self.updated(ctx, msg);
        }

        // Update view.
        self.view(ctx);
    }
}

impl WebUI {
    fn init(&self, ctx: &egui::Context) {
        // Fetch font.
        {
            let font_name = "LXGWWenKai-Regular.ttf";
            let font_name_path = format!("/assets/{}", font_name);

            let ctx = ctx.clone();
            let tx = self.tx.clone();
            self.client.get(font_name_path.as_str(), move |data| {
                let data = data.unwrap();
                tx.send(WebUiMessage::ActFont((font_name.to_string(), data)))
                    .unwrap();
                ctx.request_repaint();
            });
        }

        // Fetch homedir
        {
            let tx = self.tx.clone();
            let ctx = ctx.clone();
            self.client.post(
                crate::protocol::DirsRequest {
                    kind: crate::protocol::DirsRequestKind::HomeDir,
                },
                move |rsp: Result<crate::protocol::DirsResponse, String>| {
                    let rsp = rsp.unwrap();
                    let homedir = rsp.path.unwrap();

                    tx.send(WebUiMessage::SetHomeDir(homedir.clone())).unwrap();
                    tx.send(WebUiMessage::ActCWD(homedir.clone())).unwrap();
                    ctx.request_repaint();
                },
            );
        }
    }

    fn updated(&mut self, ctx: &egui::Context, msg: WebUiMessage) {
        match msg {
            WebUiMessage::ActFont((name, data)) => {
                self.install_font(ctx, name, data);
            }
            WebUiMessage::ActCWD(path) => {
                self.cd(ctx, path.as_str());
            }
            WebUiMessage::SetHomeDir(path) => {
                self.homedir = Some(path);
            }
            WebUiMessage::SetCWD(path) => {
                self.cwd = Some(path);
            }
            WebUiMessage::SetFileList(filelist) => {
                self.filelist = Some(filelist);
            }
        }
    }

    fn view(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.view_top_panel(ctx, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.view_body_panel(ctx, ui);
        });

        egui::TopBottomPanel::bottom("bottem_panel").show(ctx, |ui| {
            self.view_bottom_panel(ctx, ui);
        });
    }

    fn view_top_panel(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Switch between dark and light mode.
            egui::widgets::global_dark_light_mode_buttons(ui);

            // Home.
            {
                let btn = egui::Button::new("🏠");
                let rsp = ui.add(btn).on_hover_text("Goto Home");
                if rsp.clicked() {
                    if let Some(path) = &self.homedir {
                        self.cd(ctx, path.as_str());
                    }
                }
            }

            // Refresh.
            {
                let btn = egui::Button::new("🔃");
                let rsp = ui.add(btn).on_hover_text("Refresh");
                if rsp.clicked() {
                    self.refresh(ctx);
                }
            }

            if let Some(cwd) = &self.cwd {
                ui.label(cwd);
            }
        });
    }

    fn view_bottom_panel(&self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    }

    fn view_body_panel(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(egui_extras::Column::exact(8.0).resizable(false))
            .column(egui_extras::Column::remainder().resizable(false))
            .column(egui_extras::Column::exact(64.0).resizable(false))
            .column(egui_extras::Column::exact(64.0).resizable(false))
            .column(egui_extras::Column::exact(128.0).resizable(false))
            .header(20.0, |mut header| {
                header.col(|_ui| {});
                header.col(|ui| {
                    ui.heading("Name");
                });
                header.col(|ui| {
                    ui.heading("Type");
                });
                header.col(|ui| {
                    ui.heading("Size");
                });
                header.col(|ui| {
                    ui.heading("Modified");
                });
            })
            .body(|body| {
                let mut size = 0;
                if let Some(filelist) = &self.filelist {
                    size = filelist.entries.len();
                }

                body.rows(20.0, size, |idx, mut row| {
                    let mut item: Option<crate::protocol::ReaddirResponseItem> = None;
                    if let Some(filelist) = &self.filelist {
                        item = Some(filelist.entries[idx].clone());
                    }

                    if let Some(item) = item {
                        row.col(|ui| {
                            if item.f_type == "DIR" {
                                ui.label("📁");
                            } else {
                                ui.label("📒");
                            }
                        });
                        row.col(|ui| {
                            let mut label = egui::Label::new(item.f_name.clone()).truncate(true);

                            if item.f_type == "DIR" {
                                label = label.sense(egui::Sense::click());
                            }

                            if ui.add(label).clicked() {
                                self.cd(ctx, item.f_path.as_str());
                            }
                        });
                        row.col(|ui| {
                            ui.label(item.f_type.clone());
                        });
                        row.col(|ui| {
                            ui.label(format_size(item.f_size));
                        });
                        row.col(|ui| {
                            ui.label(convert_epoch_to_local_time(item.f_modified));
                        });
                    }
                });
            });
    }

    fn install_font(&self, ctx: &egui::Context, name: String, data: Vec<u8>) {
        // Install my own font. `.ttf` and `.otf` files supported.
        let mut fonts = egui::FontDefinitions::default();
        let font = egui::FontData::from_owned(data);
        fonts.font_data.insert(name.clone(), font);

        // Put my font first (highest priority) for proportional text
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, name.clone());

        // Put my font as last fallback for monospace
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push(name.clone());

        // Tell egui to use these fonts.
        ctx.set_fonts(fonts);
    }

    /// Refresh current directory.
    fn refresh(&self, ctx: &egui::Context) {
        if let Some(v) = &self.cwd {
            self.cd(ctx, v.as_str());
        }
    }

    /// Change current directory.
    ///
    /// # Arguments
    /// + `path`: path to change to.
    fn cd(&self, ctx: &egui::Context, path: &str) {
        let path = path.to_string();

        let ctx = ctx.clone();
        let tx = self.tx.clone();
        self.client.post(
            crate::protocol::ReaddirRequest { path: path.clone() },
            move |rsp: Result<crate::protocol::ReaddirResponse, String>| {
                let mut rsp = rsp.unwrap();
                rsp.sort();

                tx.send(WebUiMessage::SetCWD(path)).unwrap();
                tx.send(WebUiMessage::SetFileList(rsp)).unwrap();
                ctx.request_repaint();
            },
        );
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

/// Convert size in bytes into a human-readable format.
fn format_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut size = size as f64;
    let mut index = 0;

    while size >= 1024.0 && index < units.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    return format!("{:.2} {}", size, units[index]);
}

fn convert_epoch_to_local_time(epoch: u64) -> String {
    // Create a NaiveDateTime from the timestamp
    let naive_datetime = chrono::NaiveDateTime::from_timestamp_opt(epoch as i64, 0).unwrap();

    // Convert it to UTC DateTime, then to local timezone
    let datetime_utc: chrono::DateTime<chrono::Utc> =
        chrono::DateTime::from_naive_utc_and_offset(naive_datetime, chrono::Utc);
    let datetime_local = datetime_utc.with_timezone(&chrono::Local);

    // Format the datetime to a string in a human-readable format
    datetime_local.format("%Y-%m-%d %H:%M:%S").to_string()
}
