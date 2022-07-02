use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of t.me URLs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrls {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of URLs

    #[serde(default)]
    urls: Vec<TMeUrl>,
}

impl RObject for TMeUrls {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TMeUrls {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TMeUrlsBuilder {
        let mut inner = TMeUrls::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TMeUrlsBuilder { inner }
    }

    pub fn urls(&self) -> &Vec<TMeUrl> {
        &self.urls
    }
}

#[doc(hidden)]
pub struct TMeUrlsBuilder {
    inner: TMeUrls,
}

#[deprecated]
pub type RTDTMeUrlsBuilder = TMeUrlsBuilder;

impl TMeUrlsBuilder {
    pub fn build(&self) -> TMeUrls {
        self.inner.clone()
    }

    pub fn urls(&mut self, urls: Vec<TMeUrl>) -> &mut Self {
        self.inner.urls = urls;
        self
    }
}

impl AsRef<TMeUrls> for TMeUrls {
    fn as_ref(&self) -> &TMeUrls {
        self
    }
}

impl AsRef<TMeUrls> for TMeUrlsBuilder {
    fn as_ref(&self) -> &TMeUrls {
        &self.inner
    }
}
