use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains a detailed statistics about a chat
pub trait TDChatStatistics: Debug + RObject {}

/// Contains a detailed statistics about a chat
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatistics {
    #[doc(hidden)]
    _Default,
    /// A detailed statistics about a channel chat
    #[serde(rename = "chatStatisticsChannel")]
    Channel(ChatStatisticsChannel),
    /// A detailed statistics about a supergroup chat
    #[serde(rename = "chatStatisticsSupergroup")]
    Supergroup(ChatStatisticsSupergroup),
    /// Returns detailed statistics about a chat. Currently, this method can be used only for supergroups and channels. Can be used only if supergroupFullInfo.can_get_statistics == true
    #[serde(rename = "getChatStatistics")]
    GetChatStatistics(GetChatStatistics),
}

impl Default for ChatStatistics {
    fn default() -> Self {
        ChatStatistics::_Default
    }
}

impl RObject for ChatStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatStatistics::Channel(t) => t.extra(),
            ChatStatistics::Supergroup(t) => t.extra(),
            ChatStatistics::GetChatStatistics(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatStatistics::Channel(t) => t.client_id(),
            ChatStatistics::Supergroup(t) => t.client_id(),
            ChatStatistics::GetChatStatistics(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatStatistics::_Default)
    }
}

impl AsRef<ChatStatistics> for ChatStatistics {
    fn as_ref(&self) -> &ChatStatistics {
        self
    }
}

/// A detailed statistics about a channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsChannel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A period to which the statistics applies
    period: DateRange,
    /// Number of members in the chat
    member_count: StatisticalValue,
    /// Mean number of times the recently sent messages was viewed
    mean_view_count: StatisticalValue,
    /// Mean number of times the recently sent messages was shared
    mean_share_count: StatisticalValue,
    /// A percentage of users with enabled notifications for the chat

    #[serde(default)]
    enabled_notifications_percentage: f32,
    /// A graph containing number of members in the chat

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    member_count_graph: StatisticalGraph,
    /// A graph containing number of members joined and left the chat

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    join_graph: StatisticalGraph,
    /// A graph containing number of members muted and unmuted the chat

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    mute_graph: StatisticalGraph,
    /// A graph containing number of message views in a given hour in the last two weeks

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    view_count_by_hour_graph: StatisticalGraph,
    /// A graph containing number of message views per source

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    view_count_by_source_graph: StatisticalGraph,
    /// A graph containing number of new member joins per source

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    join_by_source_graph: StatisticalGraph,
    /// A graph containing number of users viewed chat messages per language

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    language_graph: StatisticalGraph,
    /// A graph containing number of chat message views and shares

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    message_interaction_graph: StatisticalGraph,
    /// A graph containing number of views of associated with the chat instant views

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    instant_view_interaction_graph: StatisticalGraph,
    /// Detailed statistics about number of views and shares of recently sent messages

    #[serde(default)]
    recent_message_interactions: Vec<ChatStatisticsMessageInteractionInfo>,
}

impl RObject for ChatStatisticsChannel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatStatistics for ChatStatisticsChannel {}

impl ChatStatisticsChannel {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatStatisticsChannelBuilder {
        let mut inner = ChatStatisticsChannel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatStatisticsChannelBuilder { inner }
    }

    pub fn period(&self) -> &DateRange {
        &self.period
    }

    pub fn member_count(&self) -> &StatisticalValue {
        &self.member_count
    }

    pub fn mean_view_count(&self) -> &StatisticalValue {
        &self.mean_view_count
    }

    pub fn mean_share_count(&self) -> &StatisticalValue {
        &self.mean_share_count
    }

    pub fn enabled_notifications_percentage(&self) -> f32 {
        self.enabled_notifications_percentage
    }

    pub fn member_count_graph(&self) -> &StatisticalGraph {
        &self.member_count_graph
    }

    pub fn join_graph(&self) -> &StatisticalGraph {
        &self.join_graph
    }

    pub fn mute_graph(&self) -> &StatisticalGraph {
        &self.mute_graph
    }

    pub fn view_count_by_hour_graph(&self) -> &StatisticalGraph {
        &self.view_count_by_hour_graph
    }

    pub fn view_count_by_source_graph(&self) -> &StatisticalGraph {
        &self.view_count_by_source_graph
    }

    pub fn join_by_source_graph(&self) -> &StatisticalGraph {
        &self.join_by_source_graph
    }

    pub fn language_graph(&self) -> &StatisticalGraph {
        &self.language_graph
    }

    pub fn message_interaction_graph(&self) -> &StatisticalGraph {
        &self.message_interaction_graph
    }

    pub fn instant_view_interaction_graph(&self) -> &StatisticalGraph {
        &self.instant_view_interaction_graph
    }

    pub fn recent_message_interactions(&self) -> &Vec<ChatStatisticsMessageInteractionInfo> {
        &self.recent_message_interactions
    }
}

#[doc(hidden)]
pub struct ChatStatisticsChannelBuilder {
    inner: ChatStatisticsChannel,
}

#[deprecated]
pub type RTDChatStatisticsChannelBuilder = ChatStatisticsChannelBuilder;

impl ChatStatisticsChannelBuilder {
    pub fn build(&self) -> ChatStatisticsChannel {
        self.inner.clone()
    }

    pub fn period<T: AsRef<DateRange>>(&mut self, period: T) -> &mut Self {
        self.inner.period = period.as_ref().clone();
        self
    }

    pub fn member_count<T: AsRef<StatisticalValue>>(&mut self, member_count: T) -> &mut Self {
        self.inner.member_count = member_count.as_ref().clone();
        self
    }

    pub fn mean_view_count<T: AsRef<StatisticalValue>>(&mut self, mean_view_count: T) -> &mut Self {
        self.inner.mean_view_count = mean_view_count.as_ref().clone();
        self
    }

    pub fn mean_share_count<T: AsRef<StatisticalValue>>(
        &mut self,
        mean_share_count: T,
    ) -> &mut Self {
        self.inner.mean_share_count = mean_share_count.as_ref().clone();
        self
    }

    pub fn enabled_notifications_percentage(
        &mut self,
        enabled_notifications_percentage: f32,
    ) -> &mut Self {
        self.inner.enabled_notifications_percentage = enabled_notifications_percentage;
        self
    }

    pub fn member_count_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        member_count_graph: T,
    ) -> &mut Self {
        self.inner.member_count_graph = member_count_graph.as_ref().clone();
        self
    }

    pub fn join_graph<T: AsRef<StatisticalGraph>>(&mut self, join_graph: T) -> &mut Self {
        self.inner.join_graph = join_graph.as_ref().clone();
        self
    }

    pub fn mute_graph<T: AsRef<StatisticalGraph>>(&mut self, mute_graph: T) -> &mut Self {
        self.inner.mute_graph = mute_graph.as_ref().clone();
        self
    }

    pub fn view_count_by_hour_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        view_count_by_hour_graph: T,
    ) -> &mut Self {
        self.inner.view_count_by_hour_graph = view_count_by_hour_graph.as_ref().clone();
        self
    }

    pub fn view_count_by_source_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        view_count_by_source_graph: T,
    ) -> &mut Self {
        self.inner.view_count_by_source_graph = view_count_by_source_graph.as_ref().clone();
        self
    }

    pub fn join_by_source_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        join_by_source_graph: T,
    ) -> &mut Self {
        self.inner.join_by_source_graph = join_by_source_graph.as_ref().clone();
        self
    }

    pub fn language_graph<T: AsRef<StatisticalGraph>>(&mut self, language_graph: T) -> &mut Self {
        self.inner.language_graph = language_graph.as_ref().clone();
        self
    }

    pub fn message_interaction_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        message_interaction_graph: T,
    ) -> &mut Self {
        self.inner.message_interaction_graph = message_interaction_graph.as_ref().clone();
        self
    }

    pub fn instant_view_interaction_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        instant_view_interaction_graph: T,
    ) -> &mut Self {
        self.inner.instant_view_interaction_graph = instant_view_interaction_graph.as_ref().clone();
        self
    }

    pub fn recent_message_interactions(
        &mut self,
        recent_message_interactions: Vec<ChatStatisticsMessageInteractionInfo>,
    ) -> &mut Self {
        self.inner.recent_message_interactions = recent_message_interactions;
        self
    }
}

impl AsRef<ChatStatisticsChannel> for ChatStatisticsChannel {
    fn as_ref(&self) -> &ChatStatisticsChannel {
        self
    }
}

impl AsRef<ChatStatisticsChannel> for ChatStatisticsChannelBuilder {
    fn as_ref(&self) -> &ChatStatisticsChannel {
        &self.inner
    }
}

/// A detailed statistics about a supergroup chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsSupergroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A period to which the statistics applies
    period: DateRange,
    /// Number of members in the chat
    member_count: StatisticalValue,
    /// Number of messages sent to the chat
    message_count: StatisticalValue,
    /// Number of users who viewed messages in the chat
    viewer_count: StatisticalValue,
    /// Number of users who sent messages to the chat
    sender_count: StatisticalValue,
    /// A graph containing number of members in the chat

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    member_count_graph: StatisticalGraph,
    /// A graph containing number of members joined and left the chat

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    join_graph: StatisticalGraph,
    /// A graph containing number of new member joins per source

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    join_by_source_graph: StatisticalGraph,
    /// A graph containing distribution of active users per language

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    language_graph: StatisticalGraph,
    /// A graph containing distribution of sent messages by content type

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    message_content_graph: StatisticalGraph,
    /// A graph containing number of different actions in the chat

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    action_graph: StatisticalGraph,
    /// A graph containing distribution of message views per hour

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    day_graph: StatisticalGraph,
    /// A graph containing distribution of message views per day of week

    #[serde(skip_serializing_if = "StatisticalGraph::_is_default")]
    week_graph: StatisticalGraph,
    /// List of users sent most messages in the last week

    #[serde(default)]
    top_senders: Vec<ChatStatisticsMessageSenderInfo>,
    /// List of most active administrators in the last week

    #[serde(default)]
    top_administrators: Vec<ChatStatisticsAdministratorActionsInfo>,
    /// List of most active inviters of new members in the last week

    #[serde(default)]
    top_inviters: Vec<ChatStatisticsInviterInfo>,
}

impl RObject for ChatStatisticsSupergroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatStatistics for ChatStatisticsSupergroup {}

impl ChatStatisticsSupergroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatStatisticsSupergroupBuilder {
        let mut inner = ChatStatisticsSupergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatStatisticsSupergroupBuilder { inner }
    }

    pub fn period(&self) -> &DateRange {
        &self.period
    }

    pub fn member_count(&self) -> &StatisticalValue {
        &self.member_count
    }

    pub fn message_count(&self) -> &StatisticalValue {
        &self.message_count
    }

    pub fn viewer_count(&self) -> &StatisticalValue {
        &self.viewer_count
    }

    pub fn sender_count(&self) -> &StatisticalValue {
        &self.sender_count
    }

    pub fn member_count_graph(&self) -> &StatisticalGraph {
        &self.member_count_graph
    }

    pub fn join_graph(&self) -> &StatisticalGraph {
        &self.join_graph
    }

    pub fn join_by_source_graph(&self) -> &StatisticalGraph {
        &self.join_by_source_graph
    }

    pub fn language_graph(&self) -> &StatisticalGraph {
        &self.language_graph
    }

    pub fn message_content_graph(&self) -> &StatisticalGraph {
        &self.message_content_graph
    }

    pub fn action_graph(&self) -> &StatisticalGraph {
        &self.action_graph
    }

    pub fn day_graph(&self) -> &StatisticalGraph {
        &self.day_graph
    }

    pub fn week_graph(&self) -> &StatisticalGraph {
        &self.week_graph
    }

    pub fn top_senders(&self) -> &Vec<ChatStatisticsMessageSenderInfo> {
        &self.top_senders
    }

    pub fn top_administrators(&self) -> &Vec<ChatStatisticsAdministratorActionsInfo> {
        &self.top_administrators
    }

    pub fn top_inviters(&self) -> &Vec<ChatStatisticsInviterInfo> {
        &self.top_inviters
    }
}

#[doc(hidden)]
pub struct ChatStatisticsSupergroupBuilder {
    inner: ChatStatisticsSupergroup,
}

#[deprecated]
pub type RTDChatStatisticsSupergroupBuilder = ChatStatisticsSupergroupBuilder;

impl ChatStatisticsSupergroupBuilder {
    pub fn build(&self) -> ChatStatisticsSupergroup {
        self.inner.clone()
    }

    pub fn period<T: AsRef<DateRange>>(&mut self, period: T) -> &mut Self {
        self.inner.period = period.as_ref().clone();
        self
    }

    pub fn member_count<T: AsRef<StatisticalValue>>(&mut self, member_count: T) -> &mut Self {
        self.inner.member_count = member_count.as_ref().clone();
        self
    }

    pub fn message_count<T: AsRef<StatisticalValue>>(&mut self, message_count: T) -> &mut Self {
        self.inner.message_count = message_count.as_ref().clone();
        self
    }

    pub fn viewer_count<T: AsRef<StatisticalValue>>(&mut self, viewer_count: T) -> &mut Self {
        self.inner.viewer_count = viewer_count.as_ref().clone();
        self
    }

    pub fn sender_count<T: AsRef<StatisticalValue>>(&mut self, sender_count: T) -> &mut Self {
        self.inner.sender_count = sender_count.as_ref().clone();
        self
    }

    pub fn member_count_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        member_count_graph: T,
    ) -> &mut Self {
        self.inner.member_count_graph = member_count_graph.as_ref().clone();
        self
    }

    pub fn join_graph<T: AsRef<StatisticalGraph>>(&mut self, join_graph: T) -> &mut Self {
        self.inner.join_graph = join_graph.as_ref().clone();
        self
    }

    pub fn join_by_source_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        join_by_source_graph: T,
    ) -> &mut Self {
        self.inner.join_by_source_graph = join_by_source_graph.as_ref().clone();
        self
    }

    pub fn language_graph<T: AsRef<StatisticalGraph>>(&mut self, language_graph: T) -> &mut Self {
        self.inner.language_graph = language_graph.as_ref().clone();
        self
    }

    pub fn message_content_graph<T: AsRef<StatisticalGraph>>(
        &mut self,
        message_content_graph: T,
    ) -> &mut Self {
        self.inner.message_content_graph = message_content_graph.as_ref().clone();
        self
    }

    pub fn action_graph<T: AsRef<StatisticalGraph>>(&mut self, action_graph: T) -> &mut Self {
        self.inner.action_graph = action_graph.as_ref().clone();
        self
    }

    pub fn day_graph<T: AsRef<StatisticalGraph>>(&mut self, day_graph: T) -> &mut Self {
        self.inner.day_graph = day_graph.as_ref().clone();
        self
    }

    pub fn week_graph<T: AsRef<StatisticalGraph>>(&mut self, week_graph: T) -> &mut Self {
        self.inner.week_graph = week_graph.as_ref().clone();
        self
    }

    pub fn top_senders(&mut self, top_senders: Vec<ChatStatisticsMessageSenderInfo>) -> &mut Self {
        self.inner.top_senders = top_senders;
        self
    }

    pub fn top_administrators(
        &mut self,
        top_administrators: Vec<ChatStatisticsAdministratorActionsInfo>,
    ) -> &mut Self {
        self.inner.top_administrators = top_administrators;
        self
    }

    pub fn top_inviters(&mut self, top_inviters: Vec<ChatStatisticsInviterInfo>) -> &mut Self {
        self.inner.top_inviters = top_inviters;
        self
    }
}

impl AsRef<ChatStatisticsSupergroup> for ChatStatisticsSupergroup {
    fn as_ref(&self) -> &ChatStatisticsSupergroup {
        self
    }
}

impl AsRef<ChatStatisticsSupergroup> for ChatStatisticsSupergroupBuilder {
    fn as_ref(&self) -> &ChatStatisticsSupergroup {
        &self.inner
    }
}
