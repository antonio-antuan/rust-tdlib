use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New chat photo; pass null to delete the chat photo

    #[serde(skip_serializing_if = "InputChatPhoto::_is_default")]
    photo: InputChatPhoto,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatPhoto {}

impl SetChatPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatPhotoBuilder {
        let mut inner = SetChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatPhoto".to_string();

        SetChatPhotoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn photo(&self) -> &InputChatPhoto {
        &self.photo
    }
}

#[doc(hidden)]
pub struct SetChatPhotoBuilder {
    inner: SetChatPhoto,
}

#[deprecated]
pub type RTDSetChatPhotoBuilder = SetChatPhotoBuilder;

impl SetChatPhotoBuilder {
    pub fn build(&self) -> SetChatPhoto {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn photo<T: AsRef<InputChatPhoto>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }
}

impl AsRef<SetChatPhoto> for SetChatPhoto {
    fn as_ref(&self) -> &SetChatPhoto {
        self
    }
}

impl AsRef<SetChatPhoto> for SetChatPhotoBuilder {
    fn as_ref(&self) -> &SetChatPhoto {
        &self.inner
    }
}
