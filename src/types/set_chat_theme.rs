use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the chat theme. Supported only in private and secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatTheme {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Name of the new chat theme; pass an empty string to return the default theme
    theme_name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatTheme {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatTheme {}

impl SetChatTheme {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatThemeBuilder {
        let mut inner = SetChatTheme::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatTheme".to_string();

        RTDSetChatThemeBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn theme_name(&self) -> &String {
        &self.theme_name
    }
}

#[doc(hidden)]
pub struct RTDSetChatThemeBuilder {
    inner: SetChatTheme,
}

impl RTDSetChatThemeBuilder {
    pub fn build(&self) -> SetChatTheme {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn theme_name<T: AsRef<str>>(&mut self, theme_name: T) -> &mut Self {
        self.inner.theme_name = theme_name.as_ref().to_string();
        self
    }
}

impl AsRef<SetChatTheme> for SetChatTheme {
    fn as_ref(&self) -> &SetChatTheme {
        self
    }
}

impl AsRef<SetChatTheme> for RTDSetChatThemeBuilder {
    fn as_ref(&self) -> &SetChatTheme {
        &self.inner
    }
}