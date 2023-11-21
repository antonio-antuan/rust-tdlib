use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Suggests a profile photo to another regular user with common messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestUserProfilePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// Profile photo to suggest; inputChatPhotoPrevious isn't supported in this function

    #[serde(skip_serializing_if = "InputChatPhoto::_is_default")]
    photo: InputChatPhoto,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SuggestUserProfilePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SuggestUserProfilePhoto {}

impl SuggestUserProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SuggestUserProfilePhotoBuilder {
        let mut inner = SuggestUserProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "suggestUserProfilePhoto".to_string();

        SuggestUserProfilePhotoBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn photo(&self) -> &InputChatPhoto {
        &self.photo
    }
}

#[doc(hidden)]
pub struct SuggestUserProfilePhotoBuilder {
    inner: SuggestUserProfilePhoto,
}

#[deprecated]
pub type RTDSuggestUserProfilePhotoBuilder = SuggestUserProfilePhotoBuilder;

impl SuggestUserProfilePhotoBuilder {
    pub fn build(&self) -> SuggestUserProfilePhoto {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn photo<T: AsRef<InputChatPhoto>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }
}

impl AsRef<SuggestUserProfilePhoto> for SuggestUserProfilePhoto {
    fn as_ref(&self) -> &SuggestUserProfilePhoto {
        self
    }
}

impl AsRef<SuggestUserProfilePhoto> for SuggestUserProfilePhotoBuilder {
    fn as_ref(&self) -> &SuggestUserProfilePhoto {
        &self.inner
    }
}
