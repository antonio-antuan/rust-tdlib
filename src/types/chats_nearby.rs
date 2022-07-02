use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of chats located nearby
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatsNearby {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of users nearby

    #[serde(default)]
    users_nearby: Vec<ChatNearby>,
    /// List of location-based supergroups nearby

    #[serde(default)]
    supergroups_nearby: Vec<ChatNearby>,
}

impl RObject for ChatsNearby {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatsNearby {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatsNearbyBuilder {
        let mut inner = ChatsNearby::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatsNearbyBuilder { inner }
    }

    pub fn users_nearby(&self) -> &Vec<ChatNearby> {
        &self.users_nearby
    }

    pub fn supergroups_nearby(&self) -> &Vec<ChatNearby> {
        &self.supergroups_nearby
    }
}

#[doc(hidden)]
pub struct ChatsNearbyBuilder {
    inner: ChatsNearby,
}

#[deprecated]
pub type RTDChatsNearbyBuilder = ChatsNearbyBuilder;

impl ChatsNearbyBuilder {
    pub fn build(&self) -> ChatsNearby {
        self.inner.clone()
    }

    pub fn users_nearby(&mut self, users_nearby: Vec<ChatNearby>) -> &mut Self {
        self.inner.users_nearby = users_nearby;
        self
    }

    pub fn supergroups_nearby(&mut self, supergroups_nearby: Vec<ChatNearby>) -> &mut Self {
        self.inner.supergroups_nearby = supergroups_nearby;
        self
    }
}

impl AsRef<ChatsNearby> for ChatsNearby {
    fn as_ref(&self) -> &ChatsNearby {
        self
    }
}

impl AsRef<ChatsNearby> for ChatsNearbyBuilder {
    fn as_ref(&self) -> &ChatsNearby {
        &self.inner
    }
}
