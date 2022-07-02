use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether the message history of a supergroup is available to new members; requires can_change_info administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupIsAllHistoryAvailable {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the supergroup

    #[serde(default)]
    supergroup_id: i64,
    /// The new value of is_all_history_available

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleSupergroupIsAllHistoryAvailableBuilder {
        let mut inner = ToggleSupergroupIsAllHistoryAvailable::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupIsAllHistoryAvailable".to_string();

        ToggleSupergroupIsAllHistoryAvailableBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn is_all_history_available(&self) -> bool {
        self.is_all_history_available
    }
}

#[doc(hidden)]
pub struct ToggleSupergroupIsAllHistoryAvailableBuilder {
    inner: ToggleSupergroupIsAllHistoryAvailable,
}

#[deprecated]
pub type RTDToggleSupergroupIsAllHistoryAvailableBuilder =
    ToggleSupergroupIsAllHistoryAvailableBuilder;

impl ToggleSupergroupIsAllHistoryAvailableBuilder {
    pub fn build(&self) -> ToggleSupergroupIsAllHistoryAvailable {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
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

impl AsRef<ToggleSupergroupIsAllHistoryAvailable> for ToggleSupergroupIsAllHistoryAvailableBuilder {
    fn as_ref(&self) -> &ToggleSupergroupIsAllHistoryAvailable {
        &self.inner
    }
}
