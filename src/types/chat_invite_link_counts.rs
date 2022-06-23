use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat invite link counts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkCounts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of invite link counts
    invite_link_counts: Vec<ChatInviteLinkCount>,
}

impl RObject for ChatInviteLinkCounts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatInviteLinkCounts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatInviteLinkCountsBuilder {
        let mut inner = ChatInviteLinkCounts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatInviteLinkCountsBuilder { inner }
    }

    pub fn invite_link_counts(&self) -> &Vec<ChatInviteLinkCount> {
        &self.invite_link_counts
    }
}

#[doc(hidden)]
pub struct RTDChatInviteLinkCountsBuilder {
    inner: ChatInviteLinkCounts,
}

impl RTDChatInviteLinkCountsBuilder {
    pub fn build(&self) -> ChatInviteLinkCounts {
        self.inner.clone()
    }

    pub fn invite_link_counts(
        &mut self,
        invite_link_counts: Vec<ChatInviteLinkCount>,
    ) -> &mut Self {
        self.inner.invite_link_counts = invite_link_counts;
        self
    }
}

impl AsRef<ChatInviteLinkCounts> for ChatInviteLinkCounts {
    fn as_ref(&self) -> &ChatInviteLinkCounts {
        self
    }
}

impl AsRef<ChatInviteLinkCounts> for RTDChatInviteLinkCountsBuilder {
    fn as_ref(&self) -> &ChatInviteLinkCounts {
        &self.inner
    }
}
