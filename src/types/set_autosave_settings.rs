use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets autosave settings for the given scope. The method is guaranteed to work only after at least one call to getAutosaveSettings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAutosaveSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Autosave settings scope

    #[serde(skip_serializing_if = "AutosaveSettingsScope::_is_default")]
    scope: AutosaveSettingsScope,
    /// New autosave settings for the scope; pass null to set autosave settings to default
    settings: ScopeAutosaveSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetAutosaveSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetAutosaveSettings {}

impl SetAutosaveSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetAutosaveSettingsBuilder {
        let mut inner = SetAutosaveSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setAutosaveSettings".to_string();

        SetAutosaveSettingsBuilder { inner }
    }

    pub fn scope(&self) -> &AutosaveSettingsScope {
        &self.scope
    }

    pub fn settings(&self) -> &ScopeAutosaveSettings {
        &self.settings
    }
}

#[doc(hidden)]
pub struct SetAutosaveSettingsBuilder {
    inner: SetAutosaveSettings,
}

#[deprecated]
pub type RTDSetAutosaveSettingsBuilder = SetAutosaveSettingsBuilder;

impl SetAutosaveSettingsBuilder {
    pub fn build(&self) -> SetAutosaveSettings {
        self.inner.clone()
    }

    pub fn scope<T: AsRef<AutosaveSettingsScope>>(&mut self, scope: T) -> &mut Self {
        self.inner.scope = scope.as_ref().clone();
        self
    }

    pub fn settings<T: AsRef<ScopeAutosaveSettings>>(&mut self, settings: T) -> &mut Self {
        self.inner.settings = settings.as_ref().clone();
        self
    }
}

impl AsRef<SetAutosaveSettings> for SetAutosaveSettings {
    fn as_ref(&self) -> &SetAutosaveSettings {
        self
    }
}

impl AsRef<SetAutosaveSettings> for SetAutosaveSettingsBuilder {
    fn as_ref(&self) -> &SetAutosaveSettings {
        &self.inner
    }
}
