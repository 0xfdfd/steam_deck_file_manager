pub mod app;
pub mod http_client;
pub mod protocol;
pub mod widget;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn start_ui(canvas_id: &str, host: &str) -> eframe::Result<(), wasm_bindgen::JsValue> {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_ui_opt = crate::app::WebUiConfig {
        host: host.to_string(),
    };

    eframe::WebRunner::new()
        .start(
            canvas_id,
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(app::WebUI::new(cc, web_ui_opt))),
        )
        .await?;

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn start_ui(app_name: &str, host: &str) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([1280.0, 800.0]),
        ..Default::default()
    };

    let web_ui_opt = crate::app::WebUiConfig {
        host: host.to_string(),
    };

    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| Box::new(app::WebUI::new(cc, web_ui_opt))),
    )
}
