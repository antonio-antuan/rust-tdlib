use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Hides a suggested action
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HideSuggestedAction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Suggested action to hide

    #[serde(skip_serializing_if = "SuggestedAction::_is_default")]
    action: SuggestedAction,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for HideSuggestedAction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for HideSuggestedAction {}

impl HideSuggestedAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> HideSuggestedActionBuilder {
        let mut inner = HideSuggestedAction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "hideSuggestedAction".to_string();

        HideSuggestedActionBuilder { inner }
    }

    pub fn action(&self) -> &SuggestedAction {
        &self.action
    }
}

#[doc(hidden)]
pub struct HideSuggestedActionBuilder {
    inner: HideSuggestedAction,
}

#[deprecated]
pub type RTDHideSuggestedActionBuilder = HideSuggestedActionBuilder;

impl HideSuggestedActionBuilder {
    pub fn build(&self) -> HideSuggestedAction {
        self.inner.clone()
    }

    pub fn action<T: AsRef<SuggestedAction>>(&mut self, action: T) -> &mut Self {
        self.inner.action = action.as_ref().clone();
        self
    }
}

impl AsRef<HideSuggestedAction> for HideSuggestedAction {
    fn as_ref(&self) -> &HideSuggestedAction {
        self
    }
}

impl AsRef<HideSuggestedAction> for HideSuggestedActionBuilder {
    fn as_ref(&self) -> &HideSuggestedAction {
        &self.inner
    }
}
