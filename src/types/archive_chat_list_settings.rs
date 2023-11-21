use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains settings for automatic moving of chats to and from the Archive chat lists
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveChatListSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if new chats from non-contacts will be automatically archived and muted. Can be set to true only if the option "can_archive_and_mute_new_chats_from_unknown_users" is true

    #[serde(default)]
    archive_and_mute_new_chats_from_unknown_users: bool,
    /// True, if unmuted chats will be kept in the Archive chat list when they get a new message

    #[serde(default)]
    keep_unmuted_chats_archived: bool,
    /// True, if unmuted chats, that are always included or pinned in a folder, will be kept in the Archive chat list when they get a new message. Ignored if keep_unmuted_chats_archived == true

    #[serde(default)]
    keep_chats_from_folders_archived: bool,
}

impl RObject for ArchiveChatListSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ArchiveChatListSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ArchiveChatListSettingsBuilder {
        let mut inner = ArchiveChatListSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ArchiveChatListSettingsBuilder { inner }
    }

    pub fn archive_and_mute_new_chats_from_unknown_users(&self) -> bool {
        self.archive_and_mute_new_chats_from_unknown_users
    }

    pub fn keep_unmuted_chats_archived(&self) -> bool {
        self.keep_unmuted_chats_archived
    }

    pub fn keep_chats_from_folders_archived(&self) -> bool {
        self.keep_chats_from_folders_archived
    }
}

#[doc(hidden)]
pub struct ArchiveChatListSettingsBuilder {
    inner: ArchiveChatListSettings,
}

#[deprecated]
pub type RTDArchiveChatListSettingsBuilder = ArchiveChatListSettingsBuilder;

impl ArchiveChatListSettingsBuilder {
    pub fn build(&self) -> ArchiveChatListSettings {
        self.inner.clone()
    }

    pub fn archive_and_mute_new_chats_from_unknown_users(
        &mut self,
        archive_and_mute_new_chats_from_unknown_users: bool,
    ) -> &mut Self {
        self.inner.archive_and_mute_new_chats_from_unknown_users =
            archive_and_mute_new_chats_from_unknown_users;
        self
    }

    pub fn keep_unmuted_chats_archived(&mut self, keep_unmuted_chats_archived: bool) -> &mut Self {
        self.inner.keep_unmuted_chats_archived = keep_unmuted_chats_archived;
        self
    }

    pub fn keep_chats_from_folders_archived(
        &mut self,
        keep_chats_from_folders_archived: bool,
    ) -> &mut Self {
        self.inner.keep_chats_from_folders_archived = keep_chats_from_folders_archived;
        self
    }
}

impl AsRef<ArchiveChatListSettings> for ArchiveChatListSettings {
    fn as_ref(&self) -> &ArchiveChatListSettings {
        self
    }
}

impl AsRef<ArchiveChatListSettings> for ArchiveChatListSettingsBuilder {
    fn as_ref(&self) -> &ArchiveChatListSettings {
        &self.inner
    }
}
