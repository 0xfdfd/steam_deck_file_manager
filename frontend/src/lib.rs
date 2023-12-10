pub mod app;
pub mod widget;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub async fn start_ui(canvas_id: &str) -> eframe::Result<(), wasm_bindgen::JsValue> {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

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

/// Fetch a POST request asynchronously
///
/// # Arguments
///
/// + `url` - The url of the request
/// + `body` - The body of the request
/// + `func` - The callback function
pub fn fetch_post<F>(url: &str, body: Option<&serde_json::Value>, func: F)
where
    F: FnOnce(Result<serde_json::Value, String>) + 'static,
{
    use wasm_bindgen::JsCast;

    let window = web_sys::window().unwrap();

    let headers = web_sys::Headers::new().unwrap();

    let mut opt = web_sys::RequestInit::new();
    opt.method("POST");
    if let Some(body) = body {
        opt.body(Some(&body.to_string().into()));
        headers.set("Content-Type", "application/json").unwrap();
    }
    opt.headers(&headers);

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
