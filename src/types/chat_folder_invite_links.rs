use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of chat folder invite links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderInviteLinks {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of the invite links

    #[serde(default)]
    invite_links: Vec<ChatFolderInviteLink>,
}

impl RObject for ChatFolderInviteLinks {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatFolderInviteLinks {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatFolderInviteLinksBuilder {
        let mut inner = ChatFolderInviteLinks::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatFolderInviteLinksBuilder { inner }
    }

    pub fn invite_links(&self) -> &Vec<ChatFolderInviteLink> {
        &self.invite_links
    }
}

#[doc(hidden)]
pub struct ChatFolderInviteLinksBuilder {
    inner: ChatFolderInviteLinks,
}

#[deprecated]
pub type RTDChatFolderInviteLinksBuilder = ChatFolderInviteLinksBuilder;

impl ChatFolderInviteLinksBuilder {
    pub fn build(&self) -> ChatFolderInviteLinks {
        self.inner.clone()
    }

    pub fn invite_links(&mut self, invite_links: Vec<ChatFolderInviteLink>) -> &mut Self {
        self.inner.invite_links = invite_links;
        self
    }
}

impl AsRef<ChatFolderInviteLinks> for ChatFolderInviteLinks {
    fn as_ref(&self) -> &ChatFolderInviteLinks {
        self
    }
}

impl AsRef<ChatFolderInviteLinks> for ChatFolderInviteLinksBuilder {
    fn as_ref(&self) -> &ChatFolderInviteLinks {
        &self.inner
    }
}
