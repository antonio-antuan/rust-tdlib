use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Options to be used when a message content is copied without a link to the original message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCopyOptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if content of the message needs to be copied without a link to the original message. Always true if the message is forwarded to a secret chat
    send_copy: bool,
    /// True, if media caption of the message copy needs to be replaced. Ignored if send_copy is false
    replace_caption: bool,
    /// New message caption. Ignored if replace_caption is false
    new_caption: FormattedText,
}

impl RObject for MessageCopyOptions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageCopyOptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageCopyOptionsBuilder {
        let mut inner = MessageCopyOptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageCopyOptionsBuilder { inner }
    }

    pub fn send_copy(&self) -> bool {
        self.send_copy
    }

    pub fn replace_caption(&self) -> bool {
        self.replace_caption
    }

    pub fn new_caption(&self) -> &FormattedText {
        &self.new_caption
    }
}

#[doc(hidden)]
pub struct RTDMessageCopyOptionsBuilder {
    inner: MessageCopyOptions,
}

impl RTDMessageCopyOptionsBuilder {
    pub fn build(&self) -> MessageCopyOptions {
        self.inner.clone()
    }

    pub fn send_copy(&mut self, send_copy: bool) -> &mut Self {
        self.inner.send_copy = send_copy;
        self
    }

    pub fn replace_caption(&mut self, replace_caption: bool) -> &mut Self {
        self.inner.replace_caption = replace_caption;
        self
    }

    pub fn new_caption<T: AsRef<FormattedText>>(&mut self, new_caption: T) -> &mut Self {
        self.inner.new_caption = new_caption.as_ref().clone();
        self
    }
}

impl AsRef<MessageCopyOptions> for MessageCopyOptions {
    fn as_ref(&self) -> &MessageCopyOptions {
        self
    }
}

impl AsRef<MessageCopyOptions> for RTDMessageCopyOptionsBuilder {
    fn as_ref(&self) -> &MessageCopyOptions {
        &self.inner
    }
}
