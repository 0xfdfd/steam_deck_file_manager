pub mod assets;
pub mod index;

pub struct BackendData<'a> {
    render: handlebars::Handlebars<'a>,
}

impl BackendData<'_> {
    pub fn new() -> Self {
        let mut handlebars = handlebars::Handlebars::new();

        {
            let file = crate::assets::get("index.html").unwrap();
            let data = std::str::from_utf8(file.data.as_ref()).unwrap();
            handlebars
                .register_template_string("index.html", data)
                .unwrap();
        }

        BackendData { render: handlebars }
    }
}
