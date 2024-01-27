pub mod data;
pub mod domain;
pub mod service;
pub mod web;

pub use data::DbError;
pub use domain::clip::field::ShortCode;
pub use domain::clip::{Clip, ClipError};
pub use domain::time::Time;
pub use service::ServiceError;
