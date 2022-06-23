use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Replaces current primary invite link for a chat with a new primary invite link. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplacePrimaryChatInviteLink {
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

impl RObject for ReplacePrimaryChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReplacePrimaryChatInviteLink {}

impl ReplacePrimaryChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReplacePrimaryChatInviteLinkBuilder {
        let mut inner = ReplacePrimaryChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "replacePrimaryChatInviteLink".to_string();

        RTDReplacePrimaryChatInviteLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDReplacePrimaryChatInviteLinkBuilder {
    inner: ReplacePrimaryChatInviteLink,
}

impl RTDReplacePrimaryChatInviteLinkBuilder {
    pub fn build(&self) -> ReplacePrimaryChatInviteLink {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<ReplacePrimaryChatInviteLink> for ReplacePrimaryChatInviteLink {
    fn as_ref(&self) -> &ReplacePrimaryChatInviteLink {
        self
    }
}

impl AsRef<ReplacePrimaryChatInviteLink> for RTDReplacePrimaryChatInviteLinkBuilder {
    fn as_ref(&self) -> &ReplacePrimaryChatInviteLink {
        &self.inner
    }
}
