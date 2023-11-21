use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a Web App by its short name. Returns a 404 error if the Web App is not found
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// Short name of the Web App

    #[serde(default)]
    web_app_short_name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchWebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchWebApp {}

impl SearchWebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchWebAppBuilder {
        let mut inner = SearchWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchWebApp".to_string();

        SearchWebAppBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn web_app_short_name(&self) -> &String {
        &self.web_app_short_name
    }
}

#[doc(hidden)]
pub struct SearchWebAppBuilder {
    inner: SearchWebApp,
}

#[deprecated]
pub type RTDSearchWebAppBuilder = SearchWebAppBuilder;

impl SearchWebAppBuilder {
    pub fn build(&self) -> SearchWebApp {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn web_app_short_name<T: AsRef<str>>(&mut self, web_app_short_name: T) -> &mut Self {
        self.inner.web_app_short_name = web_app_short_name.as_ref().to_string();
        self
    }
}

impl AsRef<SearchWebApp> for SearchWebApp {
    fn as_ref(&self) -> &SearchWebApp {
        self
    }
}

impl AsRef<SearchWebApp> for SearchWebAppBuilder {
    fn as_ref(&self) -> &SearchWebApp {
        &self.inner
    }
}
