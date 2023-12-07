#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/frontend/pkg"]
#[exclude = ".gitignore"]
struct Assets0;

#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
struct Assets1;

pub fn get(path: &str) -> Option<rust_embed::EmbeddedFile> {
    match Assets0::get(path) {
        Some(v) => return Some(v),
        None => (),
    };

    match Assets1::get(path) {
        Some(v) => return Some(v),
        None => (),
    };

    return None;
}
