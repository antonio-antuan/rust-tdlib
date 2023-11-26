use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of chats with non-default notification settings for stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStoryNotificationSettingsExceptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStoryNotificationSettingsExceptions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStoryNotificationSettingsExceptions {}

impl GetStoryNotificationSettingsExceptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetStoryNotificationSettingsExceptionsBuilder {
        let mut inner = GetStoryNotificationSettingsExceptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStoryNotificationSettingsExceptions".to_string();

        GetStoryNotificationSettingsExceptionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetStoryNotificationSettingsExceptionsBuilder {
    inner: GetStoryNotificationSettingsExceptions,
}

#[deprecated]
pub type RTDGetStoryNotificationSettingsExceptionsBuilder =
    GetStoryNotificationSettingsExceptionsBuilder;

impl GetStoryNotificationSettingsExceptionsBuilder {
    pub fn build(&self) -> GetStoryNotificationSettingsExceptions {
        self.inner.clone()
    }
}

impl AsRef<GetStoryNotificationSettingsExceptions> for GetStoryNotificationSettingsExceptions {
    fn as_ref(&self) -> &GetStoryNotificationSettingsExceptions {
        self
    }
}

impl AsRef<GetStoryNotificationSettingsExceptions>
    for GetStoryNotificationSettingsExceptionsBuilder
{
    fn as_ref(&self) -> &GetStoryNotificationSettingsExceptions {
        &self.inner
    }
}
