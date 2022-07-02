use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat members
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total count of chat members found

    #[serde(default)]
    total_count: i32,
    /// A list of chat members

    #[serde(default)]
    members: Vec<ChatMember>,
}

impl RObject for ChatMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMembersBuilder {
        let mut inner = ChatMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMembersBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn members(&self) -> &Vec<ChatMember> {
        &self.members
    }
}

#[doc(hidden)]
pub struct ChatMembersBuilder {
    inner: ChatMembers,
}

#[deprecated]
pub type RTDChatMembersBuilder = ChatMembersBuilder;

impl ChatMembersBuilder {
    pub fn build(&self) -> ChatMembers {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn members(&mut self, members: Vec<ChatMember>) -> &mut Self {
        self.inner.members = members;
        self
    }
}

impl AsRef<ChatMembers> for ChatMembers {
    fn as_ref(&self) -> &ChatMembers {
        self
    }
}

impl AsRef<ChatMembers> for ChatMembersBuilder {
    fn as_ref(&self) -> &ChatMembers {
        &self.inner
    }
}
