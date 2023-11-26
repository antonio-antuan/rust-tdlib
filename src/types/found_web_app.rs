use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Web App found by its short name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The Web App
    web_app: WebApp,
    /// True, if the user must be asked for the permission to the bot to send them messages

    #[serde(default)]
    request_write_access: bool,
    /// True, if there is no need to show an ordinary open URL confirmation before opening the Web App. The field must be ignored and confirmation must be shown anyway if the Web App link was hidden

    #[serde(default)]
    skip_confirmation: bool,
}

impl RObject for FoundWebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FoundWebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FoundWebAppBuilder {
        let mut inner = FoundWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FoundWebAppBuilder { inner }
    }

    pub fn web_app(&self) -> &WebApp {
        &self.web_app
    }

    pub fn request_write_access(&self) -> bool {
        self.request_write_access
    }

    pub fn skip_confirmation(&self) -> bool {
        self.skip_confirmation
    }
}

#[doc(hidden)]
pub struct FoundWebAppBuilder {
    inner: FoundWebApp,
}

#[deprecated]
pub type RTDFoundWebAppBuilder = FoundWebAppBuilder;

impl FoundWebAppBuilder {
    pub fn build(&self) -> FoundWebApp {
        self.inner.clone()
    }

    pub fn web_app<T: AsRef<WebApp>>(&mut self, web_app: T) -> &mut Self {
        self.inner.web_app = web_app.as_ref().clone();
        self
    }

    pub fn request_write_access(&mut self, request_write_access: bool) -> &mut Self {
        self.inner.request_write_access = request_write_access;
        self
    }

    pub fn skip_confirmation(&mut self, skip_confirmation: bool) -> &mut Self {
        self.inner.skip_confirmation = skip_confirmation;
        self
    }
}

impl AsRef<FoundWebApp> for FoundWebApp {
    fn as_ref(&self) -> &FoundWebApp {
        self
    }
}

impl AsRef<FoundWebApp> for FoundWebAppBuilder {
    fn as_ref(&self) -> &FoundWebApp {
        &self.inner
    }
}
