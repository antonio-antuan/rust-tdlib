use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    id: i32,
    /// Group call title

    #[serde(default)]
    title: String,
    /// Point in time (Unix timestamp) when the group call is supposed to be started by an administrator; 0 if it is already active or was ended

    #[serde(default)]
    scheduled_start_date: i32,
    /// True, if the group call is scheduled and the current user will receive a notification when the group call will start

    #[serde(default)]
    enabled_start_notification: bool,
    /// True, if the call is active

    #[serde(default)]
    is_active: bool,
    /// True, if the call is joined

    #[serde(default)]
    is_joined: bool,
    /// True, if user was kicked from the call because of network loss and the call needs to be rejoined

    #[serde(default)]
    need_rejoin: bool,
    /// True, if the current user can manage the group call

    #[serde(default)]
    can_be_managed: bool,
    /// Number of participants in the group call

    #[serde(default)]
    participant_count: i32,
    /// True, if all group call participants are loaded

    #[serde(default)]
    loaded_all_participants: bool,
    /// At most 3 recently speaking users in the group call

    #[serde(default)]
    recent_speakers: Vec<GroupCallRecentSpeaker>,
    /// True, if the current user's video is enabled

    #[serde(default)]
    is_my_video_enabled: bool,
    /// True, if the current user's video is paused

    #[serde(default)]
    is_my_video_paused: bool,
    /// True, if the current user can broadcast video or share screen

    #[serde(default)]
    can_enable_video: bool,
    /// True, if only group call administrators can unmute new participants

    #[serde(default)]
    mute_new_participants: bool,
    /// True, if the current user can enable or disable mute_new_participants setting

    #[serde(default)]
    can_toggle_mute_new_participants: bool,
    /// Duration of the ongoing group call recording, in seconds; 0 if none. An updateGroupCall update is not triggered when value of this field changes, but the same recording goes on

    #[serde(default)]
    record_duration: i32,
    /// True, if a video file is being recorded for the call

    #[serde(default)]
    is_video_recorded: bool,
    /// Call duration, in seconds; for ended calls only

    #[serde(default)]
    duration: i32,
}

impl RObject for GroupCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GroupCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallBuilder {
        let mut inner = GroupCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn scheduled_start_date(&self) -> i32 {
        self.scheduled_start_date
    }

    pub fn enabled_start_notification(&self) -> bool {
        self.enabled_start_notification
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_joined(&self) -> bool {
        self.is_joined
    }

    pub fn need_rejoin(&self) -> bool {
        self.need_rejoin
    }

    pub fn can_be_managed(&self) -> bool {
        self.can_be_managed
    }

    pub fn participant_count(&self) -> i32 {
        self.participant_count
    }

    pub fn loaded_all_participants(&self) -> bool {
        self.loaded_all_participants
    }

    pub fn recent_speakers(&self) -> &Vec<GroupCallRecentSpeaker> {
        &self.recent_speakers
    }

    pub fn is_my_video_enabled(&self) -> bool {
        self.is_my_video_enabled
    }

    pub fn is_my_video_paused(&self) -> bool {
        self.is_my_video_paused
    }

    pub fn can_enable_video(&self) -> bool {
        self.can_enable_video
    }

    pub fn mute_new_participants(&self) -> bool {
        self.mute_new_participants
    }

    pub fn can_toggle_mute_new_participants(&self) -> bool {
        self.can_toggle_mute_new_participants
    }

    pub fn record_duration(&self) -> i32 {
        self.record_duration
    }

    pub fn is_video_recorded(&self) -> bool {
        self.is_video_recorded
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }
}

#[doc(hidden)]
pub struct GroupCallBuilder {
    inner: GroupCall,
}

#[deprecated]
pub type RTDGroupCallBuilder = GroupCallBuilder;

impl GroupCallBuilder {
    pub fn build(&self) -> GroupCall {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn scheduled_start_date(&mut self, scheduled_start_date: i32) -> &mut Self {
        self.inner.scheduled_start_date = scheduled_start_date;
        self
    }

    pub fn enabled_start_notification(&mut self, enabled_start_notification: bool) -> &mut Self {
        self.inner.enabled_start_notification = enabled_start_notification;
        self
    }

    pub fn is_active(&mut self, is_active: bool) -> &mut Self {
        self.inner.is_active = is_active;
        self
    }

    pub fn is_joined(&mut self, is_joined: bool) -> &mut Self {
        self.inner.is_joined = is_joined;
        self
    }

    pub fn need_rejoin(&mut self, need_rejoin: bool) -> &mut Self {
        self.inner.need_rejoin = need_rejoin;
        self
    }

    pub fn can_be_managed(&mut self, can_be_managed: bool) -> &mut Self {
        self.inner.can_be_managed = can_be_managed;
        self
    }

    pub fn participant_count(&mut self, participant_count: i32) -> &mut Self {
        self.inner.participant_count = participant_count;
        self
    }

    pub fn loaded_all_participants(&mut self, loaded_all_participants: bool) -> &mut Self {
        self.inner.loaded_all_participants = loaded_all_participants;
        self
    }

    pub fn recent_speakers(&mut self, recent_speakers: Vec<GroupCallRecentSpeaker>) -> &mut Self {
        self.inner.recent_speakers = recent_speakers;
        self
    }

    pub fn is_my_video_enabled(&mut self, is_my_video_enabled: bool) -> &mut Self {
        self.inner.is_my_video_enabled = is_my_video_enabled;
        self
    }

    pub fn is_my_video_paused(&mut self, is_my_video_paused: bool) -> &mut Self {
        self.inner.is_my_video_paused = is_my_video_paused;
        self
    }

    pub fn can_enable_video(&mut self, can_enable_video: bool) -> &mut Self {
        self.inner.can_enable_video = can_enable_video;
        self
    }

    pub fn mute_new_participants(&mut self, mute_new_participants: bool) -> &mut Self {
        self.inner.mute_new_participants = mute_new_participants;
        self
    }

    pub fn can_toggle_mute_new_participants(
        &mut self,
        can_toggle_mute_new_participants: bool,
    ) -> &mut Self {
        self.inner.can_toggle_mute_new_participants = can_toggle_mute_new_participants;
        self
    }

    pub fn record_duration(&mut self, record_duration: i32) -> &mut Self {
        self.inner.record_duration = record_duration;
        self
    }

    pub fn is_video_recorded(&mut self, is_video_recorded: bool) -> &mut Self {
        self.inner.is_video_recorded = is_video_recorded;
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }
}

impl AsRef<GroupCall> for GroupCall {
    fn as_ref(&self) -> &GroupCall {
        self
    }
}

impl AsRef<GroupCall> for GroupCallBuilder {
    fn as_ref(&self) -> &GroupCall {
        &self.inner
    }
}
