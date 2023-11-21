use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes actions that a user is allowed to take in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPermissions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the user can send text messages, contacts, giveaways, invoices, locations, and venues

    #[serde(default)]
    can_send_basic_messages: bool,
    /// True, if the user can send music files

    #[serde(default)]
    can_send_audios: bool,
    /// True, if the user can send documents

    #[serde(default)]
    can_send_documents: bool,
    /// True, if the user can send photos

    #[serde(default)]
    can_send_photos: bool,
    /// True, if the user can send videos

    #[serde(default)]
    can_send_videos: bool,
    /// True, if the user can send video notes

    #[serde(default)]
    can_send_video_notes: bool,
    /// True, if the user can send voice notes

    #[serde(default)]
    can_send_voice_notes: bool,
    /// True, if the user can send polls

    #[serde(default)]
    can_send_polls: bool,
    /// True, if the user can send animations, games, stickers, and dice and use inline bots

    #[serde(default)]
    can_send_other_messages: bool,
    /// True, if the user may add a web page preview to their messages

    #[serde(default)]
    can_add_web_page_previews: bool,
    /// True, if the user can change the chat title, photo, and other settings

    #[serde(default)]
    can_change_info: bool,
    /// True, if the user can invite new users to the chat

    #[serde(default)]
    can_invite_users: bool,
    /// True, if the user can pin messages

    #[serde(default)]
    can_pin_messages: bool,
    /// True, if the user can manage topics

    #[serde(default)]
    can_manage_topics: bool,
}

impl RObject for ChatPermissions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatPermissions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatPermissionsBuilder {
        let mut inner = ChatPermissions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatPermissionsBuilder { inner }
    }

    pub fn can_send_basic_messages(&self) -> bool {
        self.can_send_basic_messages
    }

    pub fn can_send_audios(&self) -> bool {
        self.can_send_audios
    }

    pub fn can_send_documents(&self) -> bool {
        self.can_send_documents
    }

    pub fn can_send_photos(&self) -> bool {
        self.can_send_photos
    }

    pub fn can_send_videos(&self) -> bool {
        self.can_send_videos
    }

    pub fn can_send_video_notes(&self) -> bool {
        self.can_send_video_notes
    }

    pub fn can_send_voice_notes(&self) -> bool {
        self.can_send_voice_notes
    }

    pub fn can_send_polls(&self) -> bool {
        self.can_send_polls
    }

    pub fn can_send_other_messages(&self) -> bool {
        self.can_send_other_messages
    }

    pub fn can_add_web_page_previews(&self) -> bool {
        self.can_add_web_page_previews
    }

    pub fn can_change_info(&self) -> bool {
        self.can_change_info
    }

    pub fn can_invite_users(&self) -> bool {
        self.can_invite_users
    }

    pub fn can_pin_messages(&self) -> bool {
        self.can_pin_messages
    }

    pub fn can_manage_topics(&self) -> bool {
        self.can_manage_topics
    }
}

#[doc(hidden)]
pub struct ChatPermissionsBuilder {
    inner: ChatPermissions,
}

#[deprecated]
pub type RTDChatPermissionsBuilder = ChatPermissionsBuilder;

impl ChatPermissionsBuilder {
    pub fn build(&self) -> ChatPermissions {
        self.inner.clone()
    }

    pub fn can_send_basic_messages(&mut self, can_send_basic_messages: bool) -> &mut Self {
        self.inner.can_send_basic_messages = can_send_basic_messages;
        self
    }

    pub fn can_send_audios(&mut self, can_send_audios: bool) -> &mut Self {
        self.inner.can_send_audios = can_send_audios;
        self
    }

    pub fn can_send_documents(&mut self, can_send_documents: bool) -> &mut Self {
        self.inner.can_send_documents = can_send_documents;
        self
    }

    pub fn can_send_photos(&mut self, can_send_photos: bool) -> &mut Self {
        self.inner.can_send_photos = can_send_photos;
        self
    }

    pub fn can_send_videos(&mut self, can_send_videos: bool) -> &mut Self {
        self.inner.can_send_videos = can_send_videos;
        self
    }

    pub fn can_send_video_notes(&mut self, can_send_video_notes: bool) -> &mut Self {
        self.inner.can_send_video_notes = can_send_video_notes;
        self
    }

    pub fn can_send_voice_notes(&mut self, can_send_voice_notes: bool) -> &mut Self {
        self.inner.can_send_voice_notes = can_send_voice_notes;
        self
    }

    pub fn can_send_polls(&mut self, can_send_polls: bool) -> &mut Self {
        self.inner.can_send_polls = can_send_polls;
        self
    }

    pub fn can_send_other_messages(&mut self, can_send_other_messages: bool) -> &mut Self {
        self.inner.can_send_other_messages = can_send_other_messages;
        self
    }

    pub fn can_add_web_page_previews(&mut self, can_add_web_page_previews: bool) -> &mut Self {
        self.inner.can_add_web_page_previews = can_add_web_page_previews;
        self
    }

    pub fn can_change_info(&mut self, can_change_info: bool) -> &mut Self {
        self.inner.can_change_info = can_change_info;
        self
    }

    pub fn can_invite_users(&mut self, can_invite_users: bool) -> &mut Self {
        self.inner.can_invite_users = can_invite_users;
        self
    }

    pub fn can_pin_messages(&mut self, can_pin_messages: bool) -> &mut Self {
        self.inner.can_pin_messages = can_pin_messages;
        self
    }

    pub fn can_manage_topics(&mut self, can_manage_topics: bool) -> &mut Self {
        self.inner.can_manage_topics = can_manage_topics;
        self
    }
}

impl AsRef<ChatPermissions> for ChatPermissions {
    fn as_ref(&self) -> &ChatPermissions {
        self
    }
}

impl AsRef<ChatPermissions> for ChatPermissionsBuilder {
    fn as_ref(&self) -> &ChatPermissions {
        &self.inner
    }
}
