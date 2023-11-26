use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateNewSupergroupChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Title of the new chat; 1-128 characters

    #[serde(default)]
    title: String,
    /// Pass true to create a forum supergroup chat

    #[serde(default)]
    is_forum: bool,
    /// Pass true to create a channel chat; ignored if a forum is created

    #[serde(default)]
    is_channel: bool,
    /// Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat

    #[serde(default)]
    description: String,
    /// Chat location if a location-based supergroup is being created; pass null to create an ordinary supergroup chat
    location: ChatLocation,
    /// Message auto-delete time value, in seconds; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically

    #[serde(default)]
    message_auto_delete_time: i32,
    /// Pass true to create a supergroup for importing messages using importMessages

    #[serde(default)]
    for_import: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateNewSupergroupChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateNewSupergroupChat {}

impl CreateNewSupergroupChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateNewSupergroupChatBuilder {
        let mut inner = CreateNewSupergroupChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createNewSupergroupChat".to_string();

        CreateNewSupergroupChatBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn is_forum(&self) -> bool {
        self.is_forum
    }

    pub fn is_channel(&self) -> bool {
        self.is_channel
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn location(&self) -> &ChatLocation {
        &self.location
    }

    pub fn message_auto_delete_time(&self) -> i32 {
        self.message_auto_delete_time
    }

    pub fn for_import(&self) -> bool {
        self.for_import
    }
}

#[doc(hidden)]
pub struct CreateNewSupergroupChatBuilder {
    inner: CreateNewSupergroupChat,
}

#[deprecated]
pub type RTDCreateNewSupergroupChatBuilder = CreateNewSupergroupChatBuilder;

impl CreateNewSupergroupChatBuilder {
    pub fn build(&self) -> CreateNewSupergroupChat {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn is_forum(&mut self, is_forum: bool) -> &mut Self {
        self.inner.is_forum = is_forum;
        self
    }

    pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
        self.inner.is_channel = is_channel;
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn location<T: AsRef<ChatLocation>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }

    pub fn message_auto_delete_time(&mut self, message_auto_delete_time: i32) -> &mut Self {
        self.inner.message_auto_delete_time = message_auto_delete_time;
        self
    }

    pub fn for_import(&mut self, for_import: bool) -> &mut Self {
        self.inner.for_import = for_import;
        self
    }
}

impl AsRef<CreateNewSupergroupChat> for CreateNewSupergroupChat {
    fn as_ref(&self) -> &CreateNewSupergroupChat {
        self
    }
}

impl AsRef<CreateNewSupergroupChat> for CreateNewSupergroupChatBuilder {
    fn as_ref(&self) -> &CreateNewSupergroupChat {
        &self.inner
    }
}
