use wasm_bindgen::JsCast;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct WebUiImpl {
    label: String,
    value: f32,

    homedir: Option<String>,
}

#[derive(Clone)]
pub struct WebUI(std::sync::Arc<std::sync::RwLock<WebUiImpl>>);

impl WebUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut ctx = WebUiImpl::default();

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            ctx = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self(std::sync::Arc::new(std::sync::RwLock::new(ctx)))
    }
}

impl std::fmt::Debug for WebUI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebUI").finish_non_exhaustive()
    }
}

impl eframe::App for WebUI {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let ctx = WebUiImpl {
            label: self.0.read().unwrap().label.clone(),
            value: self.0.read().unwrap().value,
            ..Default::default()
        };

        eframe::set_value(storage, eframe::APP_KEY, &ctx);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });

            if ui.button("Home").clicked() {
                log::info!("Home button clicked");

                let ctx = self.clone();

                fetch_post("/api/homedir", move |json| {
                    ctx.0.write().unwrap().homedir = Some(json.unwrap().to_string());
                });
            }

            if self.0.read().unwrap().homedir.is_some() {
                let txt = format!(
                    "homedir: {}",
                    self.0.read().unwrap().homedir.as_ref().unwrap()
                );
                ui.label(txt);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.0.write().unwrap().label);
            });

            ui.add(egui::Slider::new(&mut self.0.write().unwrap().value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.0.write().unwrap().value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
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

fn fetch_post<F>(url: &str, mut func: F)
where
    F: FnMut(Result<serde_json::Value, String>) + 'static,
{
    let window = web_sys::window().unwrap();

    let mut opt = web_sys::RequestInit::new();
    opt.method("POST");

    let request = web_sys::Request::new_with_str_and_init(url, &opt).unwrap();
    let request: wasm_bindgen::JsValue = request.into();

    wasm_bindgen_futures::spawn_local(async move {
        let promise = window.fetch_with_request(&request.into());
        let rsp = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
        let rsp: web_sys::Response = rsp.dyn_into().unwrap();

        let data = rsp.text().unwrap();
        let json = wasm_bindgen_futures::JsFuture::from(data).await.unwrap();
        let json = json.as_string().unwrap();

        if rsp.ok() {
            func(Ok(serde_json::from_str(json.as_str()).unwrap()));
        } else {
            func(Err(json));
        }
    });
}
