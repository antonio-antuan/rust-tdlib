use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Web App
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAppInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier for the Web App launch

    #[serde(deserialize_with = "super::_common::number_from_string")]
    launch_id: i64,
    /// A Web App URL to open in a web view
    url: String,
}

impl RObject for WebAppInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl WebAppInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDWebAppInfoBuilder {
        let mut inner = WebAppInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDWebAppInfoBuilder { inner }
    }

    pub fn launch_id(&self) -> i64 {
        self.launch_id
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDWebAppInfoBuilder {
    inner: WebAppInfo,
}

impl RTDWebAppInfoBuilder {
    pub fn build(&self) -> WebAppInfo {
        self.inner.clone()
    }

    pub fn launch_id(&mut self, launch_id: i64) -> &mut Self {
        self.inner.launch_id = launch_id;
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<WebAppInfo> for WebAppInfo {
    fn as_ref(&self) -> &WebAppInfo {
        self
    }
}

impl AsRef<WebAppInfo> for RTDWebAppInfoBuilder {
    fn as_ref(&self) -> &WebAppInfo {
        &self.inner
    }
}
