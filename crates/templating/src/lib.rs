use askama::Template;
use serde::{Deserialize, Deserializer, de};
use std::{fmt, str::FromStr};

pub mod car;
pub mod cars;
pub mod client;
pub mod clients;
pub mod home;

#[derive(Debug, Deserialize)]
pub struct Body {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub search: Option<String>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub trait Render {
    fn render_template(&self) -> Result<String, String>;
}
impl<T> Render for T
where
    T: Template,
{
    fn render_template(&self) -> Result<String, String> {
        self.render().map_err(|e| e.to_string())
    }
}
