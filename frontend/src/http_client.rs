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
    pub fn post<T, F, R>(&self, req: &T, func: F)
    where
        T: crate::protocol::Request + ?Sized,
        F: FnOnce(Result<R, String>) + Send + 'static,
        R: crate::protocol::Response,
    {
        let url = self.url(req.url());
        let mut rs = reqwest::Client::new().post(url);
        if let Some(body) = req.to_json() {
            rs = rs.json(&body);
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

            let val: R = R::from_json(body.as_str());

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
