use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the translatable state of a chat; for Telegram Premium users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleChatIsTranslatable {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of is_translatable

    #[serde(default)]
    is_translatable: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleChatIsTranslatable {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleChatIsTranslatable {}

impl ToggleChatIsTranslatable {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleChatIsTranslatableBuilder {
        let mut inner = ToggleChatIsTranslatable::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleChatIsTranslatable".to_string();

        ToggleChatIsTranslatableBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_translatable(&self) -> bool {
        self.is_translatable
    }
}

#[doc(hidden)]
pub struct ToggleChatIsTranslatableBuilder {
    inner: ToggleChatIsTranslatable,
}

#[deprecated]
pub type RTDToggleChatIsTranslatableBuilder = ToggleChatIsTranslatableBuilder;

impl ToggleChatIsTranslatableBuilder {
    pub fn build(&self) -> ToggleChatIsTranslatable {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn is_translatable(&mut self, is_translatable: bool) -> &mut Self {
        self.inner.is_translatable = is_translatable;
        self
    }
}

impl AsRef<ToggleChatIsTranslatable> for ToggleChatIsTranslatable {
    fn as_ref(&self) -> &ToggleChatIsTranslatable {
        self
    }
}

impl AsRef<ToggleChatIsTranslatable> for ToggleChatIsTranslatableBuilder {
    fn as_ref(&self) -> &ToggleChatIsTranslatable {
        &self.inner
    }
}
