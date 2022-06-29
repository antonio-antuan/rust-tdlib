use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS URL of a Web App to open after keyboardButtonTypeWebApp button is pressed
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
    /// The URL from the keyboardButtonTypeWebApp button

    #[serde(default)]
    url: String,
    /// Preferred Web App theme; pass null to use the default theme
    theme: ThemeParameters,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetWebAppUrlBuilder {
        let mut inner = GetWebAppUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getWebAppUrl".to_string();

        RTDGetWebAppUrlBuilder { inner }
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
}

#[doc(hidden)]
pub struct RTDGetWebAppUrlBuilder {
    inner: GetWebAppUrl,
}

impl RTDGetWebAppUrlBuilder {
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
}

impl AsRef<GetWebAppUrl> for GetWebAppUrl {
    fn as_ref(&self) -> &GetWebAppUrl {
        self
    }
}

impl AsRef<GetWebAppUrl> for RTDGetWebAppUrlBuilder {
    fn as_ref(&self) -> &GetWebAppUrl {
        &self.inner
    }
}
