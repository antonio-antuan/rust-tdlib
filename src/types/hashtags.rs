use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDHashtagsBuilder {
        let mut inner = Hashtags::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDHashtagsBuilder { inner }
    }

    pub fn hashtags(&self) -> &Vec<String> {
        &self.hashtags
    }
}

#[doc(hidden)]
pub struct RTDHashtagsBuilder {
    inner: Hashtags,
}

impl RTDHashtagsBuilder {
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

impl AsRef<Hashtags> for RTDHashtagsBuilder {
    fn as_ref(&self) -> &Hashtags {
        &self.inner
    }
}
