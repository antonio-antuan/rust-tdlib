use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a link preview by the text of a message. Do not call this function too often. Returns a 404 error if the text has no link preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWebPagePreview {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message text with formatting
    text: FormattedText,
    /// Options to be used for generation of the link preview; pass null to use default link preview options
    link_preview_options: LinkPreviewOptions,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetWebPagePreviewBuilder {
        let mut inner = GetWebPagePreview::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getWebPagePreview".to_string();

        GetWebPagePreviewBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn link_preview_options(&self) -> &LinkPreviewOptions {
        &self.link_preview_options
    }
}

#[doc(hidden)]
pub struct GetWebPagePreviewBuilder {
    inner: GetWebPagePreview,
}

#[deprecated]
pub type RTDGetWebPagePreviewBuilder = GetWebPagePreviewBuilder;

impl GetWebPagePreviewBuilder {
    pub fn build(&self) -> GetWebPagePreview {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn link_preview_options<T: AsRef<LinkPreviewOptions>>(
        &mut self,
        link_preview_options: T,
    ) -> &mut Self {
        self.inner.link_preview_options = link_preview_options.as_ref().clone();
        self
    }
}

impl AsRef<GetWebPagePreview> for GetWebPagePreview {
    fn as_ref(&self) -> &GetWebPagePreview {
        self
    }
}

impl AsRef<GetWebPagePreview> for GetWebPagePreviewBuilder {
    fn as_ref(&self) -> &GetWebPagePreview {
        &self.inner
    }
}
