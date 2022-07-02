use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Supergroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Supergroup or channel identifier

    #[serde(default)]
    id: i64,
    /// Username of the supergroup or channel; empty for private supergroups or channels

    #[serde(default)]
    username: String,
    /// Point in time (Unix timestamp) when the current user joined, or the point in time when the supergroup or channel was created, in case the user is not a member

    #[serde(default)]
    date: i32,
    /// Status of the current user in the supergroup or channel; custom title will be always empty

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    status: ChatMemberStatus,
    /// Number of members in the supergroup or channel; 0 if unknown. Currently, it is guaranteed to be known only if the supergroup or channel was received through searchPublicChats, searchChatsNearby, getInactiveSupergroupChats, getSuitableDiscussionChats, getGroupsInCommon, or getUserPrivacySettingRules

    #[serde(default)]
    member_count: i32,
    /// True, if the channel has a discussion group, or the supergroup is the designated discussion group for a channel

    #[serde(default)]
    has_linked_chat: bool,
    /// True, if the supergroup is connected to a location, i.e. the supergroup is a location-based supergroup

    #[serde(default)]
    has_location: bool,
    /// True, if messages sent to the channel need to contain information about the sender. This field is only applicable to channels

    #[serde(default)]
    sign_messages: bool,
    /// True, if the slow mode is enabled in the supergroup

    #[serde(default)]
    is_slow_mode_enabled: bool,
    /// True, if the supergroup is a channel

    #[serde(default)]
    is_channel: bool,
    /// True, if the supergroup is a broadcast group, i.e. only administrators can send messages and there is no limit on the number of members

    #[serde(default)]
    is_broadcast_group: bool,
    /// True, if the supergroup or channel is verified

    #[serde(default)]
    is_verified: bool,
    /// If non-empty, contains a human-readable description of the reason why access to this supergroup or channel must be restricted

    #[serde(default)]
    restriction_reason: String,
    /// True, if many users reported this supergroup or channel as a scam

    #[serde(default)]
    is_scam: bool,
    /// True, if many users reported this supergroup or channel as a fake account

    #[serde(default)]
    is_fake: bool,
}

impl RObject for Supergroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Supergroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupBuilder {
        let mut inner = Supergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn status(&self) -> &ChatMemberStatus {
        &self.status
    }

    pub fn member_count(&self) -> i32 {
        self.member_count
    }

    pub fn has_linked_chat(&self) -> bool {
        self.has_linked_chat
    }

    pub fn has_location(&self) -> bool {
        self.has_location
    }

    pub fn sign_messages(&self) -> bool {
        self.sign_messages
    }

    pub fn is_slow_mode_enabled(&self) -> bool {
        self.is_slow_mode_enabled
    }

    pub fn is_channel(&self) -> bool {
        self.is_channel
    }

    pub fn is_broadcast_group(&self) -> bool {
        self.is_broadcast_group
    }

    pub fn is_verified(&self) -> bool {
        self.is_verified
    }

    pub fn restriction_reason(&self) -> &String {
        &self.restriction_reason
    }

    pub fn is_scam(&self) -> bool {
        self.is_scam
    }

    pub fn is_fake(&self) -> bool {
        self.is_fake
    }
}

#[doc(hidden)]
pub struct SupergroupBuilder {
    inner: Supergroup,
}

#[deprecated]
pub type RTDSupergroupBuilder = SupergroupBuilder;

impl SupergroupBuilder {
    pub fn build(&self) -> Supergroup {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
        self.inner.status = status.as_ref().clone();
        self
    }

    pub fn member_count(&mut self, member_count: i32) -> &mut Self {
        self.inner.member_count = member_count;
        self
    }

    pub fn has_linked_chat(&mut self, has_linked_chat: bool) -> &mut Self {
        self.inner.has_linked_chat = has_linked_chat;
        self
    }

    pub fn has_location(&mut self, has_location: bool) -> &mut Self {
        self.inner.has_location = has_location;
        self
    }

    pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
        self.inner.sign_messages = sign_messages;
        self
    }

    pub fn is_slow_mode_enabled(&mut self, is_slow_mode_enabled: bool) -> &mut Self {
        self.inner.is_slow_mode_enabled = is_slow_mode_enabled;
        self
    }

    pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
        self.inner.is_channel = is_channel;
        self
    }

    pub fn is_broadcast_group(&mut self, is_broadcast_group: bool) -> &mut Self {
        self.inner.is_broadcast_group = is_broadcast_group;
        self
    }

    pub fn is_verified(&mut self, is_verified: bool) -> &mut Self {
        self.inner.is_verified = is_verified;
        self
    }

    pub fn restriction_reason<T: AsRef<str>>(&mut self, restriction_reason: T) -> &mut Self {
        self.inner.restriction_reason = restriction_reason.as_ref().to_string();
        self
    }

    pub fn is_scam(&mut self, is_scam: bool) -> &mut Self {
        self.inner.is_scam = is_scam;
        self
    }

    pub fn is_fake(&mut self, is_fake: bool) -> &mut Self {
        self.inner.is_fake = is_fake;
        self
    }
}

impl AsRef<Supergroup> for Supergroup {
    fn as_ref(&self) -> &Supergroup {
        self
    }
}

impl AsRef<Supergroup> for SupergroupBuilder {
    fn as_ref(&self) -> &Supergroup {
        &self.inner
    }
}
