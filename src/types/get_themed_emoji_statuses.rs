use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns up to 8 emoji statuses, which must be shown right after the default Premium Badge in the emoji status list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetThemedEmojiStatuses {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetThemedEmojiStatuses {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetThemedEmojiStatuses {}

impl GetThemedEmojiStatuses {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetThemedEmojiStatusesBuilder {
        let mut inner = GetThemedEmojiStatuses::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getThemedEmojiStatuses".to_string();

        GetThemedEmojiStatusesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetThemedEmojiStatusesBuilder {
    inner: GetThemedEmojiStatuses,
}

#[deprecated]
pub type RTDGetThemedEmojiStatusesBuilder = GetThemedEmojiStatusesBuilder;

impl GetThemedEmojiStatusesBuilder {
    pub fn build(&self) -> GetThemedEmojiStatuses {
        self.inner.clone()
    }
}

impl AsRef<GetThemedEmojiStatuses> for GetThemedEmojiStatuses {
    fn as_ref(&self) -> &GetThemedEmojiStatuses {
        self
    }
}

impl AsRef<GetThemedEmojiStatuses> for GetThemedEmojiStatusesBuilder {
    fn as_ref(&self) -> &GetThemedEmojiStatuses {
        &self.inner
    }
}
