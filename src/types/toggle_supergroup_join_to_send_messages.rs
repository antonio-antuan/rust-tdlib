use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether joining is mandatory to send messages to a discussion supergroup; requires can_restrict_members administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupJoinToSendMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup
    supergroup_id: i64,
    /// New value of join_to_send_messages
    join_to_send_messages: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupJoinToSendMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupJoinToSendMessages {}

impl ToggleSupergroupJoinToSendMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleSupergroupJoinToSendMessagesBuilder {
        let mut inner = ToggleSupergroupJoinToSendMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupJoinToSendMessages".to_string();

        RTDToggleSupergroupJoinToSendMessagesBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn join_to_send_messages(&self) -> bool {
        self.join_to_send_messages
    }
}

#[doc(hidden)]
pub struct RTDToggleSupergroupJoinToSendMessagesBuilder {
    inner: ToggleSupergroupJoinToSendMessages,
}

impl RTDToggleSupergroupJoinToSendMessagesBuilder {
    pub fn build(&self) -> ToggleSupergroupJoinToSendMessages {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn join_to_send_messages(&mut self, join_to_send_messages: bool) -> &mut Self {
        self.inner.join_to_send_messages = join_to_send_messages;
        self
    }
}

impl AsRef<ToggleSupergroupJoinToSendMessages> for ToggleSupergroupJoinToSendMessages {
    fn as_ref(&self) -> &ToggleSupergroupJoinToSendMessages {
        self
    }
}

impl AsRef<ToggleSupergroupJoinToSendMessages> for RTDToggleSupergroupJoinToSendMessagesBuilder {
    fn as_ref(&self) -> &ToggleSupergroupJoinToSendMessages {
        &self.inner
    }
}
