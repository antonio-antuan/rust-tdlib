use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Reports a chat photo to the Telegram moderators. A chat photo can be reported only if chat.can_be_reported
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportChatPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the photo to report. Only full photos from chatPhoto can be reported

    #[serde(default)]
    file_id: i32,
    /// The reason for reporting the chat photo

    #[serde(skip_serializing_if = "ChatReportReason::_is_default")]
    reason: ChatReportReason,
    /// Additional report details; 0-1024 characters

    #[serde(default)]
    text: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReportChatPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReportChatPhoto {}

impl ReportChatPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportChatPhotoBuilder {
        let mut inner = ReportChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reportChatPhoto".to_string();

        ReportChatPhotoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn reason(&self) -> &ChatReportReason {
        &self.reason
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct ReportChatPhotoBuilder {
    inner: ReportChatPhoto,
}

#[deprecated]
pub type RTDReportChatPhotoBuilder = ReportChatPhotoBuilder;

impl ReportChatPhotoBuilder {
    pub fn build(&self) -> ReportChatPhoto {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn reason<T: AsRef<ChatReportReason>>(&mut self, reason: T) -> &mut Self {
        self.inner.reason = reason.as_ref().clone();
        self
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<ReportChatPhoto> for ReportChatPhoto {
    fn as_ref(&self) -> &ReportChatPhoto {
        self
    }
}

impl AsRef<ReportChatPhoto> for ReportChatPhotoBuilder {
    fn as_ref(&self) -> &ReportChatPhoto {
        &self.inner
    }
}
