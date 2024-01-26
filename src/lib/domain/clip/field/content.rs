use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content(String);
impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if content.trim().is_empty() {
            Err(ClipError::EmptyContent)
        } else {
            Ok(Self(content.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}