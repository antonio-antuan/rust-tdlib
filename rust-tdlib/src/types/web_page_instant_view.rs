use crate::errors::*;
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
    page_blocks: Vec<PageBlock>,
    /// Number of the instant view views; 0 if unknown
    view_count: i32,
    /// Version of the instant view, currently can be 1 or 2
    version: i32,
    /// True, if the instant view must be shown from right to left
    is_rtl: bool,
    /// True, if the instant view contains the full page. A network request might be needed to get the full web page instant view
    is_full: bool,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDWebPageInstantViewBuilder {
        let mut inner = WebPageInstantView::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDWebPageInstantViewBuilder { inner }
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
}

#[doc(hidden)]
pub struct RTDWebPageInstantViewBuilder {
    inner: WebPageInstantView,
}

impl RTDWebPageInstantViewBuilder {
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
}

impl AsRef<WebPageInstantView> for WebPageInstantView {
    fn as_ref(&self) -> &WebPageInstantView {
        self
    }
}

impl AsRef<WebPageInstantView> for RTDWebPageInstantViewBuilder {
    fn as_ref(&self) -> &WebPageInstantView {
        &self.inner
    }
}
