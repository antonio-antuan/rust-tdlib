use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of hashtags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Hashtags {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of hashtags

    #[serde(default)]
    hashtags: Vec<String>,
}

impl RObject for Hashtags {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Hashtags {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> HashtagsBuilder {
        let mut inner = Hashtags::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        HashtagsBuilder { inner }
    }

    pub fn hashtags(&self) -> &Vec<String> {
        &self.hashtags
    }
}

#[doc(hidden)]
pub struct HashtagsBuilder {
    inner: Hashtags,
}

#[deprecated]
pub type RTDHashtagsBuilder = HashtagsBuilder;

impl HashtagsBuilder {
    pub fn build(&self) -> Hashtags {
        self.inner.clone()
    }

    pub fn hashtags(&mut self, hashtags: Vec<String>) -> &mut Self {
        self.inner.hashtags = hashtags;
        self
    }
}

impl AsRef<Hashtags> for Hashtags {
    fn as_ref(&self) -> &Hashtags {
        self
    }
}

impl AsRef<Hashtags> for HashtagsBuilder {
    fn as_ref(&self) -> &Hashtags {
        &self.inner
    }
}
