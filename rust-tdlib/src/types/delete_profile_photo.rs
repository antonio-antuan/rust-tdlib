use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes a profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteProfilePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the profile photo to delete

    #[serde(deserialize_with = "super::_common::number_from_string")]
    profile_photo_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteProfilePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteProfilePhoto {}

impl DeleteProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteProfilePhotoBuilder {
        let mut inner = DeleteProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteProfilePhoto".to_string();

        RTDDeleteProfilePhotoBuilder { inner }
    }

    pub fn profile_photo_id(&self) -> i64 {
        self.profile_photo_id
    }
}

#[doc(hidden)]
pub struct RTDDeleteProfilePhotoBuilder {
    inner: DeleteProfilePhoto,
}

impl RTDDeleteProfilePhotoBuilder {
    pub fn build(&self) -> DeleteProfilePhoto {
        self.inner.clone()
    }

    pub fn profile_photo_id(&mut self, profile_photo_id: i64) -> &mut Self {
        self.inner.profile_photo_id = profile_photo_id;
        self
    }
}

impl AsRef<DeleteProfilePhoto> for DeleteProfilePhoto {
    fn as_ref(&self) -> &DeleteProfilePhoto {
        self
    }
}

impl AsRef<DeleteProfilePhoto> for RTDDeleteProfilePhotoBuilder {
    fn as_ref(&self) -> &DeleteProfilePhoto {
        &self.inner
    }
}
