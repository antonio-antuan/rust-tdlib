use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Parses Markdown entities in a human-friendly format, ignoring markup errors. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParseMarkdown {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text to parse. For example, "__italic__ ~~strikethrough~~ **bold** `code` ```pre``` __[italic__ text_url](telegram.org) __italic**bold italic__bold**"
    text: FormattedText,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ParseMarkdown {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ParseMarkdown {}

impl ParseMarkdown {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDParseMarkdownBuilder {
        let mut inner = ParseMarkdown::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "parseMarkdown".to_string();

        RTDParseMarkdownBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDParseMarkdownBuilder {
    inner: ParseMarkdown,
}

impl RTDParseMarkdownBuilder {
    pub fn build(&self) -> ParseMarkdown {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<ParseMarkdown> for ParseMarkdown {
    fn as_ref(&self) -> &ParseMarkdown {
        self
    }
}

impl AsRef<ParseMarkdown> for RTDParseMarkdownBuilder {
    fn as_ref(&self) -> &ParseMarkdown {
        &self.inner
    }
}
