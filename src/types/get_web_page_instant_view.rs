use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWebPageInstantView {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The web page URL

    #[serde(default)]
    url: String,
    /// If true, the full instant view for the web page will be returned

    #[serde(default)]
    force_full: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetWebPageInstantView {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetWebPageInstantView {}

impl GetWebPageInstantView {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetWebPageInstantViewBuilder {
        let mut inner = GetWebPageInstantView::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getWebPageInstantView".to_string();

        GetWebPageInstantViewBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn force_full(&self) -> bool {
        self.force_full
    }
}

#[doc(hidden)]
pub struct GetWebPageInstantViewBuilder {
    inner: GetWebPageInstantView,
}

#[deprecated]
pub type RTDGetWebPageInstantViewBuilder = GetWebPageInstantViewBuilder;

impl GetWebPageInstantViewBuilder {
    pub fn build(&self) -> GetWebPageInstantView {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn force_full(&mut self, force_full: bool) -> &mut Self {
        self.inner.force_full = force_full;
        self
    }
}

impl AsRef<GetWebPageInstantView> for GetWebPageInstantView {
    fn as_ref(&self) -> &GetWebPageInstantView {
        self
    }
}

impl AsRef<GetWebPageInstantView> for GetWebPageInstantViewBuilder {
    fn as_ref(&self) -> &GetWebPageInstantView {
        &self.inner
    }
}
