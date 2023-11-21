use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a reason why a bot was allowed to write messages to the current user
pub trait TDBotWriteAccessAllowReason: Debug + RObject {}

/// Describes a reason why a bot was allowed to write messages to the current user
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum BotWriteAccessAllowReason {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The user accepted bot's request to send messages with allowBotToSendMessages
    #[serde(rename = "botWriteAccessAllowReasonAcceptedRequest")]
    AcceptedRequest(BotWriteAccessAllowReasonAcceptedRequest),
    /// The user added the bot to attachment or side menu using toggleBotIsAddedToAttachmentMenu
    #[serde(rename = "botWriteAccessAllowReasonAddedToAttachmentMenu")]
    AddedToAttachmentMenu(BotWriteAccessAllowReasonAddedToAttachmentMenu),
    /// The user connected a website by logging in using Telegram Login Widget on it
    #[serde(rename = "botWriteAccessAllowReasonConnectedWebsite")]
    ConnectedWebsite(BotWriteAccessAllowReasonConnectedWebsite),
    /// The user launched a Web App using getWebAppLinkUrl
    #[serde(rename = "botWriteAccessAllowReasonLaunchedWebApp")]
    LaunchedWebApp(BotWriteAccessAllowReasonLaunchedWebApp),
}

impl RObject for BotWriteAccessAllowReason {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            BotWriteAccessAllowReason::AcceptedRequest(t) => t.extra(),
            BotWriteAccessAllowReason::AddedToAttachmentMenu(t) => t.extra(),
            BotWriteAccessAllowReason::ConnectedWebsite(t) => t.extra(),
            BotWriteAccessAllowReason::LaunchedWebApp(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            BotWriteAccessAllowReason::AcceptedRequest(t) => t.client_id(),
            BotWriteAccessAllowReason::AddedToAttachmentMenu(t) => t.client_id(),
            BotWriteAccessAllowReason::ConnectedWebsite(t) => t.client_id(),
            BotWriteAccessAllowReason::LaunchedWebApp(t) => t.client_id(),

            _ => None,
        }
    }
}

impl BotWriteAccessAllowReason {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, BotWriteAccessAllowReason::_Default)
    }
}

impl AsRef<BotWriteAccessAllowReason> for BotWriteAccessAllowReason {
    fn as_ref(&self) -> &BotWriteAccessAllowReason {
        self
    }
}

/// The user accepted bot's request to send messages with allowBotToSendMessages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotWriteAccessAllowReasonAcceptedRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for BotWriteAccessAllowReasonAcceptedRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotWriteAccessAllowReason for BotWriteAccessAllowReasonAcceptedRequest {}

impl BotWriteAccessAllowReasonAcceptedRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BotWriteAccessAllowReasonAcceptedRequestBuilder {
        let mut inner = BotWriteAccessAllowReasonAcceptedRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BotWriteAccessAllowReasonAcceptedRequestBuilder { inner }
    }
}

#[doc(hidden)]
pub struct BotWriteAccessAllowReasonAcceptedRequestBuilder {
    inner: BotWriteAccessAllowReasonAcceptedRequest,
}

#[deprecated]
pub type RTDBotWriteAccessAllowReasonAcceptedRequestBuilder =
    BotWriteAccessAllowReasonAcceptedRequestBuilder;

impl BotWriteAccessAllowReasonAcceptedRequestBuilder {
    pub fn build(&self) -> BotWriteAccessAllowReasonAcceptedRequest {
        self.inner.clone()
    }
}

impl AsRef<BotWriteAccessAllowReasonAcceptedRequest> for BotWriteAccessAllowReasonAcceptedRequest {
    fn as_ref(&self) -> &BotWriteAccessAllowReasonAcceptedRequest {
        self
    }
}

impl AsRef<BotWriteAccessAllowReasonAcceptedRequest>
    for BotWriteAccessAllowReasonAcceptedRequestBuilder
{
    fn as_ref(&self) -> &BotWriteAccessAllowReasonAcceptedRequest {
        &self.inner
    }
}

/// The user added the bot to attachment or side menu using toggleBotIsAddedToAttachmentMenu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotWriteAccessAllowReasonAddedToAttachmentMenu {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for BotWriteAccessAllowReasonAddedToAttachmentMenu {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotWriteAccessAllowReason for BotWriteAccessAllowReasonAddedToAttachmentMenu {}

impl BotWriteAccessAllowReasonAddedToAttachmentMenu {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BotWriteAccessAllowReasonAddedToAttachmentMenuBuilder {
        let mut inner = BotWriteAccessAllowReasonAddedToAttachmentMenu::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BotWriteAccessAllowReasonAddedToAttachmentMenuBuilder { inner }
    }
}

#[doc(hidden)]
pub struct BotWriteAccessAllowReasonAddedToAttachmentMenuBuilder {
    inner: BotWriteAccessAllowReasonAddedToAttachmentMenu,
}

#[deprecated]
pub type RTDBotWriteAccessAllowReasonAddedToAttachmentMenuBuilder =
    BotWriteAccessAllowReasonAddedToAttachmentMenuBuilder;

impl BotWriteAccessAllowReasonAddedToAttachmentMenuBuilder {
    pub fn build(&self) -> BotWriteAccessAllowReasonAddedToAttachmentMenu {
        self.inner.clone()
    }
}

impl AsRef<BotWriteAccessAllowReasonAddedToAttachmentMenu>
    for BotWriteAccessAllowReasonAddedToAttachmentMenu
{
    fn as_ref(&self) -> &BotWriteAccessAllowReasonAddedToAttachmentMenu {
        self
    }
}

impl AsRef<BotWriteAccessAllowReasonAddedToAttachmentMenu>
    for BotWriteAccessAllowReasonAddedToAttachmentMenuBuilder
{
    fn as_ref(&self) -> &BotWriteAccessAllowReasonAddedToAttachmentMenu {
        &self.inner
    }
}

/// The user connected a website by logging in using Telegram Login Widget on it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotWriteAccessAllowReasonConnectedWebsite {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Domain name of the connected website

    #[serde(default)]
    domain_name: String,
}

impl RObject for BotWriteAccessAllowReasonConnectedWebsite {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotWriteAccessAllowReason for BotWriteAccessAllowReasonConnectedWebsite {}

impl BotWriteAccessAllowReasonConnectedWebsite {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BotWriteAccessAllowReasonConnectedWebsiteBuilder {
        let mut inner = BotWriteAccessAllowReasonConnectedWebsite::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BotWriteAccessAllowReasonConnectedWebsiteBuilder { inner }
    }

    pub fn domain_name(&self) -> &String {
        &self.domain_name
    }
}

#[doc(hidden)]
pub struct BotWriteAccessAllowReasonConnectedWebsiteBuilder {
    inner: BotWriteAccessAllowReasonConnectedWebsite,
}

#[deprecated]
pub type RTDBotWriteAccessAllowReasonConnectedWebsiteBuilder =
    BotWriteAccessAllowReasonConnectedWebsiteBuilder;

impl BotWriteAccessAllowReasonConnectedWebsiteBuilder {
    pub fn build(&self) -> BotWriteAccessAllowReasonConnectedWebsite {
        self.inner.clone()
    }

    pub fn domain_name<T: AsRef<str>>(&mut self, domain_name: T) -> &mut Self {
        self.inner.domain_name = domain_name.as_ref().to_string();
        self
    }
}

impl AsRef<BotWriteAccessAllowReasonConnectedWebsite>
    for BotWriteAccessAllowReasonConnectedWebsite
{
    fn as_ref(&self) -> &BotWriteAccessAllowReasonConnectedWebsite {
        self
    }
}

impl AsRef<BotWriteAccessAllowReasonConnectedWebsite>
    for BotWriteAccessAllowReasonConnectedWebsiteBuilder
{
    fn as_ref(&self) -> &BotWriteAccessAllowReasonConnectedWebsite {
        &self.inner
    }
}

/// The user launched a Web App using getWebAppLinkUrl
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotWriteAccessAllowReasonLaunchedWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Information about the Web App
    web_app: WebApp,
}

impl RObject for BotWriteAccessAllowReasonLaunchedWebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotWriteAccessAllowReason for BotWriteAccessAllowReasonLaunchedWebApp {}

impl BotWriteAccessAllowReasonLaunchedWebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BotWriteAccessAllowReasonLaunchedWebAppBuilder {
        let mut inner = BotWriteAccessAllowReasonLaunchedWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BotWriteAccessAllowReasonLaunchedWebAppBuilder { inner }
    }

    pub fn web_app(&self) -> &WebApp {
        &self.web_app
    }
}

#[doc(hidden)]
pub struct BotWriteAccessAllowReasonLaunchedWebAppBuilder {
    inner: BotWriteAccessAllowReasonLaunchedWebApp,
}

#[deprecated]
pub type RTDBotWriteAccessAllowReasonLaunchedWebAppBuilder =
    BotWriteAccessAllowReasonLaunchedWebAppBuilder;

impl BotWriteAccessAllowReasonLaunchedWebAppBuilder {
    pub fn build(&self) -> BotWriteAccessAllowReasonLaunchedWebApp {
        self.inner.clone()
    }

    pub fn web_app<T: AsRef<WebApp>>(&mut self, web_app: T) -> &mut Self {
        self.inner.web_app = web_app.as_ref().clone();
        self
    }
}

impl AsRef<BotWriteAccessAllowReasonLaunchedWebApp> for BotWriteAccessAllowReasonLaunchedWebApp {
    fn as_ref(&self) -> &BotWriteAccessAllowReasonLaunchedWebApp {
        self
    }
}

impl AsRef<BotWriteAccessAllowReasonLaunchedWebApp>
    for BotWriteAccessAllowReasonLaunchedWebAppBuilder
{
    fn as_ref(&self) -> &BotWriteAccessAllowReasonLaunchedWebApp {
        &self.inner
    }
}
