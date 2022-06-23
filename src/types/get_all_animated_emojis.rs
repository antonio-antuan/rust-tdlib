use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns all emojis, which has a corresponding animated emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAllAnimatedEmojis {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAllAnimatedEmojis {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAllAnimatedEmojis {}

impl GetAllAnimatedEmojis {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetAllAnimatedEmojisBuilder {
        let mut inner = GetAllAnimatedEmojis::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAllAnimatedEmojis".to_string();

        RTDGetAllAnimatedEmojisBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetAllAnimatedEmojisBuilder {
    inner: GetAllAnimatedEmojis,
}

impl RTDGetAllAnimatedEmojisBuilder {
    pub fn build(&self) -> GetAllAnimatedEmojis {
        self.inner.clone()
    }
}

impl AsRef<GetAllAnimatedEmojis> for GetAllAnimatedEmojis {
    fn as_ref(&self) -> &GetAllAnimatedEmojis {
        self
    }
}

impl AsRef<GetAllAnimatedEmojis> for RTDGetAllAnimatedEmojisBuilder {
    fn as_ref(&self) -> &GetAllAnimatedEmojis {
        &self.inner
    }
}
