use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserProfilePhotos {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// The number of photos to skip; must be non-negative

    #[serde(default)]
    offset: i32,
    /// The maximum number of photos to be returned; up to 100

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetUserProfilePhotos {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetUserProfilePhotos {}

impl GetUserProfilePhotos {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetUserProfilePhotosBuilder {
        let mut inner = GetUserProfilePhotos::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getUserProfilePhotos".to_string();

        GetUserProfilePhotosBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetUserProfilePhotosBuilder {
    inner: GetUserProfilePhotos,
}

#[deprecated]
pub type RTDGetUserProfilePhotosBuilder = GetUserProfilePhotosBuilder;

impl GetUserProfilePhotosBuilder {
    pub fn build(&self) -> GetUserProfilePhotos {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetUserProfilePhotos> for GetUserProfilePhotos {
    fn as_ref(&self) -> &GetUserProfilePhotos {
        self
    }
}

impl AsRef<GetUserProfilePhotos> for GetUserProfilePhotosBuilder {
    fn as_ref(&self) -> &GetUserProfilePhotos {
        &self.inner
    }
}
