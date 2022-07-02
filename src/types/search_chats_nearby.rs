use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of users and location-based supergroups nearby. The list of users nearby will be updated for 60 seconds after the request by the updates updateUsersNearby. The request must be sent again every 25 seconds with adjusted location to not miss new chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatsNearby {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current user location
    location: Location,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchChatsNearby {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchChatsNearby {}

impl SearchChatsNearby {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchChatsNearbyBuilder {
        let mut inner = SearchChatsNearby::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchChatsNearby".to_string();

        SearchChatsNearbyBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

#[doc(hidden)]
pub struct SearchChatsNearbyBuilder {
    inner: SearchChatsNearby,
}

#[deprecated]
pub type RTDSearchChatsNearbyBuilder = SearchChatsNearbyBuilder;

impl SearchChatsNearbyBuilder {
    pub fn build(&self) -> SearchChatsNearby {
        self.inner.clone()
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }
}

impl AsRef<SearchChatsNearby> for SearchChatsNearby {
    fn as_ref(&self) -> &SearchChatsNearby {
        self
    }
}

impl AsRef<SearchChatsNearby> for SearchChatsNearbyBuilder {
    fn as_ref(&self) -> &SearchChatsNearby {
        &self.inner
    }
}
