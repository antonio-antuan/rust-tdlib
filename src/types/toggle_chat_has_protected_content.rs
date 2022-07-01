use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the ability of users to save, forward, or copy chat content. Supported only for basic groups, supergroups and channels. Requires owner privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleChatHasProtectedContent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// True, if chat content can't be saved locally, forwarded, or copied

    #[serde(default)]
    has_protected_content: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleChatHasProtectedContent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleChatHasProtectedContent {}

impl ToggleChatHasProtectedContent {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleChatHasProtectedContentBuilder {
        let mut inner = ToggleChatHasProtectedContent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleChatHasProtectedContent".to_string();

        RTDToggleChatHasProtectedContentBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn has_protected_content(&self) -> bool {
        self.has_protected_content
    }
}

#[doc(hidden)]
pub struct RTDToggleChatHasProtectedContentBuilder {
    inner: ToggleChatHasProtectedContent,
}

impl RTDToggleChatHasProtectedContentBuilder {
    pub fn build(&self) -> ToggleChatHasProtectedContent {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn has_protected_content(&mut self, has_protected_content: bool) -> &mut Self {
        self.inner.has_protected_content = has_protected_content;
        self
    }
}

impl AsRef<ToggleChatHasProtectedContent> for ToggleChatHasProtectedContent {
    fn as_ref(&self) -> &ToggleChatHasProtectedContent {
        self
    }
}

impl AsRef<ToggleChatHasProtectedContent> for RTDToggleChatHasProtectedContentBuilder {
    fn as_ref(&self) -> &ToggleChatHasProtectedContent {
        &self.inner
    }
}
