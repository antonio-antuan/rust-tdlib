use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns list of saved notification sounds. If a sound isn't in the list, then default sound needs to be used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSavedNotificationSounds {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSavedNotificationSounds {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSavedNotificationSounds {}

impl GetSavedNotificationSounds {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetSavedNotificationSoundsBuilder {
        let mut inner = GetSavedNotificationSounds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSavedNotificationSounds".to_string();

        RTDGetSavedNotificationSoundsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetSavedNotificationSoundsBuilder {
    inner: GetSavedNotificationSounds,
}

impl RTDGetSavedNotificationSoundsBuilder {
    pub fn build(&self) -> GetSavedNotificationSounds {
        self.inner.clone()
    }
}

impl AsRef<GetSavedNotificationSounds> for GetSavedNotificationSounds {
    fn as_ref(&self) -> &GetSavedNotificationSounds {
        self
    }
}

impl AsRef<GetSavedNotificationSounds> for RTDGetSavedNotificationSoundsBuilder {
    fn as_ref(&self) -> &GetSavedNotificationSounds {
        &self.inner
    }
}
