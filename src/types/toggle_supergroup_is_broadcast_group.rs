use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Upgrades supergroup to a broadcast group; requires owner privileges in the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupIsBroadcastGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup
    supergroup_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupIsBroadcastGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupIsBroadcastGroup {}

impl ToggleSupergroupIsBroadcastGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleSupergroupIsBroadcastGroupBuilder {
        let mut inner = ToggleSupergroupIsBroadcastGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupIsBroadcastGroup".to_string();

        RTDToggleSupergroupIsBroadcastGroupBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }
}

#[doc(hidden)]
pub struct RTDToggleSupergroupIsBroadcastGroupBuilder {
    inner: ToggleSupergroupIsBroadcastGroup,
}

impl RTDToggleSupergroupIsBroadcastGroupBuilder {
    pub fn build(&self) -> ToggleSupergroupIsBroadcastGroup {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }
}

impl AsRef<ToggleSupergroupIsBroadcastGroup> for ToggleSupergroupIsBroadcastGroup {
    fn as_ref(&self) -> &ToggleSupergroupIsBroadcastGroup {
        self
    }
}

impl AsRef<ToggleSupergroupIsBroadcastGroup> for RTDToggleSupergroupIsBroadcastGroupBuilder {
    fn as_ref(&self) -> &ToggleSupergroupIsBroadcastGroup {
        &self.inner
    }
}
