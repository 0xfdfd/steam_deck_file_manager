struct FileExplorerImpl {
    homedir: Option<String>,
    cwd: Option<String>,
    filelist: Option<ReaddirResponse>,
}

#[derive(Clone)]
pub struct FileExplorer {
    client: std::sync::Arc<crate::http_client::HttpClient>,
    iner: std::sync::Arc<std::sync::RwLock<FileExplorerImpl>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReaddirItem {
    /// The name of the file.
    #[serde(rename = "name")]
    pub f_name: String,

    /// The absolute path to the file.
    #[serde(rename = "path")]
    pub f_path: String,

    /// The type of the file, 'FILE' or 'DIR'.
    #[serde(rename = "type")]
    pub f_type: String,

    /// The size of the file in bytes.
    #[serde(rename = "size")]
    pub f_size: u64,

    /// The last modified time of the file in seconds.
    #[serde(rename = "modified")]
    pub f_modified: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReaddirResponse {
    pub entries: Vec<ReaddirItem>,
}

pub fn new(host: &str) -> FileExplorer {
    let client = crate::http_client::new(host);

    let iner = FileExplorerImpl {
        homedir: None,
        cwd: None,
        filelist: None,
    };

    let outter = FileExplorer {
        client: std::sync::Arc::new(client),
        iner: std::sync::Arc::new(std::sync::RwLock::new(iner)),
    };

    let explorer = outter.clone();
    outter.client.post("/api/homedir", None, move |json| {
        let value = json.unwrap();
        let homedir = value["path"].as_str().unwrap().to_string();

        // Update homedir and cwd.
        {
            let mut guard = explorer.iner.write().unwrap();
            guard.homedir = Some(homedir.clone());
            guard.cwd = Some(homedir.clone());
        }

        explorer.refresh();
    });

    return outter;
}

impl FileExplorer {
    pub fn show(&self, ctx: &egui::Context) {
        let mut open = true;

        egui::Window::new("File Explorer")
            .open(&mut open)
            .default_height(800.0)
            .default_width(1280.0)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&self, ui: &mut egui::Ui) {
        self.ui_top_panel(ui);

        if let Some(filelist) = &self.iner.read().unwrap().filelist {
            for item in &filelist.entries {
                ui.label(item.f_name.clone());
            }
        }
    }

    fn ui_top_panel(&self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("file_explorer_top_panel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    {
                        let mut guard = self.iner.write().unwrap();
                        guard.cwd = guard.homedir.clone();
                    }

                    self.refresh();
                }

                #[cfg(target_arch = "wasm32")]
                if ui.button("Upload").clicked() {
                    let explorer = self.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some(file) = rfd::AsyncFileDialog::new().pick_file().await {
                            let upload_path = format!(
                                "/upload?path={}",
                                explorer.iner.read().unwrap().cwd.clone().unwrap()
                            );

                            let xhr = web_sys::XmlHttpRequest::new().unwrap();
                            xhr.open("POST", upload_path.as_str()).unwrap();

                            let formdata = web_sys::FormData::new().unwrap();
                            formdata.append_with_blob("file", file.inner()).unwrap();

                            xhr.send_with_opt_form_data(Some(&formdata)).unwrap();
                        }
                    });
                }

                if let Some(cwd) = &self.iner.read().unwrap().cwd {
                    ui.label(cwd);
                };
            });
        });
    }

    /// Read the directory and show contents.
    ///
    /// Note: The cwd is changed to the `path`.
    ///
    /// # Arguments
    /// + `path`: The path of the directory to read.
    fn readdir(&self, path: &str) {
        let explorer = self.clone();
        let path = path.to_string();

        let body = serde_json::json!({ "path": path });
        self.client.post("/api/readdir", Some(&body), move |json| {
            let value = json.unwrap();
            let rsp: ReaddirResponse = serde_json::from_value(value).unwrap();

            {
                let mut guard = explorer.iner.write().unwrap();
                guard.cwd = Some(path);
                guard.filelist = Some(rsp);
            }
        });
    }

    fn refresh(&self) {
        let mut cwd: Option<String> = None;
        {
            let guard = self.iner.read().unwrap();
            if let Some(v) = &guard.cwd {
                cwd = Some(v.clone());
            }
        }

        if let Some(v) = &cwd {
            self.readdir(v.as_str());
        };
    }
}
