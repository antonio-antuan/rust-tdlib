use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a group call participant
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallParticipant {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the group call participant

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    participant_id: MessageSender,
    /// User's audio channel synchronization source identifier

    #[serde(default)]
    audio_source_id: i32,
    /// User's screen sharing audio channel synchronization source identifier

    #[serde(default)]
    screen_sharing_audio_source_id: i32,
    /// Information about user's video channel; may be null if there is no active video
    video_info: Option<GroupCallParticipantVideoInfo>,
    /// Information about user's screen sharing video channel; may be null if there is no active screen sharing video
    screen_sharing_video_info: Option<GroupCallParticipantVideoInfo>,
    /// The participant user's bio or the participant chat's description

    #[serde(default)]
    bio: String,
    /// True, if the participant is the current user

    #[serde(default)]
    is_current_user: bool,
    /// True, if the participant is speaking as set by setGroupCallParticipantIsSpeaking

    #[serde(default)]
    is_speaking: bool,
    /// True, if the participant hand is raised

    #[serde(default)]
    is_hand_raised: bool,
    /// True, if the current user can mute the participant for all other group call participants

    #[serde(default)]
    can_be_muted_for_all_users: bool,
    /// True, if the current user can allow the participant to unmute themselves or unmute the participant (if the participant is the current user)

    #[serde(default)]
    can_be_unmuted_for_all_users: bool,
    /// True, if the current user can mute the participant only for self

    #[serde(default)]
    can_be_muted_for_current_user: bool,
    /// True, if the current user can unmute the participant for self

    #[serde(default)]
    can_be_unmuted_for_current_user: bool,
    /// True, if the participant is muted for all users

    #[serde(default)]
    is_muted_for_all_users: bool,
    /// True, if the participant is muted for the current user

    #[serde(default)]
    is_muted_for_current_user: bool,
    /// True, if the participant is muted for all users, but can unmute themselves

    #[serde(default)]
    can_unmute_self: bool,
    /// Participant's volume level; 1-20000 in hundreds of percents

    #[serde(default)]
    volume_level: i32,
    /// User's order in the group call participant list. Orders must be compared lexicographically. The bigger is order, the higher is user in the list. If order is empty, the user must be removed from the participant list

    #[serde(default)]
    order: String,
}

impl RObject for GroupCallParticipant {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GroupCallParticipant {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallParticipantBuilder {
        let mut inner = GroupCallParticipant::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallParticipantBuilder { inner }
    }

    pub fn participant_id(&self) -> &MessageSender {
        &self.participant_id
    }

    pub fn audio_source_id(&self) -> i32 {
        self.audio_source_id
    }

    pub fn screen_sharing_audio_source_id(&self) -> i32 {
        self.screen_sharing_audio_source_id
    }

    pub fn video_info(&self) -> &Option<GroupCallParticipantVideoInfo> {
        &self.video_info
    }

    pub fn screen_sharing_video_info(&self) -> &Option<GroupCallParticipantVideoInfo> {
        &self.screen_sharing_video_info
    }

    pub fn bio(&self) -> &String {
        &self.bio
    }

    pub fn is_current_user(&self) -> bool {
        self.is_current_user
    }

    pub fn is_speaking(&self) -> bool {
        self.is_speaking
    }

    pub fn is_hand_raised(&self) -> bool {
        self.is_hand_raised
    }

    pub fn can_be_muted_for_all_users(&self) -> bool {
        self.can_be_muted_for_all_users
    }

    pub fn can_be_unmuted_for_all_users(&self) -> bool {
        self.can_be_unmuted_for_all_users
    }

    pub fn can_be_muted_for_current_user(&self) -> bool {
        self.can_be_muted_for_current_user
    }

    pub fn can_be_unmuted_for_current_user(&self) -> bool {
        self.can_be_unmuted_for_current_user
    }

    pub fn is_muted_for_all_users(&self) -> bool {
        self.is_muted_for_all_users
    }

    pub fn is_muted_for_current_user(&self) -> bool {
        self.is_muted_for_current_user
    }

    pub fn can_unmute_self(&self) -> bool {
        self.can_unmute_self
    }

    pub fn volume_level(&self) -> i32 {
        self.volume_level
    }

    pub fn order(&self) -> &String {
        &self.order
    }
}

#[doc(hidden)]
pub struct GroupCallParticipantBuilder {
    inner: GroupCallParticipant,
}

#[deprecated]
pub type RTDGroupCallParticipantBuilder = GroupCallParticipantBuilder;

impl GroupCallParticipantBuilder {
    pub fn build(&self) -> GroupCallParticipant {
        self.inner.clone()
    }

    pub fn participant_id<T: AsRef<MessageSender>>(&mut self, participant_id: T) -> &mut Self {
        self.inner.participant_id = participant_id.as_ref().clone();
        self
    }

    pub fn audio_source_id(&mut self, audio_source_id: i32) -> &mut Self {
        self.inner.audio_source_id = audio_source_id;
        self
    }

    pub fn screen_sharing_audio_source_id(
        &mut self,
        screen_sharing_audio_source_id: i32,
    ) -> &mut Self {
        self.inner.screen_sharing_audio_source_id = screen_sharing_audio_source_id;
        self
    }

    pub fn video_info<T: AsRef<GroupCallParticipantVideoInfo>>(
        &mut self,
        video_info: T,
    ) -> &mut Self {
        self.inner.video_info = Some(video_info.as_ref().clone());
        self
    }

    pub fn screen_sharing_video_info<T: AsRef<GroupCallParticipantVideoInfo>>(
        &mut self,
        screen_sharing_video_info: T,
    ) -> &mut Self {
        self.inner.screen_sharing_video_info = Some(screen_sharing_video_info.as_ref().clone());
        self
    }

    pub fn bio<T: AsRef<str>>(&mut self, bio: T) -> &mut Self {
        self.inner.bio = bio.as_ref().to_string();
        self
    }

    pub fn is_current_user(&mut self, is_current_user: bool) -> &mut Self {
        self.inner.is_current_user = is_current_user;
        self
    }

    pub fn is_speaking(&mut self, is_speaking: bool) -> &mut Self {
        self.inner.is_speaking = is_speaking;
        self
    }

    pub fn is_hand_raised(&mut self, is_hand_raised: bool) -> &mut Self {
        self.inner.is_hand_raised = is_hand_raised;
        self
    }

    pub fn can_be_muted_for_all_users(&mut self, can_be_muted_for_all_users: bool) -> &mut Self {
        self.inner.can_be_muted_for_all_users = can_be_muted_for_all_users;
        self
    }

    pub fn can_be_unmuted_for_all_users(
        &mut self,
        can_be_unmuted_for_all_users: bool,
    ) -> &mut Self {
        self.inner.can_be_unmuted_for_all_users = can_be_unmuted_for_all_users;
        self
    }

    pub fn can_be_muted_for_current_user(
        &mut self,
        can_be_muted_for_current_user: bool,
    ) -> &mut Self {
        self.inner.can_be_muted_for_current_user = can_be_muted_for_current_user;
        self
    }

    pub fn can_be_unmuted_for_current_user(
        &mut self,
        can_be_unmuted_for_current_user: bool,
    ) -> &mut Self {
        self.inner.can_be_unmuted_for_current_user = can_be_unmuted_for_current_user;
        self
    }

    pub fn is_muted_for_all_users(&mut self, is_muted_for_all_users: bool) -> &mut Self {
        self.inner.is_muted_for_all_users = is_muted_for_all_users;
        self
    }

    pub fn is_muted_for_current_user(&mut self, is_muted_for_current_user: bool) -> &mut Self {
        self.inner.is_muted_for_current_user = is_muted_for_current_user;
        self
    }

    pub fn can_unmute_self(&mut self, can_unmute_self: bool) -> &mut Self {
        self.inner.can_unmute_self = can_unmute_self;
        self
    }

    pub fn volume_level(&mut self, volume_level: i32) -> &mut Self {
        self.inner.volume_level = volume_level;
        self
    }

    pub fn order<T: AsRef<str>>(&mut self, order: T) -> &mut Self {
        self.inner.order = order.as_ref().to_string();
        self
    }
}

impl AsRef<GroupCallParticipant> for GroupCallParticipant {
    fn as_ref(&self) -> &GroupCallParticipant {
        self
    }
}

impl AsRef<GroupCallParticipant> for GroupCallParticipantBuilder {
    fn as_ref(&self) -> &GroupCallParticipant {
        &self.inner
    }
}
