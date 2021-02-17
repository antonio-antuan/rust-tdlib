use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatDescription {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat
    chat_id: i64,
    /// Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info rights
    description: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatDescription {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatDescription {}

impl SetChatDescription {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatDescriptionBuilder {
        let mut inner = SetChatDescription::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatDescription".to_string();

        RTDSetChatDescriptionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct RTDSetChatDescriptionBuilder {
    inner: SetChatDescription,
}

impl RTDSetChatDescriptionBuilder {
    pub fn build(&self) -> SetChatDescription {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<SetChatDescription> for SetChatDescription {
    fn as_ref(&self) -> &SetChatDescription {
        self
    }
}

impl AsRef<SetChatDescription> for RTDSetChatDescriptionBuilder {
    fn as_ref(&self) -> &SetChatDescription {
        &self.inner
    }
}
