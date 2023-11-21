use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a viewer of a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageViewer {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the viewer

    #[serde(default)]
    user_id: i64,
    /// Approximate point in time (Unix timestamp) when the message was viewed

    #[serde(default)]
    view_date: i32,
}

impl RObject for MessageViewer {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageViewer {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageViewerBuilder {
        let mut inner = MessageViewer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageViewerBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn view_date(&self) -> i32 {
        self.view_date
    }
}

#[doc(hidden)]
pub struct MessageViewerBuilder {
    inner: MessageViewer,
}

#[deprecated]
pub type RTDMessageViewerBuilder = MessageViewerBuilder;

impl MessageViewerBuilder {
    pub fn build(&self) -> MessageViewer {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn view_date(&mut self, view_date: i32) -> &mut Self {
        self.inner.view_date = view_date;
        self
    }
}

impl AsRef<MessageViewer> for MessageViewer {
    fn as_ref(&self) -> &MessageViewer {
        self
    }
}

impl AsRef<MessageViewer> for MessageViewerBuilder {
    fn as_ref(&self) -> &MessageViewer {
        &self.inner
    }
}
