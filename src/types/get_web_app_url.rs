use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS URL of a Web App to open from the side menu, a keyboardButtonTypeWebApp button, an inlineQueryResultsButtonTypeWebApp button, or an internalLinkTypeSideMenuBot link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWebAppUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// The URL from a keyboardButtonTypeWebApp button, inlineQueryResultsButtonTypeWebApp button, an internalLinkTypeSideMenuBot link, or an empty when the bot is opened from the side menu

    #[serde(default)]
    url: String,
    /// Preferred Web App theme; pass null to use the default theme
    theme: ThemeParameters,
    /// Short name of the application; 0-64 English letters, digits, and underscores

    #[serde(default)]
    application_name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetWebAppUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetWebAppUrl {}

impl GetWebAppUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetWebAppUrlBuilder {
        let mut inner = GetWebAppUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getWebAppUrl".to_string();

        GetWebAppUrlBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn theme(&self) -> &ThemeParameters {
        &self.theme
    }

    pub fn application_name(&self) -> &String {
        &self.application_name
    }
}

#[doc(hidden)]
pub struct GetWebAppUrlBuilder {
    inner: GetWebAppUrl,
}

#[deprecated]
pub type RTDGetWebAppUrlBuilder = GetWebAppUrlBuilder;

impl GetWebAppUrlBuilder {
    pub fn build(&self) -> GetWebAppUrl {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
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
}

impl AsRef<GetWebAppUrl> for GetWebAppUrl {
    fn as_ref(&self) -> &GetWebAppUrl {
        self
    }
}

impl AsRef<GetWebAppUrl> for GetWebAppUrlBuilder {
    fn as_ref(&self) -> &GetWebAppUrl {
        &self.inner
    }
}
