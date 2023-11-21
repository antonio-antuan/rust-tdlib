use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains autosave settings for a chat, which overrides default settings for the corresponding scope
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsException {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Autosave settings for the chat
    settings: ScopeAutosaveSettings,
}

impl RObject for AutosaveSettingsException {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AutosaveSettingsException {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AutosaveSettingsExceptionBuilder {
        let mut inner = AutosaveSettingsException::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AutosaveSettingsExceptionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn settings(&self) -> &ScopeAutosaveSettings {
        &self.settings
    }
}

#[doc(hidden)]
pub struct AutosaveSettingsExceptionBuilder {
    inner: AutosaveSettingsException,
}

#[deprecated]
pub type RTDAutosaveSettingsExceptionBuilder = AutosaveSettingsExceptionBuilder;

impl AutosaveSettingsExceptionBuilder {
    pub fn build(&self) -> AutosaveSettingsException {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn settings<T: AsRef<ScopeAutosaveSettings>>(&mut self, settings: T) -> &mut Self {
        self.inner.settings = settings.as_ref().clone();
        self
    }
}

impl AsRef<AutosaveSettingsException> for AutosaveSettingsException {
    fn as_ref(&self) -> &AutosaveSettingsException {
        self
    }
}

impl AsRef<AutosaveSettingsException> for AutosaveSettingsExceptionBuilder {
    fn as_ref(&self) -> &AutosaveSettingsException {
        &self.inner
    }
}
