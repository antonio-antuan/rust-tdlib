use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl. Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLoginUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the message with the button
    chat_id: i64,
    /// Message identifier of the message with the button
    message_id: i64,
    /// Button identifier
    button_id: i32,
    /// True, if the user allowed the bot to send them messages
    allow_write_access: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLoginUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetLoginUrl {}

impl GetLoginUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetLoginUrlBuilder {
        let mut inner = GetLoginUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLoginUrl".to_string();

        RTDGetLoginUrlBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn button_id(&self) -> i32 {
        self.button_id
    }

    pub fn allow_write_access(&self) -> bool {
        self.allow_write_access
    }
}

#[doc(hidden)]
pub struct RTDGetLoginUrlBuilder {
    inner: GetLoginUrl,
}

impl RTDGetLoginUrlBuilder {
    pub fn build(&self) -> GetLoginUrl {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn button_id(&mut self, button_id: i32) -> &mut Self {
        self.inner.button_id = button_id;
        self
    }

    pub fn allow_write_access(&mut self, allow_write_access: bool) -> &mut Self {
        self.inner.allow_write_access = allow_write_access;
        self
    }
}

impl AsRef<GetLoginUrl> for GetLoginUrl {
    fn as_ref(&self) -> &GetLoginUrl {
        self
    }
}

impl AsRef<GetLoginUrl> for RTDGetLoginUrlBuilder {
    fn as_ref(&self) -> &GetLoginUrl {
        &self.inner
    }
}
