use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns the newly created chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateNewBasicGroupChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifiers of users to be added to the basic group; may be empty to create a basic group without other members

    #[serde(default)]
    user_ids: Vec<i64>,
    /// Title of the new basic group; 1-128 characters

    #[serde(default)]
    title: String,
    /// Message auto-delete time value, in seconds; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically

    #[serde(default)]
    message_auto_delete_time: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateNewBasicGroupChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateNewBasicGroupChat {}

impl CreateNewBasicGroupChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateNewBasicGroupChatBuilder {
        let mut inner = CreateNewBasicGroupChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createNewBasicGroupChat".to_string();

        CreateNewBasicGroupChatBuilder { inner }
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn message_auto_delete_time(&self) -> i32 {
        self.message_auto_delete_time
    }
}

#[doc(hidden)]
pub struct CreateNewBasicGroupChatBuilder {
    inner: CreateNewBasicGroupChat,
}

#[deprecated]
pub type RTDCreateNewBasicGroupChatBuilder = CreateNewBasicGroupChatBuilder;

impl CreateNewBasicGroupChatBuilder {
    pub fn build(&self) -> CreateNewBasicGroupChat {
        self.inner.clone()
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn message_auto_delete_time(&mut self, message_auto_delete_time: i32) -> &mut Self {
        self.inner.message_auto_delete_time = message_auto_delete_time;
        self
    }
}

impl AsRef<CreateNewBasicGroupChat> for CreateNewBasicGroupChat {
    fn as_ref(&self) -> &CreateNewBasicGroupChat {
        self
    }
}

impl AsRef<CreateNewBasicGroupChat> for CreateNewBasicGroupChatBuilder {
    fn as_ref(&self) -> &CreateNewBasicGroupChat {
        &self.inner
    }
}
