use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl
pub trait TDLoginUrlInfo: Debug + RObject {}

/// Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LoginUrlInfo {
    #[doc(hidden)]
    _Default,
    /// Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
    #[serde(rename(serialize = "getLoginUrlInfo", deserialize = "getLoginUrlInfo"))]
    GetLoginUrlInfo(GetLoginUrlInfo),
    /// An HTTP url needs to be open
    #[serde(rename(serialize = "loginUrlInfoOpen", deserialize = "loginUrlInfoOpen"))]
    Open(LoginUrlInfoOpen),
    /// An authorization confirmation dialog needs to be shown to the user
    #[serde(rename(
        serialize = "loginUrlInfoRequestConfirmation",
        deserialize = "loginUrlInfoRequestConfirmation"
    ))]
    RequestConfirmation(LoginUrlInfoRequestConfirmation),
}

impl Default for LoginUrlInfo {
    fn default() -> Self {
        LoginUrlInfo::_Default
    }
}

impl RObject for LoginUrlInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            LoginUrlInfo::GetLoginUrlInfo(t) => t.extra(),
            LoginUrlInfo::Open(t) => t.extra(),
            LoginUrlInfo::RequestConfirmation(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            LoginUrlInfo::GetLoginUrlInfo(t) => t.client_id(),
            LoginUrlInfo::Open(t) => t.client_id(),
            LoginUrlInfo::RequestConfirmation(t) => t.client_id(),

            _ => None,
        }
    }
}

impl LoginUrlInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, LoginUrlInfo::_Default)
    }
}

impl AsRef<LoginUrlInfo> for LoginUrlInfo {
    fn as_ref(&self) -> &LoginUrlInfo {
        self
    }
}

/// An HTTP url needs to be open
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginUrlInfoOpen {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The URL to open
    url: String,
    /// True, if there is no need to show an ordinary open URL confirm
    skip_confirm: bool,
}

impl RObject for LoginUrlInfoOpen {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLoginUrlInfo for LoginUrlInfoOpen {}

impl LoginUrlInfoOpen {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLoginUrlInfoOpenBuilder {
        let mut inner = LoginUrlInfoOpen::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLoginUrlInfoOpenBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn skip_confirm(&self) -> bool {
        self.skip_confirm
    }
}

#[doc(hidden)]
pub struct RTDLoginUrlInfoOpenBuilder {
    inner: LoginUrlInfoOpen,
}

impl RTDLoginUrlInfoOpenBuilder {
    pub fn build(&self) -> LoginUrlInfoOpen {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn skip_confirm(&mut self, skip_confirm: bool) -> &mut Self {
        self.inner.skip_confirm = skip_confirm;
        self
    }
}

impl AsRef<LoginUrlInfoOpen> for LoginUrlInfoOpen {
    fn as_ref(&self) -> &LoginUrlInfoOpen {
        self
    }
}

impl AsRef<LoginUrlInfoOpen> for RTDLoginUrlInfoOpenBuilder {
    fn as_ref(&self) -> &LoginUrlInfoOpen {
        &self.inner
    }
}

/// An authorization confirmation dialog needs to be shown to the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginUrlInfoRequestConfirmation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// An HTTP URL to be opened
    url: String,
    /// A domain of the URL
    domain: String,
    /// User identifier of a bot linked with the website
    bot_user_id: i32,
    /// True, if the user needs to be requested to give the permission to the bot to send them messages
    request_write_access: bool,
}

impl RObject for LoginUrlInfoRequestConfirmation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLoginUrlInfo for LoginUrlInfoRequestConfirmation {}

impl LoginUrlInfoRequestConfirmation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLoginUrlInfoRequestConfirmationBuilder {
        let mut inner = LoginUrlInfoRequestConfirmation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLoginUrlInfoRequestConfirmationBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn domain(&self) -> &String {
        &self.domain
    }

    pub fn bot_user_id(&self) -> i32 {
        self.bot_user_id
    }

    pub fn request_write_access(&self) -> bool {
        self.request_write_access
    }
}

#[doc(hidden)]
pub struct RTDLoginUrlInfoRequestConfirmationBuilder {
    inner: LoginUrlInfoRequestConfirmation,
}

impl RTDLoginUrlInfoRequestConfirmationBuilder {
    pub fn build(&self) -> LoginUrlInfoRequestConfirmation {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn domain<T: AsRef<str>>(&mut self, domain: T) -> &mut Self {
        self.inner.domain = domain.as_ref().to_string();
        self
    }

    pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn request_write_access(&mut self, request_write_access: bool) -> &mut Self {
        self.inner.request_write_access = request_write_access;
        self
    }
}

impl AsRef<LoginUrlInfoRequestConfirmation> for LoginUrlInfoRequestConfirmation {
    fn as_ref(&self) -> &LoginUrlInfoRequestConfirmation {
        self
    }
}

impl AsRef<LoginUrlInfoRequestConfirmation> for RTDLoginUrlInfoRequestConfirmationBuilder {
    fn as_ref(&self) -> &LoginUrlInfoRequestConfirmation {
        &self.inner
    }
}
