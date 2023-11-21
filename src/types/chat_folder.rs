use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a folder for user chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolder {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The title of the folder; 1-12 characters without line feeds

    #[serde(default)]
    title: String,
    /// The chosen icon for the chat folder; may be null. If null, use getChatFolderDefaultIconName to get default icon name for the folder
    icon: Option<ChatFolderIcon>,
    /// True, if at least one link has been created for the folder

    #[serde(default)]
    is_shareable: bool,
    /// The chat identifiers of pinned chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") pinned and always included non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium

    #[serde(default)]
    pinned_chat_ids: Vec<i64>,
    /// The chat identifiers of always included chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") pinned and always included non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium

    #[serde(default)]
    included_chat_ids: Vec<i64>,
    /// The chat identifiers of always excluded chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") always excluded non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium

    #[serde(default)]
    excluded_chat_ids: Vec<i64>,
    /// True, if muted chats need to be excluded

    #[serde(default)]
    exclude_muted: bool,
    /// True, if read chats need to be excluded

    #[serde(default)]
    exclude_read: bool,
    /// True, if archived chats need to be excluded

    #[serde(default)]
    exclude_archived: bool,
    /// True, if contacts need to be included

    #[serde(default)]
    include_contacts: bool,
    /// True, if non-contact users need to be included

    #[serde(default)]
    include_non_contacts: bool,
    /// True, if bots need to be included

    #[serde(default)]
    include_bots: bool,
    /// True, if basic groups and supergroups need to be included

    #[serde(default)]
    include_groups: bool,
    /// True, if channels need to be included

    #[serde(default)]
    include_channels: bool,
}

impl RObject for ChatFolder {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatFolder {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatFolderBuilder {
        let mut inner = ChatFolder::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatFolderBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn icon(&self) -> &Option<ChatFolderIcon> {
        &self.icon
    }

    pub fn is_shareable(&self) -> bool {
        self.is_shareable
    }

    pub fn pinned_chat_ids(&self) -> &Vec<i64> {
        &self.pinned_chat_ids
    }

    pub fn included_chat_ids(&self) -> &Vec<i64> {
        &self.included_chat_ids
    }

    pub fn excluded_chat_ids(&self) -> &Vec<i64> {
        &self.excluded_chat_ids
    }

    pub fn exclude_muted(&self) -> bool {
        self.exclude_muted
    }

    pub fn exclude_read(&self) -> bool {
        self.exclude_read
    }

    pub fn exclude_archived(&self) -> bool {
        self.exclude_archived
    }

    pub fn include_contacts(&self) -> bool {
        self.include_contacts
    }

    pub fn include_non_contacts(&self) -> bool {
        self.include_non_contacts
    }

    pub fn include_bots(&self) -> bool {
        self.include_bots
    }

    pub fn include_groups(&self) -> bool {
        self.include_groups
    }

    pub fn include_channels(&self) -> bool {
        self.include_channels
    }
}

#[doc(hidden)]
pub struct ChatFolderBuilder {
    inner: ChatFolder,
}

#[deprecated]
pub type RTDChatFolderBuilder = ChatFolderBuilder;

impl ChatFolderBuilder {
    pub fn build(&self) -> ChatFolder {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn icon<T: AsRef<ChatFolderIcon>>(&mut self, icon: T) -> &mut Self {
        self.inner.icon = Some(icon.as_ref().clone());
        self
    }

    pub fn is_shareable(&mut self, is_shareable: bool) -> &mut Self {
        self.inner.is_shareable = is_shareable;
        self
    }

    pub fn pinned_chat_ids(&mut self, pinned_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.pinned_chat_ids = pinned_chat_ids;
        self
    }

    pub fn included_chat_ids(&mut self, included_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.included_chat_ids = included_chat_ids;
        self
    }

    pub fn excluded_chat_ids(&mut self, excluded_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.excluded_chat_ids = excluded_chat_ids;
        self
    }

    pub fn exclude_muted(&mut self, exclude_muted: bool) -> &mut Self {
        self.inner.exclude_muted = exclude_muted;
        self
    }

    pub fn exclude_read(&mut self, exclude_read: bool) -> &mut Self {
        self.inner.exclude_read = exclude_read;
        self
    }

    pub fn exclude_archived(&mut self, exclude_archived: bool) -> &mut Self {
        self.inner.exclude_archived = exclude_archived;
        self
    }

    pub fn include_contacts(&mut self, include_contacts: bool) -> &mut Self {
        self.inner.include_contacts = include_contacts;
        self
    }

    pub fn include_non_contacts(&mut self, include_non_contacts: bool) -> &mut Self {
        self.inner.include_non_contacts = include_non_contacts;
        self
    }

    pub fn include_bots(&mut self, include_bots: bool) -> &mut Self {
        self.inner.include_bots = include_bots;
        self
    }

    pub fn include_groups(&mut self, include_groups: bool) -> &mut Self {
        self.inner.include_groups = include_groups;
        self
    }

    pub fn include_channels(&mut self, include_channels: bool) -> &mut Self {
        self.inner.include_channels = include_channels;
        self
    }
}

impl AsRef<ChatFolder> for ChatFolder {
    fn as_ref(&self) -> &ChatFolder {
        self
    }
}

impl AsRef<ChatFolder> for ChatFolderBuilder {
    fn as_ref(&self) -> &ChatFolder {
        &self.inner
    }
}
