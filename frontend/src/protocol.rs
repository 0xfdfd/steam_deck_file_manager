/// A request.
pub trait Request {
    /// The URL of the request.
    fn url(&self) -> &str;
    /// Convert the request to JSON.
    fn to_json(&self) -> Option<serde_json::Value>;
}

// A response.
pub trait Response {
    /// Convert the response from JSON.
    fn from_json(s: &str) -> Self;
}

/// `/api/dirs`: Request a sepcific directory path.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DirsRequest {
    /// The kind of path to request.
    pub kind: DirsRequestKind,
}

impl Request for DirsRequest {
    fn url(&self) -> &str {
        return "/api/dirs";
    }
    fn to_json(&self) -> Option<serde_json::Value> {
        return Some(serde_json::to_value(self).unwrap());
    }
}

/// The kind of path to request.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DirsRequestKind {
    /// ($HOME) The home directory.
    HomeDir,

    /// ($XDG_CACHE_HOME) The cache directory.
    CacheDir,

    /// ($XDG_CONFIG_HOME) The config directory.
    ConfigDir,

    /// ($XDG_CONFIG_HOME) The local directory.
    ConfigLocalDir,

    /// ($XDG_DATA_HOME) The data directory.
    DataDir,

    /// ($XDG_DATA_HOME) The data local directory.
    DataLocalDir,

    /// ($XDG_BIN_HOME) The executable directory.
    ExecutableDir,

    /// ($XDG_CONFIG_HOME) The state directory.
    PreferenceDir,

    /// ($XDG_RUNTIME_DIR) The runtime directory.
    RuntimeDir,

    /// ($XDG_STATE_HOME) The state directory.
    StateDir,

    /// ($XDG_MUSIC_DIR) The music directory.
    AudioDir,

    /// ($XDG_DESKTOP_DIR) The desktop directory.
    DesktopDir,

    /// ($XDG_DOCUMENTS_DIR) The documents directory.
    DocumentDir,

    /// ($XDG_DOWNLOAD_DIR) The download directory.
    DownloadDir,

    /// ($XDG_DATA_HOME/fonts/) The fonts directory.
    FontDir,

    /// ($XDG_PICTURES_DIR) The pictures directory.
    PictureDir,

    /// ($XDG_PUBLICSHARE_DIR) The public directory.
    PublicDir,

    /// ($XDG_TEMPLATES_DIR) The templates directory.
    TemplateDir,

    /// ($XDG_VIDEOS_DIR) The videos directory.
    VideoDir,
}

/// `/api/dirs`: Response of [DirsRequest].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DirsResponse {
    /// The absolute path to the directory.
    pub path: Option<String>,
}

impl Response for DirsResponse {
    fn from_json(s: &str) -> Self {
        return serde_json::from_str(s).unwrap();
    }
}

/// `/api/readdir`: Request a directory listing.
/// See [ReaddirResponse] for the response.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReaddirRequest {
    /// The absolute path to the directory.
    pub path: String,
}

impl Request for ReaddirRequest {
    fn url(&self) -> &str {
        return "/api/readdir";
    }
    fn to_json(&self) -> Option<serde_json::Value> {
        return Some(serde_json::to_value(self).unwrap());
    }
}

/// `/api/readdir`: Response of [ReaddirRequest].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReaddirResponse {
    /// The entries of the directory.
    pub entries: Vec<ReaddirResponseItem>,
}

/// An entry of [ReaddirResponse].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReaddirResponseItem {
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

impl ReaddirResponse {
    pub fn sort(&mut self) {
        self.entries.sort_by(|a, b| a.f_type.cmp(&b.f_type));
    }
}

impl Response for ReaddirResponse {
    fn from_json(s: &str) -> Self {
        return serde_json::from_str(s).unwrap();
    }
}
