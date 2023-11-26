use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Information about the sponsor of a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSponsor {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the sponsor

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "MessageSponsorType::_is_default")]
    type_: MessageSponsorType,
    /// Photo of the sponsor; may be null if must not be shown
    photo: Option<ChatPhotoInfo>,
    /// Additional optional information about the sponsor to be shown along with the message

    #[serde(default)]
    info: String,
}

impl RObject for MessageSponsor {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageSponsor {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSponsorBuilder {
        let mut inner = MessageSponsor::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSponsorBuilder { inner }
    }

    pub fn type_(&self) -> &MessageSponsorType {
        &self.type_
    }

    pub fn photo(&self) -> &Option<ChatPhotoInfo> {
        &self.photo
    }

    pub fn info(&self) -> &String {
        &self.info
    }
}

#[doc(hidden)]
pub struct MessageSponsorBuilder {
    inner: MessageSponsor,
}

#[deprecated]
pub type RTDMessageSponsorBuilder = MessageSponsorBuilder;

impl MessageSponsorBuilder {
    pub fn build(&self) -> MessageSponsor {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<MessageSponsorType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn photo<T: AsRef<ChatPhotoInfo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn info<T: AsRef<str>>(&mut self, info: T) -> &mut Self {
        self.inner.info = info.as_ref().to_string();
        self
    }
}

impl AsRef<MessageSponsor> for MessageSponsor {
    fn as_ref(&self) -> &MessageSponsor {
        self
    }
}

impl AsRef<MessageSponsor> for MessageSponsorBuilder {
    fn as_ref(&self) -> &MessageSponsor {
        &self.inner
    }
}
