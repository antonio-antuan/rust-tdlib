use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether a General topic is hidden in a forum supergroup chat; requires can_manage_topics administrator right in the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleGeneralForumTopicIsHidden {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Pass true to hide and close the General topic; pass false to unhide it

    #[serde(default)]
    is_hidden: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleGeneralForumTopicIsHidden {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleGeneralForumTopicIsHidden {}

impl ToggleGeneralForumTopicIsHidden {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleGeneralForumTopicIsHiddenBuilder {
        let mut inner = ToggleGeneralForumTopicIsHidden::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleGeneralForumTopicIsHidden".to_string();

        ToggleGeneralForumTopicIsHiddenBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }
}

#[doc(hidden)]
pub struct ToggleGeneralForumTopicIsHiddenBuilder {
    inner: ToggleGeneralForumTopicIsHidden,
}

#[deprecated]
pub type RTDToggleGeneralForumTopicIsHiddenBuilder = ToggleGeneralForumTopicIsHiddenBuilder;

impl ToggleGeneralForumTopicIsHiddenBuilder {
    pub fn build(&self) -> ToggleGeneralForumTopicIsHidden {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn is_hidden(&mut self, is_hidden: bool) -> &mut Self {
        self.inner.is_hidden = is_hidden;
        self
    }
}

impl AsRef<ToggleGeneralForumTopicIsHidden> for ToggleGeneralForumTopicIsHidden {
    fn as_ref(&self) -> &ToggleGeneralForumTopicIsHidden {
        self
    }
}

impl AsRef<ToggleGeneralForumTopicIsHidden> for ToggleGeneralForumTopicIsHiddenBuilder {
    fn as_ref(&self) -> &ToggleGeneralForumTopicIsHidden {
        &self.inner
    }
}
