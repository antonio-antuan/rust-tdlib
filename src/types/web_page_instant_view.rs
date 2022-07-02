use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an instant view page for a web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebPageInstantView {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Content of the web page

    #[serde(default)]
    page_blocks: Vec<PageBlock>,
    /// Number of the instant view views; 0 if unknown

    #[serde(default)]
    view_count: i32,
    /// Version of the instant view; currently, can be 1 or 2

    #[serde(default)]
    version: i32,
    /// True, if the instant view must be shown from right to left

    #[serde(default)]
    is_rtl: bool,
    /// True, if the instant view contains the full page. A network request might be needed to get the full web page instant view

    #[serde(default)]
    is_full: bool,
    /// An internal link to be opened to leave feedback about the instant view

    #[serde(skip_serializing_if = "InternalLinkType::_is_default")]
    feedback_link: InternalLinkType,
}

impl RObject for WebPageInstantView {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl WebPageInstantView {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> WebPageInstantViewBuilder {
        let mut inner = WebPageInstantView::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        WebPageInstantViewBuilder { inner }
    }

    pub fn page_blocks(&self) -> &Vec<PageBlock> {
        &self.page_blocks
    }

    pub fn view_count(&self) -> i32 {
        self.view_count
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn is_rtl(&self) -> bool {
        self.is_rtl
    }

    pub fn is_full(&self) -> bool {
        self.is_full
    }

    pub fn feedback_link(&self) -> &InternalLinkType {
        &self.feedback_link
    }
}

#[doc(hidden)]
pub struct WebPageInstantViewBuilder {
    inner: WebPageInstantView,
}

#[deprecated]
pub type RTDWebPageInstantViewBuilder = WebPageInstantViewBuilder;

impl WebPageInstantViewBuilder {
    pub fn build(&self) -> WebPageInstantView {
        self.inner.clone()
    }

    pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
        self.inner.page_blocks = page_blocks;
        self
    }

    pub fn view_count(&mut self, view_count: i32) -> &mut Self {
        self.inner.view_count = view_count;
        self
    }

    pub fn version(&mut self, version: i32) -> &mut Self {
        self.inner.version = version;
        self
    }

    pub fn is_rtl(&mut self, is_rtl: bool) -> &mut Self {
        self.inner.is_rtl = is_rtl;
        self
    }

    pub fn is_full(&mut self, is_full: bool) -> &mut Self {
        self.inner.is_full = is_full;
        self
    }

    pub fn feedback_link<T: AsRef<InternalLinkType>>(&mut self, feedback_link: T) -> &mut Self {
        self.inner.feedback_link = feedback_link.as_ref().clone();
        self
    }
}

impl AsRef<WebPageInstantView> for WebPageInstantView {
    fn as_ref(&self) -> &WebPageInstantView {
        self
    }
}

impl AsRef<WebPageInstantView> for WebPageInstantViewBuilder {
    fn as_ref(&self) -> &WebPageInstantView {
        &self.inner
    }
}
