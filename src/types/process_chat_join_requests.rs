use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Handles all pending join requests for a given link in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessChatJoinRequests {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Invite link for which to process join requests. If empty, all join requests will be processed. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links

    #[serde(default)]
    invite_link: String,
    /// True, if the requests are approved. Otherwise the requests are declived

    #[serde(default)]
    approve: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ProcessChatJoinRequests {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ProcessChatJoinRequests {}

impl ProcessChatJoinRequests {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ProcessChatJoinRequestsBuilder {
        let mut inner = ProcessChatJoinRequests::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "processChatJoinRequests".to_string();

        ProcessChatJoinRequestsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }

    pub fn approve(&self) -> bool {
        self.approve
    }
}

#[doc(hidden)]
pub struct ProcessChatJoinRequestsBuilder {
    inner: ProcessChatJoinRequests,
}

#[deprecated]
pub type RTDProcessChatJoinRequestsBuilder = ProcessChatJoinRequestsBuilder;

impl ProcessChatJoinRequestsBuilder {
    pub fn build(&self) -> ProcessChatJoinRequests {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }

    pub fn approve(&mut self, approve: bool) -> &mut Self {
        self.inner.approve = approve;
        self
    }
}

impl AsRef<ProcessChatJoinRequests> for ProcessChatJoinRequests {
    fn as_ref(&self) -> &ProcessChatJoinRequests {
        self
    }
}

impl AsRef<ProcessChatJoinRequests> for ProcessChatJoinRequestsBuilder {
    fn as_ref(&self) -> &ProcessChatJoinRequests {
        &self.inner
    }
}
