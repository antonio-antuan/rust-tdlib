use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether the message history of a supergroup is available to new members; requires can_change_info rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupIsAllHistoryAvailable {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the supergroup
    supergroup_id: i32,
    /// The new value of is_all_history_available
    is_all_history_available: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupIsAllHistoryAvailable {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupIsAllHistoryAvailable {}

impl ToggleSupergroupIsAllHistoryAvailable {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleSupergroupIsAllHistoryAvailableBuilder {
        let mut inner = ToggleSupergroupIsAllHistoryAvailable::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupIsAllHistoryAvailable".to_string();

        RTDToggleSupergroupIsAllHistoryAvailableBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i32 {
        self.supergroup_id
    }

    pub fn is_all_history_available(&self) -> bool {
        self.is_all_history_available
    }
}

#[doc(hidden)]
pub struct RTDToggleSupergroupIsAllHistoryAvailableBuilder {
    inner: ToggleSupergroupIsAllHistoryAvailable,
}

impl RTDToggleSupergroupIsAllHistoryAvailableBuilder {
    pub fn build(&self) -> ToggleSupergroupIsAllHistoryAvailable {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
        self.inner.is_all_history_available = is_all_history_available;
        self
    }
}

impl AsRef<ToggleSupergroupIsAllHistoryAvailable> for ToggleSupergroupIsAllHistoryAvailable {
    fn as_ref(&self) -> &ToggleSupergroupIsAllHistoryAvailable {
        self
    }
}

impl AsRef<ToggleSupergroupIsAllHistoryAvailable>
    for RTDToggleSupergroupIsAllHistoryAvailableBuilder
{
    fn as_ref(&self) -> &ToggleSupergroupIsAllHistoryAvailable {
        &self.inner
    }
}
