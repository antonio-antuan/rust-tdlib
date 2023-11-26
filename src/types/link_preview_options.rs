use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Options to be used for generation of a link preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if link preview must be disabled

    #[serde(default)]
    is_disabled: bool,
    /// URL to use for link preview. If empty, then the first URL found in the message text will be used

    #[serde(default)]
    url: String,
    /// True, if shown media preview must be small; ignored in secret chats or if the URL isn't explicitly specified

    #[serde(default)]
    force_small_media: bool,
    /// True, if shown media preview must be large; ignored in secret chats or if the URL isn't explicitly specified

    #[serde(default)]
    force_large_media: bool,
    /// True, if link preview must be shown above message text; otherwise, the link preview will be shown below the message text; ignored in secret chats

    #[serde(default)]
    show_above_text: bool,
}

impl RObject for LinkPreviewOptions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl LinkPreviewOptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LinkPreviewOptionsBuilder {
        let mut inner = LinkPreviewOptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LinkPreviewOptionsBuilder { inner }
    }

    pub fn is_disabled(&self) -> bool {
        self.is_disabled
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn force_small_media(&self) -> bool {
        self.force_small_media
    }

    pub fn force_large_media(&self) -> bool {
        self.force_large_media
    }

    pub fn show_above_text(&self) -> bool {
        self.show_above_text
    }
}

#[doc(hidden)]
pub struct LinkPreviewOptionsBuilder {
    inner: LinkPreviewOptions,
}

#[deprecated]
pub type RTDLinkPreviewOptionsBuilder = LinkPreviewOptionsBuilder;

impl LinkPreviewOptionsBuilder {
    pub fn build(&self) -> LinkPreviewOptions {
        self.inner.clone()
    }

    pub fn is_disabled(&mut self, is_disabled: bool) -> &mut Self {
        self.inner.is_disabled = is_disabled;
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn force_small_media(&mut self, force_small_media: bool) -> &mut Self {
        self.inner.force_small_media = force_small_media;
        self
    }

    pub fn force_large_media(&mut self, force_large_media: bool) -> &mut Self {
        self.inner.force_large_media = force_large_media;
        self
    }

    pub fn show_above_text(&mut self, show_above_text: bool) -> &mut Self {
        self.inner.show_above_text = show_above_text;
        self
    }
}

impl AsRef<LinkPreviewOptions> for LinkPreviewOptions {
    fn as_ref(&self) -> &LinkPreviewOptions {
        self
    }
}

impl AsRef<LinkPreviewOptions> for LinkPreviewOptionsBuilder {
    fn as_ref(&self) -> &LinkPreviewOptions {
        &self.inner
    }
}
