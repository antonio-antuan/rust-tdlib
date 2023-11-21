use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Reports a story to the Telegram moderators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the sender of the story to report

    #[serde(default)]
    story_sender_chat_id: i64,
    /// The identifier of the story to report

    #[serde(default)]
    story_id: i32,
    /// The reason for reporting the story

    #[serde(skip_serializing_if = "ReportReason::_is_default")]
    reason: ReportReason,
    /// Additional report details; 0-1024 characters

    #[serde(default)]
    text: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReportStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReportStory {}

impl ReportStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportStoryBuilder {
        let mut inner = ReportStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reportStory".to_string();

        ReportStoryBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }

    pub fn reason(&self) -> &ReportReason {
        &self.reason
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct ReportStoryBuilder {
    inner: ReportStory,
}

#[deprecated]
pub type RTDReportStoryBuilder = ReportStoryBuilder;

impl ReportStoryBuilder {
    pub fn build(&self) -> ReportStory {
        self.inner.clone()
    }

    pub fn story_sender_chat_id(&mut self, story_sender_chat_id: i64) -> &mut Self {
        self.inner.story_sender_chat_id = story_sender_chat_id;
        self
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }

    pub fn reason<T: AsRef<ReportReason>>(&mut self, reason: T) -> &mut Self {
        self.inner.reason = reason.as_ref().clone();
        self
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<ReportStory> for ReportStory {
    fn as_ref(&self) -> &ReportStory {
        self
    }
}

impl AsRef<ReportStory> for ReportStoryBuilder {
    fn as_ref(&self) -> &ReportStory {
        &self.inner
    }
}
