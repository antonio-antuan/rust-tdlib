use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds multiple new members to a chat. Currently, this method is only available for supergroups and channels. This method can't be used to join a chat. Members can't be added to a channel if it has more than 200 members
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddChatMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifiers of the users to be added to the chat. The maximum number of added users is 20 for supergroups and 100 for channels

    #[serde(default)]
    user_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddChatMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddChatMembers {}

impl AddChatMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddChatMembersBuilder {
        let mut inner = AddChatMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addChatMembers".to_string();

        AddChatMembersBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct AddChatMembersBuilder {
    inner: AddChatMembers,
}

#[deprecated]
pub type RTDAddChatMembersBuilder = AddChatMembersBuilder;

impl AddChatMembersBuilder {
    pub fn build(&self) -> AddChatMembers {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<AddChatMembers> for AddChatMembers {
    fn as_ref(&self) -> &AddChatMembers {
        self
    }
}

impl AsRef<AddChatMembers> for AddChatMembersBuilder {
    fn as_ref(&self) -> &AddChatMembers {
        &self.inner
    }
}
