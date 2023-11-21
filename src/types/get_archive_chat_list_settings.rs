use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns settings for automatic moving of chats to and from the Archive chat lists
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArchiveChatListSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetArchiveChatListSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetArchiveChatListSettings {}

impl GetArchiveChatListSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetArchiveChatListSettingsBuilder {
        let mut inner = GetArchiveChatListSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getArchiveChatListSettings".to_string();

        GetArchiveChatListSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetArchiveChatListSettingsBuilder {
    inner: GetArchiveChatListSettings,
}

#[deprecated]
pub type RTDGetArchiveChatListSettingsBuilder = GetArchiveChatListSettingsBuilder;

impl GetArchiveChatListSettingsBuilder {
    pub fn build(&self) -> GetArchiveChatListSettings {
        self.inner.clone()
    }
}

impl AsRef<GetArchiveChatListSettings> for GetArchiveChatListSettings {
    fn as_ref(&self) -> &GetArchiveChatListSettings {
        self
    }
}

impl AsRef<GetArchiveChatListSettings> for GetArchiveChatListSettingsBuilder {
    fn as_ref(&self) -> &GetArchiveChatListSettings {
        &self.inner
    }
}
