use std::future::Future;

pub struct HttpClient {
    base: url::Url,

    #[cfg(not(target_arch = "wasm32"))]
    rt: tokio::runtime::Runtime,
}

pub fn new(host: &str) -> HttpClient {
    let base_url = url::Url::parse(host).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    return HttpClient {
        base: base_url,

        #[cfg(not(target_arch = "wasm32"))]
        rt: rt,
    };
}

impl HttpClient {
    pub fn post<F>(&self, path: &str, body: Option<&serde_json::Value>, func: F)
    where
        F: FnOnce(Result<serde_json::Value, String>) + Send + 'static,
    {
        let url = self.url(path);
        let mut rs = reqwest::Client::new().post(url);
        if let Some(body) = body {
            rs = rs.json(body);
        };

        self.spawn(async move {
            let ret = match rs.send().await {
                Ok(v) => v,
                Err(e) => {
                    func(Err(e.to_string()));
                    return;
                }
            };

            let status_code = ret.status();

            let body = match ret.text().await {
                Ok(v) => v,
                Err(e) => {
                    func(Err(e.to_string()));
                    return;
                }
            };

            if status_code != 200 {
                func(Err(body));
                return;
            }

            let val: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
            func(Ok(val));
        });
    }

    fn url(&self, path: &str) -> String {
        let url = self.base.join(path).unwrap();
        return url.as_str().to_string();
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(future);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.rt.spawn(future);
    }
}
