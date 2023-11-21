use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits title and icon of a topic in a forum supergroup chat; requires can_manage_topics administrator right in the supergroup unless the user is creator of the topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditForumTopic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier of the forum topic

    #[serde(default)]
    message_thread_id: i64,
    /// New name of the topic; 0-128 characters. If empty, the previous topic name is kept

    #[serde(default)]
    name: String,
    /// Pass true to edit the icon of the topic. Icon of the General topic can't be edited

    #[serde(default)]
    edit_icon_custom_emoji: bool,
    /// Identifier of the new custom emoji for topic icon; pass 0 to remove the custom emoji. Ignored if edit_icon_custom_emoji is false. Telegram Premium users can use any custom emoji, other users can use only a custom emoji returned by getForumTopicDefaultIcons

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    icon_custom_emoji_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditForumTopic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditForumTopic {}

impl EditForumTopic {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditForumTopicBuilder {
        let mut inner = EditForumTopic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editForumTopic".to_string();

        EditForumTopicBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn edit_icon_custom_emoji(&self) -> bool {
        self.edit_icon_custom_emoji
    }

    pub fn icon_custom_emoji_id(&self) -> i64 {
        self.icon_custom_emoji_id
    }
}

#[doc(hidden)]
pub struct EditForumTopicBuilder {
    inner: EditForumTopic,
}

#[deprecated]
pub type RTDEditForumTopicBuilder = EditForumTopicBuilder;

impl EditForumTopicBuilder {
    pub fn build(&self) -> EditForumTopic {
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

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn edit_icon_custom_emoji(&mut self, edit_icon_custom_emoji: bool) -> &mut Self {
        self.inner.edit_icon_custom_emoji = edit_icon_custom_emoji;
        self
    }

    pub fn icon_custom_emoji_id(&mut self, icon_custom_emoji_id: i64) -> &mut Self {
        self.inner.icon_custom_emoji_id = icon_custom_emoji_id;
        self
    }
}

impl AsRef<EditForumTopic> for EditForumTopic {
    fn as_ref(&self) -> &EditForumTopic {
        self
    }
}

impl AsRef<EditForumTopic> for EditForumTopicBuilder {
    fn as_ref(&self) -> &EditForumTopic {
        &self.inner
    }
}
