use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS URL of a Web App to open after a link of the type internalLinkTypeWebApp is clicked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWebAppLinkUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which the link was clicked; pass 0 if none

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// Short name of the Web App

    #[serde(default)]
    web_app_short_name: String,
    /// Start parameter from internalLinkTypeWebApp

    #[serde(default)]
    start_parameter: String,
    /// Preferred Web App theme; pass null to use the default theme
    theme: ThemeParameters,
    /// Short name of the application; 0-64 English letters, digits, and underscores

    #[serde(default)]
    application_name: String,
    /// Pass true if the current user allowed the bot to send them messages

    #[serde(default)]
    allow_write_access: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetWebAppLinkUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetWebAppLinkUrl {}

impl GetWebAppLinkUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetWebAppLinkUrlBuilder {
        let mut inner = GetWebAppLinkUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getWebAppLinkUrl".to_string();

        GetWebAppLinkUrlBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn web_app_short_name(&self) -> &String {
        &self.web_app_short_name
    }

    pub fn start_parameter(&self) -> &String {
        &self.start_parameter
    }

    pub fn theme(&self) -> &ThemeParameters {
        &self.theme
    }

    pub fn application_name(&self) -> &String {
        &self.application_name
    }

    pub fn allow_write_access(&self) -> bool {
        self.allow_write_access
    }
}

#[doc(hidden)]
pub struct GetWebAppLinkUrlBuilder {
    inner: GetWebAppLinkUrl,
}

#[deprecated]
pub type RTDGetWebAppLinkUrlBuilder = GetWebAppLinkUrlBuilder;

impl GetWebAppLinkUrlBuilder {
    pub fn build(&self) -> GetWebAppLinkUrl {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn web_app_short_name<T: AsRef<str>>(&mut self, web_app_short_name: T) -> &mut Self {
        self.inner.web_app_short_name = web_app_short_name.as_ref().to_string();
        self
    }

    pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
        self.inner.start_parameter = start_parameter.as_ref().to_string();
        self
    }

    pub fn theme<T: AsRef<ThemeParameters>>(&mut self, theme: T) -> &mut Self {
        self.inner.theme = theme.as_ref().clone();
        self
    }

    pub fn application_name<T: AsRef<str>>(&mut self, application_name: T) -> &mut Self {
        self.inner.application_name = application_name.as_ref().to_string();
        self
    }

    pub fn allow_write_access(&mut self, allow_write_access: bool) -> &mut Self {
        self.inner.allow_write_access = allow_write_access;
        self
    }
}

impl AsRef<GetWebAppLinkUrl> for GetWebAppLinkUrl {
    fn as_ref(&self) -> &GetWebAppLinkUrl {
        self
    }
}

impl AsRef<GetWebAppLinkUrl> for GetWebAppLinkUrlBuilder {
    fn as_ref(&self) -> &GetWebAppLinkUrl {
        &self.inner
    }
}
