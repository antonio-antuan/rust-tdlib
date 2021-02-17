use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a user profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfilePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Photo identifier; 0 for an empty photo. Can be used to find a photo in a list of user profile photos

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// A small (160x160) user profile photo. The file can be downloaded only before the photo is changed
    small: File,
    /// A big (640x640) user profile photo. The file can be downloaded only before the photo is changed
    big: File,
    /// True, if the photo has animated variant
    has_animation: Option<bool>,
}

impl RObject for ProfilePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDProfilePhotoBuilder {
        let mut inner = ProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDProfilePhotoBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn small(&self) -> &File {
        &self.small
    }

    pub fn big(&self) -> &File {
        &self.big
    }

    pub fn has_animation(&self) -> &Option<bool> {
        &self.has_animation
    }
}

#[doc(hidden)]
pub struct RTDProfilePhotoBuilder {
    inner: ProfilePhoto,
}

impl RTDProfilePhotoBuilder {
    pub fn build(&self) -> ProfilePhoto {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn small<T: AsRef<File>>(&mut self, small: T) -> &mut Self {
        self.inner.small = small.as_ref().clone();
        self
    }

    pub fn big<T: AsRef<File>>(&mut self, big: T) -> &mut Self {
        self.inner.big = big.as_ref().clone();
        self
    }

    pub fn has_animation(&mut self, has_animation: bool) -> &mut Self {
        self.inner.has_animation = Some(has_animation);
        self
    }
}

impl AsRef<ProfilePhoto> for ProfilePhoto {
    fn as_ref(&self) -> &ProfilePhoto {
        self
    }
}

impl AsRef<ProfilePhoto> for RTDProfilePhotoBuilder {
    fn as_ref(&self) -> &ProfilePhoto {
        &self.inner
    }
}
