use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds a chat to a chat list. A chat can't be simultaneously in Main and Archive chat lists, so it is automatically removed from another one if needed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddChatToList {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The chat list. Use getChatListsToAddChat to get suitable chat lists

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddChatToList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddChatToList {}

impl AddChatToList {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddChatToListBuilder {
        let mut inner = AddChatToList::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addChatToList".to_string();

        AddChatToListBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }
}

#[doc(hidden)]
pub struct AddChatToListBuilder {
    inner: AddChatToList,
}

#[deprecated]
pub type RTDAddChatToListBuilder = AddChatToListBuilder;

impl AddChatToListBuilder {
    pub fn build(&self) -> AddChatToList {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }
}

impl AsRef<AddChatToList> for AddChatToList {
    fn as_ref(&self) -> &AddChatToList {
        self
    }
}

impl AsRef<AddChatToList> for AddChatToListBuilder {
    fn as_ref(&self) -> &AddChatToList {
        &self.inner
    }
}
