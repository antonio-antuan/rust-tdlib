use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a location to which a chat is connected
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The location
    location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    address: String,
}

impl RObject for ChatLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatLocationBuilder {
        let mut inner = ChatLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatLocationBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn address(&self) -> &String {
        &self.address
    }
}

#[doc(hidden)]
pub struct RTDChatLocationBuilder {
    inner: ChatLocation,
}

impl RTDChatLocationBuilder {
    pub fn build(&self) -> ChatLocation {
        self.inner.clone()
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }

    pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
        self.inner.address = address.as_ref().to_string();
        self
    }
}

impl AsRef<ChatLocation> for ChatLocation {
    fn as_ref(&self) -> &ChatLocation {
        self
    }
}

impl AsRef<ChatLocation> for RTDChatLocationBuilder {
    fn as_ref(&self) -> &ChatLocation {
        &self.inner
    }
}
