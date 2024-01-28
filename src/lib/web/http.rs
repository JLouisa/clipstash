use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{ctx, form, renderer::Renderer, PageError};
use crate::{ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{content::RawHtml, status, Redirect};
use rocket::{uri, State};

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let ctx = ctx::Home::default();
    println!("{:?}", ctx);
    RawHtml(renderer.render(ctx, &[]))
    // RawHtml("<h1>Hello World</h1>".to_owned())
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home]
}

pub mod catcher {

    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        println!("General Error {:?}", req);
        "An unexpected error occurred."
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        println!("Internal Error {:?}", req);
        "Internal server error."
    }

    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![default, internal_error, not_found]
    }
}
