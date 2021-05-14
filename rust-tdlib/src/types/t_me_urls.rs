use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTMeUrlsBuilder {
        let mut inner = TMeUrls::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTMeUrlsBuilder { inner }
    }

    pub fn urls(&self) -> &Vec<TMeUrl> {
        &self.urls
    }
}

#[doc(hidden)]
pub struct RTDTMeUrlsBuilder {
    inner: TMeUrls,
}

impl RTDTMeUrlsBuilder {
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

impl AsRef<TMeUrls> for RTDTMeUrlsBuilder {
    fn as_ref(&self) -> &TMeUrls {
        &self.inner
    }
}
