use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Replaces text entities with Markdown formatting in a human-friendly format. Entities that can't be represented in Markdown unambiguously are kept as is. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMarkdownText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text
    text: FormattedText,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMarkdownText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMarkdownText {}

impl GetMarkdownText {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMarkdownTextBuilder {
        let mut inner = GetMarkdownText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMarkdownText".to_string();

        GetMarkdownTextBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }
}

#[doc(hidden)]
pub struct GetMarkdownTextBuilder {
    inner: GetMarkdownText,
}

#[deprecated]
pub type RTDGetMarkdownTextBuilder = GetMarkdownTextBuilder;

impl GetMarkdownTextBuilder {
    pub fn build(&self) -> GetMarkdownText {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<GetMarkdownText> for GetMarkdownText {
    fn as_ref(&self) -> &GetMarkdownText {
        self
    }
}

impl AsRef<GetMarkdownText> for GetMarkdownTextBuilder {
    fn as_ref(&self) -> &GetMarkdownText {
        &self.inner
    }
}
