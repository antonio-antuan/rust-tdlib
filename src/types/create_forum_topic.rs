use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Creates a topic in a forum supergroup chat; requires can_manage_topics rights in the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateForumTopic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Name of the topic; 1-128 characters

    #[serde(default)]
    name: String,
    /// Icon of the topic. Icon color must be one of 0x6FB9F0, 0xFFD67E, 0xCB86DB, 0x8EEE98, 0xFF93B2, or 0xFB6F5F. Telegram Premium users can use any custom emoji as topic icon, other users can use only a custom emoji returned by getForumTopicDefaultIcons
    icon: ForumTopicIcon,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateForumTopic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateForumTopic {}

impl CreateForumTopic {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateForumTopicBuilder {
        let mut inner = CreateForumTopic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createForumTopic".to_string();

        CreateForumTopicBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn icon(&self) -> &ForumTopicIcon {
        &self.icon
    }
}

#[doc(hidden)]
pub struct CreateForumTopicBuilder {
    inner: CreateForumTopic,
}

#[deprecated]
pub type RTDCreateForumTopicBuilder = CreateForumTopicBuilder;

impl CreateForumTopicBuilder {
    pub fn build(&self) -> CreateForumTopic {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn icon<T: AsRef<ForumTopicIcon>>(&mut self, icon: T) -> &mut Self {
        self.inner.icon = icon.as_ref().clone();
        self
    }
}

impl AsRef<CreateForumTopic> for CreateForumTopic {
    fn as_ref(&self) -> &CreateForumTopic {
        self
    }
}

impl AsRef<CreateForumTopic> for CreateForumTopicBuilder {
    fn as_ref(&self) -> &CreateForumTopic {
        &self.inner
    }
}
