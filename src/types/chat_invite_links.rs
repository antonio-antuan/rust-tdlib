use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat invite links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinks {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total number of chat invite links found
    total_count: i32,
    /// List of invite links
    invite_links: Vec<ChatInviteLink>,
}

impl RObject for ChatInviteLinks {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatInviteLinks {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatInviteLinksBuilder {
        let mut inner = ChatInviteLinks::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatInviteLinksBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn invite_links(&self) -> &Vec<ChatInviteLink> {
        &self.invite_links
    }
}

#[doc(hidden)]
pub struct RTDChatInviteLinksBuilder {
    inner: ChatInviteLinks,
}

impl RTDChatInviteLinksBuilder {
    pub fn build(&self) -> ChatInviteLinks {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn invite_links(&mut self, invite_links: Vec<ChatInviteLink>) -> &mut Self {
        self.inner.invite_links = invite_links;
        self
    }
}

impl AsRef<ChatInviteLinks> for ChatInviteLinks {
    fn as_ref(&self) -> &ChatInviteLinks {
        self
    }
}

impl AsRef<ChatInviteLinks> for RTDChatInviteLinksBuilder {
    fn as_ref(&self) -> &ChatInviteLinks {
        &self.inner
    }
}
