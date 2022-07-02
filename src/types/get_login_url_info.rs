use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLoginUrlInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the message with the button

    #[serde(default)]
    chat_id: i64,
    /// Message identifier of the message with the button

    #[serde(default)]
    message_id: i64,
    /// Button identifier

    #[serde(default)]
    button_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLoginUrlInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLoginUrlInfo for GetLoginUrlInfo {}

impl RFunction for GetLoginUrlInfo {}

impl GetLoginUrlInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetLoginUrlInfoBuilder {
        let mut inner = GetLoginUrlInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLoginUrlInfo".to_string();

        GetLoginUrlInfoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn button_id(&self) -> i64 {
        self.button_id
    }
}

#[doc(hidden)]
pub struct GetLoginUrlInfoBuilder {
    inner: GetLoginUrlInfo,
}

#[deprecated]
pub type RTDGetLoginUrlInfoBuilder = GetLoginUrlInfoBuilder;

impl GetLoginUrlInfoBuilder {
    pub fn build(&self) -> GetLoginUrlInfo {
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

    pub fn button_id(&mut self, button_id: i64) -> &mut Self {
        self.inner.button_id = button_id;
        self
    }
}

impl AsRef<GetLoginUrlInfo> for GetLoginUrlInfo {
    fn as_ref(&self) -> &GetLoginUrlInfo {
        self
    }
}

impl AsRef<GetLoginUrlInfo> for GetLoginUrlInfoBuilder {
    fn as_ref(&self) -> &GetLoginUrlInfo {
        &self.inner
    }
}
