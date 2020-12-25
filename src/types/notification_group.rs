
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a group of notifications
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique persistent auto-incremented from 1 identifier of the notification group
  id: i64,
  /// Type of the group
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: NotificationGroupType,
  /// Identifier of a chat to which all notifications in the group belong
  chat_id: i64,
  /// Total number of active notifications in the group
  total_count: i64,
  /// The list of active notifications
  notifications: Vec<Notification>,
  
}

impl RObject for NotificationGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl NotificationGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNotificationGroupBuilder {
    let mut inner = NotificationGroup::default();
    inner.td_name = "notificationGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDNotificationGroupBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn type_(&self) -> &NotificationGroupType { &self.type_ }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn notifications(&self) -> &Vec<Notification> { &self.notifications }

}

#[doc(hidden)]
pub struct RTDNotificationGroupBuilder {
  inner: NotificationGroup
}

impl RTDNotificationGroupBuilder {
  pub fn build(&self) -> NotificationGroup { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn type_<T: AsRef<NotificationGroupType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn notifications(&mut self, notifications: Vec<Notification>) -> &mut Self {
    self.inner.notifications = notifications;
    self
  }

}

impl AsRef<NotificationGroup> for NotificationGroup {
  fn as_ref(&self) -> &NotificationGroup { self }
}

impl AsRef<NotificationGroup> for RTDNotificationGroupBuilder {
  fn as_ref(&self) -> &NotificationGroup { &self.inner }
}



