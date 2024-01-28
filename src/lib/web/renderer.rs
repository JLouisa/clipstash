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

    // * Problem here why template is not rendering
    pub fn render<P>(&self, context: P, errors: &[&str]) -> String
    where
        P: ctx::PageContext + serde::Serialize + std::fmt::Debug,
    {
        let mut value = Self::convert_to_value(&context);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(), context.title().into());
            value.insert("_parent".into(), context.parent().into());
        }
        println!("Path {:?}", context.template_path());
        self.do_render(context.template_path(), value)
    }

    pub fn render_with_data<P, D>(&self, context: P, data: (&str, D), errors: &[&str]) -> String
    where
        P: ctx::PageContext + serde::Serialize + std::fmt::Debug,
        D: serde::Serialize + std::fmt::Debug,
    {
        use handlebars::to_json;

        let mut value = Self::convert_to_value(&context);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(), context.title().into());
            value.insert("_parent".into(), context.parent().into());
            value.insert(data.0.into(), to_json(data.1));
        }
        self.do_render(context.template_path(), value)
    }

    pub fn do_render(&self, path: &str, ctx: serde_json::Value) -> String {
        println!("Rendering template: {:?}", path);
        dbg!("Rendering template: {:?}", &ctx);
        // let rendered_html = self.0.render(path, &ctx);
        // println!("Rendered HTML: {:?}", rendered_html);
        // rendered_html.expect("Failed to render")
        let result = self.0.render(path, &ctx);

        match result {
            Ok(rendered_html) => {
                println!("The Rendered HTML: {:?}", rendered_html);
                rendered_html
            }
            Err(error) =>
            // Handle the error here (e.g., log it or return an error page)
            // For example, you can return an error message like this:
            // Html(format!("Error rendering template: {}", error))
            // Or, return an error page
            // Html(render_error_page())
            {
                format!("{}", error)
            } // Example function to render an error page
        }
    }
}
