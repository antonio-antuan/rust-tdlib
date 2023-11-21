use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a type of a button in results of inline query
pub trait TDInlineQueryResultsButtonType: Debug + RObject {}

/// Represents a type of a button in results of inline query
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum InlineQueryResultsButtonType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Describes the button that opens a private chat with the bot and sends a start message to the bot with the given parameter
    #[serde(rename = "inlineQueryResultsButtonTypeStartBot")]
    StartBot(InlineQueryResultsButtonTypeStartBot),
    /// Describes the button that opens a Web App by calling getWebAppUrl
    #[serde(rename = "inlineQueryResultsButtonTypeWebApp")]
    WebApp(InlineQueryResultsButtonTypeWebApp),
}

impl RObject for InlineQueryResultsButtonType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InlineQueryResultsButtonType::StartBot(t) => t.extra(),
            InlineQueryResultsButtonType::WebApp(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InlineQueryResultsButtonType::StartBot(t) => t.client_id(),
            InlineQueryResultsButtonType::WebApp(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InlineQueryResultsButtonType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InlineQueryResultsButtonType::_Default)
    }
}

impl AsRef<InlineQueryResultsButtonType> for InlineQueryResultsButtonType {
    fn as_ref(&self) -> &InlineQueryResultsButtonType {
        self
    }
}

/// Describes the button that opens a private chat with the bot and sends a start message to the bot with the given parameter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultsButtonTypeStartBot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The parameter for the bot start message

    #[serde(default)]
    parameter: String,
}

impl RObject for InlineQueryResultsButtonTypeStartBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResultsButtonType for InlineQueryResultsButtonTypeStartBot {}

impl InlineQueryResultsButtonTypeStartBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineQueryResultsButtonTypeStartBotBuilder {
        let mut inner = InlineQueryResultsButtonTypeStartBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineQueryResultsButtonTypeStartBotBuilder { inner }
    }

    pub fn parameter(&self) -> &String {
        &self.parameter
    }
}

#[doc(hidden)]
pub struct InlineQueryResultsButtonTypeStartBotBuilder {
    inner: InlineQueryResultsButtonTypeStartBot,
}

#[deprecated]
pub type RTDInlineQueryResultsButtonTypeStartBotBuilder =
    InlineQueryResultsButtonTypeStartBotBuilder;

impl InlineQueryResultsButtonTypeStartBotBuilder {
    pub fn build(&self) -> InlineQueryResultsButtonTypeStartBot {
        self.inner.clone()
    }

    pub fn parameter<T: AsRef<str>>(&mut self, parameter: T) -> &mut Self {
        self.inner.parameter = parameter.as_ref().to_string();
        self
    }
}

impl AsRef<InlineQueryResultsButtonTypeStartBot> for InlineQueryResultsButtonTypeStartBot {
    fn as_ref(&self) -> &InlineQueryResultsButtonTypeStartBot {
        self
    }
}

impl AsRef<InlineQueryResultsButtonTypeStartBot> for InlineQueryResultsButtonTypeStartBotBuilder {
    fn as_ref(&self) -> &InlineQueryResultsButtonTypeStartBot {
        &self.inner
    }
}

/// Describes the button that opens a Web App by calling getWebAppUrl
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultsButtonTypeWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// An HTTP URL to pass to getWebAppUrl

    #[serde(default)]
    url: String,
}

impl RObject for InlineQueryResultsButtonTypeWebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResultsButtonType for InlineQueryResultsButtonTypeWebApp {}

impl InlineQueryResultsButtonTypeWebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineQueryResultsButtonTypeWebAppBuilder {
        let mut inner = InlineQueryResultsButtonTypeWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineQueryResultsButtonTypeWebAppBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct InlineQueryResultsButtonTypeWebAppBuilder {
    inner: InlineQueryResultsButtonTypeWebApp,
}

#[deprecated]
pub type RTDInlineQueryResultsButtonTypeWebAppBuilder = InlineQueryResultsButtonTypeWebAppBuilder;

impl InlineQueryResultsButtonTypeWebAppBuilder {
    pub fn build(&self) -> InlineQueryResultsButtonTypeWebApp {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<InlineQueryResultsButtonTypeWebApp> for InlineQueryResultsButtonTypeWebApp {
    fn as_ref(&self) -> &InlineQueryResultsButtonTypeWebApp {
        self
    }
}

impl AsRef<InlineQueryResultsButtonTypeWebApp> for InlineQueryResultsButtonTypeWebAppBuilder {
    fn as_ref(&self) -> &InlineQueryResultsButtonTypeWebApp {
        &self.inner
    }
}
