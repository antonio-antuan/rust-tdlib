use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWebPagePreview {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message text with formatting
    text: FormattedText,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetWebPagePreview {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetWebPagePreview {}

impl GetWebPagePreview {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetWebPagePreviewBuilder {
        let mut inner = GetWebPagePreview::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getWebPagePreview".to_string();

        RTDGetWebPagePreviewBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDGetWebPagePreviewBuilder {
    inner: GetWebPagePreview,
}

impl RTDGetWebPagePreviewBuilder {
    pub fn build(&self) -> GetWebPagePreview {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<GetWebPagePreview> for GetWebPagePreview {
    fn as_ref(&self) -> &GetWebPagePreview {
        self
    }
}

impl AsRef<GetWebPagePreview> for RTDGetWebPagePreviewBuilder {
    fn as_ref(&self) -> &GetWebPagePreview {
        &self.inner
    }
}
