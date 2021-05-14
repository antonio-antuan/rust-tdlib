use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a hashtag from the list of recently used hashtags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveRecentHashtag {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Hashtag to delete
    hashtag: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveRecentHashtag {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveRecentHashtag {}

impl RemoveRecentHashtag {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveRecentHashtagBuilder {
        let mut inner = RemoveRecentHashtag::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeRecentHashtag".to_string();

        RTDRemoveRecentHashtagBuilder { inner }
    }

    pub fn hashtag(&self) -> &String {
        &self.hashtag
    }
}

#[doc(hidden)]
pub struct RTDRemoveRecentHashtagBuilder {
    inner: RemoveRecentHashtag,
}

impl RTDRemoveRecentHashtagBuilder {
    pub fn build(&self) -> RemoveRecentHashtag {
        self.inner.clone()
    }

    pub fn hashtag<T: AsRef<str>>(&mut self, hashtag: T) -> &mut Self {
        self.inner.hashtag = hashtag.as_ref().to_string();
        self
    }
}

impl AsRef<RemoveRecentHashtag> for RemoveRecentHashtag {
    fn as_ref(&self) -> &RemoveRecentHashtag {
        self
    }
}

impl AsRef<RemoveRecentHashtag> for RTDRemoveRecentHashtagBuilder {
    fn as_ref(&self) -> &RemoveRecentHashtag {
        &self.inner
    }
}
