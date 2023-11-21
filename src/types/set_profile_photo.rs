use crate::errors::Result;
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
    /// Pass true to set a public photo, which will be visible even the main photo is hidden by privacy settings

    #[serde(default)]
    is_public: bool,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetProfilePhotoBuilder {
        let mut inner = SetProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setProfilePhoto".to_string();

        SetProfilePhotoBuilder { inner }
    }

    pub fn photo(&self) -> &InputChatPhoto {
        &self.photo
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }
}

#[doc(hidden)]
pub struct SetProfilePhotoBuilder {
    inner: SetProfilePhoto,
}

#[deprecated]
pub type RTDSetProfilePhotoBuilder = SetProfilePhotoBuilder;

impl SetProfilePhotoBuilder {
    pub fn build(&self) -> SetProfilePhoto {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<InputChatPhoto>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }

    pub fn is_public(&mut self, is_public: bool) -> &mut Self {
        self.inner.is_public = is_public;
        self
    }
}

impl AsRef<SetProfilePhoto> for SetProfilePhoto {
    fn as_ref(&self) -> &SetProfilePhoto {
        self
    }
}

impl AsRef<SetProfilePhoto> for SetProfilePhotoBuilder {
    fn as_ref(&self) -> &SetProfilePhoto {
        &self.inner
    }
}
