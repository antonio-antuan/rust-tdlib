use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that a previously opened Web App was closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloseWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of Web App launch, received from openWebApp

    #[serde(deserialize_with = "super::_common::number_from_string")]
    web_app_launch_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CloseWebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CloseWebApp {}

impl CloseWebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCloseWebAppBuilder {
        let mut inner = CloseWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "closeWebApp".to_string();

        RTDCloseWebAppBuilder { inner }
    }

    pub fn web_app_launch_id(&self) -> i64 {
        self.web_app_launch_id
    }
}

#[doc(hidden)]
pub struct RTDCloseWebAppBuilder {
    inner: CloseWebApp,
}

impl RTDCloseWebAppBuilder {
    pub fn build(&self) -> CloseWebApp {
        self.inner.clone()
    }

    pub fn web_app_launch_id(&mut self, web_app_launch_id: i64) -> &mut Self {
        self.inner.web_app_launch_id = web_app_launch_id;
        self
    }
}

impl AsRef<CloseWebApp> for CloseWebApp {
    fn as_ref(&self) -> &CloseWebApp {
        self
    }
}

impl AsRef<CloseWebApp> for RTDCloseWebAppBuilder {
    fn as_ref(&self) -> &CloseWebApp {
        &self.inner
    }
}