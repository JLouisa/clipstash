use crate::{data, web::ctx};

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Rendering error: {0}")]
    Render(#[from] handlebars::RenderError),
}

pub struct Renderer<'a>(handlebars::Handlebars<'a>);
impl<'a> Renderer<'a> {
    pub fn new(templates_dir: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();
        renderer
            .register_templates_directory(".hbs", &templates_dir)
            .expect("Failed to register templates directory");
        Self(renderer)
    }

    pub fn convert_to_value<S>(serializable: &S) -> serde_json::Value
    where
        S: serde::Serialize + std::fmt::Debug,
    {
        serde_json::to_value(&serializable).expect("Failed to convert to value")
    }

    pub fn render<P>(&self, context: P, errors: &[&str]) -> String
    where
        P: ctx::PageContext + serde::Serialize + std::fmt::Debug,
    {
        let mut data = Self::convert_to_value(&context);
        if let Some(data) = data.as_object_mut() {
            data.insert("_errors".into(), errors.into());
            data.insert("_title".into(), context.title().into());
            data.insert("_parent".into(), context.parent().into());
        }
        self.do_render(context.template_path(), data)
    }

    pub fn render_with_data<P, D>(&self, context: P, value: (&str, D), errors: &[&str]) -> String
    where
        P: ctx::PageContext + serde::Serialize + std::fmt::Debug,
        D: serde::Serialize + std::fmt::Debug,
    {
        use handlebars::to_json;

        let mut data = Self::convert_to_value(&context);
        if let Some(data) = data.as_object_mut() {
            data.insert("_errors".into(), errors.into());
            data.insert("_title".into(), context.title().into());
            data.insert("_parent".into(), context.parent().into());
            data.insert(value.0.into(), to_json(value.1));
        }
        self.do_render(context.template_path(), data)
    }

    pub fn do_render(&self, path: &str, ctx: serde_json::Value) -> String {
        self.0.render(path, &ctx).expect("Failed to render")
    }
}
