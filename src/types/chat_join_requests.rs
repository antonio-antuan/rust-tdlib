use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of requests to join a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequests {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total count of requests found

    #[serde(default)]
    total_count: i32,
    /// List of the requests

    #[serde(default)]
    requests: Vec<ChatJoinRequest>,
}

impl RObject for ChatJoinRequests {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatJoinRequests {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatJoinRequestsBuilder {
        let mut inner = ChatJoinRequests::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatJoinRequestsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn requests(&self) -> &Vec<ChatJoinRequest> {
        &self.requests
    }
}

#[doc(hidden)]
pub struct RTDChatJoinRequestsBuilder {
    inner: ChatJoinRequests,
}

impl RTDChatJoinRequestsBuilder {
    pub fn build(&self) -> ChatJoinRequests {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn requests(&mut self, requests: Vec<ChatJoinRequest>) -> &mut Self {
        self.inner.requests = requests;
        self
    }
}

impl AsRef<ChatJoinRequests> for ChatJoinRequests {
    fn as_ref(&self) -> &ChatJoinRequests {
        self
    }
}

impl AsRef<ChatJoinRequests> for RTDChatJoinRequestsBuilder {
    fn as_ref(&self) -> &ChatJoinRequests {
        &self.inner
    }
}
