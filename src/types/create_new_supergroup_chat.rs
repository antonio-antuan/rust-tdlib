use crate::errors::*;
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
    title: String,
    /// True, if a channel chat should be created
    is_channel: bool,
    /// Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
    description: String,
    /// Chat location if a location-based supergroup is being created
    location: ChatLocation,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateNewSupergroupChatBuilder {
        let mut inner = CreateNewSupergroupChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createNewSupergroupChat".to_string();

        RTDCreateNewSupergroupChatBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
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
}

#[doc(hidden)]
pub struct RTDCreateNewSupergroupChatBuilder {
    inner: CreateNewSupergroupChat,
}

impl RTDCreateNewSupergroupChatBuilder {
    pub fn build(&self) -> CreateNewSupergroupChat {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
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
}

impl AsRef<CreateNewSupergroupChat> for CreateNewSupergroupChat {
    fn as_ref(&self) -> &CreateNewSupergroupChat {
        self
    }
}

impl AsRef<CreateNewSupergroupChat> for RTDCreateNewSupergroupChatBuilder {
    fn as_ref(&self) -> &CreateNewSupergroupChat {
        &self.inner
    }
}
