use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes a profile photo for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetProfilePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Profile photo to set

    #[serde(skip_serializing_if = "InputChatPhoto::_is_default")]
    photo: InputChatPhoto,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetProfilePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetProfilePhoto {}

impl SetProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetProfilePhotoBuilder {
        let mut inner = SetProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setProfilePhoto".to_string();

        RTDSetProfilePhotoBuilder { inner }
    }

    pub fn photo(&self) -> &InputChatPhoto {
        &self.photo
    }
}

#[doc(hidden)]
pub struct RTDSetProfilePhotoBuilder {
    inner: SetProfilePhoto,
}

impl RTDSetProfilePhotoBuilder {
    pub fn build(&self) -> SetProfilePhoto {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<InputChatPhoto>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }
}

impl AsRef<SetProfilePhoto> for SetProfilePhoto {
    fn as_ref(&self) -> &SetProfilePhoto {
        self
    }
}

impl AsRef<SetProfilePhoto> for RTDSetProfilePhotoBuilder {
    fn as_ref(&self) -> &SetProfilePhoto {
        &self.inner
    }
}
