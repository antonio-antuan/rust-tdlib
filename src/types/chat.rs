
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat unique identifier
  id: i64,
  /// Type of the chat
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: ChatType,
  /// A chat list to which the chat belongs; may be null
  chat_list: Option<ChatList>,
  /// Chat title
  title: String,
  /// Chat photo; may be null
  photo: Option<ChatPhoto>,
  /// Actions that non-administrator chat members are allowed to take in the chat
  permissions: ChatPermissions,
  /// Last message in the chat; may be null
  last_message: Option<Message>,
  /// Descending parameter by which chats are sorted in the main chat list. If the order number of two chats is the same, they must be sorted in descending order by ID. If 0, the position of the chat in the list is undetermined
  #[serde(deserialize_with = "super::_common::number_from_string")] order: isize,
  /// True, if the chat is pinned
  is_pinned: bool,
  /// True, if the chat is marked as unread
  is_marked_as_unread: bool,
  /// True, if the chat is sponsored by the user's MTProxy server
  is_sponsored: bool,
  /// True, if the chat has scheduled messages
  has_scheduled_messages: bool,
  /// True, if the chat messages can be deleted only for the current user while other users will continue to see the messages
  can_be_deleted_only_for_self: bool,
  /// True, if the chat messages can be deleted for all users
  can_be_deleted_for_all_users: bool,
  /// True, if the chat can be reported to Telegram moderators through reportChat
  can_be_reported: bool,
  /// Default value of the disable_notification parameter, used when a message is sent to the chat
  default_disable_notification: bool,
  /// Number of unread messages in the chat
  unread_count: i64,
  /// Identifier of the last read incoming message
  last_read_inbox_message_id: i64,
  /// Identifier of the last read outgoing message
  last_read_outbox_message_id: i64,
  /// Number of unread messages with a mention/reply in the chat
  unread_mention_count: i64,
  /// Notification settings for this chat
  notification_settings: ChatNotificationSettings,
  /// Describes actions which should be possible to do through a chat action bar; may be null
  action_bar: Option<ChatActionBar>,
  /// Identifier of the pinned message in the chat; 0 if none
  pinned_message_id: i64,
  /// Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat
  reply_markup_message_id: i64,
  /// A draft of a message in the chat; may be null
  draft_message: Option<DraftMessage>,
  /// Contains client-specific data associated with the chat. (For example, the chat position or local chat notification settings can be stored here.) Persistent if the message database is used
  client_data: String,
  
}

impl RObject for Chat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Chat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatBuilder {
    let mut inner = Chat::default();
    inner.td_name = "chat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn type_(&self) -> &ChatType { &self.type_ }

  pub fn chat_list(&self) -> &Option<ChatList> { &self.chat_list }

  pub fn title(&self) -> &String { &self.title }

  pub fn photo(&self) -> &Option<ChatPhoto> { &self.photo }

  pub fn permissions(&self) -> &ChatPermissions { &self.permissions }

  pub fn last_message(&self) -> &Option<Message> { &self.last_message }

  pub fn order(&self) -> isize { self.order }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

  pub fn is_marked_as_unread(&self) -> bool { self.is_marked_as_unread }

  pub fn is_sponsored(&self) -> bool { self.is_sponsored }

  pub fn has_scheduled_messages(&self) -> bool { self.has_scheduled_messages }

  pub fn can_be_deleted_only_for_self(&self) -> bool { self.can_be_deleted_only_for_self }

  pub fn can_be_deleted_for_all_users(&self) -> bool { self.can_be_deleted_for_all_users }

  pub fn can_be_reported(&self) -> bool { self.can_be_reported }

  pub fn default_disable_notification(&self) -> bool { self.default_disable_notification }

  pub fn unread_count(&self) -> i64 { self.unread_count }

  pub fn last_read_inbox_message_id(&self) -> i64 { self.last_read_inbox_message_id }

  pub fn last_read_outbox_message_id(&self) -> i64 { self.last_read_outbox_message_id }

  pub fn unread_mention_count(&self) -> i64 { self.unread_mention_count }

  pub fn notification_settings(&self) -> &ChatNotificationSettings { &self.notification_settings }

  pub fn action_bar(&self) -> &Option<ChatActionBar> { &self.action_bar }

  pub fn pinned_message_id(&self) -> i64 { self.pinned_message_id }

  pub fn reply_markup_message_id(&self) -> i64 { self.reply_markup_message_id }

  pub fn draft_message(&self) -> &Option<DraftMessage> { &self.draft_message }

  pub fn client_data(&self) -> &String { &self.client_data }

}

#[doc(hidden)]
pub struct RTDChatBuilder {
  inner: Chat
}

impl RTDChatBuilder {
  pub fn build(&self) -> Chat { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn type_<T: AsRef<ChatType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = Some(chat_list.as_ref().clone());
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

   
  pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
    self.inner.permissions = permissions.as_ref().clone();
    self
  }

   
  pub fn last_message<T: AsRef<Message>>(&mut self, last_message: T) -> &mut Self {
    self.inner.last_message = Some(last_message.as_ref().clone());
    self
  }

   
  pub fn order(&mut self, order: isize) -> &mut Self {
    self.inner.order = order;
    self
  }

   
  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

   
  pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
    self.inner.is_marked_as_unread = is_marked_as_unread;
    self
  }

   
  pub fn is_sponsored(&mut self, is_sponsored: bool) -> &mut Self {
    self.inner.is_sponsored = is_sponsored;
    self
  }

   
  pub fn has_scheduled_messages(&mut self, has_scheduled_messages: bool) -> &mut Self {
    self.inner.has_scheduled_messages = has_scheduled_messages;
    self
  }

   
  pub fn can_be_deleted_only_for_self(&mut self, can_be_deleted_only_for_self: bool) -> &mut Self {
    self.inner.can_be_deleted_only_for_self = can_be_deleted_only_for_self;
    self
  }

   
  pub fn can_be_deleted_for_all_users(&mut self, can_be_deleted_for_all_users: bool) -> &mut Self {
    self.inner.can_be_deleted_for_all_users = can_be_deleted_for_all_users;
    self
  }

   
  pub fn can_be_reported(&mut self, can_be_reported: bool) -> &mut Self {
    self.inner.can_be_reported = can_be_reported;
    self
  }

   
  pub fn default_disable_notification(&mut self, default_disable_notification: bool) -> &mut Self {
    self.inner.default_disable_notification = default_disable_notification;
    self
  }

   
  pub fn unread_count(&mut self, unread_count: i64) -> &mut Self {
    self.inner.unread_count = unread_count;
    self
  }

   
  pub fn last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self {
    self.inner.last_read_inbox_message_id = last_read_inbox_message_id;
    self
  }

   
  pub fn last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self {
    self.inner.last_read_outbox_message_id = last_read_outbox_message_id;
    self
  }

   
  pub fn unread_mention_count(&mut self, unread_mention_count: i64) -> &mut Self {
    self.inner.unread_mention_count = unread_mention_count;
    self
  }

   
  pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(&mut self, notification_settings: T) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }

   
  pub fn action_bar<T: AsRef<ChatActionBar>>(&mut self, action_bar: T) -> &mut Self {
    self.inner.action_bar = Some(action_bar.as_ref().clone());
    self
  }

   
  pub fn pinned_message_id(&mut self, pinned_message_id: i64) -> &mut Self {
    self.inner.pinned_message_id = pinned_message_id;
    self
  }

   
  pub fn reply_markup_message_id(&mut self, reply_markup_message_id: i64) -> &mut Self {
    self.inner.reply_markup_message_id = reply_markup_message_id;
    self
  }

   
  pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
    self.inner.draft_message = Some(draft_message.as_ref().clone());
    self
  }

   
  pub fn client_data<T: AsRef<str>>(&mut self, client_data: T) -> &mut Self {
    self.inner.client_data = client_data.as_ref().to_string();
    self
  }

}

impl AsRef<Chat> for Chat {
  fn as_ref(&self) -> &Chat { self }
}

impl AsRef<Chat> for RTDChatBuilder {
  fn as_ref(&self) -> &Chat { &self.inner }
}



