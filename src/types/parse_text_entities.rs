use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Parses Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities contained in the text. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParseTextEntities {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text to parse

    #[serde(default)]
    text: String,
    /// Text parse mode

    #[serde(skip_serializing_if = "TextParseMode::_is_default")]
    parse_mode: TextParseMode,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ParseTextEntities {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ParseTextEntities {}

impl ParseTextEntities {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ParseTextEntitiesBuilder {
        let mut inner = ParseTextEntities::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "parseTextEntities".to_string();

        ParseTextEntitiesBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn parse_mode(&self) -> &TextParseMode {
        &self.parse_mode
    }
}

#[doc(hidden)]
pub struct ParseTextEntitiesBuilder {
    inner: ParseTextEntities,
}

#[deprecated]
pub type RTDParseTextEntitiesBuilder = ParseTextEntitiesBuilder;

impl ParseTextEntitiesBuilder {
    pub fn build(&self) -> ParseTextEntities {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn parse_mode<T: AsRef<TextParseMode>>(&mut self, parse_mode: T) -> &mut Self {
        self.inner.parse_mode = parse_mode.as_ref().clone();
        self
    }
}

impl AsRef<ParseTextEntities> for ParseTextEntities {
    fn as_ref(&self) -> &ParseTextEntities {
        self
    }
}

impl AsRef<ParseTextEntities> for ParseTextEntitiesBuilder {
    fn as_ref(&self) -> &ParseTextEntities {
        &self.inner
    }
}
