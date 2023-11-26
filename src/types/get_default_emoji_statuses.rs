use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns default emoji statuses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDefaultEmojiStatuses {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetDefaultEmojiStatuses {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetDefaultEmojiStatuses {}

impl GetDefaultEmojiStatuses {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetDefaultEmojiStatusesBuilder {
        let mut inner = GetDefaultEmojiStatuses::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getDefaultEmojiStatuses".to_string();

        GetDefaultEmojiStatusesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetDefaultEmojiStatusesBuilder {
    inner: GetDefaultEmojiStatuses,
}

#[deprecated]
pub type RTDGetDefaultEmojiStatusesBuilder = GetDefaultEmojiStatusesBuilder;

impl GetDefaultEmojiStatusesBuilder {
    pub fn build(&self) -> GetDefaultEmojiStatuses {
        self.inner.clone()
    }
}

impl AsRef<GetDefaultEmojiStatuses> for GetDefaultEmojiStatuses {
    fn as_ref(&self) -> &GetDefaultEmojiStatuses {
        self
    }
}

impl AsRef<GetDefaultEmojiStatuses> for GetDefaultEmojiStatusesBuilder {
    fn as_ref(&self) -> &GetDefaultEmojiStatuses {
        &self.inner
    }
}
