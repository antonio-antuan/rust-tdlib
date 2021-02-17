use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of basic group and supergroup chats, which can be used as a discussion group for a channel. Returned basic group chats must be first upgraded to supergroups before they can be set as a discussion group. To set a returned supergroup as a discussion group, access to its old messages must be enabled using toggleSupergroupIsAllHistoryAvailable first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSuitableDiscussionChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSuitableDiscussionChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSuitableDiscussionChats {}

impl GetSuitableDiscussionChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetSuitableDiscussionChatsBuilder {
        let mut inner = GetSuitableDiscussionChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSuitableDiscussionChats".to_string();

        RTDGetSuitableDiscussionChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetSuitableDiscussionChatsBuilder {
    inner: GetSuitableDiscussionChats,
}

impl RTDGetSuitableDiscussionChatsBuilder {
    pub fn build(&self) -> GetSuitableDiscussionChats {
        self.inner.clone()
    }
}

impl AsRef<GetSuitableDiscussionChats> for GetSuitableDiscussionChats {
    fn as_ref(&self) -> &GetSuitableDiscussionChats {
        self
    }
}

impl AsRef<GetSuitableDiscussionChats> for RTDGetSuitableDiscussionChatsBuilder {
    fn as_ref(&self) -> &GetSuitableDiscussionChats {
        &self.inner
    }
}
