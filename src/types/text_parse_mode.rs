use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the way the text should be parsed for TextEntities
pub trait TDTextParseMode: Debug + RObject {}

/// Describes the way the text should be parsed for TextEntities
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TextParseMode {
    #[doc(hidden)]
    _Default(()),
    /// The text uses HTML-style formatting. The same as Telegram Bot API "HTML" parse mode
    HTML(TextParseModeHTML),
    /// The text uses Markdown-style formatting
    Markdown(TextParseModeMarkdown),
}

impl Default for TextParseMode {
    fn default() -> Self {
        TextParseMode::_Default(())
    }
}

impl<'de> Deserialize<'de> for TextParseMode {
    fn deserialize<D>(deserializer: D) -> Result<TextParseMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          TextParseMode,
          (textParseModeHTML, HTML);
          (textParseModeMarkdown, Markdown);

        )(deserializer)
    }
}

impl RObject for TextParseMode {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            TextParseMode::HTML(t) => t.td_name(),
            TextParseMode::Markdown(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            TextParseMode::HTML(t) => t.extra(),
            TextParseMode::Markdown(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TextParseMode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, TextParseMode::_Default(_))
    }
}

impl AsRef<TextParseMode> for TextParseMode {
    fn as_ref(&self) -> &TextParseMode {
        self
    }
}

/// The text uses HTML-style formatting. The same as Telegram Bot API "HTML" parse mode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextParseModeHTML {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for TextParseModeHTML {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textParseModeHTML"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDTextParseMode for TextParseModeHTML {}

impl TextParseModeHTML {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextParseModeHTMLBuilder {
        let mut inner = TextParseModeHTML::default();
        inner.td_name = "textParseModeHTML".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDTextParseModeHTMLBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextParseModeHTMLBuilder {
    inner: TextParseModeHTML,
}

impl RTDTextParseModeHTMLBuilder {
    pub fn build(&self) -> TextParseModeHTML {
        self.inner.clone()
    }
}

impl AsRef<TextParseModeHTML> for TextParseModeHTML {
    fn as_ref(&self) -> &TextParseModeHTML {
        self
    }
}

impl AsRef<TextParseModeHTML> for RTDTextParseModeHTMLBuilder {
    fn as_ref(&self) -> &TextParseModeHTML {
        &self.inner
    }
}

/// The text uses Markdown-style formatting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextParseModeMarkdown {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Version of the parser: 0 or 1  Telegram Bot API "Markdown" parse mode, 2  Telegram Bot API "MarkdownV2" parse mode
    version: i32,
}

impl RObject for TextParseModeMarkdown {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textParseModeMarkdown"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDTextParseMode for TextParseModeMarkdown {}

impl TextParseModeMarkdown {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextParseModeMarkdownBuilder {
        let mut inner = TextParseModeMarkdown::default();
        inner.td_name = "textParseModeMarkdown".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDTextParseModeMarkdownBuilder { inner }
    }

    pub fn version(&self) -> i32 {
        self.version
    }
}

#[doc(hidden)]
pub struct RTDTextParseModeMarkdownBuilder {
    inner: TextParseModeMarkdown,
}

impl RTDTextParseModeMarkdownBuilder {
    pub fn build(&self) -> TextParseModeMarkdown {
        self.inner.clone()
    }

    pub fn version(&mut self, version: i32) -> &mut Self {
        self.inner.version = version;
        self
    }
}

impl AsRef<TextParseModeMarkdown> for TextParseModeMarkdown {
    fn as_ref(&self) -> &TextParseModeMarkdown {
        self
    }
}

impl AsRef<TextParseModeMarkdown> for RTDTextParseModeMarkdownBuilder {
    fn as_ref(&self) -> &TextParseModeMarkdown {
        &self.inner
    }
}
