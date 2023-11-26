use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that a Web App is being opened from the attachment menu, a botMenuButton button, an internalLinkTypeAttachmentMenuBot link, or an inlineKeyboardButtonTypeWebApp button. For each bot, a confirmation alert about data sent to the bot must be shown once
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat in which the Web App is opened. The Web App can't be opened in secret chats

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the bot, providing the Web App

    #[serde(default)]
    bot_user_id: i64,
    /// The URL from an inlineKeyboardButtonTypeWebApp button, a botMenuButton button, an internalLinkTypeAttachmentMenuBot link, or an empty string otherwise

    #[serde(default)]
    url: String,
    /// Preferred Web App theme; pass null to use the default theme
    theme: ThemeParameters,
    /// Short name of the application; 0-64 English letters, digits, and underscores

    #[serde(default)]
    application_name: String,
    /// If not 0, a message thread identifier in which the message will be sent

    #[serde(default)]
    message_thread_id: i64,
    /// Information about the message or story to be replied in the message sent by the Web App; pass null if none

    #[serde(skip_serializing_if = "InputMessageReplyTo::_is_default")]
    reply_to: InputMessageReplyTo,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OpenWebAppBuilder {
        let mut inner = OpenWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "openWebApp".to_string();

        OpenWebAppBuilder { inner }
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

    pub fn application_name(&self) -> &String {
        &self.application_name
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn reply_to(&self) -> &InputMessageReplyTo {
        &self.reply_to
    }
}

#[doc(hidden)]
pub struct OpenWebAppBuilder {
    inner: OpenWebApp,
}

#[deprecated]
pub type RTDOpenWebAppBuilder = OpenWebAppBuilder;

impl OpenWebAppBuilder {
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

    pub fn application_name<T: AsRef<str>>(&mut self, application_name: T) -> &mut Self {
        self.inner.application_name = application_name.as_ref().to_string();
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn reply_to<T: AsRef<InputMessageReplyTo>>(&mut self, reply_to: T) -> &mut Self {
        self.inner.reply_to = reply_to.as_ref().clone();
        self
    }
}

impl AsRef<OpenWebApp> for OpenWebApp {
    fn as_ref(&self) -> &OpenWebApp {
        self
    }
}

impl AsRef<OpenWebApp> for OpenWebAppBuilder {
    fn as_ref(&self) -> &OpenWebApp {
        &self.inner
    }
}
