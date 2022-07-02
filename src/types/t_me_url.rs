use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a URL linking to an internal Telegram entity
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// URL

    #[serde(default)]
    url: String,
    /// Type of the URL

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "TMeUrlType::_is_default")]
    type_: TMeUrlType,
}

impl RObject for TMeUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TMeUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TMeUrlBuilder {
        let mut inner = TMeUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TMeUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn type_(&self) -> &TMeUrlType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct TMeUrlBuilder {
    inner: TMeUrl,
}

#[deprecated]
pub type RTDTMeUrlBuilder = TMeUrlBuilder;

impl TMeUrlBuilder {
    pub fn build(&self) -> TMeUrl {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn type_<T: AsRef<TMeUrlType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<TMeUrl> for TMeUrl {
    fn as_ref(&self) -> &TMeUrl {
        self
    }
}

impl AsRef<TMeUrl> for TMeUrlBuilder {
    fn as_ref(&self) -> &TMeUrl {
        &self.inner
    }
}
