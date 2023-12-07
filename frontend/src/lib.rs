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
