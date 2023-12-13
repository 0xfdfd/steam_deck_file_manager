#[actix_web::post("/api/dirs")]
pub async fn post(
    body: actix_web::web::Json<frontend::protocol::DirsRequest>,
) -> impl actix_web::Responder {
    let path = match body.kind {
        frontend::protocol::DirsRequestKind::HomeDir => dirs::home_dir(),
        frontend::protocol::DirsRequestKind::CacheDir => dirs::cache_dir(),
        frontend::protocol::DirsRequestKind::ConfigDir => dirs::config_dir(),
        frontend::protocol::DirsRequestKind::ConfigLocalDir => dirs::data_local_dir(),
        frontend::protocol::DirsRequestKind::DataDir => dirs::data_dir(),
        frontend::protocol::DirsRequestKind::DataLocalDir => dirs::data_local_dir(),
        frontend::protocol::DirsRequestKind::ExecutableDir => dirs::executable_dir(),
        frontend::protocol::DirsRequestKind::PreferenceDir => dirs::preference_dir(),
        frontend::protocol::DirsRequestKind::RuntimeDir => dirs::runtime_dir(),
        frontend::protocol::DirsRequestKind::StateDir => dirs::state_dir(),
        frontend::protocol::DirsRequestKind::AudioDir => dirs::audio_dir(),
        frontend::protocol::DirsRequestKind::DesktopDir => dirs::desktop_dir(),
        frontend::protocol::DirsRequestKind::DocumentDir => dirs::document_dir(),
        frontend::protocol::DirsRequestKind::DownloadDir => dirs::download_dir(),
        frontend::protocol::DirsRequestKind::FontDir => dirs::font_dir(),
        frontend::protocol::DirsRequestKind::PictureDir => dirs::picture_dir(),
        frontend::protocol::DirsRequestKind::PublicDir => dirs::public_dir(),
        frontend::protocol::DirsRequestKind::TemplateDir => dirs::template_dir(),
        frontend::protocol::DirsRequestKind::VideoDir => dirs::video_dir(),
    };
    let path = match path {
        Some(v) => Some(v.to_str().unwrap().to_string()),
        None => None,
    };

    let obj = frontend::protocol::DirsResponse { path: path };

    actix_web::web::Json(obj)
}
