use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a set of filters used to obtain a chat event log
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventLogFilters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if message edits need to be returned

    #[serde(default)]
    message_edits: bool,
    /// True, if message deletions need to be returned

    #[serde(default)]
    message_deletions: bool,
    /// True, if pin/unpin events need to be returned

    #[serde(default)]
    message_pins: bool,
    /// True, if members joining events need to be returned

    #[serde(default)]
    member_joins: bool,
    /// True, if members leaving events need to be returned

    #[serde(default)]
    member_leaves: bool,
    /// True, if invited member events need to be returned

    #[serde(default)]
    member_invites: bool,
    /// True, if member promotion/demotion events need to be returned

    #[serde(default)]
    member_promotions: bool,
    /// True, if member restricted/unrestricted/banned/unbanned events need to be returned

    #[serde(default)]
    member_restrictions: bool,
    /// True, if changes in chat information need to be returned

    #[serde(default)]
    info_changes: bool,
    /// True, if changes in chat settings need to be returned

    #[serde(default)]
    setting_changes: bool,
    /// True, if changes to invite links need to be returned

    #[serde(default)]
    invite_link_changes: bool,
    /// True, if video chat actions need to be returned

    #[serde(default)]
    video_chat_changes: bool,
}

impl RObject for ChatEventLogFilters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatEventLogFilters {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatEventLogFiltersBuilder {
        let mut inner = ChatEventLogFilters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatEventLogFiltersBuilder { inner }
    }

    pub fn message_edits(&self) -> bool {
        self.message_edits
    }

    pub fn message_deletions(&self) -> bool {
        self.message_deletions
    }

    pub fn message_pins(&self) -> bool {
        self.message_pins
    }

    pub fn member_joins(&self) -> bool {
        self.member_joins
    }

    pub fn member_leaves(&self) -> bool {
        self.member_leaves
    }

    pub fn member_invites(&self) -> bool {
        self.member_invites
    }

    pub fn member_promotions(&self) -> bool {
        self.member_promotions
    }

    pub fn member_restrictions(&self) -> bool {
        self.member_restrictions
    }

    pub fn info_changes(&self) -> bool {
        self.info_changes
    }

    pub fn setting_changes(&self) -> bool {
        self.setting_changes
    }

    pub fn invite_link_changes(&self) -> bool {
        self.invite_link_changes
    }

    pub fn video_chat_changes(&self) -> bool {
        self.video_chat_changes
    }
}

#[doc(hidden)]
pub struct ChatEventLogFiltersBuilder {
    inner: ChatEventLogFilters,
}

#[deprecated]
pub type RTDChatEventLogFiltersBuilder = ChatEventLogFiltersBuilder;

impl ChatEventLogFiltersBuilder {
    pub fn build(&self) -> ChatEventLogFilters {
        self.inner.clone()
    }

    pub fn message_edits(&mut self, message_edits: bool) -> &mut Self {
        self.inner.message_edits = message_edits;
        self
    }

    pub fn message_deletions(&mut self, message_deletions: bool) -> &mut Self {
        self.inner.message_deletions = message_deletions;
        self
    }

    pub fn message_pins(&mut self, message_pins: bool) -> &mut Self {
        self.inner.message_pins = message_pins;
        self
    }

    pub fn member_joins(&mut self, member_joins: bool) -> &mut Self {
        self.inner.member_joins = member_joins;
        self
    }

    pub fn member_leaves(&mut self, member_leaves: bool) -> &mut Self {
        self.inner.member_leaves = member_leaves;
        self
    }

    pub fn member_invites(&mut self, member_invites: bool) -> &mut Self {
        self.inner.member_invites = member_invites;
        self
    }

    pub fn member_promotions(&mut self, member_promotions: bool) -> &mut Self {
        self.inner.member_promotions = member_promotions;
        self
    }

    pub fn member_restrictions(&mut self, member_restrictions: bool) -> &mut Self {
        self.inner.member_restrictions = member_restrictions;
        self
    }

    pub fn info_changes(&mut self, info_changes: bool) -> &mut Self {
        self.inner.info_changes = info_changes;
        self
    }

    pub fn setting_changes(&mut self, setting_changes: bool) -> &mut Self {
        self.inner.setting_changes = setting_changes;
        self
    }

    pub fn invite_link_changes(&mut self, invite_link_changes: bool) -> &mut Self {
        self.inner.invite_link_changes = invite_link_changes;
        self
    }

    pub fn video_chat_changes(&mut self, video_chat_changes: bool) -> &mut Self {
        self.inner.video_chat_changes = video_chat_changes;
        self
    }
}

impl AsRef<ChatEventLogFilters> for ChatEventLogFilters {
    fn as_ref(&self) -> &ChatEventLogFilters {
        self
    }
}

impl AsRef<ChatEventLogFilters> for ChatEventLogFiltersBuilder {
    fn as_ref(&self) -> &ChatEventLogFilters {
        &self.inner
    }
}
