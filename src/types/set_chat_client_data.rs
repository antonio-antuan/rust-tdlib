use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes application-specific data associated with a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatClientData {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of client_data

    #[serde(default)]
    client_data: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatClientData {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatClientData {}

impl SetChatClientData {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatClientDataBuilder {
        let mut inner = SetChatClientData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatClientData".to_string();

        SetChatClientDataBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn client_data(&self) -> &String {
        &self.client_data
    }
}

#[doc(hidden)]
pub struct SetChatClientDataBuilder {
    inner: SetChatClientData,
}

#[deprecated]
pub type RTDSetChatClientDataBuilder = SetChatClientDataBuilder;

impl SetChatClientDataBuilder {
    pub fn build(&self) -> SetChatClientData {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn client_data<T: AsRef<str>>(&mut self, client_data: T) -> &mut Self {
        self.inner.client_data = client_data.as_ref().to_string();
        self
    }
}

impl AsRef<SetChatClientData> for SetChatClientData {
    fn as_ref(&self) -> &SetChatClientData {
        self
    }
}

impl AsRef<SetChatClientData> for SetChatClientDataBuilder {
    fn as_ref(&self) -> &SetChatClientData {
        &self.inner
    }
}
