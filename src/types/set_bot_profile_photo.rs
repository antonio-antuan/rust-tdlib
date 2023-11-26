use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes a profile photo for a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBotProfilePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// Profile photo to set; pass null to delete the chat photo

    #[serde(skip_serializing_if = "InputChatPhoto::_is_default")]
    photo: InputChatPhoto,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetBotProfilePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetBotProfilePhoto {}

impl SetBotProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetBotProfilePhotoBuilder {
        let mut inner = SetBotProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setBotProfilePhoto".to_string();

        SetBotProfilePhotoBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn photo(&self) -> &InputChatPhoto {
        &self.photo
    }
}

#[doc(hidden)]
pub struct SetBotProfilePhotoBuilder {
    inner: SetBotProfilePhoto,
}

#[deprecated]
pub type RTDSetBotProfilePhotoBuilder = SetBotProfilePhotoBuilder;

impl SetBotProfilePhotoBuilder {
    pub fn build(&self) -> SetBotProfilePhoto {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn photo<T: AsRef<InputChatPhoto>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }
}

impl AsRef<SetBotProfilePhoto> for SetBotProfilePhoto {
    fn as_ref(&self) -> &SetBotProfilePhoto {
        self
    }
}

impl AsRef<SetBotProfilePhoto> for SetBotProfilePhotoBuilder {
    fn as_ref(&self) -> &SetBotProfilePhoto {
        &self.inner
    }
}
