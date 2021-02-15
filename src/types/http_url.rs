use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains an HTTP URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The URL
    url: String,
}

impl RObject for HttpUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl HttpUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDHttpUrlBuilder {
        let mut inner = HttpUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDHttpUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDHttpUrlBuilder {
    inner: HttpUrl,
}

impl RTDHttpUrlBuilder {
    pub fn build(&self) -> HttpUrl {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<HttpUrl> for HttpUrl {
    fn as_ref(&self) -> &HttpUrl {
        self
    }
}

impl AsRef<HttpUrl> for RTDHttpUrlBuilder {
    fn as_ref(&self) -> &HttpUrl {
        &self.inner
    }
}
