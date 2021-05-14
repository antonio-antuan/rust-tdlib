use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatTitle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New title of the chat; 1-128 characters
    title: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatTitle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatTitle {}

impl SetChatTitle {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatTitleBuilder {
        let mut inner = SetChatTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatTitle".to_string();

        RTDSetChatTitleBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct RTDSetChatTitleBuilder {
    inner: SetChatTitle,
}

impl RTDSetChatTitleBuilder {
    pub fn build(&self) -> SetChatTitle {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<SetChatTitle> for SetChatTitle {
    fn as_ref(&self) -> &SetChatTitle {
        self
    }
}

impl AsRef<SetChatTitle> for RTDSetChatTitleBuilder {
    fn as_ref(&self) -> &SetChatTitle {
        &self.inner
    }
}
