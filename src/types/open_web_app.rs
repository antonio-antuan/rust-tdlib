use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that a Web App is being opened from attachment menu, a botMenuButton button, an internalLinkTypeAttachmentMenuBot link, or an inlineKeyboardButtonTypeWebApp button. For each bot, a confirmation alert about data sent to the bot must be shown once
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which the Web App is opened
    chat_id: i64,
    /// Identifier of the bot, providing the Web App
    bot_user_id: i64,
    /// The URL from an inlineKeyboardButtonTypeWebApp button, a botMenuButton button, or an internalLinkTypeAttachmentMenuBot link, or an empty string otherwise
    url: String,
    /// Preferred Web App theme; pass null to use the default theme
    theme: ThemeParameters,
    /// Identifier of the replied message for the message sent by the Web App; 0 if none
    reply_to_message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for OpenWebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for OpenWebApp {}

impl OpenWebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDOpenWebAppBuilder {
        let mut inner = OpenWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "openWebApp".to_string();

        RTDOpenWebAppBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
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

    pub fn reply_to_message_id(&self) -> i64 {
        self.reply_to_message_id
    }
}

#[doc(hidden)]
pub struct RTDOpenWebAppBuilder {
    inner: OpenWebApp,
}

impl RTDOpenWebAppBuilder {
    pub fn build(&self) -> OpenWebApp {
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

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn theme<T: AsRef<ThemeParameters>>(&mut self, theme: T) -> &mut Self {
        self.inner.theme = theme.as_ref().clone();
        self
    }

    pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
        self.inner.reply_to_message_id = reply_to_message_id;
        self
    }
}

impl AsRef<OpenWebApp> for OpenWebApp {
    fn as_ref(&self) -> &OpenWebApp {
        self
    }
}

impl AsRef<OpenWebApp> for RTDOpenWebAppBuilder {
    fn as_ref(&self) -> &OpenWebApp {
        &self.inner
    }
}
