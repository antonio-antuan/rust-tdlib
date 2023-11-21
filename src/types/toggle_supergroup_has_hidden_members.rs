use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether non-administrators can receive only administrators and bots using getSupergroupMembers or searchChatMembers. Can be called only if supergroupFullInfo.can_hide_members == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupHasHiddenMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup

    #[serde(default)]
    supergroup_id: i64,
    /// New value of has_hidden_members

    #[serde(default)]
    has_hidden_members: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupHasHiddenMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupHasHiddenMembers {}

impl ToggleSupergroupHasHiddenMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleSupergroupHasHiddenMembersBuilder {
        let mut inner = ToggleSupergroupHasHiddenMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupHasHiddenMembers".to_string();

        ToggleSupergroupHasHiddenMembersBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn has_hidden_members(&self) -> bool {
        self.has_hidden_members
    }
}

#[doc(hidden)]
pub struct ToggleSupergroupHasHiddenMembersBuilder {
    inner: ToggleSupergroupHasHiddenMembers,
}

#[deprecated]
pub type RTDToggleSupergroupHasHiddenMembersBuilder = ToggleSupergroupHasHiddenMembersBuilder;

impl ToggleSupergroupHasHiddenMembersBuilder {
    pub fn build(&self) -> ToggleSupergroupHasHiddenMembers {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn has_hidden_members(&mut self, has_hidden_members: bool) -> &mut Self {
        self.inner.has_hidden_members = has_hidden_members;
        self
    }
}

impl AsRef<ToggleSupergroupHasHiddenMembers> for ToggleSupergroupHasHiddenMembers {
    fn as_ref(&self) -> &ToggleSupergroupHasHiddenMembers {
        self
    }
}

impl AsRef<ToggleSupergroupHasHiddenMembers> for ToggleSupergroupHasHiddenMembersBuilder {
    fn as_ref(&self) -> &ToggleSupergroupHasHiddenMembers {
        &self.inner
    }
}
