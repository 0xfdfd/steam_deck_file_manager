struct FileExplorerImpl {
    homedir: Option<String>,
    cwd: Option<String>,
    filelist: Option<crate::protocol::ReaddirResponse>,
}

#[derive(Clone)]
pub struct FileExplorer {
    client: std::sync::Arc<crate::http_client::HttpClient>,
    iner: std::sync::Arc<std::sync::RwLock<FileExplorerImpl>>,
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

    let req = crate::protocol::DirsRequest {
        kind: crate::protocol::DirsRequestKind::HomeDir,
    };

    let explorer = outter.clone();
    outter.client.post(
        &req,
        move |rsp: Result<crate::protocol::DirsResponse, String>| {
            let rsp = rsp.unwrap();
            let homedir = rsp.path.unwrap();

            // Update homedir and cwd.
            {
                let mut guard = explorer.iner.write().unwrap();
                guard.homedir = Some(homedir.clone());
                guard.cwd = Some(homedir.clone());
            }

            explorer.refresh();
        },
    );

    return outter;
}

impl FileExplorer {
    /// Show the file explorer.
    ///
    /// # Returns
    /// Whether the window is open or not.
    pub fn show(&self, ctx: &egui::Context) -> bool {
        let mut open = true;

        egui::Window::new("File Explorer")
            .open(&mut open)
            .show(ctx, |ui| {
                self.ui(ui);
            });

        return open;
    }

    fn ui(&self, ui: &mut egui::Ui) {
        self.ui_top_panel(ui);
        self.ui_body_panel(ui);
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

    fn ui_body_panel(&self, ui: &mut egui::Ui) {
        egui_extras::TableBuilder::new(ui)
            .column(egui_extras::Column::remainder().resizable(true))
            .column(egui_extras::Column::exact(64.0).resizable(false))
            .column(egui_extras::Column::exact(64.0).resizable(false))
            .column(egui_extras::Column::exact(128.0).resizable(false))
            .header(20.0, |mut header| {
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
                if let Some(filelist) = &self.iner.read().unwrap().filelist {
                    body.rows(20.0, filelist.entries.len(), |idx, mut row| {
                        let item = &filelist.entries[idx];
                        row.col(|ui| {
                            let label = egui::Label::new(item.f_name.clone()).truncate(true);
                            ui.add(label);
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
                    });
                }
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

        let req = crate::protocol::ReaddirRequest { path: path.clone() };

        self.client.post(
            &req,
            move |rsp: Result<crate::protocol::ReaddirResponse, String>| {
                let mut rsp = rsp.unwrap();
                rsp.sort();

                {
                    let mut guard = explorer.iner.write().unwrap();
                    guard.cwd = Some(path);
                    guard.filelist = Some(rsp);
                }
            },
        );
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
