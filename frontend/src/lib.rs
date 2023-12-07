mod app;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn start_ui(canvas_id: &str) -> eframe::Result<(), wasm_bindgen::JsValue> {
    eframe::WebRunner::new()
        .start(
            canvas_id,
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(app::WebUI::new(cc))),
        )
        .await?;

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn start_ui(app_name: &str) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| Box::new(app::WebUI::new(cc))),
    )
}
