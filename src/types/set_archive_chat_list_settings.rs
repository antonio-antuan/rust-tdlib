use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes settings for automatic moving of chats to and from the Archive chat lists
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetArchiveChatListSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New settings
    settings: ArchiveChatListSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetArchiveChatListSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetArchiveChatListSettings {}

impl SetArchiveChatListSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetArchiveChatListSettingsBuilder {
        let mut inner = SetArchiveChatListSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setArchiveChatListSettings".to_string();

        SetArchiveChatListSettingsBuilder { inner }
    }

    pub fn settings(&self) -> &ArchiveChatListSettings {
        &self.settings
    }
}

#[doc(hidden)]
pub struct SetArchiveChatListSettingsBuilder {
    inner: SetArchiveChatListSettings,
}

#[deprecated]
pub type RTDSetArchiveChatListSettingsBuilder = SetArchiveChatListSettingsBuilder;

impl SetArchiveChatListSettingsBuilder {
    pub fn build(&self) -> SetArchiveChatListSettings {
        self.inner.clone()
    }

    pub fn settings<T: AsRef<ArchiveChatListSettings>>(&mut self, settings: T) -> &mut Self {
        self.inner.settings = settings.as_ref().clone();
        self
    }
}

impl AsRef<SetArchiveChatListSettings> for SetArchiveChatListSettings {
    fn as_ref(&self) -> &SetArchiveChatListSettings {
        self
    }
}

impl AsRef<SetArchiveChatListSettings> for SetArchiveChatListSettingsBuilder {
    fn as_ref(&self) -> &SetArchiveChatListSettings {
        &self.inner
    }
}
