pub mod ctx;
pub mod form;
pub mod http;
pub mod renderer;

#[derive(rocket::Responder)]
pub enum PageError {
    #[response(status = 500)]
    SerializationError(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    InternalServerError(String),
    #[response(status = 500)]
    Render(String),
}

impl From<handlebars::RenderError> for PageError {
    fn from(err: handlebars::RenderError) -> Self {
        Self::Render(format!("{}", err))
    }
}

impl From<serde_json::Error> for PageError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(format!("{}", err))
    }
}
