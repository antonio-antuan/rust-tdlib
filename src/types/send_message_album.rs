use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends 2-10 messages grouped together into an album. Currently, only audio, document, photo and video messages can be grouped into an album. Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageAlbum {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Target chat
    chat_id: i64,
    /// If not 0, a message thread identifier in which the messages will be sent
    message_thread_id: i64,
    /// Identifier of a replied message; 0 if none
    reply_to_message_id: i64,
    /// Options to be used to send the messages; pass null to use default options
    options: MessageSendOptions,
    /// Contents of messages to be sent. At most 10 messages can be added to an album
    input_message_contents: Vec<InputMessageContent>,
    /// Pass true to get fake messages instead of actually sending them
    only_preview: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendMessageAlbum {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendMessageAlbum {}

impl SendMessageAlbum {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendMessageAlbumBuilder {
        let mut inner = SendMessageAlbum::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendMessageAlbum".to_string();

        RTDSendMessageAlbumBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn reply_to_message_id(&self) -> i64 {
        self.reply_to_message_id
    }

    pub fn options(&self) -> &MessageSendOptions {
        &self.options
    }

    pub fn input_message_contents(&self) -> &Vec<InputMessageContent> {
        &self.input_message_contents
    }

    pub fn only_preview(&self) -> bool {
        self.only_preview
    }
}

#[doc(hidden)]
pub struct RTDSendMessageAlbumBuilder {
    inner: SendMessageAlbum,
}

impl RTDSendMessageAlbumBuilder {
    pub fn build(&self) -> SendMessageAlbum {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
        self.inner.reply_to_message_id = reply_to_message_id;
        self
    }

    pub fn options<T: AsRef<MessageSendOptions>>(&mut self, options: T) -> &mut Self {
        self.inner.options = options.as_ref().clone();
        self
    }

    pub fn input_message_contents(
        &mut self,
        input_message_contents: Vec<InputMessageContent>,
    ) -> &mut Self {
        self.inner.input_message_contents = input_message_contents;
        self
    }

    pub fn only_preview(&mut self, only_preview: bool) -> &mut Self {
        self.inner.only_preview = only_preview;
        self
    }
}

impl AsRef<SendMessageAlbum> for SendMessageAlbum {
    fn as_ref(&self) -> &SendMessageAlbum {
        self
    }
}

impl AsRef<SendMessageAlbum> for RTDSendMessageAlbumBuilder {
    fn as_ref(&self) -> &SendMessageAlbum {
        &self.inner
    }
}
