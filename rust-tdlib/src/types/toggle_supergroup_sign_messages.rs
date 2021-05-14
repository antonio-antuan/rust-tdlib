use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Toggles sender signatures messages sent in a channel; requires can_change_info rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupSignMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel
    supergroup_id: i32,
    /// New value of sign_messages
    sign_messages: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupSignMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupSignMessages {}

impl ToggleSupergroupSignMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleSupergroupSignMessagesBuilder {
        let mut inner = ToggleSupergroupSignMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupSignMessages".to_string();

        RTDToggleSupergroupSignMessagesBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i32 {
        self.supergroup_id
    }

    pub fn sign_messages(&self) -> bool {
        self.sign_messages
    }
}

#[doc(hidden)]
pub struct RTDToggleSupergroupSignMessagesBuilder {
    inner: ToggleSupergroupSignMessages,
}

impl RTDToggleSupergroupSignMessagesBuilder {
    pub fn build(&self) -> ToggleSupergroupSignMessages {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
        self.inner.sign_messages = sign_messages;
        self
    }
}

impl AsRef<ToggleSupergroupSignMessages> for ToggleSupergroupSignMessages {
    fn as_ref(&self) -> &ToggleSupergroupSignMessages {
        self
    }
}

impl AsRef<ToggleSupergroupSignMessages> for RTDToggleSupergroupSignMessagesBuilder {
    fn as_ref(&self) -> &ToggleSupergroupSignMessages {
        &self.inner
    }
}
