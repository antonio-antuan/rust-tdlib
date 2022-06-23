use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Handles a pending join request in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessChatJoinRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Identifier of the user that sent the request
    user_id: i64,
    /// Pass true to approve the request; pass false to decline it
    approve: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ProcessChatJoinRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ProcessChatJoinRequest {}

impl ProcessChatJoinRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDProcessChatJoinRequestBuilder {
        let mut inner = ProcessChatJoinRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "processChatJoinRequest".to_string();

        RTDProcessChatJoinRequestBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn approve(&self) -> bool {
        self.approve
    }
}

#[doc(hidden)]
pub struct RTDProcessChatJoinRequestBuilder {
    inner: ProcessChatJoinRequest,
}

impl RTDProcessChatJoinRequestBuilder {
    pub fn build(&self) -> ProcessChatJoinRequest {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn approve(&mut self, approve: bool) -> &mut Self {
        self.inner.approve = approve;
        self
    }
}

impl AsRef<ProcessChatJoinRequest> for ProcessChatJoinRequest {
    fn as_ref(&self) -> &ProcessChatJoinRequest {
        self
    }
}

impl AsRef<ProcessChatJoinRequest> for RTDProcessChatJoinRequestBuilder {
    fn as_ref(&self) -> &ProcessChatJoinRequest {
        &self.inner
    }
}
