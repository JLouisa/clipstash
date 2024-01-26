use super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content(String);
impl Content {
    pub fn new(content: String) -> Result<Self, ClipError> {
        if content.trim().is_empty() {
            Err(ClipError::EmptyContent)
        } else {
            Ok(Self(content))
        }
    }
}
