use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Generates a new invite link for a chat; the previously generated link is revoked. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenerateChatInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GenerateChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GenerateChatInviteLink {}

impl GenerateChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGenerateChatInviteLinkBuilder {
        let mut inner = GenerateChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "generateChatInviteLink".to_string();

        RTDGenerateChatInviteLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDGenerateChatInviteLinkBuilder {
    inner: GenerateChatInviteLink,
}

impl RTDGenerateChatInviteLinkBuilder {
    pub fn build(&self) -> GenerateChatInviteLink {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GenerateChatInviteLink> for GenerateChatInviteLink {
    fn as_ref(&self) -> &GenerateChatInviteLink {
        self
    }
}

impl AsRef<GenerateChatInviteLink> for RTDGenerateChatInviteLinkBuilder {
    fn as_ref(&self) -> &GenerateChatInviteLink {
        &self.inner
    }
}
