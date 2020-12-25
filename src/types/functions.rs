
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Accepts an incoming call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcceptCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Call identifier
  call_id: i64,
  /// Description of the call protocols supported by the client
  protocol: CallProtocol,
  
}

impl RObject for AcceptCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "acceptCall" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AcceptCall {}

impl AcceptCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAcceptCallBuilder {
    let mut inner = AcceptCall::default();
    inner.td_name = "acceptCall".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAcceptCallBuilder { inner }
  }

  pub fn call_id(&self) -> i64 { self.call_id }

  pub fn protocol(&self) -> &CallProtocol { &self.protocol }

}

#[doc(hidden)]
pub struct RTDAcceptCallBuilder {
  inner: AcceptCall
}

impl RTDAcceptCallBuilder {
  pub fn build(&self) -> AcceptCall { self.inner.clone() }

   
  pub fn call_id(&mut self, call_id: i64) -> &mut Self {
    self.inner.call_id = call_id;
    self
  }

   
  pub fn protocol<T: AsRef<CallProtocol>>(&mut self, protocol: T) -> &mut Self {
    self.inner.protocol = protocol.as_ref().clone();
    self
  }

}

impl AsRef<AcceptCall> for AcceptCall {
  fn as_ref(&self) -> &AcceptCall { self }
}

impl AsRef<AcceptCall> for RTDAcceptCallBuilder {
  fn as_ref(&self) -> &AcceptCall { &self.inner }
}







/// Accepts Telegram terms of services
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcceptTermsOfService {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Terms of service identifier
  terms_of_service_id: String,
  
}

impl RObject for AcceptTermsOfService {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "acceptTermsOfService" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AcceptTermsOfService {}

impl AcceptTermsOfService {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAcceptTermsOfServiceBuilder {
    let mut inner = AcceptTermsOfService::default();
    inner.td_name = "acceptTermsOfService".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAcceptTermsOfServiceBuilder { inner }
  }

  pub fn terms_of_service_id(&self) -> &String { &self.terms_of_service_id }

}

#[doc(hidden)]
pub struct RTDAcceptTermsOfServiceBuilder {
  inner: AcceptTermsOfService
}

impl RTDAcceptTermsOfServiceBuilder {
  pub fn build(&self) -> AcceptTermsOfService { self.inner.clone() }

   
  pub fn terms_of_service_id<T: AsRef<str>>(&mut self, terms_of_service_id: T) -> &mut Self {
    self.inner.terms_of_service_id = terms_of_service_id.as_ref().to_string();
    self
  }

}

impl AsRef<AcceptTermsOfService> for AcceptTermsOfService {
  fn as_ref(&self) -> &AcceptTermsOfService { self }
}

impl AsRef<AcceptTermsOfService> for RTDAcceptTermsOfServiceBuilder {
  fn as_ref(&self) -> &AcceptTermsOfService { &self.inner }
}







/// Adds a new member to a chat. Members can't be added to private or secret chats. Members will not be added until the chat state has been synchronized with the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddChatMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifier of the user
  user_id: i64,
  /// The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels
  forward_limit: i64,
  
}

impl RObject for AddChatMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addChatMember" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddChatMember {}

impl AddChatMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddChatMemberBuilder {
    let mut inner = AddChatMember::default();
    inner.td_name = "addChatMember".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddChatMemberBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn forward_limit(&self) -> i64 { self.forward_limit }

}

#[doc(hidden)]
pub struct RTDAddChatMemberBuilder {
  inner: AddChatMember
}

impl RTDAddChatMemberBuilder {
  pub fn build(&self) -> AddChatMember { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn forward_limit(&mut self, forward_limit: i64) -> &mut Self {
    self.inner.forward_limit = forward_limit;
    self
  }

}

impl AsRef<AddChatMember> for AddChatMember {
  fn as_ref(&self) -> &AddChatMember { self }
}

impl AsRef<AddChatMember> for RTDAddChatMemberBuilder {
  fn as_ref(&self) -> &AddChatMember { &self.inner }
}







/// Adds multiple new members to a chat. Currently this option is only available for supergroups and channels. This option can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Members will not be added until the chat state has been synchronized with the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifiers of the users to be added to the chat
  user_ids: Vec<i64>,
  
}

impl RObject for AddChatMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addChatMembers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddChatMembers {}

impl AddChatMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddChatMembersBuilder {
    let mut inner = AddChatMembers::default();
    inner.td_name = "addChatMembers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddChatMembersBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn user_ids(&self) -> &Vec<i64> { &self.user_ids }

}

#[doc(hidden)]
pub struct RTDAddChatMembersBuilder {
  inner: AddChatMembers
}

impl RTDAddChatMembersBuilder {
  pub fn build(&self) -> AddChatMembers { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

}

impl AsRef<AddChatMembers> for AddChatMembers {
  fn as_ref(&self) -> &AddChatMembers { self }
}

impl AsRef<AddChatMembers> for RTDAddChatMembersBuilder {
  fn as_ref(&self) -> &AddChatMembers { &self.inner }
}







/// Adds a user to the contact list or edits an existing contact by their user identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The contact to add or edit; phone number can be empty and needs to be specified only if known, vCard is ignored
  contact: Contact,
  /// True, if the new contact needs to be allowed to see current user's phone number. A corresponding rule to userPrivacySettingShowPhoneNumber will be added if needed. Use the field UserFullInfo.need_phone_number_privacy_exception to check whether the current user needs to be asked to share their phone number
  share_phone_number: bool,
  
}

impl RObject for AddContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addContact" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddContact {}

impl AddContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddContactBuilder {
    let mut inner = AddContact::default();
    inner.td_name = "addContact".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddContactBuilder { inner }
  }

  pub fn contact(&self) -> &Contact { &self.contact }

  pub fn share_phone_number(&self) -> bool { self.share_phone_number }

}

#[doc(hidden)]
pub struct RTDAddContactBuilder {
  inner: AddContact
}

impl RTDAddContactBuilder {
  pub fn build(&self) -> AddContact { self.inner.clone() }

   
  pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
    self.inner.contact = contact.as_ref().clone();
    self
  }

   
  pub fn share_phone_number(&mut self, share_phone_number: bool) -> &mut Self {
    self.inner.share_phone_number = share_phone_number;
    self
  }

}

impl AsRef<AddContact> for AddContact {
  fn as_ref(&self) -> &AddContact { self }
}

impl AsRef<AddContact> for RTDAddContactBuilder {
  fn as_ref(&self) -> &AddContact { &self.inner }
}







/// Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddCustomServerLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a language pack to be added; may be different from a name that is used in an "https://t.me/setlanguage/" link
  language_pack_id: String,
  
}

impl RObject for AddCustomServerLanguagePack {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addCustomServerLanguagePack" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddCustomServerLanguagePack {}

impl AddCustomServerLanguagePack {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddCustomServerLanguagePackBuilder {
    let mut inner = AddCustomServerLanguagePack::default();
    inner.td_name = "addCustomServerLanguagePack".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddCustomServerLanguagePackBuilder { inner }
  }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

}

#[doc(hidden)]
pub struct RTDAddCustomServerLanguagePackBuilder {
  inner: AddCustomServerLanguagePack
}

impl RTDAddCustomServerLanguagePackBuilder {
  pub fn build(&self) -> AddCustomServerLanguagePack { self.inner.clone() }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

}

impl AsRef<AddCustomServerLanguagePack> for AddCustomServerLanguagePack {
  fn as_ref(&self) -> &AddCustomServerLanguagePack { self }
}

impl AsRef<AddCustomServerLanguagePack> for RTDAddCustomServerLanguagePackBuilder {
  fn as_ref(&self) -> &AddCustomServerLanguagePack { &self.inner }
}







/// Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddFavoriteSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker file to add
  sticker: InputFile,
  
}

impl RObject for AddFavoriteSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addFavoriteSticker" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddFavoriteSticker {}

impl AddFavoriteSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddFavoriteStickerBuilder {
    let mut inner = AddFavoriteSticker::default();
    inner.td_name = "addFavoriteSticker".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddFavoriteStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

}

#[doc(hidden)]
pub struct RTDAddFavoriteStickerBuilder {
  inner: AddFavoriteSticker
}

impl RTDAddFavoriteStickerBuilder {
  pub fn build(&self) -> AddFavoriteSticker { self.inner.clone() }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<AddFavoriteSticker> for AddFavoriteSticker {
  fn as_ref(&self) -> &AddFavoriteSticker { self }
}

impl AsRef<AddFavoriteSticker> for RTDAddFavoriteStickerBuilder {
  fn as_ref(&self) -> &AddFavoriteSticker { &self.inner }
}







/// Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddLocalMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Target chat
  chat_id: i64,
  /// Identifier of the user who will be shown as the sender of the message; may be 0 for channel posts
  sender_user_id: i64,
  /// Identifier of the message to reply to or 0
  reply_to_message_id: i64,
  /// Pass true to disable notification for the message
  disable_notification: bool,
  /// The content of the message to be added
  input_message_content: InputMessageContent,
  
}

impl RObject for AddLocalMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addLocalMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddLocalMessage {}

impl AddLocalMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddLocalMessageBuilder {
    let mut inner = AddLocalMessage::default();
    inner.td_name = "addLocalMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddLocalMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn reply_to_message_id(&self) -> i64 { self.reply_to_message_id }

  pub fn disable_notification(&self) -> bool { self.disable_notification }

  pub fn input_message_content(&self) -> &InputMessageContent { &self.input_message_content }

}

#[doc(hidden)]
pub struct RTDAddLocalMessageBuilder {
  inner: AddLocalMessage
}

impl RTDAddLocalMessageBuilder {
  pub fn build(&self) -> AddLocalMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.reply_to_message_id = reply_to_message_id;
    self
  }

   
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.disable_notification = disable_notification;
    self
  }

   
  pub fn input_message_content<T: AsRef<InputMessageContent>>(&mut self, input_message_content: T) -> &mut Self {
    self.inner.input_message_content = input_message_content.as_ref().clone();
    self
  }

}

impl AsRef<AddLocalMessage> for AddLocalMessage {
  fn as_ref(&self) -> &AddLocalMessage { self }
}

impl AsRef<AddLocalMessage> for RTDAddLocalMessageBuilder {
  fn as_ref(&self) -> &AddLocalMessage { &self.inner }
}







/// Adds a message to TDLib internal log. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddLogMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The minimum verbosity level needed for the message to be logged, 0-1023
  verbosity_level: i64,
  /// Text of a message to log
  text: String,
  
}

impl RObject for AddLogMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addLogMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddLogMessage {}

impl AddLogMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddLogMessageBuilder {
    let mut inner = AddLogMessage::default();
    inner.td_name = "addLogMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddLogMessageBuilder { inner }
  }

  pub fn verbosity_level(&self) -> i64 { self.verbosity_level }

  pub fn text(&self) -> &String { &self.text }

}

#[doc(hidden)]
pub struct RTDAddLogMessageBuilder {
  inner: AddLogMessage
}

impl RTDAddLogMessageBuilder {
  pub fn build(&self) -> AddLogMessage { self.inner.clone() }

   
  pub fn verbosity_level(&mut self, verbosity_level: i64) -> &mut Self {
    self.inner.verbosity_level = verbosity_level;
    self
  }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

}

impl AsRef<AddLogMessage> for AddLogMessage {
  fn as_ref(&self) -> &AddLogMessage { self }
}

impl AsRef<AddLogMessage> for RTDAddLogMessageBuilder {
  fn as_ref(&self) -> &AddLogMessage { &self.inner }
}







/// Adds the specified data to data usage statistics. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddNetworkStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The network statistics entry with the data to be added to statistics
  entry: NetworkStatisticsEntry,
  
}

impl RObject for AddNetworkStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addNetworkStatistics" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddNetworkStatistics {}

impl AddNetworkStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddNetworkStatisticsBuilder {
    let mut inner = AddNetworkStatistics::default();
    inner.td_name = "addNetworkStatistics".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddNetworkStatisticsBuilder { inner }
  }

  pub fn entry(&self) -> &NetworkStatisticsEntry { &self.entry }

}

#[doc(hidden)]
pub struct RTDAddNetworkStatisticsBuilder {
  inner: AddNetworkStatistics
}

impl RTDAddNetworkStatisticsBuilder {
  pub fn build(&self) -> AddNetworkStatistics { self.inner.clone() }

   
  pub fn entry<T: AsRef<NetworkStatisticsEntry>>(&mut self, entry: T) -> &mut Self {
    self.inner.entry = entry.as_ref().clone();
    self
  }

}

impl AsRef<AddNetworkStatistics> for AddNetworkStatistics {
  fn as_ref(&self) -> &AddNetworkStatistics { self }
}

impl AsRef<AddNetworkStatistics> for RTDAddNetworkStatisticsBuilder {
  fn as_ref(&self) -> &AddNetworkStatistics { &self.inner }
}







/// Adds a proxy server for network requests. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Proxy server IP address
  server: String,
  /// Proxy server port
  port: i64,
  /// True, if the proxy should be enabled
  enable: bool,
  /// Proxy type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: ProxyType,
  
}

impl RObject for AddProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddProxy {}

impl AddProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddProxyBuilder {
    let mut inner = AddProxy::default();
    inner.td_name = "addProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddProxyBuilder { inner }
  }

  pub fn server(&self) -> &String { &self.server }

  pub fn port(&self) -> i64 { self.port }

  pub fn enable(&self) -> bool { self.enable }

  pub fn type_(&self) -> &ProxyType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDAddProxyBuilder {
  inner: AddProxy
}

impl RTDAddProxyBuilder {
  pub fn build(&self) -> AddProxy { self.inner.clone() }

   
  pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
    self.inner.server = server.as_ref().to_string();
    self
  }

   
  pub fn port(&mut self, port: i64) -> &mut Self {
    self.inner.port = port;
    self
  }

   
  pub fn enable(&mut self, enable: bool) -> &mut Self {
    self.inner.enable = enable;
    self
  }

   
  pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<AddProxy> for AddProxy {
  fn as_ref(&self) -> &AddProxy { self }
}

impl AsRef<AddProxy> for RTDAddProxyBuilder {
  fn as_ref(&self) -> &AddProxy { &self.inner }
}







/// Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddRecentSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to add the sticker to the list of stickers recently attached to photo or video files; pass false to add the sticker to the list of recently sent stickers
  is_attached: bool,
  /// Sticker file to add
  sticker: InputFile,
  
}

impl RObject for AddRecentSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addRecentSticker" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddRecentSticker {}

impl AddRecentSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddRecentStickerBuilder {
    let mut inner = AddRecentSticker::default();
    inner.td_name = "addRecentSticker".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddRecentStickerBuilder { inner }
  }

  pub fn is_attached(&self) -> bool { self.is_attached }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

}

#[doc(hidden)]
pub struct RTDAddRecentStickerBuilder {
  inner: AddRecentSticker
}

impl RTDAddRecentStickerBuilder {
  pub fn build(&self) -> AddRecentSticker { self.inner.clone() }

   
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.is_attached = is_attached;
    self
  }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<AddRecentSticker> for AddRecentSticker {
  fn as_ref(&self) -> &AddRecentSticker { self }
}

impl AsRef<AddRecentSticker> for RTDAddRecentStickerBuilder {
  fn as_ref(&self) -> &AddRecentSticker { &self.inner }
}







/// Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddRecentlyFoundChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to add
  chat_id: i64,
  
}

impl RObject for AddRecentlyFoundChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addRecentlyFoundChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddRecentlyFoundChat {}

impl AddRecentlyFoundChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddRecentlyFoundChatBuilder {
    let mut inner = AddRecentlyFoundChat::default();
    inner.td_name = "addRecentlyFoundChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddRecentlyFoundChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDAddRecentlyFoundChatBuilder {
  inner: AddRecentlyFoundChat
}

impl RTDAddRecentlyFoundChatBuilder {
  pub fn build(&self) -> AddRecentlyFoundChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<AddRecentlyFoundChat> for AddRecentlyFoundChat {
  fn as_ref(&self) -> &AddRecentlyFoundChat { self }
}

impl AsRef<AddRecentlyFoundChat> for RTDAddRecentlyFoundChatBuilder {
  fn as_ref(&self) -> &AddRecentlyFoundChat { &self.inner }
}







/// Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type "video/mp4" can be added to the list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddSavedAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The animation file to be added. Only animations known to the server (i.e. successfully sent via a message) can be added to the list
  animation: InputFile,
  
}

impl RObject for AddSavedAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addSavedAnimation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddSavedAnimation {}

impl AddSavedAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddSavedAnimationBuilder {
    let mut inner = AddSavedAnimation::default();
    inner.td_name = "addSavedAnimation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddSavedAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &InputFile { &self.animation }

}

#[doc(hidden)]
pub struct RTDAddSavedAnimationBuilder {
  inner: AddSavedAnimation
}

impl RTDAddSavedAnimationBuilder {
  pub fn build(&self) -> AddSavedAnimation { self.inner.clone() }

   
  pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

}

impl AsRef<AddSavedAnimation> for AddSavedAnimation {
  fn as_ref(&self) -> &AddSavedAnimation { self }
}

impl AsRef<AddSavedAnimation> for RTDAddSavedAnimationBuilder {
  fn as_ref(&self) -> &AddSavedAnimation { &self.inner }
}







/// Adds a new sticker to a set; for bots only. Returns the sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddStickerToSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker set owner
  user_id: i64,
  /// Sticker set name
  name: String,
  /// Sticker to add to the set
  sticker: InputSticker,
  
}

impl RObject for AddStickerToSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addStickerToSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AddStickerToSet {}

impl AddStickerToSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddStickerToSetBuilder {
    let mut inner = AddStickerToSet::default();
    inner.td_name = "addStickerToSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddStickerToSetBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn name(&self) -> &String { &self.name }

  pub fn sticker(&self) -> &InputSticker { &self.sticker }

}

#[doc(hidden)]
pub struct RTDAddStickerToSetBuilder {
  inner: AddStickerToSet
}

impl RTDAddStickerToSetBuilder {
  pub fn build(&self) -> AddStickerToSet { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn sticker<T: AsRef<InputSticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<AddStickerToSet> for AddStickerToSet {
  fn as_ref(&self) -> &AddStickerToSet { self }
}

impl AsRef<AddStickerToSet> for RTDAddStickerToSetBuilder {
  fn as_ref(&self) -> &AddStickerToSet { &self.inner }
}







/// Sets the result of a callback query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerCallbackQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the callback query
  callback_query_id: isize,
  /// Text of the answer
  text: String,
  /// If true, an alert should be shown to the user instead of a toast notification
  show_alert: bool,
  /// URL to be opened
  url: String,
  /// Time during which the result of the query can be cached, in seconds
  cache_time: i64,
  
}

impl RObject for AnswerCallbackQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerCallbackQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AnswerCallbackQuery {}

impl AnswerCallbackQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAnswerCallbackQueryBuilder {
    let mut inner = AnswerCallbackQuery::default();
    inner.td_name = "answerCallbackQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAnswerCallbackQueryBuilder { inner }
  }

  pub fn callback_query_id(&self) -> isize { self.callback_query_id }

  pub fn text(&self) -> &String { &self.text }

  pub fn show_alert(&self) -> bool { self.show_alert }

  pub fn url(&self) -> &String { &self.url }

  pub fn cache_time(&self) -> i64 { self.cache_time }

}

#[doc(hidden)]
pub struct RTDAnswerCallbackQueryBuilder {
  inner: AnswerCallbackQuery
}

impl RTDAnswerCallbackQueryBuilder {
  pub fn build(&self) -> AnswerCallbackQuery { self.inner.clone() }

   
  pub fn callback_query_id(&mut self, callback_query_id: isize) -> &mut Self {
    self.inner.callback_query_id = callback_query_id;
    self
  }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

   
  pub fn show_alert(&mut self, show_alert: bool) -> &mut Self {
    self.inner.show_alert = show_alert;
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn cache_time(&mut self, cache_time: i64) -> &mut Self {
    self.inner.cache_time = cache_time;
    self
  }

}

impl AsRef<AnswerCallbackQuery> for AnswerCallbackQuery {
  fn as_ref(&self) -> &AnswerCallbackQuery { self }
}

impl AsRef<AnswerCallbackQuery> for RTDAnswerCallbackQueryBuilder {
  fn as_ref(&self) -> &AnswerCallbackQuery { &self.inner }
}







/// Answers a custom query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerCustomQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a custom query
  custom_query_id: isize,
  /// JSON-serialized answer to the query
  data: String,
  
}

impl RObject for AnswerCustomQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerCustomQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AnswerCustomQuery {}

impl AnswerCustomQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAnswerCustomQueryBuilder {
    let mut inner = AnswerCustomQuery::default();
    inner.td_name = "answerCustomQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAnswerCustomQueryBuilder { inner }
  }

  pub fn custom_query_id(&self) -> isize { self.custom_query_id }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDAnswerCustomQueryBuilder {
  inner: AnswerCustomQuery
}

impl RTDAnswerCustomQueryBuilder {
  pub fn build(&self) -> AnswerCustomQuery { self.inner.clone() }

   
  pub fn custom_query_id(&mut self, custom_query_id: isize) -> &mut Self {
    self.inner.custom_query_id = custom_query_id;
    self
  }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<AnswerCustomQuery> for AnswerCustomQuery {
  fn as_ref(&self) -> &AnswerCustomQuery { self }
}

impl AsRef<AnswerCustomQuery> for RTDAnswerCustomQueryBuilder {
  fn as_ref(&self) -> &AnswerCustomQuery { &self.inner }
}







/// Sets the result of an inline query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerInlineQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the inline query
  inline_query_id: isize,
  /// True, if the result of the query can be cached for the specified user
  is_personal: bool,
  /// The results of the query
  results: Vec<InputInlineQueryResult>,
  /// Allowed time to cache the results of the query, in seconds
  cache_time: i64,
  /// Offset for the next inline query; pass an empty string if there are no more results
  next_offset: String,
  /// If non-empty, this text should be shown on the button that opens a private chat with the bot and sends a start message to the bot with the parameter switch_pm_parameter
  switch_pm_text: String,
  /// The parameter for the bot start message
  switch_pm_parameter: String,
  
}

impl RObject for AnswerInlineQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerInlineQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AnswerInlineQuery {}

impl AnswerInlineQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAnswerInlineQueryBuilder {
    let mut inner = AnswerInlineQuery::default();
    inner.td_name = "answerInlineQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAnswerInlineQueryBuilder { inner }
  }

  pub fn inline_query_id(&self) -> isize { self.inline_query_id }

  pub fn is_personal(&self) -> bool { self.is_personal }

  pub fn results(&self) -> &Vec<InputInlineQueryResult> { &self.results }

  pub fn cache_time(&self) -> i64 { self.cache_time }

  pub fn next_offset(&self) -> &String { &self.next_offset }

  pub fn switch_pm_text(&self) -> &String { &self.switch_pm_text }

  pub fn switch_pm_parameter(&self) -> &String { &self.switch_pm_parameter }

}

#[doc(hidden)]
pub struct RTDAnswerInlineQueryBuilder {
  inner: AnswerInlineQuery
}

impl RTDAnswerInlineQueryBuilder {
  pub fn build(&self) -> AnswerInlineQuery { self.inner.clone() }

   
  pub fn inline_query_id(&mut self, inline_query_id: isize) -> &mut Self {
    self.inner.inline_query_id = inline_query_id;
    self
  }

   
  pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
    self.inner.is_personal = is_personal;
    self
  }

   
  pub fn results(&mut self, results: Vec<InputInlineQueryResult>) -> &mut Self {
    self.inner.results = results;
    self
  }

   
  pub fn cache_time(&mut self, cache_time: i64) -> &mut Self {
    self.inner.cache_time = cache_time;
    self
  }

   
  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }

   
  pub fn switch_pm_text<T: AsRef<str>>(&mut self, switch_pm_text: T) -> &mut Self {
    self.inner.switch_pm_text = switch_pm_text.as_ref().to_string();
    self
  }

   
  pub fn switch_pm_parameter<T: AsRef<str>>(&mut self, switch_pm_parameter: T) -> &mut Self {
    self.inner.switch_pm_parameter = switch_pm_parameter.as_ref().to_string();
    self
  }

}

impl AsRef<AnswerInlineQuery> for AnswerInlineQuery {
  fn as_ref(&self) -> &AnswerInlineQuery { self }
}

impl AsRef<AnswerInlineQuery> for RTDAnswerInlineQueryBuilder {
  fn as_ref(&self) -> &AnswerInlineQuery { &self.inner }
}







/// Sets the result of a pre-checkout query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerPreCheckoutQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the pre-checkout query
  pre_checkout_query_id: isize,
  /// An error message, empty on success
  error_message: String,
  
}

impl RObject for AnswerPreCheckoutQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerPreCheckoutQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AnswerPreCheckoutQuery {}

impl AnswerPreCheckoutQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAnswerPreCheckoutQueryBuilder {
    let mut inner = AnswerPreCheckoutQuery::default();
    inner.td_name = "answerPreCheckoutQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAnswerPreCheckoutQueryBuilder { inner }
  }

  pub fn pre_checkout_query_id(&self) -> isize { self.pre_checkout_query_id }

  pub fn error_message(&self) -> &String { &self.error_message }

}

#[doc(hidden)]
pub struct RTDAnswerPreCheckoutQueryBuilder {
  inner: AnswerPreCheckoutQuery
}

impl RTDAnswerPreCheckoutQueryBuilder {
  pub fn build(&self) -> AnswerPreCheckoutQuery { self.inner.clone() }

   
  pub fn pre_checkout_query_id(&mut self, pre_checkout_query_id: isize) -> &mut Self {
    self.inner.pre_checkout_query_id = pre_checkout_query_id;
    self
  }

   
  pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
    self.inner.error_message = error_message.as_ref().to_string();
    self
  }

}

impl AsRef<AnswerPreCheckoutQuery> for AnswerPreCheckoutQuery {
  fn as_ref(&self) -> &AnswerPreCheckoutQuery { self }
}

impl AsRef<AnswerPreCheckoutQuery> for RTDAnswerPreCheckoutQueryBuilder {
  fn as_ref(&self) -> &AnswerPreCheckoutQuery { &self.inner }
}







/// Sets the result of a shipping query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerShippingQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the shipping query
  shipping_query_id: isize,
  /// Available shipping options
  shipping_options: Vec<ShippingOption>,
  /// An error message, empty on success
  error_message: String,
  
}

impl RObject for AnswerShippingQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerShippingQuery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for AnswerShippingQuery {}

impl AnswerShippingQuery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAnswerShippingQueryBuilder {
    let mut inner = AnswerShippingQuery::default();
    inner.td_name = "answerShippingQuery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAnswerShippingQueryBuilder { inner }
  }

  pub fn shipping_query_id(&self) -> isize { self.shipping_query_id }

  pub fn shipping_options(&self) -> &Vec<ShippingOption> { &self.shipping_options }

  pub fn error_message(&self) -> &String { &self.error_message }

}

#[doc(hidden)]
pub struct RTDAnswerShippingQueryBuilder {
  inner: AnswerShippingQuery
}

impl RTDAnswerShippingQueryBuilder {
  pub fn build(&self) -> AnswerShippingQuery { self.inner.clone() }

   
  pub fn shipping_query_id(&mut self, shipping_query_id: isize) -> &mut Self {
    self.inner.shipping_query_id = shipping_query_id;
    self
  }

   
  pub fn shipping_options(&mut self, shipping_options: Vec<ShippingOption>) -> &mut Self {
    self.inner.shipping_options = shipping_options;
    self
  }

   
  pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
    self.inner.error_message = error_message.as_ref().to_string();
    self
  }

}

impl AsRef<AnswerShippingQuery> for AnswerShippingQuery {
  fn as_ref(&self) -> &AnswerShippingQuery { self }
}

impl AsRef<AnswerShippingQuery> for RTDAnswerShippingQueryBuilder {
  fn as_ref(&self) -> &AnswerShippingQuery { &self.inner }
}







/// Adds a user to the blacklist
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  
}

impl RObject for BlockUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "blockUser" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for BlockUser {}

impl BlockUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBlockUserBuilder {
    let mut inner = BlockUser::default();
    inner.td_name = "blockUser".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBlockUserBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDBlockUserBuilder {
  inner: BlockUser
}

impl RTDBlockUserBuilder {
  pub fn build(&self) -> BlockUser { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<BlockUser> for BlockUser {
  fn as_ref(&self) -> &BlockUser { self }
}

impl AsRef<BlockUser> for RTDBlockUserBuilder {
  fn as_ref(&self) -> &BlockUser { &self.inner }
}







/// Checks whether the current session can be used to transfer a chat ownership to another user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnership {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for CanTransferOwnership {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "canTransferOwnership" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCanTransferOwnershipResult for CanTransferOwnership {}

impl RFunction for CanTransferOwnership {}

impl CanTransferOwnership {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCanTransferOwnershipBuilder {
    let mut inner = CanTransferOwnership::default();
    inner.td_name = "canTransferOwnership".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCanTransferOwnershipBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCanTransferOwnershipBuilder {
  inner: CanTransferOwnership
}

impl RTDCanTransferOwnershipBuilder {
  pub fn build(&self) -> CanTransferOwnership { self.inner.clone() }

}

impl AsRef<CanTransferOwnership> for CanTransferOwnership {
  fn as_ref(&self) -> &CanTransferOwnership { self }
}

impl AsRef<CanTransferOwnership> for RTDCanTransferOwnershipBuilder {
  fn as_ref(&self) -> &CanTransferOwnership { &self.inner }
}







/// Stops the downloading of a file. If a file has already been downloaded, does nothing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelDownloadFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a file to stop downloading
  file_id: i64,
  /// Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server
  only_if_pending: bool,
  
}

impl RObject for CancelDownloadFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "cancelDownloadFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CancelDownloadFile {}

impl CancelDownloadFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCancelDownloadFileBuilder {
    let mut inner = CancelDownloadFile::default();
    inner.td_name = "cancelDownloadFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCancelDownloadFileBuilder { inner }
  }

  pub fn file_id(&self) -> i64 { self.file_id }

  pub fn only_if_pending(&self) -> bool { self.only_if_pending }

}

#[doc(hidden)]
pub struct RTDCancelDownloadFileBuilder {
  inner: CancelDownloadFile
}

impl RTDCancelDownloadFileBuilder {
  pub fn build(&self) -> CancelDownloadFile { self.inner.clone() }

   
  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

   
  pub fn only_if_pending(&mut self, only_if_pending: bool) -> &mut Self {
    self.inner.only_if_pending = only_if_pending;
    self
  }

}

impl AsRef<CancelDownloadFile> for CancelDownloadFile {
  fn as_ref(&self) -> &CancelDownloadFile { self }
}

impl AsRef<CancelDownloadFile> for RTDCancelDownloadFileBuilder {
  fn as_ref(&self) -> &CancelDownloadFile { &self.inner }
}







/// Stops the uploading of a file. Supported only for files uploaded by using uploadFile. For other files the behavior is undefined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelUploadFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the file to stop uploading
  file_id: i64,
  
}

impl RObject for CancelUploadFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "cancelUploadFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CancelUploadFile {}

impl CancelUploadFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCancelUploadFileBuilder {
    let mut inner = CancelUploadFile::default();
    inner.td_name = "cancelUploadFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCancelUploadFileBuilder { inner }
  }

  pub fn file_id(&self) -> i64 { self.file_id }

}

#[doc(hidden)]
pub struct RTDCancelUploadFileBuilder {
  inner: CancelUploadFile
}

impl RTDCancelUploadFileBuilder {
  pub fn build(&self) -> CancelUploadFile { self.inner.clone() }

   
  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

}

impl AsRef<CancelUploadFile> for CancelUploadFile {
  fn as_ref(&self) -> &CancelUploadFile { self }
}

impl AsRef<CancelUploadFile> for RTDCancelUploadFileBuilder {
  fn as_ref(&self) -> &CancelUploadFile { &self.inner }
}







/// Changes imported contacts using the list of current user contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeImportedContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new list of contacts, contact's vCard are ignored and are not imported
  contacts: Vec<Contact>,
  
}

impl RObject for ChangeImportedContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "changeImportedContacts" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ChangeImportedContacts {}

impl ChangeImportedContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChangeImportedContactsBuilder {
    let mut inner = ChangeImportedContacts::default();
    inner.td_name = "changeImportedContacts".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChangeImportedContactsBuilder { inner }
  }

  pub fn contacts(&self) -> &Vec<Contact> { &self.contacts }

}

#[doc(hidden)]
pub struct RTDChangeImportedContactsBuilder {
  inner: ChangeImportedContacts
}

impl RTDChangeImportedContactsBuilder {
  pub fn build(&self) -> ChangeImportedContacts { self.inner.clone() }

   
  pub fn contacts(&mut self, contacts: Vec<Contact>) -> &mut Self {
    self.inner.contacts = contacts;
    self
  }

}

impl AsRef<ChangeImportedContacts> for ChangeImportedContacts {
  fn as_ref(&self) -> &ChangeImportedContacts { self }
}

impl AsRef<ChangeImportedContacts> for RTDChangeImportedContactsBuilder {
  fn as_ref(&self) -> &ChangeImportedContacts { &self.inner }
}







/// Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new phone number of the user in international format
  phone_number: String,
  /// Settings for the authentication of the user's phone number
  settings: PhoneNumberAuthenticationSettings,
  
}

impl RObject for ChangePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "changePhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ChangePhoneNumber {}

impl ChangePhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChangePhoneNumberBuilder {
    let mut inner = ChangePhoneNumber::default();
    inner.td_name = "changePhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChangePhoneNumberBuilder { inner }
  }

  pub fn phone_number(&self) -> &String { &self.phone_number }

  pub fn settings(&self) -> &PhoneNumberAuthenticationSettings { &self.settings }

}

#[doc(hidden)]
pub struct RTDChangePhoneNumberBuilder {
  inner: ChangePhoneNumber
}

impl RTDChangePhoneNumberBuilder {
  pub fn build(&self) -> ChangePhoneNumber { self.inner.clone() }

   
  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

   
  pub fn settings<T: AsRef<PhoneNumberAuthenticationSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = settings.as_ref().clone();
    self
  }

}

impl AsRef<ChangePhoneNumber> for ChangePhoneNumber {
  fn as_ref(&self) -> &ChangePhoneNumber { self }
}

impl AsRef<ChangePhoneNumber> for RTDChangePhoneNumberBuilder {
  fn as_ref(&self) -> &ChangePhoneNumber { &self.inner }
}







/// Installs/uninstalls or activates/archives a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the sticker set
  set_id: isize,
  /// The new value of is_installed
  is_installed: bool,
  /// The new value of is_archived. A sticker set can't be installed and archived simultaneously
  is_archived: bool,
  
}

impl RObject for ChangeStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "changeStickerSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ChangeStickerSet {}

impl ChangeStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChangeStickerSetBuilder {
    let mut inner = ChangeStickerSet::default();
    inner.td_name = "changeStickerSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChangeStickerSetBuilder { inner }
  }

  pub fn set_id(&self) -> isize { self.set_id }

  pub fn is_installed(&self) -> bool { self.is_installed }

  pub fn is_archived(&self) -> bool { self.is_archived }

}

#[doc(hidden)]
pub struct RTDChangeStickerSetBuilder {
  inner: ChangeStickerSet
}

impl RTDChangeStickerSetBuilder {
  pub fn build(&self) -> ChangeStickerSet { self.inner.clone() }

   
  pub fn set_id(&mut self, set_id: isize) -> &mut Self {
    self.inner.set_id = set_id;
    self
  }

   
  pub fn is_installed(&mut self, is_installed: bool) -> &mut Self {
    self.inner.is_installed = is_installed;
    self
  }

   
  pub fn is_archived(&mut self, is_archived: bool) -> &mut Self {
    self.inner.is_archived = is_archived;
    self
  }

}

impl AsRef<ChangeStickerSet> for ChangeStickerSet {
  fn as_ref(&self) -> &ChangeStickerSet { self }
}

impl AsRef<ChangeStickerSet> for RTDChangeStickerSetBuilder {
  fn as_ref(&self) -> &ChangeStickerSet { &self.inner }
}







/// Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckAuthenticationBotToken {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The bot token
  token: String,
  
}

impl RObject for CheckAuthenticationBotToken {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkAuthenticationBotToken" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckAuthenticationBotToken {}

impl CheckAuthenticationBotToken {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckAuthenticationBotTokenBuilder {
    let mut inner = CheckAuthenticationBotToken::default();
    inner.td_name = "checkAuthenticationBotToken".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckAuthenticationBotTokenBuilder { inner }
  }

  pub fn token(&self) -> &String { &self.token }

}

#[doc(hidden)]
pub struct RTDCheckAuthenticationBotTokenBuilder {
  inner: CheckAuthenticationBotToken
}

impl RTDCheckAuthenticationBotTokenBuilder {
  pub fn build(&self) -> CheckAuthenticationBotToken { self.inner.clone() }

   
  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

}

impl AsRef<CheckAuthenticationBotToken> for CheckAuthenticationBotToken {
  fn as_ref(&self) -> &CheckAuthenticationBotToken { self }
}

impl AsRef<CheckAuthenticationBotToken> for RTDCheckAuthenticationBotTokenBuilder {
  fn as_ref(&self) -> &CheckAuthenticationBotToken { &self.inner }
}







/// Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckAuthenticationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The verification code received via SMS, Telegram message, phone call, or flash call
  code: String,
  
}

impl RObject for CheckAuthenticationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkAuthenticationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckAuthenticationCode {}

impl CheckAuthenticationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckAuthenticationCodeBuilder {
    let mut inner = CheckAuthenticationCode::default();
    inner.td_name = "checkAuthenticationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckAuthenticationCodeBuilder { inner }
  }

  pub fn code(&self) -> &String { &self.code }

}

#[doc(hidden)]
pub struct RTDCheckAuthenticationCodeBuilder {
  inner: CheckAuthenticationCode
}

impl RTDCheckAuthenticationCodeBuilder {
  pub fn build(&self) -> CheckAuthenticationCode { self.inner.clone() }

   
  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }

}

impl AsRef<CheckAuthenticationCode> for CheckAuthenticationCode {
  fn as_ref(&self) -> &CheckAuthenticationCode { self }
}

impl AsRef<CheckAuthenticationCode> for RTDCheckAuthenticationCodeBuilder {
  fn as_ref(&self) -> &CheckAuthenticationCode { &self.inner }
}







/// Checks the authentication password for correctness. Works only when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckAuthenticationPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The password to check
  password: String,
  
}

impl RObject for CheckAuthenticationPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkAuthenticationPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckAuthenticationPassword {}

impl CheckAuthenticationPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckAuthenticationPasswordBuilder {
    let mut inner = CheckAuthenticationPassword::default();
    inner.td_name = "checkAuthenticationPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckAuthenticationPasswordBuilder { inner }
  }

  pub fn password(&self) -> &String { &self.password }

}

#[doc(hidden)]
pub struct RTDCheckAuthenticationPasswordBuilder {
  inner: CheckAuthenticationPassword
}

impl RTDCheckAuthenticationPasswordBuilder {
  pub fn build(&self) -> CheckAuthenticationPassword { self.inner.clone() }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

}

impl AsRef<CheckAuthenticationPassword> for CheckAuthenticationPassword {
  fn as_ref(&self) -> &CheckAuthenticationPassword { self }
}

impl AsRef<CheckAuthenticationPassword> for RTDCheckAuthenticationPasswordBuilder {
  fn as_ref(&self) -> &CheckAuthenticationPassword { &self.inner }
}







/// Checks the authentication code sent to confirm a new phone number of the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChangePhoneNumberCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Verification code received by SMS, phone call or flash call
  code: String,
  
}

impl RObject for CheckChangePhoneNumberCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChangePhoneNumberCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckChangePhoneNumberCode {}

impl CheckChangePhoneNumberCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckChangePhoneNumberCodeBuilder {
    let mut inner = CheckChangePhoneNumberCode::default();
    inner.td_name = "checkChangePhoneNumberCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckChangePhoneNumberCodeBuilder { inner }
  }

  pub fn code(&self) -> &String { &self.code }

}

#[doc(hidden)]
pub struct RTDCheckChangePhoneNumberCodeBuilder {
  inner: CheckChangePhoneNumberCode
}

impl RTDCheckChangePhoneNumberCodeBuilder {
  pub fn build(&self) -> CheckChangePhoneNumberCode { self.inner.clone() }

   
  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }

}

impl AsRef<CheckChangePhoneNumberCode> for CheckChangePhoneNumberCode {
  fn as_ref(&self) -> &CheckChangePhoneNumberCode { self }
}

impl AsRef<CheckChangePhoneNumberCode> for RTDCheckChangePhoneNumberCodeBuilder {
  fn as_ref(&self) -> &CheckChangePhoneNumberCode { &self.inner }
}







/// Checks the validity of an invite link for a chat and returns information about the corresponding chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Invite link to be checked; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/"
  invite_link: String,
  
}

impl RObject for CheckChatInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatInviteLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckChatInviteLink {}

impl CheckChatInviteLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckChatInviteLinkBuilder {
    let mut inner = CheckChatInviteLink::default();
    inner.td_name = "checkChatInviteLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckChatInviteLinkBuilder { inner }
  }

  pub fn invite_link(&self) -> &String { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDCheckChatInviteLinkBuilder {
  inner: CheckChatInviteLink
}

impl RTDCheckChatInviteLinkBuilder {
  pub fn build(&self) -> CheckChatInviteLink { self.inner.clone() }

   
  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

}

impl AsRef<CheckChatInviteLink> for CheckChatInviteLink {
  fn as_ref(&self) -> &CheckChatInviteLink { self }
}

impl AsRef<CheckChatInviteLink> for RTDCheckChatInviteLinkBuilder {
  fn as_ref(&self) -> &CheckChatInviteLink { &self.inner }
}







/// Checks whether a username can be set for a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsername {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier; should be identifier of a supergroup chat, or a channel chat, or a private chat with self, or zero if chat is being created
  chat_id: i64,
  /// Username to be checked
  username: String,
  
}

impl RObject for CheckChatUsername {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChatUsername" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCheckChatUsernameResult for CheckChatUsername {}

impl RFunction for CheckChatUsername {}

impl CheckChatUsername {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckChatUsernameBuilder {
    let mut inner = CheckChatUsername::default();
    inner.td_name = "checkChatUsername".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckChatUsernameBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn username(&self) -> &String { &self.username }

}

#[doc(hidden)]
pub struct RTDCheckChatUsernameBuilder {
  inner: CheckChatUsername
}

impl RTDCheckChatUsernameBuilder {
  pub fn build(&self) -> CheckChatUsername { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

}

impl AsRef<CheckChatUsername> for CheckChatUsername {
  fn as_ref(&self) -> &CheckChatUsername { self }
}

impl AsRef<CheckChatUsername> for RTDCheckChatUsernameBuilder {
  fn as_ref(&self) -> &CheckChatUsername { &self.inner }
}







/// Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckCreatedPublicChatsLimit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the public chats, for which to check the limit
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: PublicChatType,
  
}

impl RObject for CheckCreatedPublicChatsLimit {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkCreatedPublicChatsLimit" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckCreatedPublicChatsLimit {}

impl CheckCreatedPublicChatsLimit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckCreatedPublicChatsLimitBuilder {
    let mut inner = CheckCreatedPublicChatsLimit::default();
    inner.td_name = "checkCreatedPublicChatsLimit".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckCreatedPublicChatsLimitBuilder { inner }
  }

  pub fn type_(&self) -> &PublicChatType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDCheckCreatedPublicChatsLimitBuilder {
  inner: CheckCreatedPublicChatsLimit
}

impl RTDCheckCreatedPublicChatsLimitBuilder {
  pub fn build(&self) -> CheckCreatedPublicChatsLimit { self.inner.clone() }

   
  pub fn type_<T: AsRef<PublicChatType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<CheckCreatedPublicChatsLimit> for CheckCreatedPublicChatsLimit {
  fn as_ref(&self) -> &CheckCreatedPublicChatsLimit { self }
}

impl AsRef<CheckCreatedPublicChatsLimit> for RTDCheckCreatedPublicChatsLimitBuilder {
  fn as_ref(&self) -> &CheckCreatedPublicChatsLimit { &self.inner }
}







/// Checks the database encryption key for correctness. Works only when the current authorization state is authorizationStateWaitEncryptionKey
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckDatabaseEncryptionKey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Encryption key to check or set up
  encryption_key: String,
  
}

impl RObject for CheckDatabaseEncryptionKey {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkDatabaseEncryptionKey" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckDatabaseEncryptionKey {}

impl CheckDatabaseEncryptionKey {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckDatabaseEncryptionKeyBuilder {
    let mut inner = CheckDatabaseEncryptionKey::default();
    inner.td_name = "checkDatabaseEncryptionKey".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckDatabaseEncryptionKeyBuilder { inner }
  }

  pub fn encryption_key(&self) -> &String { &self.encryption_key }

}

#[doc(hidden)]
pub struct RTDCheckDatabaseEncryptionKeyBuilder {
  inner: CheckDatabaseEncryptionKey
}

impl RTDCheckDatabaseEncryptionKeyBuilder {
  pub fn build(&self) -> CheckDatabaseEncryptionKey { self.inner.clone() }

   
  pub fn encryption_key<T: AsRef<str>>(&mut self, encryption_key: T) -> &mut Self {
    self.inner.encryption_key = encryption_key.as_ref().to_string();
    self
  }

}

impl AsRef<CheckDatabaseEncryptionKey> for CheckDatabaseEncryptionKey {
  fn as_ref(&self) -> &CheckDatabaseEncryptionKey { self }
}

impl AsRef<CheckDatabaseEncryptionKey> for RTDCheckDatabaseEncryptionKeyBuilder {
  fn as_ref(&self) -> &CheckDatabaseEncryptionKey { &self.inner }
}







/// Checks the email address verification code for Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckEmailAddressVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Verification code
  code: String,
  
}

impl RObject for CheckEmailAddressVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkEmailAddressVerificationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckEmailAddressVerificationCode {}

impl CheckEmailAddressVerificationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckEmailAddressVerificationCodeBuilder {
    let mut inner = CheckEmailAddressVerificationCode::default();
    inner.td_name = "checkEmailAddressVerificationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckEmailAddressVerificationCodeBuilder { inner }
  }

  pub fn code(&self) -> &String { &self.code }

}

#[doc(hidden)]
pub struct RTDCheckEmailAddressVerificationCodeBuilder {
  inner: CheckEmailAddressVerificationCode
}

impl RTDCheckEmailAddressVerificationCodeBuilder {
  pub fn build(&self) -> CheckEmailAddressVerificationCode { self.inner.clone() }

   
  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }

}

impl AsRef<CheckEmailAddressVerificationCode> for CheckEmailAddressVerificationCode {
  fn as_ref(&self) -> &CheckEmailAddressVerificationCode { self }
}

impl AsRef<CheckEmailAddressVerificationCode> for RTDCheckEmailAddressVerificationCodeBuilder {
  fn as_ref(&self) -> &CheckEmailAddressVerificationCode { &self.inner }
}







/// Checks phone number confirmation code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckPhoneNumberConfirmationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The phone number confirmation code
  code: String,
  
}

impl RObject for CheckPhoneNumberConfirmationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkPhoneNumberConfirmationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckPhoneNumberConfirmationCode {}

impl CheckPhoneNumberConfirmationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckPhoneNumberConfirmationCodeBuilder {
    let mut inner = CheckPhoneNumberConfirmationCode::default();
    inner.td_name = "checkPhoneNumberConfirmationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckPhoneNumberConfirmationCodeBuilder { inner }
  }

  pub fn code(&self) -> &String { &self.code }

}

#[doc(hidden)]
pub struct RTDCheckPhoneNumberConfirmationCodeBuilder {
  inner: CheckPhoneNumberConfirmationCode
}

impl RTDCheckPhoneNumberConfirmationCodeBuilder {
  pub fn build(&self) -> CheckPhoneNumberConfirmationCode { self.inner.clone() }

   
  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }

}

impl AsRef<CheckPhoneNumberConfirmationCode> for CheckPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &CheckPhoneNumberConfirmationCode { self }
}

impl AsRef<CheckPhoneNumberConfirmationCode> for RTDCheckPhoneNumberConfirmationCodeBuilder {
  fn as_ref(&self) -> &CheckPhoneNumberConfirmationCode { &self.inner }
}







/// Checks the phone number verification code for Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckPhoneNumberVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Verification code
  code: String,
  
}

impl RObject for CheckPhoneNumberVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkPhoneNumberVerificationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckPhoneNumberVerificationCode {}

impl CheckPhoneNumberVerificationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckPhoneNumberVerificationCodeBuilder {
    let mut inner = CheckPhoneNumberVerificationCode::default();
    inner.td_name = "checkPhoneNumberVerificationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckPhoneNumberVerificationCodeBuilder { inner }
  }

  pub fn code(&self) -> &String { &self.code }

}

#[doc(hidden)]
pub struct RTDCheckPhoneNumberVerificationCodeBuilder {
  inner: CheckPhoneNumberVerificationCode
}

impl RTDCheckPhoneNumberVerificationCodeBuilder {
  pub fn build(&self) -> CheckPhoneNumberVerificationCode { self.inner.clone() }

   
  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }

}

impl AsRef<CheckPhoneNumberVerificationCode> for CheckPhoneNumberVerificationCode {
  fn as_ref(&self) -> &CheckPhoneNumberVerificationCode { self }
}

impl AsRef<CheckPhoneNumberVerificationCode> for RTDCheckPhoneNumberVerificationCodeBuilder {
  fn as_ref(&self) -> &CheckPhoneNumberVerificationCode { &self.inner }
}







/// Checks the 2-step verification recovery email address verification code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckRecoveryEmailAddressCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Verification code
  code: String,
  
}

impl RObject for CheckRecoveryEmailAddressCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkRecoveryEmailAddressCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CheckRecoveryEmailAddressCode {}

impl CheckRecoveryEmailAddressCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCheckRecoveryEmailAddressCodeBuilder {
    let mut inner = CheckRecoveryEmailAddressCode::default();
    inner.td_name = "checkRecoveryEmailAddressCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCheckRecoveryEmailAddressCodeBuilder { inner }
  }

  pub fn code(&self) -> &String { &self.code }

}

#[doc(hidden)]
pub struct RTDCheckRecoveryEmailAddressCodeBuilder {
  inner: CheckRecoveryEmailAddressCode
}

impl RTDCheckRecoveryEmailAddressCodeBuilder {
  pub fn build(&self) -> CheckRecoveryEmailAddressCode { self.inner.clone() }

   
  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }

}

impl AsRef<CheckRecoveryEmailAddressCode> for CheckRecoveryEmailAddressCode {
  fn as_ref(&self) -> &CheckRecoveryEmailAddressCode { self }
}

impl AsRef<CheckRecoveryEmailAddressCode> for RTDCheckRecoveryEmailAddressCodeBuilder {
  fn as_ref(&self) -> &CheckRecoveryEmailAddressCode { &self.inner }
}







/// Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleanFileName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File name or path to the file
  file_name: String,
  
}

impl RObject for CleanFileName {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "cleanFileName" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CleanFileName {}

impl CleanFileName {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCleanFileNameBuilder {
    let mut inner = CleanFileName::default();
    inner.td_name = "cleanFileName".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCleanFileNameBuilder { inner }
  }

  pub fn file_name(&self) -> &String { &self.file_name }

}

#[doc(hidden)]
pub struct RTDCleanFileNameBuilder {
  inner: CleanFileName
}

impl RTDCleanFileNameBuilder {
  pub fn build(&self) -> CleanFileName { self.inner.clone() }

   
  pub fn file_name<T: AsRef<str>>(&mut self, file_name: T) -> &mut Self {
    self.inner.file_name = file_name.as_ref().to_string();
    self
  }

}

impl AsRef<CleanFileName> for CleanFileName {
  fn as_ref(&self) -> &CleanFileName { self }
}

impl AsRef<CleanFileName> for RTDCleanFileNameBuilder {
  fn as_ref(&self) -> &CleanFileName { &self.inner }
}







/// Clears draft messages in all chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearAllDraftMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// If true, local draft messages in secret chats will not be cleared
  exclude_secret_chats: bool,
  
}

impl RObject for ClearAllDraftMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "clearAllDraftMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ClearAllDraftMessages {}

impl ClearAllDraftMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDClearAllDraftMessagesBuilder {
    let mut inner = ClearAllDraftMessages::default();
    inner.td_name = "clearAllDraftMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDClearAllDraftMessagesBuilder { inner }
  }

  pub fn exclude_secret_chats(&self) -> bool { self.exclude_secret_chats }

}

#[doc(hidden)]
pub struct RTDClearAllDraftMessagesBuilder {
  inner: ClearAllDraftMessages
}

impl RTDClearAllDraftMessagesBuilder {
  pub fn build(&self) -> ClearAllDraftMessages { self.inner.clone() }

   
  pub fn exclude_secret_chats(&mut self, exclude_secret_chats: bool) -> &mut Self {
    self.inner.exclude_secret_chats = exclude_secret_chats;
    self
  }

}

impl AsRef<ClearAllDraftMessages> for ClearAllDraftMessages {
  fn as_ref(&self) -> &ClearAllDraftMessages { self }
}

impl AsRef<ClearAllDraftMessages> for RTDClearAllDraftMessagesBuilder {
  fn as_ref(&self) -> &ClearAllDraftMessages { &self.inner }
}







/// Clears all imported contacts, contact list remains unchanged
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearImportedContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ClearImportedContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "clearImportedContacts" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ClearImportedContacts {}

impl ClearImportedContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDClearImportedContactsBuilder {
    let mut inner = ClearImportedContacts::default();
    inner.td_name = "clearImportedContacts".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDClearImportedContactsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDClearImportedContactsBuilder {
  inner: ClearImportedContacts
}

impl RTDClearImportedContactsBuilder {
  pub fn build(&self) -> ClearImportedContacts { self.inner.clone() }

}

impl AsRef<ClearImportedContacts> for ClearImportedContacts {
  fn as_ref(&self) -> &ClearImportedContacts { self }
}

impl AsRef<ClearImportedContacts> for RTDClearImportedContactsBuilder {
  fn as_ref(&self) -> &ClearImportedContacts { &self.inner }
}







/// Clears the list of recently used stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearRecentStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers
  is_attached: bool,
  
}

impl RObject for ClearRecentStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "clearRecentStickers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ClearRecentStickers {}

impl ClearRecentStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDClearRecentStickersBuilder {
    let mut inner = ClearRecentStickers::default();
    inner.td_name = "clearRecentStickers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDClearRecentStickersBuilder { inner }
  }

  pub fn is_attached(&self) -> bool { self.is_attached }

}

#[doc(hidden)]
pub struct RTDClearRecentStickersBuilder {
  inner: ClearRecentStickers
}

impl RTDClearRecentStickersBuilder {
  pub fn build(&self) -> ClearRecentStickers { self.inner.clone() }

   
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.is_attached = is_attached;
    self
  }

}

impl AsRef<ClearRecentStickers> for ClearRecentStickers {
  fn as_ref(&self) -> &ClearRecentStickers { self }
}

impl AsRef<ClearRecentStickers> for RTDClearRecentStickersBuilder {
  fn as_ref(&self) -> &ClearRecentStickers { &self.inner }
}







/// Clears the list of recently found chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearRecentlyFoundChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ClearRecentlyFoundChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "clearRecentlyFoundChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ClearRecentlyFoundChats {}

impl ClearRecentlyFoundChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDClearRecentlyFoundChatsBuilder {
    let mut inner = ClearRecentlyFoundChats::default();
    inner.td_name = "clearRecentlyFoundChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDClearRecentlyFoundChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDClearRecentlyFoundChatsBuilder {
  inner: ClearRecentlyFoundChats
}

impl RTDClearRecentlyFoundChatsBuilder {
  pub fn build(&self) -> ClearRecentlyFoundChats { self.inner.clone() }

}

impl AsRef<ClearRecentlyFoundChats> for ClearRecentlyFoundChats {
  fn as_ref(&self) -> &ClearRecentlyFoundChats { self }
}

impl AsRef<ClearRecentlyFoundChats> for RTDClearRecentlyFoundChatsBuilder {
  fn as_ref(&self) -> &ClearRecentlyFoundChats { &self.inner }
}







/// Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, updateAuthorizationState with authorizationStateClosed will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Close {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for Close {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "close" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for Close {}

impl Close {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCloseBuilder {
    let mut inner = Close::default();
    inner.td_name = "close".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCloseBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCloseBuilder {
  inner: Close
}

impl RTDCloseBuilder {
  pub fn build(&self) -> Close { self.inner.clone() }

}

impl AsRef<Close> for Close {
  fn as_ref(&self) -> &Close { self }
}

impl AsRef<Close> for RTDCloseBuilder {
  fn as_ref(&self) -> &Close { &self.inner }
}







/// Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloseChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for CloseChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "closeChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CloseChat {}

impl CloseChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCloseChatBuilder {
    let mut inner = CloseChat::default();
    inner.td_name = "closeChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCloseChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDCloseChatBuilder {
  inner: CloseChat
}

impl RTDCloseChatBuilder {
  pub fn build(&self) -> CloseChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<CloseChat> for CloseChat {
  fn as_ref(&self) -> &CloseChat { self }
}

impl AsRef<CloseChat> for RTDCloseChatBuilder {
  fn as_ref(&self) -> &CloseChat { &self.inner }
}







/// Closes a secret chat, effectively transferring its state to secretChatStateClosed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloseSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Secret chat identifier
  secret_chat_id: i64,
  
}

impl RObject for CloseSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "closeSecretChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CloseSecretChat {}

impl CloseSecretChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCloseSecretChatBuilder {
    let mut inner = CloseSecretChat::default();
    inner.td_name = "closeSecretChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCloseSecretChatBuilder { inner }
  }

  pub fn secret_chat_id(&self) -> i64 { self.secret_chat_id }

}

#[doc(hidden)]
pub struct RTDCloseSecretChatBuilder {
  inner: CloseSecretChat
}

impl RTDCloseSecretChatBuilder {
  pub fn build(&self) -> CloseSecretChat { self.inner.clone() }

   
  pub fn secret_chat_id(&mut self, secret_chat_id: i64) -> &mut Self {
    self.inner.secret_chat_id = secret_chat_id;
    self
  }

}

impl AsRef<CloseSecretChat> for CloseSecretChat {
  fn as_ref(&self) -> &CloseSecretChat { self }
}

impl AsRef<CloseSecretChat> for RTDCloseSecretChatBuilder {
  fn as_ref(&self) -> &CloseSecretChat { &self.inner }
}







/// Confirms QR code authentication on another device. Returns created session on success
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfirmQrCodeAuthentication {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A link from a QR code. The link must be scanned by the in-app camera
  link: String,
  
}

impl RObject for ConfirmQrCodeAuthentication {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "confirmQrCodeAuthentication" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ConfirmQrCodeAuthentication {}

impl ConfirmQrCodeAuthentication {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConfirmQrCodeAuthenticationBuilder {
    let mut inner = ConfirmQrCodeAuthentication::default();
    inner.td_name = "confirmQrCodeAuthentication".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDConfirmQrCodeAuthenticationBuilder { inner }
  }

  pub fn link(&self) -> &String { &self.link }

}

#[doc(hidden)]
pub struct RTDConfirmQrCodeAuthenticationBuilder {
  inner: ConfirmQrCodeAuthentication
}

impl RTDConfirmQrCodeAuthenticationBuilder {
  pub fn build(&self) -> ConfirmQrCodeAuthentication { self.inner.clone() }

   
  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }

}

impl AsRef<ConfirmQrCodeAuthentication> for ConfirmQrCodeAuthentication {
  fn as_ref(&self) -> &ConfirmQrCodeAuthentication { self }
}

impl AsRef<ConfirmQrCodeAuthentication> for RTDConfirmQrCodeAuthenticationBuilder {
  fn as_ref(&self) -> &ConfirmQrCodeAuthentication { &self.inner }
}







/// Returns an existing chat corresponding to a known basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBasicGroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Basic group identifier
  basic_group_id: i64,
  /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect
  force: bool,
  
}

impl RObject for CreateBasicGroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createBasicGroupChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateBasicGroupChat {}

impl CreateBasicGroupChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateBasicGroupChatBuilder {
    let mut inner = CreateBasicGroupChat::default();
    inner.td_name = "createBasicGroupChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateBasicGroupChatBuilder { inner }
  }

  pub fn basic_group_id(&self) -> i64 { self.basic_group_id }

  pub fn force(&self) -> bool { self.force }

}

#[doc(hidden)]
pub struct RTDCreateBasicGroupChatBuilder {
  inner: CreateBasicGroupChat
}

impl RTDCreateBasicGroupChatBuilder {
  pub fn build(&self) -> CreateBasicGroupChat { self.inner.clone() }

   
  pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
    self.inner.basic_group_id = basic_group_id;
    self
  }

   
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.force = force;
    self
  }

}

impl AsRef<CreateBasicGroupChat> for CreateBasicGroupChat {
  fn as_ref(&self) -> &CreateBasicGroupChat { self }
}

impl AsRef<CreateBasicGroupChat> for RTDCreateBasicGroupChatBuilder {
  fn as_ref(&self) -> &CreateBasicGroupChat { &self.inner }
}







/// Creates a new call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user to be called
  user_id: i64,
  /// Description of the call protocols supported by the client
  protocol: CallProtocol,
  
}

impl RObject for CreateCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createCall" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateCall {}

impl CreateCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateCallBuilder {
    let mut inner = CreateCall::default();
    inner.td_name = "createCall".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateCallBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn protocol(&self) -> &CallProtocol { &self.protocol }

}

#[doc(hidden)]
pub struct RTDCreateCallBuilder {
  inner: CreateCall
}

impl RTDCreateCallBuilder {
  pub fn build(&self) -> CreateCall { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn protocol<T: AsRef<CallProtocol>>(&mut self, protocol: T) -> &mut Self {
    self.inner.protocol = protocol.as_ref().clone();
    self
  }

}

impl AsRef<CreateCall> for CreateCall {
  fn as_ref(&self) -> &CreateCall { self }
}

impl AsRef<CreateCall> for RTDCreateCallBuilder {
  fn as_ref(&self) -> &CreateCall { &self.inner }
}







/// Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns the newly created chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateNewBasicGroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifiers of users to be added to the basic group
  user_ids: Vec<i64>,
  /// Title of the new basic group; 1-128 characters
  title: String,
  
}

impl RObject for CreateNewBasicGroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createNewBasicGroupChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateNewBasicGroupChat {}

impl CreateNewBasicGroupChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateNewBasicGroupChatBuilder {
    let mut inner = CreateNewBasicGroupChat::default();
    inner.td_name = "createNewBasicGroupChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateNewBasicGroupChatBuilder { inner }
  }

  pub fn user_ids(&self) -> &Vec<i64> { &self.user_ids }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDCreateNewBasicGroupChatBuilder {
  inner: CreateNewBasicGroupChat
}

impl RTDCreateNewBasicGroupChatBuilder {
  pub fn build(&self) -> CreateNewBasicGroupChat { self.inner.clone() }

   
  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<CreateNewBasicGroupChat> for CreateNewBasicGroupChat {
  fn as_ref(&self) -> &CreateNewBasicGroupChat { self }
}

impl AsRef<CreateNewBasicGroupChat> for RTDCreateNewBasicGroupChatBuilder {
  fn as_ref(&self) -> &CreateNewBasicGroupChat { &self.inner }
}







/// Creates a new secret chat. Returns the newly created chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateNewSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the target user
  user_id: i64,
  
}

impl RObject for CreateNewSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createNewSecretChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateNewSecretChat {}

impl CreateNewSecretChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateNewSecretChatBuilder {
    let mut inner = CreateNewSecretChat::default();
    inner.td_name = "createNewSecretChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateNewSecretChatBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDCreateNewSecretChatBuilder {
  inner: CreateNewSecretChat
}

impl RTDCreateNewSecretChatBuilder {
  pub fn build(&self) -> CreateNewSecretChat { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<CreateNewSecretChat> for CreateNewSecretChat {
  fn as_ref(&self) -> &CreateNewSecretChat { self }
}

impl AsRef<CreateNewSecretChat> for RTDCreateNewSecretChatBuilder {
  fn as_ref(&self) -> &CreateNewSecretChat { &self.inner }
}







/// Creates a new sticker set; for bots only. Returns the newly created sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateNewStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker set owner
  user_id: i64,
  /// Sticker set title; 1-64 characters
  title: String,
  /// Sticker set name. Can contain only English letters, digits and underscores. Must end with *"_by_<bot username>"* (*<bot_username>* is case insensitive); 1-64 characters
  name: String,
  /// True, if stickers are masks
  is_masks: bool,
  /// List of stickers to be added to the set
  stickers: Vec<InputSticker>,
  
}

impl RObject for CreateNewStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createNewStickerSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateNewStickerSet {}

impl CreateNewStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateNewStickerSetBuilder {
    let mut inner = CreateNewStickerSet::default();
    inner.td_name = "createNewStickerSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateNewStickerSetBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn title(&self) -> &String { &self.title }

  pub fn name(&self) -> &String { &self.name }

  pub fn is_masks(&self) -> bool { self.is_masks }

  pub fn stickers(&self) -> &Vec<InputSticker> { &self.stickers }

}

#[doc(hidden)]
pub struct RTDCreateNewStickerSetBuilder {
  inner: CreateNewStickerSet
}

impl RTDCreateNewStickerSetBuilder {
  pub fn build(&self) -> CreateNewStickerSet { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.is_masks = is_masks;
    self
  }

   
  pub fn stickers(&mut self, stickers: Vec<InputSticker>) -> &mut Self {
    self.inner.stickers = stickers;
    self
  }

}

impl AsRef<CreateNewStickerSet> for CreateNewStickerSet {
  fn as_ref(&self) -> &CreateNewStickerSet { self }
}

impl AsRef<CreateNewStickerSet> for RTDCreateNewStickerSetBuilder {
  fn as_ref(&self) -> &CreateNewStickerSet { &self.inner }
}







/// Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateNewSupergroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title of the new chat; 1-128 characters
  title: String,
  /// True, if a channel chat should be created
  is_channel: bool,
  /// Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
  description: String,
  /// Chat location if a location-based supergroup is being created
  location: ChatLocation,
  
}

impl RObject for CreateNewSupergroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createNewSupergroupChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateNewSupergroupChat {}

impl CreateNewSupergroupChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateNewSupergroupChatBuilder {
    let mut inner = CreateNewSupergroupChat::default();
    inner.td_name = "createNewSupergroupChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateNewSupergroupChatBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

  pub fn is_channel(&self) -> bool { self.is_channel }

  pub fn description(&self) -> &String { &self.description }

  pub fn location(&self) -> &ChatLocation { &self.location }

}

#[doc(hidden)]
pub struct RTDCreateNewSupergroupChatBuilder {
  inner: CreateNewSupergroupChat
}

impl RTDCreateNewSupergroupChatBuilder {
  pub fn build(&self) -> CreateNewSupergroupChat { self.inner.clone() }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
    self.inner.is_channel = is_channel;
    self
  }

   
  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

   
  pub fn location<T: AsRef<ChatLocation>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

}

impl AsRef<CreateNewSupergroupChat> for CreateNewSupergroupChat {
  fn as_ref(&self) -> &CreateNewSupergroupChat { self }
}

impl AsRef<CreateNewSupergroupChat> for RTDCreateNewSupergroupChatBuilder {
  fn as_ref(&self) -> &CreateNewSupergroupChat { &self.inner }
}







/// Returns an existing chat corresponding to a given user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrivateChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect
  force: bool,
  
}

impl RObject for CreatePrivateChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createPrivateChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreatePrivateChat {}

impl CreatePrivateChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreatePrivateChatBuilder {
    let mut inner = CreatePrivateChat::default();
    inner.td_name = "createPrivateChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreatePrivateChatBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn force(&self) -> bool { self.force }

}

#[doc(hidden)]
pub struct RTDCreatePrivateChatBuilder {
  inner: CreatePrivateChat
}

impl RTDCreatePrivateChatBuilder {
  pub fn build(&self) -> CreatePrivateChat { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.force = force;
    self
  }

}

impl AsRef<CreatePrivateChat> for CreatePrivateChat {
  fn as_ref(&self) -> &CreatePrivateChat { self }
}

impl AsRef<CreatePrivateChat> for RTDCreatePrivateChatBuilder {
  fn as_ref(&self) -> &CreatePrivateChat { &self.inner }
}







/// Returns an existing chat corresponding to a known secret chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Secret chat identifier
  secret_chat_id: i64,
  
}

impl RObject for CreateSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createSecretChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateSecretChat {}

impl CreateSecretChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateSecretChatBuilder {
    let mut inner = CreateSecretChat::default();
    inner.td_name = "createSecretChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateSecretChatBuilder { inner }
  }

  pub fn secret_chat_id(&self) -> i64 { self.secret_chat_id }

}

#[doc(hidden)]
pub struct RTDCreateSecretChatBuilder {
  inner: CreateSecretChat
}

impl RTDCreateSecretChatBuilder {
  pub fn build(&self) -> CreateSecretChat { self.inner.clone() }

   
  pub fn secret_chat_id(&mut self, secret_chat_id: i64) -> &mut Self {
    self.inner.secret_chat_id = secret_chat_id;
    self
  }

}

impl AsRef<CreateSecretChat> for CreateSecretChat {
  fn as_ref(&self) -> &CreateSecretChat { self }
}

impl AsRef<CreateSecretChat> for RTDCreateSecretChatBuilder {
  fn as_ref(&self) -> &CreateSecretChat { &self.inner }
}







/// Returns an existing chat corresponding to a known supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSupergroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup or channel identifier
  supergroup_id: i64,
  /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect
  force: bool,
  
}

impl RObject for CreateSupergroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createSupergroupChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateSupergroupChat {}

impl CreateSupergroupChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateSupergroupChatBuilder {
    let mut inner = CreateSupergroupChat::default();
    inner.td_name = "createSupergroupChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateSupergroupChatBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn force(&self) -> bool { self.force }

}

#[doc(hidden)]
pub struct RTDCreateSupergroupChatBuilder {
  inner: CreateSupergroupChat
}

impl RTDCreateSupergroupChatBuilder {
  pub fn build(&self) -> CreateSupergroupChat { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.force = force;
    self
  }

}

impl AsRef<CreateSupergroupChat> for CreateSupergroupChat {
  fn as_ref(&self) -> &CreateSupergroupChat { self }
}

impl AsRef<CreateSupergroupChat> for RTDCreateSupergroupChatBuilder {
  fn as_ref(&self) -> &CreateSupergroupChat { &self.inner }
}







/// Creates a new temporary password for processing payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTemporaryPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Persistent user password
  password: String,
  /// Time during which the temporary password will be valid, in seconds; should be between 60 and 86400
  valid_for: i64,
  
}

impl RObject for CreateTemporaryPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createTemporaryPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for CreateTemporaryPassword {}

impl CreateTemporaryPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCreateTemporaryPasswordBuilder {
    let mut inner = CreateTemporaryPassword::default();
    inner.td_name = "createTemporaryPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCreateTemporaryPasswordBuilder { inner }
  }

  pub fn password(&self) -> &String { &self.password }

  pub fn valid_for(&self) -> i64 { self.valid_for }

}

#[doc(hidden)]
pub struct RTDCreateTemporaryPasswordBuilder {
  inner: CreateTemporaryPassword
}

impl RTDCreateTemporaryPasswordBuilder {
  pub fn build(&self) -> CreateTemporaryPassword { self.inner.clone() }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

   
  pub fn valid_for(&mut self, valid_for: i64) -> &mut Self {
    self.inner.valid_for = valid_for;
    self
  }

}

impl AsRef<CreateTemporaryPassword> for CreateTemporaryPassword {
  fn as_ref(&self) -> &CreateTemporaryPassword { self }
}

impl AsRef<CreateTemporaryPassword> for RTDCreateTemporaryPasswordBuilder {
  fn as_ref(&self) -> &CreateTemporaryPassword { &self.inner }
}







/// Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteAccount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The reason why the account was deleted; optional
  reason: String,
  
}

impl RObject for DeleteAccount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteAccount" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteAccount {}

impl DeleteAccount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteAccountBuilder {
    let mut inner = DeleteAccount::default();
    inner.td_name = "deleteAccount".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteAccountBuilder { inner }
  }

  pub fn reason(&self) -> &String { &self.reason }

}

#[doc(hidden)]
pub struct RTDDeleteAccountBuilder {
  inner: DeleteAccount
}

impl RTDDeleteAccountBuilder {
  pub fn build(&self) -> DeleteAccount { self.inner.clone() }

   
  pub fn reason<T: AsRef<str>>(&mut self, reason: T) -> &mut Self {
    self.inner.reason = reason.as_ref().to_string();
    self
  }

}

impl AsRef<DeleteAccount> for DeleteAccount {
  fn as_ref(&self) -> &DeleteAccount { self }
}

impl AsRef<DeleteAccount> for RTDDeleteAccountBuilder {
  fn as_ref(&self) -> &DeleteAccount { &self.inner }
}







/// Deletes all messages in the chat. Use Chat.can_be_deleted_only_for_self and Chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Pass true if the chat should be removed from the chat list
  remove_from_chat_list: bool,
  /// Pass true to try to delete chat history for all users
  revoke: bool,
  
}

impl RObject for DeleteChatHistory {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteChatHistory" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteChatHistory {}

impl DeleteChatHistory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteChatHistoryBuilder {
    let mut inner = DeleteChatHistory::default();
    inner.td_name = "deleteChatHistory".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteChatHistoryBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn remove_from_chat_list(&self) -> bool { self.remove_from_chat_list }

  pub fn revoke(&self) -> bool { self.revoke }

}

#[doc(hidden)]
pub struct RTDDeleteChatHistoryBuilder {
  inner: DeleteChatHistory
}

impl RTDDeleteChatHistoryBuilder {
  pub fn build(&self) -> DeleteChatHistory { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn remove_from_chat_list(&mut self, remove_from_chat_list: bool) -> &mut Self {
    self.inner.remove_from_chat_list = remove_from_chat_list;
    self
  }

   
  pub fn revoke(&mut self, revoke: bool) -> &mut Self {
    self.inner.revoke = revoke;
    self
  }

}

impl AsRef<DeleteChatHistory> for DeleteChatHistory {
  fn as_ref(&self) -> &DeleteChatHistory { self }
}

impl AsRef<DeleteChatHistory> for RTDDeleteChatHistoryBuilder {
  fn as_ref(&self) -> &DeleteChatHistory { &self.inner }
}







/// Deletes all messages sent by the specified user to a chat. Supported only for supergroups; requires can_delete_messages administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatMessagesFromUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// User identifier
  user_id: i64,
  
}

impl RObject for DeleteChatMessagesFromUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteChatMessagesFromUser" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteChatMessagesFromUser {}

impl DeleteChatMessagesFromUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteChatMessagesFromUserBuilder {
    let mut inner = DeleteChatMessagesFromUser::default();
    inner.td_name = "deleteChatMessagesFromUser".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteChatMessagesFromUserBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDDeleteChatMessagesFromUserBuilder {
  inner: DeleteChatMessagesFromUser
}

impl RTDDeleteChatMessagesFromUserBuilder {
  pub fn build(&self) -> DeleteChatMessagesFromUser { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<DeleteChatMessagesFromUser> for DeleteChatMessagesFromUser {
  fn as_ref(&self) -> &DeleteChatMessagesFromUser { self }
}

impl AsRef<DeleteChatMessagesFromUser> for RTDDeleteChatMessagesFromUserBuilder {
  fn as_ref(&self) -> &DeleteChatMessagesFromUser { &self.inner }
}







/// Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup will be changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatReplyMarkup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The message identifier of the used keyboard
  message_id: i64,
  
}

impl RObject for DeleteChatReplyMarkup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteChatReplyMarkup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteChatReplyMarkup {}

impl DeleteChatReplyMarkup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteChatReplyMarkupBuilder {
    let mut inner = DeleteChatReplyMarkup::default();
    inner.td_name = "deleteChatReplyMarkup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteChatReplyMarkupBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDDeleteChatReplyMarkupBuilder {
  inner: DeleteChatReplyMarkup
}

impl RTDDeleteChatReplyMarkupBuilder {
  pub fn build(&self) -> DeleteChatReplyMarkup { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<DeleteChatReplyMarkup> for DeleteChatReplyMarkup {
  fn as_ref(&self) -> &DeleteChatReplyMarkup { self }
}

impl AsRef<DeleteChatReplyMarkup> for RTDDeleteChatReplyMarkupBuilder {
  fn as_ref(&self) -> &DeleteChatReplyMarkup { &self.inner }
}







/// Deletes a file from the TDLib file cache
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the file to delete
  file_id: i64,
  
}

impl RObject for DeleteFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteFile {}

impl DeleteFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteFileBuilder {
    let mut inner = DeleteFile::default();
    inner.td_name = "deleteFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteFileBuilder { inner }
  }

  pub fn file_id(&self) -> i64 { self.file_id }

}

#[doc(hidden)]
pub struct RTDDeleteFileBuilder {
  inner: DeleteFile
}

impl RTDDeleteFileBuilder {
  pub fn build(&self) -> DeleteFile { self.inner.clone() }

   
  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

}

impl AsRef<DeleteFile> for DeleteFile {
  fn as_ref(&self) -> &DeleteFile { self }
}

impl AsRef<DeleteFile> for RTDDeleteFileBuilder {
  fn as_ref(&self) -> &DeleteFile { &self.inner }
}







/// Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the language pack to delete
  language_pack_id: String,
  
}

impl RObject for DeleteLanguagePack {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteLanguagePack" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteLanguagePack {}

impl DeleteLanguagePack {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteLanguagePackBuilder {
    let mut inner = DeleteLanguagePack::default();
    inner.td_name = "deleteLanguagePack".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteLanguagePackBuilder { inner }
  }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

}

#[doc(hidden)]
pub struct RTDDeleteLanguagePackBuilder {
  inner: DeleteLanguagePack
}

impl RTDDeleteLanguagePackBuilder {
  pub fn build(&self) -> DeleteLanguagePack { self.inner.clone() }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

}

impl AsRef<DeleteLanguagePack> for DeleteLanguagePack {
  fn as_ref(&self) -> &DeleteLanguagePack { self }
}

impl AsRef<DeleteLanguagePack> for RTDDeleteLanguagePackBuilder {
  fn as_ref(&self) -> &DeleteLanguagePack { &self.inner }
}







/// Deletes messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifiers of the messages to be deleted
  message_ids: Vec<i64>,
  /// Pass true to try to delete messages for all chat members. Always true for supergroups, channels and secret chats
  revoke: bool,
  
}

impl RObject for DeleteMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteMessages {}

impl DeleteMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteMessagesBuilder {
    let mut inner = DeleteMessages::default();
    inner.td_name = "deleteMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_ids(&self) -> &Vec<i64> { &self.message_ids }

  pub fn revoke(&self) -> bool { self.revoke }

}

#[doc(hidden)]
pub struct RTDDeleteMessagesBuilder {
  inner: DeleteMessages
}

impl RTDDeleteMessagesBuilder {
  pub fn build(&self) -> DeleteMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }

   
  pub fn revoke(&mut self, revoke: bool) -> &mut Self {
    self.inner.revoke = revoke;
    self
  }

}

impl AsRef<DeleteMessages> for DeleteMessages {
  fn as_ref(&self) -> &DeleteMessages { self }
}

impl AsRef<DeleteMessages> for RTDDeleteMessagesBuilder {
  fn as_ref(&self) -> &DeleteMessages { &self.inner }
}







/// Deletes a Telegram Passport element
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeletePassportElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Element type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: PassportElementType,
  
}

impl RObject for DeletePassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deletePassportElement" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeletePassportElement {}

impl DeletePassportElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeletePassportElementBuilder {
    let mut inner = DeletePassportElement::default();
    inner.td_name = "deletePassportElement".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeletePassportElementBuilder { inner }
  }

  pub fn type_(&self) -> &PassportElementType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDDeletePassportElementBuilder {
  inner: DeletePassportElement
}

impl RTDDeletePassportElementBuilder {
  pub fn build(&self) -> DeletePassportElement { self.inner.clone() }

   
  pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<DeletePassportElement> for DeletePassportElement {
  fn as_ref(&self) -> &DeletePassportElement { self }
}

impl AsRef<DeletePassportElement> for RTDDeletePassportElementBuilder {
  fn as_ref(&self) -> &DeletePassportElement { &self.inner }
}







/// Deletes a profile photo. If something changes, updateUser will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the profile photo to delete
  profile_photo_id: isize,
  
}

impl RObject for DeleteProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteProfilePhoto" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteProfilePhoto {}

impl DeleteProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteProfilePhotoBuilder {
    let mut inner = DeleteProfilePhoto::default();
    inner.td_name = "deleteProfilePhoto".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteProfilePhotoBuilder { inner }
  }

  pub fn profile_photo_id(&self) -> isize { self.profile_photo_id }

}

#[doc(hidden)]
pub struct RTDDeleteProfilePhotoBuilder {
  inner: DeleteProfilePhoto
}

impl RTDDeleteProfilePhotoBuilder {
  pub fn build(&self) -> DeleteProfilePhoto { self.inner.clone() }

   
  pub fn profile_photo_id(&mut self, profile_photo_id: isize) -> &mut Self {
    self.inner.profile_photo_id = profile_photo_id;
    self
  }

}

impl AsRef<DeleteProfilePhoto> for DeleteProfilePhoto {
  fn as_ref(&self) -> &DeleteProfilePhoto { self }
}

impl AsRef<DeleteProfilePhoto> for RTDDeleteProfilePhotoBuilder {
  fn as_ref(&self) -> &DeleteProfilePhoto { &self.inner }
}







/// Deletes saved credentials for all payment provider bots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSavedCredentials {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for DeleteSavedCredentials {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteSavedCredentials" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteSavedCredentials {}

impl DeleteSavedCredentials {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteSavedCredentialsBuilder {
    let mut inner = DeleteSavedCredentials::default();
    inner.td_name = "deleteSavedCredentials".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteSavedCredentialsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDDeleteSavedCredentialsBuilder {
  inner: DeleteSavedCredentials
}

impl RTDDeleteSavedCredentialsBuilder {
  pub fn build(&self) -> DeleteSavedCredentials { self.inner.clone() }

}

impl AsRef<DeleteSavedCredentials> for DeleteSavedCredentials {
  fn as_ref(&self) -> &DeleteSavedCredentials { self }
}

impl AsRef<DeleteSavedCredentials> for RTDDeleteSavedCredentialsBuilder {
  fn as_ref(&self) -> &DeleteSavedCredentials { &self.inner }
}







/// Deletes saved order info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSavedOrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for DeleteSavedOrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteSavedOrderInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteSavedOrderInfo {}

impl DeleteSavedOrderInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteSavedOrderInfoBuilder {
    let mut inner = DeleteSavedOrderInfo::default();
    inner.td_name = "deleteSavedOrderInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteSavedOrderInfoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDDeleteSavedOrderInfoBuilder {
  inner: DeleteSavedOrderInfo
}

impl RTDDeleteSavedOrderInfoBuilder {
  pub fn build(&self) -> DeleteSavedOrderInfo { self.inner.clone() }

}

impl AsRef<DeleteSavedOrderInfo> for DeleteSavedOrderInfo {
  fn as_ref(&self) -> &DeleteSavedOrderInfo { self }
}

impl AsRef<DeleteSavedOrderInfo> for RTDDeleteSavedOrderInfoBuilder {
  fn as_ref(&self) -> &DeleteSavedOrderInfo { &self.inner }
}







/// Deletes a supergroup or channel along with all messages in the corresponding chat. This will release the supergroup or channel username and remove all members; requires owner privileges in the supergroup or channel. Chats with more than 1000 members can't be deleted using this method
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the supergroup or channel
  supergroup_id: i64,
  
}

impl RObject for DeleteSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteSupergroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DeleteSupergroup {}

impl DeleteSupergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeleteSupergroupBuilder {
    let mut inner = DeleteSupergroup::default();
    inner.td_name = "deleteSupergroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDeleteSupergroupBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

}

#[doc(hidden)]
pub struct RTDDeleteSupergroupBuilder {
  inner: DeleteSupergroup
}

impl RTDDeleteSupergroupBuilder {
  pub fn build(&self) -> DeleteSupergroup { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

}

impl AsRef<DeleteSupergroup> for DeleteSupergroup {
  fn as_ref(&self) -> &DeleteSupergroup { self }
}

impl AsRef<DeleteSupergroup> for RTDDeleteSupergroupBuilder {
  fn as_ref(&self) -> &DeleteSupergroup { &self.inner }
}







/// Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Destroy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for Destroy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "destroy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for Destroy {}

impl Destroy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDestroyBuilder {
    let mut inner = Destroy::default();
    inner.td_name = "destroy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDestroyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDDestroyBuilder {
  inner: Destroy
}

impl RTDDestroyBuilder {
  pub fn build(&self) -> Destroy { self.inner.clone() }

}

impl AsRef<Destroy> for Destroy {
  fn as_ref(&self) -> &Destroy { self }
}

impl AsRef<Destroy> for RTDDestroyBuilder {
  fn as_ref(&self) -> &Destroy { &self.inner }
}







/// Disables the currently enabled proxy. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisableProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for DisableProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "disableProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DisableProxy {}

impl DisableProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDisableProxyBuilder {
    let mut inner = DisableProxy::default();
    inner.td_name = "disableProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDisableProxyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDDisableProxyBuilder {
  inner: DisableProxy
}

impl RTDDisableProxyBuilder {
  pub fn build(&self) -> DisableProxy { self.inner.clone() }

}

impl AsRef<DisableProxy> for DisableProxy {
  fn as_ref(&self) -> &DisableProxy { self }
}

impl AsRef<DisableProxy> for RTDDisableProxyBuilder {
  fn as_ref(&self) -> &DisableProxy { &self.inner }
}







/// Discards a call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscardCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Call identifier
  call_id: i64,
  /// True, if the user was disconnected
  is_disconnected: bool,
  /// The call duration, in seconds
  duration: i64,
  /// Identifier of the connection used during the call
  connection_id: isize,
  
}

impl RObject for DiscardCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "discardCall" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DiscardCall {}

impl DiscardCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDiscardCallBuilder {
    let mut inner = DiscardCall::default();
    inner.td_name = "discardCall".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDiscardCallBuilder { inner }
  }

  pub fn call_id(&self) -> i64 { self.call_id }

  pub fn is_disconnected(&self) -> bool { self.is_disconnected }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn connection_id(&self) -> isize { self.connection_id }

}

#[doc(hidden)]
pub struct RTDDiscardCallBuilder {
  inner: DiscardCall
}

impl RTDDiscardCallBuilder {
  pub fn build(&self) -> DiscardCall { self.inner.clone() }

   
  pub fn call_id(&mut self, call_id: i64) -> &mut Self {
    self.inner.call_id = call_id;
    self
  }

   
  pub fn is_disconnected(&mut self, is_disconnected: bool) -> &mut Self {
    self.inner.is_disconnected = is_disconnected;
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn connection_id(&mut self, connection_id: isize) -> &mut Self {
    self.inner.connection_id = connection_id;
    self
  }

}

impl AsRef<DiscardCall> for DiscardCall {
  fn as_ref(&self) -> &DiscardCall { self }
}

impl AsRef<DiscardCall> for RTDDiscardCallBuilder {
  fn as_ref(&self) -> &DiscardCall { &self.inner }
}







/// Disconnects all websites from the current user's Telegram account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisconnectAllWebsites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for DisconnectAllWebsites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "disconnectAllWebsites" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DisconnectAllWebsites {}

impl DisconnectAllWebsites {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDisconnectAllWebsitesBuilder {
    let mut inner = DisconnectAllWebsites::default();
    inner.td_name = "disconnectAllWebsites".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDisconnectAllWebsitesBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDDisconnectAllWebsitesBuilder {
  inner: DisconnectAllWebsites
}

impl RTDDisconnectAllWebsitesBuilder {
  pub fn build(&self) -> DisconnectAllWebsites { self.inner.clone() }

}

impl AsRef<DisconnectAllWebsites> for DisconnectAllWebsites {
  fn as_ref(&self) -> &DisconnectAllWebsites { self }
}

impl AsRef<DisconnectAllWebsites> for RTDDisconnectAllWebsitesBuilder {
  fn as_ref(&self) -> &DisconnectAllWebsites { &self.inner }
}







/// Disconnects website from the current user's Telegram account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisconnectWebsite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Website identifier
  website_id: isize,
  
}

impl RObject for DisconnectWebsite {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "disconnectWebsite" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DisconnectWebsite {}

impl DisconnectWebsite {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDisconnectWebsiteBuilder {
    let mut inner = DisconnectWebsite::default();
    inner.td_name = "disconnectWebsite".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDisconnectWebsiteBuilder { inner }
  }

  pub fn website_id(&self) -> isize { self.website_id }

}

#[doc(hidden)]
pub struct RTDDisconnectWebsiteBuilder {
  inner: DisconnectWebsite
}

impl RTDDisconnectWebsiteBuilder {
  pub fn build(&self) -> DisconnectWebsite { self.inner.clone() }

   
  pub fn website_id(&mut self, website_id: isize) -> &mut Self {
    self.inner.website_id = website_id;
    self
  }

}

impl AsRef<DisconnectWebsite> for DisconnectWebsite {
  fn as_ref(&self) -> &DisconnectWebsite { self }
}

impl AsRef<DisconnectWebsite> for RTDDisconnectWebsiteBuilder {
  fn as_ref(&self) -> &DisconnectWebsite { &self.inner }
}







/// Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the file to download
  file_id: i64,
  /// Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile was called will be downloaded first
  priority: i64,
  /// The starting position from which the file should be downloaded
  offset: i64,
  /// Number of bytes which should be downloaded starting from the "offset" position before the download will be automatically cancelled; use 0 to download without a limit
  limit: i64,
  /// If false, this request returns file state just after the download has been started. If true, this request returns file state only after the download has succeeded, has failed, has been cancelled or a new downloadFile request with different offset/limit parameters was sent
  synchronous: bool,
  
}

impl RObject for DownloadFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "downloadFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for DownloadFile {}

impl DownloadFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDownloadFileBuilder {
    let mut inner = DownloadFile::default();
    inner.td_name = "downloadFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDownloadFileBuilder { inner }
  }

  pub fn file_id(&self) -> i64 { self.file_id }

  pub fn priority(&self) -> i64 { self.priority }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn limit(&self) -> i64 { self.limit }

  pub fn synchronous(&self) -> bool { self.synchronous }

}

#[doc(hidden)]
pub struct RTDDownloadFileBuilder {
  inner: DownloadFile
}

impl RTDDownloadFileBuilder {
  pub fn build(&self) -> DownloadFile { self.inner.clone() }

   
  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

   
  pub fn priority(&mut self, priority: i64) -> &mut Self {
    self.inner.priority = priority;
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

   
  pub fn synchronous(&mut self, synchronous: bool) -> &mut Self {
    self.inner.synchronous = synchronous;
    self
  }

}

impl AsRef<DownloadFile> for DownloadFile {
  fn as_ref(&self) -> &DownloadFile { self }
}

impl AsRef<DownloadFile> for RTDDownloadFileBuilder {
  fn as_ref(&self) -> &DownloadFile { &self.inner }
}







/// Edits information about a custom local language pack in the current localization target. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditCustomLanguagePackInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New information about the custom local language pack
  info: LanguagePackInfo,
  
}

impl RObject for EditCustomLanguagePackInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editCustomLanguagePackInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditCustomLanguagePackInfo {}

impl EditCustomLanguagePackInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditCustomLanguagePackInfoBuilder {
    let mut inner = EditCustomLanguagePackInfo::default();
    inner.td_name = "editCustomLanguagePackInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditCustomLanguagePackInfoBuilder { inner }
  }

  pub fn info(&self) -> &LanguagePackInfo { &self.info }

}

#[doc(hidden)]
pub struct RTDEditCustomLanguagePackInfoBuilder {
  inner: EditCustomLanguagePackInfo
}

impl RTDEditCustomLanguagePackInfoBuilder {
  pub fn build(&self) -> EditCustomLanguagePackInfo { self.inner.clone() }

   
  pub fn info<T: AsRef<LanguagePackInfo>>(&mut self, info: T) -> &mut Self {
    self.inner.info = info.as_ref().clone();
    self
  }

}

impl AsRef<EditCustomLanguagePackInfo> for EditCustomLanguagePackInfo {
  fn as_ref(&self) -> &EditCustomLanguagePackInfo { self }
}

impl AsRef<EditCustomLanguagePackInfo> for RTDEditCustomLanguagePackInfoBuilder {
  fn as_ref(&self) -> &EditCustomLanguagePackInfo { &self.inner }
}







/// Edits the caption of an inline message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageCaption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Inline message identifier
  inline_message_id: String,
  /// The new message reply markup
  reply_markup: ReplyMarkup,
  /// New message content caption; 0-GetOption("message_caption_length_max") characters
  caption: FormattedText,
  
}

impl RObject for EditInlineMessageCaption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageCaption" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditInlineMessageCaption {}

impl EditInlineMessageCaption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditInlineMessageCaptionBuilder {
    let mut inner = EditInlineMessageCaption::default();
    inner.td_name = "editInlineMessageCaption".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditInlineMessageCaptionBuilder { inner }
  }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn caption(&self) -> &FormattedText { &self.caption }

}

#[doc(hidden)]
pub struct RTDEditInlineMessageCaptionBuilder {
  inner: EditInlineMessageCaption
}

impl RTDEditInlineMessageCaptionBuilder {
  pub fn build(&self) -> EditInlineMessageCaption { self.inner.clone() }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<EditInlineMessageCaption> for EditInlineMessageCaption {
  fn as_ref(&self) -> &EditInlineMessageCaption { self }
}

impl AsRef<EditInlineMessageCaption> for RTDEditInlineMessageCaptionBuilder {
  fn as_ref(&self) -> &EditInlineMessageCaption { &self.inner }
}







/// Edits the content of a live location in an inline message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageLiveLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Inline message identifier
  inline_message_id: String,
  /// The new message reply markup
  reply_markup: ReplyMarkup,
  /// New location content of the message; may be null. Pass null to stop sharing the live location
  location: Option<Location>,
  
}

impl RObject for EditInlineMessageLiveLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageLiveLocation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditInlineMessageLiveLocation {}

impl EditInlineMessageLiveLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditInlineMessageLiveLocationBuilder {
    let mut inner = EditInlineMessageLiveLocation::default();
    inner.td_name = "editInlineMessageLiveLocation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditInlineMessageLiveLocationBuilder { inner }
  }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn location(&self) -> &Option<Location> { &self.location }

}

#[doc(hidden)]
pub struct RTDEditInlineMessageLiveLocationBuilder {
  inner: EditInlineMessageLiveLocation
}

impl RTDEditInlineMessageLiveLocationBuilder {
  pub fn build(&self) -> EditInlineMessageLiveLocation { self.inner.clone() }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = Some(location.as_ref().clone());
    self
  }

}

impl AsRef<EditInlineMessageLiveLocation> for EditInlineMessageLiveLocation {
  fn as_ref(&self) -> &EditInlineMessageLiveLocation { self }
}

impl AsRef<EditInlineMessageLiveLocation> for RTDEditInlineMessageLiveLocationBuilder {
  fn as_ref(&self) -> &EditInlineMessageLiveLocation { &self.inner }
}







/// Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageMedia {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Inline message identifier
  inline_message_id: String,
  /// The new message reply markup; for bots only
  reply_markup: ReplyMarkup,
  /// New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo
  input_message_content: InputMessageContent,
  
}

impl RObject for EditInlineMessageMedia {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageMedia" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditInlineMessageMedia {}

impl EditInlineMessageMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditInlineMessageMediaBuilder {
    let mut inner = EditInlineMessageMedia::default();
    inner.td_name = "editInlineMessageMedia".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditInlineMessageMediaBuilder { inner }
  }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn input_message_content(&self) -> &InputMessageContent { &self.input_message_content }

}

#[doc(hidden)]
pub struct RTDEditInlineMessageMediaBuilder {
  inner: EditInlineMessageMedia
}

impl RTDEditInlineMessageMediaBuilder {
  pub fn build(&self) -> EditInlineMessageMedia { self.inner.clone() }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn input_message_content<T: AsRef<InputMessageContent>>(&mut self, input_message_content: T) -> &mut Self {
    self.inner.input_message_content = input_message_content.as_ref().clone();
    self
  }

}

impl AsRef<EditInlineMessageMedia> for EditInlineMessageMedia {
  fn as_ref(&self) -> &EditInlineMessageMedia { self }
}

impl AsRef<EditInlineMessageMedia> for RTDEditInlineMessageMediaBuilder {
  fn as_ref(&self) -> &EditInlineMessageMedia { &self.inner }
}







/// Edits the reply markup of an inline message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageReplyMarkup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Inline message identifier
  inline_message_id: String,
  /// The new message reply markup
  reply_markup: ReplyMarkup,
  
}

impl RObject for EditInlineMessageReplyMarkup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageReplyMarkup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditInlineMessageReplyMarkup {}

impl EditInlineMessageReplyMarkup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditInlineMessageReplyMarkupBuilder {
    let mut inner = EditInlineMessageReplyMarkup::default();
    inner.td_name = "editInlineMessageReplyMarkup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditInlineMessageReplyMarkupBuilder { inner }
  }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

}

#[doc(hidden)]
pub struct RTDEditInlineMessageReplyMarkupBuilder {
  inner: EditInlineMessageReplyMarkup
}

impl RTDEditInlineMessageReplyMarkupBuilder {
  pub fn build(&self) -> EditInlineMessageReplyMarkup { self.inner.clone() }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

}

impl AsRef<EditInlineMessageReplyMarkup> for EditInlineMessageReplyMarkup {
  fn as_ref(&self) -> &EditInlineMessageReplyMarkup { self }
}

impl AsRef<EditInlineMessageReplyMarkup> for RTDEditInlineMessageReplyMarkupBuilder {
  fn as_ref(&self) -> &EditInlineMessageReplyMarkup { &self.inner }
}







/// Edits the text of an inline text or game message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Inline message identifier
  inline_message_id: String,
  /// The new message reply markup
  reply_markup: ReplyMarkup,
  /// New text content of the message. Should be of type InputMessageText
  input_message_content: InputMessageContent,
  
}

impl RObject for EditInlineMessageText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageText" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditInlineMessageText {}

impl EditInlineMessageText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditInlineMessageTextBuilder {
    let mut inner = EditInlineMessageText::default();
    inner.td_name = "editInlineMessageText".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditInlineMessageTextBuilder { inner }
  }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn input_message_content(&self) -> &InputMessageContent { &self.input_message_content }

}

#[doc(hidden)]
pub struct RTDEditInlineMessageTextBuilder {
  inner: EditInlineMessageText
}

impl RTDEditInlineMessageTextBuilder {
  pub fn build(&self) -> EditInlineMessageText { self.inner.clone() }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn input_message_content<T: AsRef<InputMessageContent>>(&mut self, input_message_content: T) -> &mut Self {
    self.inner.input_message_content = input_message_content.as_ref().clone();
    self
  }

}

impl AsRef<EditInlineMessageText> for EditInlineMessageText {
  fn as_ref(&self) -> &EditInlineMessageText { self }
}

impl AsRef<EditInlineMessageText> for RTDEditInlineMessageTextBuilder {
  fn as_ref(&self) -> &EditInlineMessageText { &self.inner }
}







/// Edits the message content caption. Returns the edited message after the edit is completed on the server side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageCaption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat the message belongs to
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// The new message reply markup; for bots only
  reply_markup: ReplyMarkup,
  /// New message content caption; 0-GetOption("message_caption_length_max") characters
  caption: FormattedText,
  
}

impl RObject for EditMessageCaption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageCaption" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditMessageCaption {}

impl EditMessageCaption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditMessageCaptionBuilder {
    let mut inner = EditMessageCaption::default();
    inner.td_name = "editMessageCaption".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditMessageCaptionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn caption(&self) -> &FormattedText { &self.caption }

}

#[doc(hidden)]
pub struct RTDEditMessageCaptionBuilder {
  inner: EditMessageCaption
}

impl RTDEditMessageCaptionBuilder {
  pub fn build(&self) -> EditMessageCaption { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<EditMessageCaption> for EditMessageCaption {
  fn as_ref(&self) -> &EditMessageCaption { self }
}

impl AsRef<EditMessageCaption> for RTDEditMessageCaptionBuilder {
  fn as_ref(&self) -> &EditMessageCaption { &self.inner }
}







/// Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageLiveLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat the message belongs to
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// The new message reply markup; for bots only
  reply_markup: ReplyMarkup,
  /// New location content of the message; may be null. Pass null to stop sharing the live location
  location: Option<Location>,
  
}

impl RObject for EditMessageLiveLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageLiveLocation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditMessageLiveLocation {}

impl EditMessageLiveLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditMessageLiveLocationBuilder {
    let mut inner = EditMessageLiveLocation::default();
    inner.td_name = "editMessageLiveLocation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditMessageLiveLocationBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn location(&self) -> &Option<Location> { &self.location }

}

#[doc(hidden)]
pub struct RTDEditMessageLiveLocationBuilder {
  inner: EditMessageLiveLocation
}

impl RTDEditMessageLiveLocationBuilder {
  pub fn build(&self) -> EditMessageLiveLocation { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = Some(location.as_ref().clone());
    self
  }

}

impl AsRef<EditMessageLiveLocation> for EditMessageLiveLocation {
  fn as_ref(&self) -> &EditMessageLiveLocation { self }
}

impl AsRef<EditMessageLiveLocation> for RTDEditMessageLiveLocationBuilder {
  fn as_ref(&self) -> &EditMessageLiveLocation { &self.inner }
}







/// Edits the content of a message with an animation, an audio, a document, a photo or a video. The media in the message can't be replaced if the message was set to self-destruct. Media can't be replaced by self-destructing media. Media in an album can be edited only to contain a photo or a video. Returns the edited message after the edit is completed on the server side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageMedia {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat the message belongs to
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// The new message reply markup; for bots only
  reply_markup: ReplyMarkup,
  /// New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo
  input_message_content: InputMessageContent,
  
}

impl RObject for EditMessageMedia {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageMedia" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditMessageMedia {}

impl EditMessageMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditMessageMediaBuilder {
    let mut inner = EditMessageMedia::default();
    inner.td_name = "editMessageMedia".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditMessageMediaBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn input_message_content(&self) -> &InputMessageContent { &self.input_message_content }

}

#[doc(hidden)]
pub struct RTDEditMessageMediaBuilder {
  inner: EditMessageMedia
}

impl RTDEditMessageMediaBuilder {
  pub fn build(&self) -> EditMessageMedia { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn input_message_content<T: AsRef<InputMessageContent>>(&mut self, input_message_content: T) -> &mut Self {
    self.inner.input_message_content = input_message_content.as_ref().clone();
    self
  }

}

impl AsRef<EditMessageMedia> for EditMessageMedia {
  fn as_ref(&self) -> &EditMessageMedia { self }
}

impl AsRef<EditMessageMedia> for RTDEditMessageMediaBuilder {
  fn as_ref(&self) -> &EditMessageMedia { &self.inner }
}







/// Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageReplyMarkup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat the message belongs to
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// The new message reply markup
  reply_markup: ReplyMarkup,
  
}

impl RObject for EditMessageReplyMarkup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageReplyMarkup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditMessageReplyMarkup {}

impl EditMessageReplyMarkup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditMessageReplyMarkupBuilder {
    let mut inner = EditMessageReplyMarkup::default();
    inner.td_name = "editMessageReplyMarkup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditMessageReplyMarkupBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

}

#[doc(hidden)]
pub struct RTDEditMessageReplyMarkupBuilder {
  inner: EditMessageReplyMarkup
}

impl RTDEditMessageReplyMarkupBuilder {
  pub fn build(&self) -> EditMessageReplyMarkup { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

}

impl AsRef<EditMessageReplyMarkup> for EditMessageReplyMarkup {
  fn as_ref(&self) -> &EditMessageReplyMarkup { self }
}

impl AsRef<EditMessageReplyMarkup> for RTDEditMessageReplyMarkupBuilder {
  fn as_ref(&self) -> &EditMessageReplyMarkup { &self.inner }
}







/// Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageSchedulingState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat the message belongs to
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// The new message scheduling state. Pass null to send the message immediately
  scheduling_state: MessageSchedulingState,
  
}

impl RObject for EditMessageSchedulingState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageSchedulingState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditMessageSchedulingState {}

impl EditMessageSchedulingState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditMessageSchedulingStateBuilder {
    let mut inner = EditMessageSchedulingState::default();
    inner.td_name = "editMessageSchedulingState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditMessageSchedulingStateBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn scheduling_state(&self) -> &MessageSchedulingState { &self.scheduling_state }

}

#[doc(hidden)]
pub struct RTDEditMessageSchedulingStateBuilder {
  inner: EditMessageSchedulingState
}

impl RTDEditMessageSchedulingStateBuilder {
  pub fn build(&self) -> EditMessageSchedulingState { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn scheduling_state<T: AsRef<MessageSchedulingState>>(&mut self, scheduling_state: T) -> &mut Self {
    self.inner.scheduling_state = scheduling_state.as_ref().clone();
    self
  }

}

impl AsRef<EditMessageSchedulingState> for EditMessageSchedulingState {
  fn as_ref(&self) -> &EditMessageSchedulingState { self }
}

impl AsRef<EditMessageSchedulingState> for RTDEditMessageSchedulingStateBuilder {
  fn as_ref(&self) -> &EditMessageSchedulingState { &self.inner }
}







/// Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat the message belongs to
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// The new message reply markup; for bots only
  reply_markup: ReplyMarkup,
  /// New text content of the message. Should be of type InputMessageText
  input_message_content: InputMessageContent,
  
}

impl RObject for EditMessageText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editMessageText" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditMessageText {}

impl EditMessageText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditMessageTextBuilder {
    let mut inner = EditMessageText::default();
    inner.td_name = "editMessageText".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditMessageTextBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn input_message_content(&self) -> &InputMessageContent { &self.input_message_content }

}

#[doc(hidden)]
pub struct RTDEditMessageTextBuilder {
  inner: EditMessageText
}

impl RTDEditMessageTextBuilder {
  pub fn build(&self) -> EditMessageText { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn input_message_content<T: AsRef<InputMessageContent>>(&mut self, input_message_content: T) -> &mut Self {
    self.inner.input_message_content = input_message_content.as_ref().clone();
    self
  }

}

impl AsRef<EditMessageText> for EditMessageText {
  fn as_ref(&self) -> &EditMessageText { self }
}

impl AsRef<EditMessageText> for RTDEditMessageTextBuilder {
  fn as_ref(&self) -> &EditMessageText { &self.inner }
}







/// Edits an existing proxy server for network requests. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Proxy identifier
  proxy_id: i64,
  /// Proxy server IP address
  server: String,
  /// Proxy server port
  port: i64,
  /// True, if the proxy should be enabled
  enable: bool,
  /// Proxy type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: ProxyType,
  
}

impl RObject for EditProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EditProxy {}

impl EditProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEditProxyBuilder {
    let mut inner = EditProxy::default();
    inner.td_name = "editProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEditProxyBuilder { inner }
  }

  pub fn proxy_id(&self) -> i64 { self.proxy_id }

  pub fn server(&self) -> &String { &self.server }

  pub fn port(&self) -> i64 { self.port }

  pub fn enable(&self) -> bool { self.enable }

  pub fn type_(&self) -> &ProxyType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDEditProxyBuilder {
  inner: EditProxy
}

impl RTDEditProxyBuilder {
  pub fn build(&self) -> EditProxy { self.inner.clone() }

   
  pub fn proxy_id(&mut self, proxy_id: i64) -> &mut Self {
    self.inner.proxy_id = proxy_id;
    self
  }

   
  pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
    self.inner.server = server.as_ref().to_string();
    self
  }

   
  pub fn port(&mut self, port: i64) -> &mut Self {
    self.inner.port = port;
    self
  }

   
  pub fn enable(&mut self, enable: bool) -> &mut Self {
    self.inner.enable = enable;
    self
  }

   
  pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<EditProxy> for EditProxy {
  fn as_ref(&self) -> &EditProxy { self }
}

impl AsRef<EditProxy> for RTDEditProxyBuilder {
  fn as_ref(&self) -> &EditProxy { &self.inner }
}







/// Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnableProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Proxy identifier
  proxy_id: i64,
  
}

impl RObject for EnableProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "enableProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for EnableProxy {}

impl EnableProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEnableProxyBuilder {
    let mut inner = EnableProxy::default();
    inner.td_name = "enableProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEnableProxyBuilder { inner }
  }

  pub fn proxy_id(&self) -> i64 { self.proxy_id }

}

#[doc(hidden)]
pub struct RTDEnableProxyBuilder {
  inner: EnableProxy
}

impl RTDEnableProxyBuilder {
  pub fn build(&self) -> EnableProxy { self.inner.clone() }

   
  pub fn proxy_id(&mut self, proxy_id: i64) -> &mut Self {
    self.inner.proxy_id = proxy_id;
    self
  }

}

impl AsRef<EnableProxy> for EnableProxy {
  fn as_ref(&self) -> &EnableProxy { self }
}

impl AsRef<EnableProxy> for RTDEnableProxyBuilder {
  fn as_ref(&self) -> &EnableProxy { &self.inner }
}







/// Finishes the file generation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FinishFileGeneration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the generation process
  generation_id: isize,
  /// If set, means that file generation has failed and should be terminated
  error: Error,
  
}

impl RObject for FinishFileGeneration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "finishFileGeneration" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for FinishFileGeneration {}

impl FinishFileGeneration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFinishFileGenerationBuilder {
    let mut inner = FinishFileGeneration::default();
    inner.td_name = "finishFileGeneration".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDFinishFileGenerationBuilder { inner }
  }

  pub fn generation_id(&self) -> isize { self.generation_id }

  pub fn error(&self) -> &Error { &self.error }

}

#[doc(hidden)]
pub struct RTDFinishFileGenerationBuilder {
  inner: FinishFileGeneration
}

impl RTDFinishFileGenerationBuilder {
  pub fn build(&self) -> FinishFileGeneration { self.inner.clone() }

   
  pub fn generation_id(&mut self, generation_id: isize) -> &mut Self {
    self.inner.generation_id = generation_id;
    self
  }

   
  pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
    self.inner.error = error.as_ref().clone();
    self
  }

}

impl AsRef<FinishFileGeneration> for FinishFileGeneration {
  fn as_ref(&self) -> &FinishFileGeneration { self }
}

impl AsRef<FinishFileGeneration> for RTDFinishFileGenerationBuilder {
  fn as_ref(&self) -> &FinishFileGeneration { &self.inner }
}







/// Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForwardMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to which to forward messages
  chat_id: i64,
  /// Identifier of the chat from which to forward messages
  from_chat_id: i64,
  /// Identifiers of the messages to forward
  message_ids: Vec<i64>,
  /// Options to be used to send the messages
  options: SendMessageOptions,
  /// True, if the messages should be grouped into an album after forwarding. For this to work, no more than 10 messages may be forwarded, and all of them must be photo or video messages
  as_album: bool,
  /// True, if content of the messages needs to be copied without links to the original messages. Always true if the messages are forwarded to a secret chat
  send_copy: bool,
  /// True, if media captions of message copies needs to be removed. Ignored if send_copy is false
  remove_caption: bool,
  
}

impl RObject for ForwardMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "forwardMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ForwardMessages {}

impl ForwardMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDForwardMessagesBuilder {
    let mut inner = ForwardMessages::default();
    inner.td_name = "forwardMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDForwardMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn from_chat_id(&self) -> i64 { self.from_chat_id }

  pub fn message_ids(&self) -> &Vec<i64> { &self.message_ids }

  pub fn options(&self) -> &SendMessageOptions { &self.options }

  pub fn as_album(&self) -> bool { self.as_album }

  pub fn send_copy(&self) -> bool { self.send_copy }

  pub fn remove_caption(&self) -> bool { self.remove_caption }

}

#[doc(hidden)]
pub struct RTDForwardMessagesBuilder {
  inner: ForwardMessages
}

impl RTDForwardMessagesBuilder {
  pub fn build(&self) -> ForwardMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
    self.inner.from_chat_id = from_chat_id;
    self
  }

   
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }

   
  pub fn options<T: AsRef<SendMessageOptions>>(&mut self, options: T) -> &mut Self {
    self.inner.options = options.as_ref().clone();
    self
  }

   
  pub fn as_album(&mut self, as_album: bool) -> &mut Self {
    self.inner.as_album = as_album;
    self
  }

   
  pub fn send_copy(&mut self, send_copy: bool) -> &mut Self {
    self.inner.send_copy = send_copy;
    self
  }

   
  pub fn remove_caption(&mut self, remove_caption: bool) -> &mut Self {
    self.inner.remove_caption = remove_caption;
    self
  }

}

impl AsRef<ForwardMessages> for ForwardMessages {
  fn as_ref(&self) -> &ForwardMessages { self }
}

impl AsRef<ForwardMessages> for RTDForwardMessagesBuilder {
  fn as_ref(&self) -> &ForwardMessages { &self.inner }
}







/// Generates a new invite link for a chat; the previously generated link is revoked. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GenerateChatInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for GenerateChatInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "generateChatInviteLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GenerateChatInviteLink {}

impl GenerateChatInviteLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGenerateChatInviteLinkBuilder {
    let mut inner = GenerateChatInviteLink::default();
    inner.td_name = "generateChatInviteLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGenerateChatInviteLinkBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDGenerateChatInviteLinkBuilder {
  inner: GenerateChatInviteLink
}

impl RTDGenerateChatInviteLinkBuilder {
  pub fn build(&self) -> GenerateChatInviteLink { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<GenerateChatInviteLink> for GenerateChatInviteLink {
  fn as_ref(&self) -> &GenerateChatInviteLink { self }
}

impl AsRef<GenerateChatInviteLink> for RTDGenerateChatInviteLinkBuilder {
  fn as_ref(&self) -> &GenerateChatInviteLink { &self.inner }
}







/// Returns the period of inactivity after which the account of the current user will automatically be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccountTtl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetAccountTtl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAccountTtl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetAccountTtl {}

impl GetAccountTtl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetAccountTtlBuilder {
    let mut inner = GetAccountTtl::default();
    inner.td_name = "getAccountTtl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetAccountTtlBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetAccountTtlBuilder {
  inner: GetAccountTtl
}

impl RTDGetAccountTtlBuilder {
  pub fn build(&self) -> GetAccountTtl { self.inner.clone() }

}

impl AsRef<GetAccountTtl> for GetAccountTtl {
  fn as_ref(&self) -> &GetAccountTtl { self }
}

impl AsRef<GetAccountTtl> for RTDGetAccountTtlBuilder {
  fn as_ref(&self) -> &GetAccountTtl { &self.inner }
}







/// Returns all active live locations that should be updated by the client. The list is persistent across application restarts only if the message database is used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetActiveLiveLocationMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetActiveLiveLocationMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getActiveLiveLocationMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetActiveLiveLocationMessages {}

impl GetActiveLiveLocationMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetActiveLiveLocationMessagesBuilder {
    let mut inner = GetActiveLiveLocationMessages::default();
    inner.td_name = "getActiveLiveLocationMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetActiveLiveLocationMessagesBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetActiveLiveLocationMessagesBuilder {
  inner: GetActiveLiveLocationMessages
}

impl RTDGetActiveLiveLocationMessagesBuilder {
  pub fn build(&self) -> GetActiveLiveLocationMessages { self.inner.clone() }

}

impl AsRef<GetActiveLiveLocationMessages> for GetActiveLiveLocationMessages {
  fn as_ref(&self) -> &GetActiveLiveLocationMessages { self }
}

impl AsRef<GetActiveLiveLocationMessages> for RTDGetActiveLiveLocationMessagesBuilder {
  fn as_ref(&self) -> &GetActiveLiveLocationMessages { &self.inner }
}







/// Returns all active sessions of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetActiveSessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetActiveSessions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getActiveSessions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetActiveSessions {}

impl GetActiveSessions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetActiveSessionsBuilder {
    let mut inner = GetActiveSessions::default();
    inner.td_name = "getActiveSessions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetActiveSessionsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetActiveSessionsBuilder {
  inner: GetActiveSessions
}

impl RTDGetActiveSessionsBuilder {
  pub fn build(&self) -> GetActiveSessions { self.inner.clone() }

}

impl AsRef<GetActiveSessions> for GetActiveSessions {
  fn as_ref(&self) -> &GetActiveSessions { self }
}

impl AsRef<GetActiveSessions> for RTDGetActiveSessionsBuilder {
  fn as_ref(&self) -> &GetActiveSessions { &self.inner }
}







/// Returns all available Telegram Passport elements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAllPassportElements {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Password of the current user
  password: String,
  
}

impl RObject for GetAllPassportElements {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAllPassportElements" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetAllPassportElements {}

impl GetAllPassportElements {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetAllPassportElementsBuilder {
    let mut inner = GetAllPassportElements::default();
    inner.td_name = "getAllPassportElements".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetAllPassportElementsBuilder { inner }
  }

  pub fn password(&self) -> &String { &self.password }

}

#[doc(hidden)]
pub struct RTDGetAllPassportElementsBuilder {
  inner: GetAllPassportElements
}

impl RTDGetAllPassportElementsBuilder {
  pub fn build(&self) -> GetAllPassportElements { self.inner.clone() }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

}

impl AsRef<GetAllPassportElements> for GetAllPassportElements {
  fn as_ref(&self) -> &GetAllPassportElements { self }
}

impl AsRef<GetAllPassportElements> for RTDGetAllPassportElementsBuilder {
  fn as_ref(&self) -> &GetAllPassportElements { &self.inner }
}







/// Returns application config, provided by the server. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationConfig {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetApplicationConfig {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getApplicationConfig" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDJsonValue for GetApplicationConfig {}

impl RFunction for GetApplicationConfig {}

impl GetApplicationConfig {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetApplicationConfigBuilder {
    let mut inner = GetApplicationConfig::default();
    inner.td_name = "getApplicationConfig".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetApplicationConfigBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetApplicationConfigBuilder {
  inner: GetApplicationConfig
}

impl RTDGetApplicationConfigBuilder {
  pub fn build(&self) -> GetApplicationConfig { self.inner.clone() }

}

impl AsRef<GetApplicationConfig> for GetApplicationConfig {
  fn as_ref(&self) -> &GetApplicationConfig { self }
}

impl AsRef<GetApplicationConfig> for RTDGetApplicationConfigBuilder {
  fn as_ref(&self) -> &GetApplicationConfig { &self.inner }
}







/// Returns a list of archived sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArchivedStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to return mask stickers sets; pass false to return ordinary sticker sets
  is_masks: bool,
  /// Identifier of the sticker set from which to return the result
  offset_sticker_set_id: isize,
  /// The maximum number of sticker sets to return
  limit: i64,
  
}

impl RObject for GetArchivedStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getArchivedStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetArchivedStickerSets {}

impl GetArchivedStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetArchivedStickerSetsBuilder {
    let mut inner = GetArchivedStickerSets::default();
    inner.td_name = "getArchivedStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetArchivedStickerSetsBuilder { inner }
  }

  pub fn is_masks(&self) -> bool { self.is_masks }

  pub fn offset_sticker_set_id(&self) -> isize { self.offset_sticker_set_id }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetArchivedStickerSetsBuilder {
  inner: GetArchivedStickerSets
}

impl RTDGetArchivedStickerSetsBuilder {
  pub fn build(&self) -> GetArchivedStickerSets { self.inner.clone() }

   
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.is_masks = is_masks;
    self
  }

   
  pub fn offset_sticker_set_id(&mut self, offset_sticker_set_id: isize) -> &mut Self {
    self.inner.offset_sticker_set_id = offset_sticker_set_id;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetArchivedStickerSets> for GetArchivedStickerSets {
  fn as_ref(&self) -> &GetArchivedStickerSets { self }
}

impl AsRef<GetArchivedStickerSets> for RTDGetArchivedStickerSetsBuilder {
  fn as_ref(&self) -> &GetArchivedStickerSets { &self.inner }
}







/// Returns a list of sticker sets attached to a file. Currently only photos and videos can have attached sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAttachedStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File identifier
  file_id: i64,
  
}

impl RObject for GetAttachedStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAttachedStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetAttachedStickerSets {}

impl GetAttachedStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetAttachedStickerSetsBuilder {
    let mut inner = GetAttachedStickerSets::default();
    inner.td_name = "getAttachedStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetAttachedStickerSetsBuilder { inner }
  }

  pub fn file_id(&self) -> i64 { self.file_id }

}

#[doc(hidden)]
pub struct RTDGetAttachedStickerSetsBuilder {
  inner: GetAttachedStickerSets
}

impl RTDGetAttachedStickerSetsBuilder {
  pub fn build(&self) -> GetAttachedStickerSets { self.inner.clone() }

   
  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

}

impl AsRef<GetAttachedStickerSets> for GetAttachedStickerSets {
  fn as_ref(&self) -> &GetAttachedStickerSets { self }
}

impl AsRef<GetAttachedStickerSets> for RTDGetAttachedStickerSetsBuilder {
  fn as_ref(&self) -> &GetAttachedStickerSets { &self.inner }
}







/// Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAuthorizationState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetAuthorizationState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAuthorizationState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for GetAuthorizationState {}

impl RFunction for GetAuthorizationState {}

impl GetAuthorizationState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetAuthorizationStateBuilder {
    let mut inner = GetAuthorizationState::default();
    inner.td_name = "getAuthorizationState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetAuthorizationStateBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetAuthorizationStateBuilder {
  inner: GetAuthorizationState
}

impl RTDGetAuthorizationStateBuilder {
  pub fn build(&self) -> GetAuthorizationState { self.inner.clone() }

}

impl AsRef<GetAuthorizationState> for GetAuthorizationState {
  fn as_ref(&self) -> &GetAuthorizationState { self }
}

impl AsRef<GetAuthorizationState> for RTDGetAuthorizationStateBuilder {
  fn as_ref(&self) -> &GetAuthorizationState { &self.inner }
}







/// Returns auto-download settings presets for the currently logged in user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAutoDownloadSettingsPresets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetAutoDownloadSettingsPresets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAutoDownloadSettingsPresets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetAutoDownloadSettingsPresets {}

impl GetAutoDownloadSettingsPresets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetAutoDownloadSettingsPresetsBuilder {
    let mut inner = GetAutoDownloadSettingsPresets::default();
    inner.td_name = "getAutoDownloadSettingsPresets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetAutoDownloadSettingsPresetsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetAutoDownloadSettingsPresetsBuilder {
  inner: GetAutoDownloadSettingsPresets
}

impl RTDGetAutoDownloadSettingsPresetsBuilder {
  pub fn build(&self) -> GetAutoDownloadSettingsPresets { self.inner.clone() }

}

impl AsRef<GetAutoDownloadSettingsPresets> for GetAutoDownloadSettingsPresets {
  fn as_ref(&self) -> &GetAutoDownloadSettingsPresets { self }
}

impl AsRef<GetAutoDownloadSettingsPresets> for RTDGetAutoDownloadSettingsPresetsBuilder {
  fn as_ref(&self) -> &GetAutoDownloadSettingsPresets { &self.inner }
}







/// Constructs a persistent HTTP URL for a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBackgroundUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Background name
  name: String,
  /// Background type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: BackgroundType,
  
}

impl RObject for GetBackgroundUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getBackgroundUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetBackgroundUrl {}

impl GetBackgroundUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetBackgroundUrlBuilder {
    let mut inner = GetBackgroundUrl::default();
    inner.td_name = "getBackgroundUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetBackgroundUrlBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

  pub fn type_(&self) -> &BackgroundType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDGetBackgroundUrlBuilder {
  inner: GetBackgroundUrl
}

impl RTDGetBackgroundUrlBuilder {
  pub fn build(&self) -> GetBackgroundUrl { self.inner.clone() }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn type_<T: AsRef<BackgroundType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<GetBackgroundUrl> for GetBackgroundUrl {
  fn as_ref(&self) -> &GetBackgroundUrl { self }
}

impl AsRef<GetBackgroundUrl> for RTDGetBackgroundUrlBuilder {
  fn as_ref(&self) -> &GetBackgroundUrl { &self.inner }
}







/// Returns backgrounds installed by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBackgrounds {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the backgrounds needs to be ordered for dark theme
  for_dark_theme: bool,
  
}

impl RObject for GetBackgrounds {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getBackgrounds" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetBackgrounds {}

impl GetBackgrounds {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetBackgroundsBuilder {
    let mut inner = GetBackgrounds::default();
    inner.td_name = "getBackgrounds".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetBackgroundsBuilder { inner }
  }

  pub fn for_dark_theme(&self) -> bool { self.for_dark_theme }

}

#[doc(hidden)]
pub struct RTDGetBackgroundsBuilder {
  inner: GetBackgrounds
}

impl RTDGetBackgroundsBuilder {
  pub fn build(&self) -> GetBackgrounds { self.inner.clone() }

   
  pub fn for_dark_theme(&mut self, for_dark_theme: bool) -> &mut Self {
    self.inner.for_dark_theme = for_dark_theme;
    self
  }

}

impl AsRef<GetBackgrounds> for GetBackgrounds {
  fn as_ref(&self) -> &GetBackgrounds { self }
}

impl AsRef<GetBackgrounds> for RTDGetBackgroundsBuilder {
  fn as_ref(&self) -> &GetBackgrounds { &self.inner }
}







/// Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBasicGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Basic group identifier
  basic_group_id: i64,
  
}

impl RObject for GetBasicGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getBasicGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetBasicGroup {}

impl GetBasicGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetBasicGroupBuilder {
    let mut inner = GetBasicGroup::default();
    inner.td_name = "getBasicGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetBasicGroupBuilder { inner }
  }

  pub fn basic_group_id(&self) -> i64 { self.basic_group_id }

}

#[doc(hidden)]
pub struct RTDGetBasicGroupBuilder {
  inner: GetBasicGroup
}

impl RTDGetBasicGroupBuilder {
  pub fn build(&self) -> GetBasicGroup { self.inner.clone() }

   
  pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
    self.inner.basic_group_id = basic_group_id;
    self
  }

}

impl AsRef<GetBasicGroup> for GetBasicGroup {
  fn as_ref(&self) -> &GetBasicGroup { self }
}

impl AsRef<GetBasicGroup> for RTDGetBasicGroupBuilder {
  fn as_ref(&self) -> &GetBasicGroup { &self.inner }
}







/// Returns full information about a basic group by its identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBasicGroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Basic group identifier
  basic_group_id: i64,
  
}

impl RObject for GetBasicGroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getBasicGroupFullInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetBasicGroupFullInfo {}

impl GetBasicGroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetBasicGroupFullInfoBuilder {
    let mut inner = GetBasicGroupFullInfo::default();
    inner.td_name = "getBasicGroupFullInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetBasicGroupFullInfoBuilder { inner }
  }

  pub fn basic_group_id(&self) -> i64 { self.basic_group_id }

}

#[doc(hidden)]
pub struct RTDGetBasicGroupFullInfoBuilder {
  inner: GetBasicGroupFullInfo
}

impl RTDGetBasicGroupFullInfoBuilder {
  pub fn build(&self) -> GetBasicGroupFullInfo { self.inner.clone() }

   
  pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
    self.inner.basic_group_id = basic_group_id;
    self
  }

}

impl AsRef<GetBasicGroupFullInfo> for GetBasicGroupFullInfo {
  fn as_ref(&self) -> &GetBasicGroupFullInfo { self }
}

impl AsRef<GetBasicGroupFullInfo> for RTDGetBasicGroupFullInfoBuilder {
  fn as_ref(&self) -> &GetBasicGroupFullInfo { &self.inner }
}







/// Returns users that were blocked by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBlockedUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of users to skip in the result; must be non-negative
  offset: i64,
  /// The maximum number of users to return; up to 100
  limit: i64,
  
}

impl RObject for GetBlockedUsers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getBlockedUsers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetBlockedUsers {}

impl GetBlockedUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetBlockedUsersBuilder {
    let mut inner = GetBlockedUsers::default();
    inner.td_name = "getBlockedUsers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetBlockedUsersBuilder { inner }
  }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetBlockedUsersBuilder {
  inner: GetBlockedUsers
}

impl RTDGetBlockedUsersBuilder {
  pub fn build(&self) -> GetBlockedUsers { self.inner.clone() }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetBlockedUsers> for GetBlockedUsers {
  fn as_ref(&self) -> &GetBlockedUsers { self }
}

impl AsRef<GetBlockedUsers> for RTDGetBlockedUsersBuilder {
  fn as_ref(&self) -> &GetBlockedUsers { &self.inner }
}







/// Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCallbackQueryAnswer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat with the message
  chat_id: i64,
  /// Identifier of the message from which the query originated
  message_id: i64,
  /// Query payload
  payload: CallbackQueryPayload,
  
}

impl RObject for GetCallbackQueryAnswer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getCallbackQueryAnswer" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetCallbackQueryAnswer {}

impl GetCallbackQueryAnswer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetCallbackQueryAnswerBuilder {
    let mut inner = GetCallbackQueryAnswer::default();
    inner.td_name = "getCallbackQueryAnswer".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetCallbackQueryAnswerBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn payload(&self) -> &CallbackQueryPayload { &self.payload }

}

#[doc(hidden)]
pub struct RTDGetCallbackQueryAnswerBuilder {
  inner: GetCallbackQueryAnswer
}

impl RTDGetCallbackQueryAnswerBuilder {
  pub fn build(&self) -> GetCallbackQueryAnswer { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn payload<T: AsRef<CallbackQueryPayload>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().clone();
    self
  }

}

impl AsRef<GetCallbackQueryAnswer> for GetCallbackQueryAnswer {
  fn as_ref(&self) -> &GetCallbackQueryAnswer { self }
}

impl AsRef<GetCallbackQueryAnswer> for RTDGetCallbackQueryAnswerBuilder {
  fn as_ref(&self) -> &GetCallbackQueryAnswer { &self.inner }
}







/// Returns information about a chat by its identifier, this is an offline request if the current user is not a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for GetChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChat {}

impl GetChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatBuilder {
    let mut inner = GetChat::default();
    inner.td_name = "getChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDGetChatBuilder {
  inner: GetChat
}

impl RTDGetChatBuilder {
  pub fn build(&self) -> GetChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<GetChat> for GetChat {
  fn as_ref(&self) -> &GetChat { self }
}

impl AsRef<GetChat> for RTDGetChatBuilder {
  fn as_ref(&self) -> &GetChat { &self.inner }
}







/// Returns a list of administrators of the chat with their custom titles
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for GetChatAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatAdministrators" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatAdministrators {}

impl GetChatAdministrators {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatAdministratorsBuilder {
    let mut inner = GetChatAdministrators::default();
    inner.td_name = "getChatAdministrators".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatAdministratorsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDGetChatAdministratorsBuilder {
  inner: GetChatAdministrators
}

impl RTDGetChatAdministratorsBuilder {
  pub fn build(&self) -> GetChatAdministrators { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<GetChatAdministrators> for GetChatAdministrators {
  fn as_ref(&self) -> &GetChatAdministrators { self }
}

impl AsRef<GetChatAdministrators> for RTDGetChatAdministratorsBuilder {
  fn as_ref(&self) -> &GetChatAdministrators { &self.inner }
}







/// Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only for supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatEventLog {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Search query by which to filter events
  query: String,
  /// Identifier of an event from which to return results. Use 0 to get results from the latest events
  from_event_id: isize,
  /// The maximum number of events to return; up to 100
  limit: i64,
  /// The types of events to return. By default, all types will be returned
  filters: ChatEventLogFilters,
  /// User identifiers by which to filter events. By default, events relating to all users will be returned
  user_ids: Vec<i64>,
  
}

impl RObject for GetChatEventLog {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatEventLog" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatEventLog {}

impl GetChatEventLog {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatEventLogBuilder {
    let mut inner = GetChatEventLog::default();
    inner.td_name = "getChatEventLog".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatEventLogBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn query(&self) -> &String { &self.query }

  pub fn from_event_id(&self) -> isize { self.from_event_id }

  pub fn limit(&self) -> i64 { self.limit }

  pub fn filters(&self) -> &ChatEventLogFilters { &self.filters }

  pub fn user_ids(&self) -> &Vec<i64> { &self.user_ids }

}

#[doc(hidden)]
pub struct RTDGetChatEventLogBuilder {
  inner: GetChatEventLog
}

impl RTDGetChatEventLogBuilder {
  pub fn build(&self) -> GetChatEventLog { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn from_event_id(&mut self, from_event_id: isize) -> &mut Self {
    self.inner.from_event_id = from_event_id;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

   
  pub fn filters<T: AsRef<ChatEventLogFilters>>(&mut self, filters: T) -> &mut Self {
    self.inner.filters = filters.as_ref().clone();
    self
  }

   
  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

}

impl AsRef<GetChatEventLog> for GetChatEventLog {
  fn as_ref(&self) -> &GetChatEventLog { self }
}

impl AsRef<GetChatEventLog> for RTDGetChatEventLogBuilder {
  fn as_ref(&self) -> &GetChatEventLog { &self.inner }
}







/// Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library. This is an offline request if only_local is true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
  from_message_id: i64,
  /// Specify 0 to get results from exactly the from_message_id or a negative offset up to 99 to get additionally some newer messages
  offset: i64,
  /// The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater or equal to offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached
  limit: i64,
  /// If true, returns only messages that are available locally without sending network requests
  only_local: bool,
  
}

impl RObject for GetChatHistory {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatHistory" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatHistory {}

impl GetChatHistory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatHistoryBuilder {
    let mut inner = GetChatHistory::default();
    inner.td_name = "getChatHistory".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatHistoryBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn from_message_id(&self) -> i64 { self.from_message_id }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn limit(&self) -> i64 { self.limit }

  pub fn only_local(&self) -> bool { self.only_local }

}

#[doc(hidden)]
pub struct RTDGetChatHistoryBuilder {
  inner: GetChatHistory
}

impl RTDGetChatHistoryBuilder {
  pub fn build(&self) -> GetChatHistory { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
    self.inner.from_message_id = from_message_id;
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

   
  pub fn only_local(&mut self, only_local: bool) -> &mut Self {
    self.inner.only_local = only_local;
    self
  }

}

impl AsRef<GetChatHistory> for GetChatHistory {
  fn as_ref(&self) -> &GetChatHistory { self }
}

impl AsRef<GetChatHistory> for RTDGetChatHistoryBuilder {
  fn as_ref(&self) -> &GetChatHistory { &self.inner }
}







/// Returns information about a single member of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// User identifier
  user_id: i64,
  
}

impl RObject for GetChatMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatMember" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatMember {}

impl GetChatMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatMemberBuilder {
    let mut inner = GetChatMember::default();
    inner.td_name = "getChatMember".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatMemberBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDGetChatMemberBuilder {
  inner: GetChatMember
}

impl RTDGetChatMemberBuilder {
  pub fn build(&self) -> GetChatMember { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<GetChatMember> for GetChatMember {
  fn as_ref(&self) -> &GetChatMember { self }
}

impl AsRef<GetChatMember> for RTDGetChatMemberBuilder {
  fn as_ref(&self) -> &GetChatMember { &self.inner }
}







/// Returns the last message sent in a chat no later than the specified date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatMessageByDate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Point in time (Unix timestamp) relative to which to search for messages
  date: i64,
  
}

impl RObject for GetChatMessageByDate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatMessageByDate" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatMessageByDate {}

impl GetChatMessageByDate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatMessageByDateBuilder {
    let mut inner = GetChatMessageByDate::default();
    inner.td_name = "getChatMessageByDate".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatMessageByDateBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn date(&self) -> i64 { self.date }

}

#[doc(hidden)]
pub struct RTDGetChatMessageByDateBuilder {
  inner: GetChatMessageByDate
}

impl RTDGetChatMessageByDateBuilder {
  pub fn build(&self) -> GetChatMessageByDate { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

}

impl AsRef<GetChatMessageByDate> for GetChatMessageByDate {
  fn as_ref(&self) -> &GetChatMessageByDate { self }
}

impl AsRef<GetChatMessageByDate> for RTDGetChatMessageByDateBuilder {
  fn as_ref(&self) -> &GetChatMessageByDate { &self.inner }
}







/// Returns approximate number of messages of the specified type in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatMessageCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat in which to count messages
  chat_id: i64,
  /// Filter for message content; searchMessagesFilterEmpty is unsupported in this function
  filter: SearchMessagesFilter,
  /// If true, returns count that is available locally without sending network requests, returning 1 if the number of messages is unknown
  return_local: bool,
  
}

impl RObject for GetChatMessageCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatMessageCount" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatMessageCount {}

impl GetChatMessageCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatMessageCountBuilder {
    let mut inner = GetChatMessageCount::default();
    inner.td_name = "getChatMessageCount".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatMessageCountBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn filter(&self) -> &SearchMessagesFilter { &self.filter }

  pub fn return_local(&self) -> bool { self.return_local }

}

#[doc(hidden)]
pub struct RTDGetChatMessageCountBuilder {
  inner: GetChatMessageCount
}

impl RTDGetChatMessageCountBuilder {
  pub fn build(&self) -> GetChatMessageCount { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn filter<T: AsRef<SearchMessagesFilter>>(&mut self, filter: T) -> &mut Self {
    self.inner.filter = filter.as_ref().clone();
    self
  }

   
  pub fn return_local(&mut self, return_local: bool) -> &mut Self {
    self.inner.return_local = return_local;
    self
  }

}

impl AsRef<GetChatMessageCount> for GetChatMessageCount {
  fn as_ref(&self) -> &GetChatMessageCount { self }
}

impl AsRef<GetChatMessageCount> for RTDGetChatMessageCountBuilder {
  fn as_ref(&self) -> &GetChatMessageCount { &self.inner }
}







/// Returns list of chats with non-default notification settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatNotificationSettingsExceptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// If specified, only chats from the specified scope will be returned
  scope: NotificationSettingsScope,
  /// If true, also chats with non-default sound will be returned
  compare_sound: bool,
  
}

impl RObject for GetChatNotificationSettingsExceptions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatNotificationSettingsExceptions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatNotificationSettingsExceptions {}

impl GetChatNotificationSettingsExceptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatNotificationSettingsExceptionsBuilder {
    let mut inner = GetChatNotificationSettingsExceptions::default();
    inner.td_name = "getChatNotificationSettingsExceptions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatNotificationSettingsExceptionsBuilder { inner }
  }

  pub fn scope(&self) -> &NotificationSettingsScope { &self.scope }

  pub fn compare_sound(&self) -> bool { self.compare_sound }

}

#[doc(hidden)]
pub struct RTDGetChatNotificationSettingsExceptionsBuilder {
  inner: GetChatNotificationSettingsExceptions
}

impl RTDGetChatNotificationSettingsExceptionsBuilder {
  pub fn build(&self) -> GetChatNotificationSettingsExceptions { self.inner.clone() }

   
  pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
    self.inner.scope = scope.as_ref().clone();
    self
  }

   
  pub fn compare_sound(&mut self, compare_sound: bool) -> &mut Self {
    self.inner.compare_sound = compare_sound;
    self
  }

}

impl AsRef<GetChatNotificationSettingsExceptions> for GetChatNotificationSettingsExceptions {
  fn as_ref(&self) -> &GetChatNotificationSettingsExceptions { self }
}

impl AsRef<GetChatNotificationSettingsExceptions> for RTDGetChatNotificationSettingsExceptionsBuilder {
  fn as_ref(&self) -> &GetChatNotificationSettingsExceptions { &self.inner }
}







/// Returns information about a pinned chat message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatPinnedMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat the message belongs to
  chat_id: i64,
  
}

impl RObject for GetChatPinnedMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatPinnedMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatPinnedMessage {}

impl GetChatPinnedMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatPinnedMessageBuilder {
    let mut inner = GetChatPinnedMessage::default();
    inner.td_name = "getChatPinnedMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatPinnedMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDGetChatPinnedMessageBuilder {
  inner: GetChatPinnedMessage
}

impl RTDGetChatPinnedMessageBuilder {
  pub fn build(&self) -> GetChatPinnedMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<GetChatPinnedMessage> for GetChatPinnedMessage {
  fn as_ref(&self) -> &GetChatPinnedMessage { self }
}

impl AsRef<GetChatPinnedMessage> for RTDGetChatPinnedMessageBuilder {
  fn as_ref(&self) -> &GetChatPinnedMessage { &self.inner }
}







/// Returns all scheduled messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatScheduledMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for GetChatScheduledMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatScheduledMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatScheduledMessages {}

impl GetChatScheduledMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatScheduledMessagesBuilder {
    let mut inner = GetChatScheduledMessages::default();
    inner.td_name = "getChatScheduledMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatScheduledMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDGetChatScheduledMessagesBuilder {
  inner: GetChatScheduledMessages
}

impl RTDGetChatScheduledMessagesBuilder {
  pub fn build(&self) -> GetChatScheduledMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<GetChatScheduledMessages> for GetChatScheduledMessages {
  fn as_ref(&self) -> &GetChatScheduledMessages { self }
}

impl AsRef<GetChatScheduledMessages> for RTDGetChatScheduledMessagesBuilder {
  fn as_ref(&self) -> &GetChatScheduledMessages { &self.inner }
}







/// Returns an HTTP URL with the chat statistics. Currently this method can be used only for channels. Can be used only if SupergroupFullInfo.can_view_statistics == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatStatisticsUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Parameters from "tg://statsrefresh?params=******" link
  parameters: String,
  /// Pass true if a URL with the dark theme must be returned
  is_dark: bool,
  
}

impl RObject for GetChatStatisticsUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatStatisticsUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChatStatisticsUrl {}

impl GetChatStatisticsUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatStatisticsUrlBuilder {
    let mut inner = GetChatStatisticsUrl::default();
    inner.td_name = "getChatStatisticsUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatStatisticsUrlBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn parameters(&self) -> &String { &self.parameters }

  pub fn is_dark(&self) -> bool { self.is_dark }

}

#[doc(hidden)]
pub struct RTDGetChatStatisticsUrlBuilder {
  inner: GetChatStatisticsUrl
}

impl RTDGetChatStatisticsUrlBuilder {
  pub fn build(&self) -> GetChatStatisticsUrl { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn parameters<T: AsRef<str>>(&mut self, parameters: T) -> &mut Self {
    self.inner.parameters = parameters.as_ref().to_string();
    self
  }

   
  pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
    self.inner.is_dark = is_dark;
    self
  }

}

impl AsRef<GetChatStatisticsUrl> for GetChatStatisticsUrl {
  fn as_ref(&self) -> &GetChatStatisticsUrl { self }
}

impl AsRef<GetChatStatisticsUrl> for RTDGetChatStatisticsUrlBuilder {
  fn as_ref(&self) -> &GetChatStatisticsUrl { &self.inner }
}







/// Returns an ordered list of chats in a chat list. Chats are sorted by the pair (order, chat_id) in decreasing order. (For example, to get a list of chats from the beginning, the offset_order should be equal to a biggest signed 64-bit number 9223372036854775807 == 2^63  1). For optimal performance the number of returned chats is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat list in which to return chats
  chat_list: ChatList,
  /// Chat order to return chats from
  offset_order: isize,
  /// Chat identifier to return chats from
  offset_chat_id: i64,
  /// The maximum number of chats to be returned. It is possible that fewer chats than the limit are returned even if the end of the list is not reached
  limit: i64,
  
}

impl RObject for GetChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetChats {}

impl GetChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetChatsBuilder {
    let mut inner = GetChats::default();
    inner.td_name = "getChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetChatsBuilder { inner }
  }

  pub fn chat_list(&self) -> &ChatList { &self.chat_list }

  pub fn offset_order(&self) -> isize { self.offset_order }

  pub fn offset_chat_id(&self) -> i64 { self.offset_chat_id }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetChatsBuilder {
  inner: GetChats
}

impl RTDGetChatsBuilder {
  pub fn build(&self) -> GetChats { self.inner.clone() }

   
  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = chat_list.as_ref().clone();
    self
  }

   
  pub fn offset_order(&mut self, offset_order: isize) -> &mut Self {
    self.inner.offset_order = offset_order;
    self
  }

   
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
    self.inner.offset_chat_id = offset_chat_id;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetChats> for GetChats {
  fn as_ref(&self) -> &GetChats { self }
}

impl AsRef<GetChats> for RTDGetChatsBuilder {
  fn as_ref(&self) -> &GetChats { &self.inner }
}







/// Returns all website where the current user used Telegram to log in
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConnectedWebsites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetConnectedWebsites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getConnectedWebsites" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetConnectedWebsites {}

impl GetConnectedWebsites {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetConnectedWebsitesBuilder {
    let mut inner = GetConnectedWebsites::default();
    inner.td_name = "getConnectedWebsites".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetConnectedWebsitesBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetConnectedWebsitesBuilder {
  inner: GetConnectedWebsites
}

impl RTDGetConnectedWebsitesBuilder {
  pub fn build(&self) -> GetConnectedWebsites { self.inner.clone() }

}

impl AsRef<GetConnectedWebsites> for GetConnectedWebsites {
  fn as_ref(&self) -> &GetConnectedWebsites { self }
}

impl AsRef<GetConnectedWebsites> for RTDGetConnectedWebsitesBuilder {
  fn as_ref(&self) -> &GetConnectedWebsites { &self.inner }
}







/// Returns all user contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getContacts" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetContacts {}

impl GetContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetContactsBuilder {
    let mut inner = GetContacts::default();
    inner.td_name = "getContacts".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetContactsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetContactsBuilder {
  inner: GetContacts
}

impl RTDGetContactsBuilder {
  pub fn build(&self) -> GetContacts { self.inner.clone() }

}

impl AsRef<GetContacts> for GetContacts {
  fn as_ref(&self) -> &GetContacts { self }
}

impl AsRef<GetContacts> for RTDGetContactsBuilder {
  fn as_ref(&self) -> &GetContacts { &self.inner }
}







/// Uses current user IP to found their country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCountryCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetCountryCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getCountryCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetCountryCode {}

impl GetCountryCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetCountryCodeBuilder {
    let mut inner = GetCountryCode::default();
    inner.td_name = "getCountryCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetCountryCodeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetCountryCodeBuilder {
  inner: GetCountryCode
}

impl RTDGetCountryCodeBuilder {
  pub fn build(&self) -> GetCountryCode { self.inner.clone() }

}

impl AsRef<GetCountryCode> for GetCountryCode {
  fn as_ref(&self) -> &GetCountryCode { self }
}

impl AsRef<GetCountryCode> for RTDGetCountryCodeBuilder {
  fn as_ref(&self) -> &GetCountryCode { &self.inner }
}







/// Returns a list of public chats of the specified type, owned by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCreatedPublicChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the public chats to return
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: PublicChatType,
  
}

impl RObject for GetCreatedPublicChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getCreatedPublicChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetCreatedPublicChats {}

impl GetCreatedPublicChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetCreatedPublicChatsBuilder {
    let mut inner = GetCreatedPublicChats::default();
    inner.td_name = "getCreatedPublicChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetCreatedPublicChatsBuilder { inner }
  }

  pub fn type_(&self) -> &PublicChatType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDGetCreatedPublicChatsBuilder {
  inner: GetCreatedPublicChats
}

impl RTDGetCreatedPublicChatsBuilder {
  pub fn build(&self) -> GetCreatedPublicChats { self.inner.clone() }

   
  pub fn type_<T: AsRef<PublicChatType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<GetCreatedPublicChats> for GetCreatedPublicChats {
  fn as_ref(&self) -> &GetCreatedPublicChats { self }
}

impl AsRef<GetCreatedPublicChats> for RTDGetCreatedPublicChatsBuilder {
  fn as_ref(&self) -> &GetCreatedPublicChats { &self.inner }
}







/// Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially useful if TDLib is run in a separate process. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCurrentState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetCurrentState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getCurrentState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetCurrentState {}

impl GetCurrentState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetCurrentStateBuilder {
    let mut inner = GetCurrentState::default();
    inner.td_name = "getCurrentState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetCurrentStateBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetCurrentStateBuilder {
  inner: GetCurrentState
}

impl RTDGetCurrentStateBuilder {
  pub fn build(&self) -> GetCurrentState { self.inner.clone() }

}

impl AsRef<GetCurrentState> for GetCurrentState {
  fn as_ref(&self) -> &GetCurrentState { self }
}

impl AsRef<GetCurrentState> for RTDGetCurrentStateBuilder {
  fn as_ref(&self) -> &GetCurrentState { &self.inner }
}







/// Returns database statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDatabaseStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetDatabaseStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getDatabaseStatistics" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetDatabaseStatistics {}

impl GetDatabaseStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetDatabaseStatisticsBuilder {
    let mut inner = GetDatabaseStatistics::default();
    inner.td_name = "getDatabaseStatistics".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetDatabaseStatisticsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetDatabaseStatisticsBuilder {
  inner: GetDatabaseStatistics
}

impl RTDGetDatabaseStatisticsBuilder {
  pub fn build(&self) -> GetDatabaseStatistics { self.inner.clone() }

}

impl AsRef<GetDatabaseStatistics> for GetDatabaseStatistics {
  fn as_ref(&self) -> &GetDatabaseStatistics { self }
}

impl AsRef<GetDatabaseStatistics> for RTDGetDatabaseStatisticsBuilder {
  fn as_ref(&self) -> &GetDatabaseStatistics { &self.inner }
}







/// Returns information about a tg:// deep link. Use "tg://need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDeepLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The link
  link: String,
  
}

impl RObject for GetDeepLinkInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getDeepLinkInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetDeepLinkInfo {}

impl GetDeepLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetDeepLinkInfoBuilder {
    let mut inner = GetDeepLinkInfo::default();
    inner.td_name = "getDeepLinkInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetDeepLinkInfoBuilder { inner }
  }

  pub fn link(&self) -> &String { &self.link }

}

#[doc(hidden)]
pub struct RTDGetDeepLinkInfoBuilder {
  inner: GetDeepLinkInfo
}

impl RTDGetDeepLinkInfoBuilder {
  pub fn build(&self) -> GetDeepLinkInfo { self.inner.clone() }

   
  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }

}

impl AsRef<GetDeepLinkInfo> for GetDeepLinkInfo {
  fn as_ref(&self) -> &GetDeepLinkInfo { self }
}

impl AsRef<GetDeepLinkInfo> for RTDGetDeepLinkInfoBuilder {
  fn as_ref(&self) -> &GetDeepLinkInfo { &self.inner }
}







/// Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEmojiSuggestionsUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Language code for which the emoji replacements will be suggested
  language_code: String,
  
}

impl RObject for GetEmojiSuggestionsUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getEmojiSuggestionsUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetEmojiSuggestionsUrl {}

impl GetEmojiSuggestionsUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetEmojiSuggestionsUrlBuilder {
    let mut inner = GetEmojiSuggestionsUrl::default();
    inner.td_name = "getEmojiSuggestionsUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetEmojiSuggestionsUrlBuilder { inner }
  }

  pub fn language_code(&self) -> &String { &self.language_code }

}

#[doc(hidden)]
pub struct RTDGetEmojiSuggestionsUrlBuilder {
  inner: GetEmojiSuggestionsUrl
}

impl RTDGetEmojiSuggestionsUrlBuilder {
  pub fn build(&self) -> GetEmojiSuggestionsUrl { self.inner.clone() }

   
  pub fn language_code<T: AsRef<str>>(&mut self, language_code: T) -> &mut Self {
    self.inner.language_code = language_code.as_ref().to_string();
    self
  }

}

impl AsRef<GetEmojiSuggestionsUrl> for GetEmojiSuggestionsUrl {
  fn as_ref(&self) -> &GetEmojiSuggestionsUrl { self }
}

impl AsRef<GetEmojiSuggestionsUrl> for RTDGetEmojiSuggestionsUrlBuilder {
  fn as_ref(&self) -> &GetEmojiSuggestionsUrl { &self.inner }
}







/// Returns favorite stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFavoriteStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetFavoriteStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFavoriteStickers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetFavoriteStickers {}

impl GetFavoriteStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetFavoriteStickersBuilder {
    let mut inner = GetFavoriteStickers::default();
    inner.td_name = "getFavoriteStickers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetFavoriteStickersBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetFavoriteStickersBuilder {
  inner: GetFavoriteStickers
}

impl RTDGetFavoriteStickersBuilder {
  pub fn build(&self) -> GetFavoriteStickers { self.inner.clone() }

}

impl AsRef<GetFavoriteStickers> for GetFavoriteStickers {
  fn as_ref(&self) -> &GetFavoriteStickers { self }
}

impl AsRef<GetFavoriteStickers> for RTDGetFavoriteStickersBuilder {
  fn as_ref(&self) -> &GetFavoriteStickers { &self.inner }
}







/// Returns information about a file; this is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the file to get
  file_id: i64,
  
}

impl RObject for GetFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetFile {}

impl GetFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetFileBuilder {
    let mut inner = GetFile::default();
    inner.td_name = "getFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetFileBuilder { inner }
  }

  pub fn file_id(&self) -> i64 { self.file_id }

}

#[doc(hidden)]
pub struct RTDGetFileBuilder {
  inner: GetFile
}

impl RTDGetFileBuilder {
  pub fn build(&self) -> GetFile { self.inner.clone() }

   
  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

}

impl AsRef<GetFile> for GetFile {
  fn as_ref(&self) -> &GetFile { self }
}

impl AsRef<GetFile> for RTDGetFileBuilder {
  fn as_ref(&self) -> &GetFile { &self.inner }
}







/// Returns file downloaded prefix size from a given offset
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFileDownloadedPrefixSize {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the file
  file_id: i64,
  /// Offset from which downloaded prefix size should be calculated
  offset: i64,
  
}

impl RObject for GetFileDownloadedPrefixSize {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFileDownloadedPrefixSize" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetFileDownloadedPrefixSize {}

impl GetFileDownloadedPrefixSize {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetFileDownloadedPrefixSizeBuilder {
    let mut inner = GetFileDownloadedPrefixSize::default();
    inner.td_name = "getFileDownloadedPrefixSize".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetFileDownloadedPrefixSizeBuilder { inner }
  }

  pub fn file_id(&self) -> i64 { self.file_id }

  pub fn offset(&self) -> i64 { self.offset }

}

#[doc(hidden)]
pub struct RTDGetFileDownloadedPrefixSizeBuilder {
  inner: GetFileDownloadedPrefixSize
}

impl RTDGetFileDownloadedPrefixSizeBuilder {
  pub fn build(&self) -> GetFileDownloadedPrefixSize { self.inner.clone() }

   
  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

}

impl AsRef<GetFileDownloadedPrefixSize> for GetFileDownloadedPrefixSize {
  fn as_ref(&self) -> &GetFileDownloadedPrefixSize { self }
}

impl AsRef<GetFileDownloadedPrefixSize> for RTDGetFileDownloadedPrefixSizeBuilder {
  fn as_ref(&self) -> &GetFileDownloadedPrefixSize { &self.inner }
}







/// Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFileExtension {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The MIME type of the file
  mime_type: String,
  
}

impl RObject for GetFileExtension {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFileExtension" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetFileExtension {}

impl GetFileExtension {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetFileExtensionBuilder {
    let mut inner = GetFileExtension::default();
    inner.td_name = "getFileExtension".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetFileExtensionBuilder { inner }
  }

  pub fn mime_type(&self) -> &String { &self.mime_type }

}

#[doc(hidden)]
pub struct RTDGetFileExtensionBuilder {
  inner: GetFileExtension
}

impl RTDGetFileExtensionBuilder {
  pub fn build(&self) -> GetFileExtension { self.inner.clone() }

   
  pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
    self.inner.mime_type = mime_type.as_ref().to_string();
    self
  }

}

impl AsRef<GetFileExtension> for GetFileExtension {
  fn as_ref(&self) -> &GetFileExtension { self }
}

impl AsRef<GetFileExtension> for RTDGetFileExtensionBuilder {
  fn as_ref(&self) -> &GetFileExtension { &self.inner }
}







/// Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFileMimeType {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The name of the file or path to the file
  file_name: String,
  
}

impl RObject for GetFileMimeType {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFileMimeType" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetFileMimeType {}

impl GetFileMimeType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetFileMimeTypeBuilder {
    let mut inner = GetFileMimeType::default();
    inner.td_name = "getFileMimeType".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetFileMimeTypeBuilder { inner }
  }

  pub fn file_name(&self) -> &String { &self.file_name }

}

#[doc(hidden)]
pub struct RTDGetFileMimeTypeBuilder {
  inner: GetFileMimeType
}

impl RTDGetFileMimeTypeBuilder {
  pub fn build(&self) -> GetFileMimeType { self.inner.clone() }

   
  pub fn file_name<T: AsRef<str>>(&mut self, file_name: T) -> &mut Self {
    self.inner.file_name = file_name.as_ref().to_string();
    self
  }

}

impl AsRef<GetFileMimeType> for GetFileMimeType {
  fn as_ref(&self) -> &GetFileMimeType { self }
}

impl AsRef<GetFileMimeType> for RTDGetFileMimeTypeBuilder {
  fn as_ref(&self) -> &GetFileMimeType { &self.inner }
}







/// Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGameHighScores {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat that contains the message with the game
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// User identifier
  user_id: i64,
  
}

impl RObject for GetGameHighScores {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getGameHighScores" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetGameHighScores {}

impl GetGameHighScores {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetGameHighScoresBuilder {
    let mut inner = GetGameHighScores::default();
    inner.td_name = "getGameHighScores".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetGameHighScoresBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDGetGameHighScoresBuilder {
  inner: GetGameHighScores
}

impl RTDGetGameHighScoresBuilder {
  pub fn build(&self) -> GetGameHighScores { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<GetGameHighScores> for GetGameHighScores {
  fn as_ref(&self) -> &GetGameHighScores { self }
}

impl AsRef<GetGameHighScores> for RTDGetGameHighScoresBuilder {
  fn as_ref(&self) -> &GetGameHighScores { &self.inner }
}







/// Returns a list of common group chats with a given user. Chats are sorted by their type and creation date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupsInCommon {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// Chat identifier starting from which to return chats; use 0 for the first request
  offset_chat_id: i64,
  /// The maximum number of chats to be returned; up to 100
  limit: i64,
  
}

impl RObject for GetGroupsInCommon {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getGroupsInCommon" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetGroupsInCommon {}

impl GetGroupsInCommon {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetGroupsInCommonBuilder {
    let mut inner = GetGroupsInCommon::default();
    inner.td_name = "getGroupsInCommon".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetGroupsInCommonBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn offset_chat_id(&self) -> i64 { self.offset_chat_id }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetGroupsInCommonBuilder {
  inner: GetGroupsInCommon
}

impl RTDGetGroupsInCommonBuilder {
  pub fn build(&self) -> GetGroupsInCommon { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
    self.inner.offset_chat_id = offset_chat_id;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetGroupsInCommon> for GetGroupsInCommon {
  fn as_ref(&self) -> &GetGroupsInCommon { self }
}

impl AsRef<GetGroupsInCommon> for RTDGetGroupsInCommonBuilder {
  fn as_ref(&self) -> &GetGroupsInCommon { &self.inner }
}







/// Returns the total number of imported contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetImportedContactCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetImportedContactCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getImportedContactCount" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetImportedContactCount {}

impl GetImportedContactCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetImportedContactCountBuilder {
    let mut inner = GetImportedContactCount::default();
    inner.td_name = "getImportedContactCount".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetImportedContactCountBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetImportedContactCountBuilder {
  inner: GetImportedContactCount
}

impl RTDGetImportedContactCountBuilder {
  pub fn build(&self) -> GetImportedContactCount { self.inner.clone() }

}

impl AsRef<GetImportedContactCount> for GetImportedContactCount {
  fn as_ref(&self) -> &GetImportedContactCount { self }
}

impl AsRef<GetImportedContactCount> for RTDGetImportedContactCountBuilder {
  fn as_ref(&self) -> &GetImportedContactCount { &self.inner }
}







/// Returns a list of recently inactive supergroups and channels. Can be used when user reaches limit on the number of joined supergroups and channels and receives CHANNELS_TOO_MUCH error
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInactiveSupergroupChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetInactiveSupergroupChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInactiveSupergroupChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetInactiveSupergroupChats {}

impl GetInactiveSupergroupChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetInactiveSupergroupChatsBuilder {
    let mut inner = GetInactiveSupergroupChats::default();
    inner.td_name = "getInactiveSupergroupChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetInactiveSupergroupChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetInactiveSupergroupChatsBuilder {
  inner: GetInactiveSupergroupChats
}

impl RTDGetInactiveSupergroupChatsBuilder {
  pub fn build(&self) -> GetInactiveSupergroupChats { self.inner.clone() }

}

impl AsRef<GetInactiveSupergroupChats> for GetInactiveSupergroupChats {
  fn as_ref(&self) -> &GetInactiveSupergroupChats { self }
}

impl AsRef<GetInactiveSupergroupChats> for RTDGetInactiveSupergroupChatsBuilder {
  fn as_ref(&self) -> &GetInactiveSupergroupChats { &self.inner }
}







/// Returns game high scores and some part of the high score table in the range of the specified user; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInlineGameHighScores {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Inline message identifier
  inline_message_id: String,
  /// User identifier
  user_id: i64,
  
}

impl RObject for GetInlineGameHighScores {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInlineGameHighScores" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetInlineGameHighScores {}

impl GetInlineGameHighScores {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetInlineGameHighScoresBuilder {
    let mut inner = GetInlineGameHighScores::default();
    inner.td_name = "getInlineGameHighScores".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetInlineGameHighScoresBuilder { inner }
  }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDGetInlineGameHighScoresBuilder {
  inner: GetInlineGameHighScores
}

impl RTDGetInlineGameHighScoresBuilder {
  pub fn build(&self) -> GetInlineGameHighScores { self.inner.clone() }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<GetInlineGameHighScores> for GetInlineGameHighScores {
  fn as_ref(&self) -> &GetInlineGameHighScores { self }
}

impl AsRef<GetInlineGameHighScores> for RTDGetInlineGameHighScoresBuilder {
  fn as_ref(&self) -> &GetInlineGameHighScores { &self.inner }
}







/// Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInlineQueryResults {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the target bot
  bot_user_id: i64,
  /// Identifier of the chat where the query was sent
  chat_id: i64,
  /// Location of the user, only if needed
  user_location: Location,
  /// Text of the query
  query: String,
  /// Offset of the first entry to return
  offset: String,
  
}

impl RObject for GetInlineQueryResults {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInlineQueryResults" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetInlineQueryResults {}

impl GetInlineQueryResults {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetInlineQueryResultsBuilder {
    let mut inner = GetInlineQueryResults::default();
    inner.td_name = "getInlineQueryResults".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetInlineQueryResultsBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 { self.bot_user_id }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn user_location(&self) -> &Location { &self.user_location }

  pub fn query(&self) -> &String { &self.query }

  pub fn offset(&self) -> &String { &self.offset }

}

#[doc(hidden)]
pub struct RTDGetInlineQueryResultsBuilder {
  inner: GetInlineQueryResults
}

impl RTDGetInlineQueryResultsBuilder {
  pub fn build(&self) -> GetInlineQueryResults { self.inner.clone() }

   
  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn user_location<T: AsRef<Location>>(&mut self, user_location: T) -> &mut Self {
    self.inner.user_location = user_location.as_ref().clone();
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
    self.inner.offset = offset.as_ref().to_string();
    self
  }

}

impl AsRef<GetInlineQueryResults> for GetInlineQueryResults {
  fn as_ref(&self) -> &GetInlineQueryResults { self }
}

impl AsRef<GetInlineQueryResults> for RTDGetInlineQueryResultsBuilder {
  fn as_ref(&self) -> &GetInlineQueryResults { &self.inner }
}







/// Returns a list of installed sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstalledStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to return mask sticker sets; pass false to return ordinary sticker sets
  is_masks: bool,
  
}

impl RObject for GetInstalledStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInstalledStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetInstalledStickerSets {}

impl GetInstalledStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetInstalledStickerSetsBuilder {
    let mut inner = GetInstalledStickerSets::default();
    inner.td_name = "getInstalledStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetInstalledStickerSetsBuilder { inner }
  }

  pub fn is_masks(&self) -> bool { self.is_masks }

}

#[doc(hidden)]
pub struct RTDGetInstalledStickerSetsBuilder {
  inner: GetInstalledStickerSets
}

impl RTDGetInstalledStickerSetsBuilder {
  pub fn build(&self) -> GetInstalledStickerSets { self.inner.clone() }

   
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.is_masks = is_masks;
    self
  }

}

impl AsRef<GetInstalledStickerSets> for GetInstalledStickerSets {
  fn as_ref(&self) -> &GetInstalledStickerSets { self }
}

impl AsRef<GetInstalledStickerSets> for RTDGetInstalledStickerSetsBuilder {
  fn as_ref(&self) -> &GetInstalledStickerSets { &self.inner }
}







/// Returns the default text for invitation messages to be used as a placeholder when the current user invites friends to Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInviteText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetInviteText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInviteText" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetInviteText {}

impl GetInviteText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetInviteTextBuilder {
    let mut inner = GetInviteText::default();
    inner.td_name = "getInviteText".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetInviteTextBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetInviteTextBuilder {
  inner: GetInviteText
}

impl RTDGetInviteTextBuilder {
  pub fn build(&self) -> GetInviteText { self.inner.clone() }

}

impl AsRef<GetInviteText> for GetInviteText {
  fn as_ref(&self) -> &GetInviteText { self }
}

impl AsRef<GetInviteText> for RTDGetInviteTextBuilder {
  fn as_ref(&self) -> &GetInviteText { &self.inner }
}







/// Converts a JsonValue object to corresponding JSON-serialized string. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetJsonString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The JsonValue object
  json_value: JsonValue,
  
}

impl RObject for GetJsonString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getJsonString" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetJsonString {}

impl GetJsonString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetJsonStringBuilder {
    let mut inner = GetJsonString::default();
    inner.td_name = "getJsonString".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetJsonStringBuilder { inner }
  }

  pub fn json_value(&self) -> &JsonValue { &self.json_value }

}

#[doc(hidden)]
pub struct RTDGetJsonStringBuilder {
  inner: GetJsonString
}

impl RTDGetJsonStringBuilder {
  pub fn build(&self) -> GetJsonString { self.inner.clone() }

   
  pub fn json_value<T: AsRef<JsonValue>>(&mut self, json_value: T) -> &mut Self {
    self.inner.json_value = json_value.as_ref().clone();
    self
  }

}

impl AsRef<GetJsonString> for GetJsonString {
  fn as_ref(&self) -> &GetJsonString { self }
}

impl AsRef<GetJsonString> for RTDGetJsonStringBuilder {
  fn as_ref(&self) -> &GetJsonString { &self.inner }
}







/// Converts a JSON-serialized string to corresponding JsonValue object. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetJsonValue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The JSON-serialized string
  json: String,
  
}

impl RObject for GetJsonValue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getJsonValue" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDJsonValue for GetJsonValue {}

impl RFunction for GetJsonValue {}

impl GetJsonValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetJsonValueBuilder {
    let mut inner = GetJsonValue::default();
    inner.td_name = "getJsonValue".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetJsonValueBuilder { inner }
  }

  pub fn json(&self) -> &String { &self.json }

}

#[doc(hidden)]
pub struct RTDGetJsonValueBuilder {
  inner: GetJsonValue
}

impl RTDGetJsonValueBuilder {
  pub fn build(&self) -> GetJsonValue { self.inner.clone() }

   
  pub fn json<T: AsRef<str>>(&mut self, json: T) -> &mut Self {
    self.inner.json = json.as_ref().to_string();
    self
  }

}

impl AsRef<GetJsonValue> for GetJsonValue {
  fn as_ref(&self) -> &GetJsonValue { self }
}

impl AsRef<GetJsonValue> for RTDGetJsonValueBuilder {
  fn as_ref(&self) -> &GetJsonValue { &self.inner }
}







/// Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLanguagePackInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Language pack identifier
  language_pack_id: String,
  
}

impl RObject for GetLanguagePackInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLanguagePackInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetLanguagePackInfo {}

impl GetLanguagePackInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLanguagePackInfoBuilder {
    let mut inner = GetLanguagePackInfo::default();
    inner.td_name = "getLanguagePackInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLanguagePackInfoBuilder { inner }
  }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

}

#[doc(hidden)]
pub struct RTDGetLanguagePackInfoBuilder {
  inner: GetLanguagePackInfo
}

impl RTDGetLanguagePackInfoBuilder {
  pub fn build(&self) -> GetLanguagePackInfo { self.inner.clone() }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

}

impl AsRef<GetLanguagePackInfo> for GetLanguagePackInfo {
  fn as_ref(&self) -> &GetLanguagePackInfo { self }
}

impl AsRef<GetLanguagePackInfo> for RTDGetLanguagePackInfoBuilder {
  fn as_ref(&self) -> &GetLanguagePackInfo { &self.inner }
}







/// Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLanguagePackString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Path to the language pack database in which strings are stored
  language_pack_database_path: String,
  /// Localization target to which the language pack belongs
  localization_target: String,
  /// Language pack identifier
  language_pack_id: String,
  /// Language pack key of the string to be returned
  key: String,
  
}

impl RObject for GetLanguagePackString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLanguagePackString" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLanguagePackStringValue for GetLanguagePackString {}

impl RFunction for GetLanguagePackString {}

impl GetLanguagePackString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLanguagePackStringBuilder {
    let mut inner = GetLanguagePackString::default();
    inner.td_name = "getLanguagePackString".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLanguagePackStringBuilder { inner }
  }

  pub fn language_pack_database_path(&self) -> &String { &self.language_pack_database_path }

  pub fn localization_target(&self) -> &String { &self.localization_target }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

  pub fn key(&self) -> &String { &self.key }

}

#[doc(hidden)]
pub struct RTDGetLanguagePackStringBuilder {
  inner: GetLanguagePackString
}

impl RTDGetLanguagePackStringBuilder {
  pub fn build(&self) -> GetLanguagePackString { self.inner.clone() }

   
  pub fn language_pack_database_path<T: AsRef<str>>(&mut self, language_pack_database_path: T) -> &mut Self {
    self.inner.language_pack_database_path = language_pack_database_path.as_ref().to_string();
    self
  }

   
  pub fn localization_target<T: AsRef<str>>(&mut self, localization_target: T) -> &mut Self {
    self.inner.localization_target = localization_target.as_ref().to_string();
    self
  }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

   
  pub fn key<T: AsRef<str>>(&mut self, key: T) -> &mut Self {
    self.inner.key = key.as_ref().to_string();
    self
  }

}

impl AsRef<GetLanguagePackString> for GetLanguagePackString {
  fn as_ref(&self) -> &GetLanguagePackString { self }
}

impl AsRef<GetLanguagePackString> for RTDGetLanguagePackStringBuilder {
  fn as_ref(&self) -> &GetLanguagePackString { &self.inner }
}







/// Returns strings from a language pack in the current localization target by their keys. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLanguagePackStrings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Language pack identifier of the strings to be returned
  language_pack_id: String,
  /// Language pack keys of the strings to be returned; leave empty to request all available strings
  keys: Vec<String>,
  
}

impl RObject for GetLanguagePackStrings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLanguagePackStrings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetLanguagePackStrings {}

impl GetLanguagePackStrings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLanguagePackStringsBuilder {
    let mut inner = GetLanguagePackStrings::default();
    inner.td_name = "getLanguagePackStrings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLanguagePackStringsBuilder { inner }
  }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

  pub fn keys(&self) -> &Vec<String> { &self.keys }

}

#[doc(hidden)]
pub struct RTDGetLanguagePackStringsBuilder {
  inner: GetLanguagePackStrings
}

impl RTDGetLanguagePackStringsBuilder {
  pub fn build(&self) -> GetLanguagePackStrings { self.inner.clone() }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

   
  pub fn keys(&mut self, keys: Vec<String>) -> &mut Self {
    self.inner.keys = keys;
    self
  }

}

impl AsRef<GetLanguagePackStrings> for GetLanguagePackStrings {
  fn as_ref(&self) -> &GetLanguagePackStrings { self }
}

impl AsRef<GetLanguagePackStrings> for RTDGetLanguagePackStringsBuilder {
  fn as_ref(&self) -> &GetLanguagePackStrings { &self.inner }
}







/// Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLocalizationTargetInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// If true, returns only locally available information without sending network requests
  only_local: bool,
  
}

impl RObject for GetLocalizationTargetInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLocalizationTargetInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetLocalizationTargetInfo {}

impl GetLocalizationTargetInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLocalizationTargetInfoBuilder {
    let mut inner = GetLocalizationTargetInfo::default();
    inner.td_name = "getLocalizationTargetInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLocalizationTargetInfoBuilder { inner }
  }

  pub fn only_local(&self) -> bool { self.only_local }

}

#[doc(hidden)]
pub struct RTDGetLocalizationTargetInfoBuilder {
  inner: GetLocalizationTargetInfo
}

impl RTDGetLocalizationTargetInfoBuilder {
  pub fn build(&self) -> GetLocalizationTargetInfo { self.inner.clone() }

   
  pub fn only_local(&mut self, only_local: bool) -> &mut Self {
    self.inner.only_local = only_local;
    self
  }

}

impl AsRef<GetLocalizationTargetInfo> for GetLocalizationTargetInfo {
  fn as_ref(&self) -> &GetLocalizationTargetInfo { self }
}

impl AsRef<GetLocalizationTargetInfo> for RTDGetLocalizationTargetInfoBuilder {
  fn as_ref(&self) -> &GetLocalizationTargetInfo { &self.inner }
}







/// Returns information about currently used log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogStream {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetLogStream {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLogStream" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLogStream for GetLogStream {}

impl RFunction for GetLogStream {}

impl GetLogStream {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLogStreamBuilder {
    let mut inner = GetLogStream::default();
    inner.td_name = "getLogStream".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLogStreamBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetLogStreamBuilder {
  inner: GetLogStream
}

impl RTDGetLogStreamBuilder {
  pub fn build(&self) -> GetLogStream { self.inner.clone() }

}

impl AsRef<GetLogStream> for GetLogStream {
  fn as_ref(&self) -> &GetLogStream { self }
}

impl AsRef<GetLogStream> for RTDGetLogStreamBuilder {
  fn as_ref(&self) -> &GetLogStream { &self.inner }
}







/// Returns current verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogTagVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Logging tag to change verbosity level
  tag: String,
  
}

impl RObject for GetLogTagVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLogTagVerbosityLevel" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetLogTagVerbosityLevel {}

impl GetLogTagVerbosityLevel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLogTagVerbosityLevelBuilder {
    let mut inner = GetLogTagVerbosityLevel::default();
    inner.td_name = "getLogTagVerbosityLevel".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLogTagVerbosityLevelBuilder { inner }
  }

  pub fn tag(&self) -> &String { &self.tag }

}

#[doc(hidden)]
pub struct RTDGetLogTagVerbosityLevelBuilder {
  inner: GetLogTagVerbosityLevel
}

impl RTDGetLogTagVerbosityLevelBuilder {
  pub fn build(&self) -> GetLogTagVerbosityLevel { self.inner.clone() }

   
  pub fn tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.tag = tag.as_ref().to_string();
    self
  }

}

impl AsRef<GetLogTagVerbosityLevel> for GetLogTagVerbosityLevel {
  fn as_ref(&self) -> &GetLogTagVerbosityLevel { self }
}

impl AsRef<GetLogTagVerbosityLevel> for RTDGetLogTagVerbosityLevelBuilder {
  fn as_ref(&self) -> &GetLogTagVerbosityLevel { &self.inner }
}







/// Returns list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogTags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetLogTags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLogTags" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetLogTags {}

impl GetLogTags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLogTagsBuilder {
    let mut inner = GetLogTags::default();
    inner.td_name = "getLogTags".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLogTagsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetLogTagsBuilder {
  inner: GetLogTags
}

impl RTDGetLogTagsBuilder {
  pub fn build(&self) -> GetLogTags { self.inner.clone() }

}

impl AsRef<GetLogTags> for GetLogTags {
  fn as_ref(&self) -> &GetLogTags { self }
}

impl AsRef<GetLogTags> for RTDGetLogTagsBuilder {
  fn as_ref(&self) -> &GetLogTags { &self.inner }
}







/// Returns current verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetLogVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLogVerbosityLevel" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetLogVerbosityLevel {}

impl GetLogVerbosityLevel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLogVerbosityLevelBuilder {
    let mut inner = GetLogVerbosityLevel::default();
    inner.td_name = "getLogVerbosityLevel".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLogVerbosityLevelBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetLogVerbosityLevelBuilder {
  inner: GetLogVerbosityLevel
}

impl RTDGetLogVerbosityLevelBuilder {
  pub fn build(&self) -> GetLogVerbosityLevel { self.inner.clone() }

}

impl AsRef<GetLogVerbosityLevel> for GetLogVerbosityLevel {
  fn as_ref(&self) -> &GetLogVerbosityLevel { self }
}

impl AsRef<GetLogVerbosityLevel> for RTDGetLogVerbosityLevelBuilder {
  fn as_ref(&self) -> &GetLogVerbosityLevel { &self.inner }
}







/// Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl. Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLoginUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the message with the button
  chat_id: i64,
  /// Message identifier of the message with the button
  message_id: i64,
  /// Button identifier
  button_id: i64,
  /// True, if the user allowed the bot to send them messages
  allow_write_access: bool,
  
}

impl RObject for GetLoginUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLoginUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetLoginUrl {}

impl GetLoginUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLoginUrlBuilder {
    let mut inner = GetLoginUrl::default();
    inner.td_name = "getLoginUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLoginUrlBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn button_id(&self) -> i64 { self.button_id }

  pub fn allow_write_access(&self) -> bool { self.allow_write_access }

}

#[doc(hidden)]
pub struct RTDGetLoginUrlBuilder {
  inner: GetLoginUrl
}

impl RTDGetLoginUrlBuilder {
  pub fn build(&self) -> GetLoginUrl { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn button_id(&mut self, button_id: i64) -> &mut Self {
    self.inner.button_id = button_id;
    self
  }

   
  pub fn allow_write_access(&mut self, allow_write_access: bool) -> &mut Self {
    self.inner.allow_write_access = allow_write_access;
    self
  }

}

impl AsRef<GetLoginUrl> for GetLoginUrl {
  fn as_ref(&self) -> &GetLoginUrl { self }
}

impl AsRef<GetLoginUrl> for RTDGetLoginUrlBuilder {
  fn as_ref(&self) -> &GetLoginUrl { &self.inner }
}







/// Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLoginUrlInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the message with the button
  chat_id: i64,
  /// Message identifier of the message with the button
  message_id: i64,
  /// Button identifier
  button_id: i64,
  
}

impl RObject for GetLoginUrlInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLoginUrlInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLoginUrlInfo for GetLoginUrlInfo {}

impl RFunction for GetLoginUrlInfo {}

impl GetLoginUrlInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetLoginUrlInfoBuilder {
    let mut inner = GetLoginUrlInfo::default();
    inner.td_name = "getLoginUrlInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetLoginUrlInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn button_id(&self) -> i64 { self.button_id }

}

#[doc(hidden)]
pub struct RTDGetLoginUrlInfoBuilder {
  inner: GetLoginUrlInfo
}

impl RTDGetLoginUrlInfoBuilder {
  pub fn build(&self) -> GetLoginUrlInfo { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn button_id(&mut self, button_id: i64) -> &mut Self {
    self.inner.button_id = button_id;
    self
  }

}

impl AsRef<GetLoginUrlInfo> for GetLoginUrlInfo {
  fn as_ref(&self) -> &GetLoginUrlInfo { self }
}

impl AsRef<GetLoginUrlInfo> for RTDGetLoginUrlInfoBuilder {
  fn as_ref(&self) -> &GetLoginUrlInfo { &self.inner }
}







/// Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMapThumbnailFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Location of the map center
  location: Location,
  /// Map zoom level; 13-20
  zoom: i64,
  /// Map width in pixels before applying scale; 16-1024
  width: i64,
  /// Map height in pixels before applying scale; 16-1024
  height: i64,
  /// Map scale; 1-3
  scale: i64,
  /// Identifier of a chat, in which the thumbnail will be shown. Use 0 if unknown
  chat_id: i64,
  
}

impl RObject for GetMapThumbnailFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMapThumbnailFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetMapThumbnailFile {}

impl GetMapThumbnailFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetMapThumbnailFileBuilder {
    let mut inner = GetMapThumbnailFile::default();
    inner.td_name = "getMapThumbnailFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetMapThumbnailFileBuilder { inner }
  }

  pub fn location(&self) -> &Location { &self.location }

  pub fn zoom(&self) -> i64 { self.zoom }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn scale(&self) -> i64 { self.scale }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDGetMapThumbnailFileBuilder {
  inner: GetMapThumbnailFile
}

impl RTDGetMapThumbnailFileBuilder {
  pub fn build(&self) -> GetMapThumbnailFile { self.inner.clone() }

   
  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

   
  pub fn zoom(&mut self, zoom: i64) -> &mut Self {
    self.inner.zoom = zoom;
    self
  }

   
  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

   
  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

   
  pub fn scale(&mut self, scale: i64) -> &mut Self {
    self.inner.scale = scale;
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<GetMapThumbnailFile> for GetMapThumbnailFile {
  fn as_ref(&self) -> &GetMapThumbnailFile { self }
}

impl AsRef<GetMapThumbnailFile> for RTDGetMapThumbnailFileBuilder {
  fn as_ref(&self) -> &GetMapThumbnailFile { &self.inner }
}







/// Returns the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMe {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetMe {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMe" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetMe {}

impl GetMe {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetMeBuilder {
    let mut inner = GetMe::default();
    inner.td_name = "getMe".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetMeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetMeBuilder {
  inner: GetMe
}

impl RTDGetMeBuilder {
  pub fn build(&self) -> GetMe { self.inner.clone() }

}

impl AsRef<GetMe> for GetMe {
  fn as_ref(&self) -> &GetMe { self }
}

impl AsRef<GetMe> for RTDGetMeBuilder {
  fn as_ref(&self) -> &GetMe { &self.inner }
}







/// Returns information about a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat the message belongs to
  chat_id: i64,
  /// Identifier of the message to get
  message_id: i64,
  
}

impl RObject for GetMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetMessage {}

impl GetMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetMessageBuilder {
    let mut inner = GetMessage::default();
    inner.td_name = "getMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDGetMessageBuilder {
  inner: GetMessage
}

impl RTDGetMessageBuilder {
  pub fn build(&self) -> GetMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<GetMessage> for GetMessage {
  fn as_ref(&self) -> &GetMessage { self }
}

impl AsRef<GetMessage> for RTDGetMessageBuilder {
  fn as_ref(&self) -> &GetMessage { &self.inner }
}







/// Returns a private HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels. The link will work only for members of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to which the message belongs
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  
}

impl RObject for GetMessageLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMessageLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetMessageLink {}

impl GetMessageLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetMessageLinkBuilder {
    let mut inner = GetMessageLink::default();
    inner.td_name = "getMessageLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetMessageLinkBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDGetMessageLinkBuilder {
  inner: GetMessageLink
}

impl RTDGetMessageLinkBuilder {
  pub fn build(&self) -> GetMessageLink { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<GetMessageLink> for GetMessageLink {
  fn as_ref(&self) -> &GetMessageLink { self }
}

impl AsRef<GetMessageLink> for RTDGetMessageLinkBuilder {
  fn as_ref(&self) -> &GetMessageLink { &self.inner }
}







/// Returns information about a public or private message link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The message link in the format "https://t.me/c/...", or "tg://privatepost?...", or "https://t.me/username/...", or "tg://resolve?..."
  url: String,
  
}

impl RObject for GetMessageLinkInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMessageLinkInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetMessageLinkInfo {}

impl GetMessageLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetMessageLinkInfoBuilder {
    let mut inner = GetMessageLinkInfo::default();
    inner.td_name = "getMessageLinkInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetMessageLinkInfoBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

}

#[doc(hidden)]
pub struct RTDGetMessageLinkInfoBuilder {
  inner: GetMessageLinkInfo
}

impl RTDGetMessageLinkInfoBuilder {
  pub fn build(&self) -> GetMessageLinkInfo { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

}

impl AsRef<GetMessageLinkInfo> for GetMessageLinkInfo {
  fn as_ref(&self) -> &GetMessageLinkInfo { self }
}

impl AsRef<GetMessageLinkInfo> for RTDGetMessageLinkInfoBuilder {
  fn as_ref(&self) -> &GetMessageLinkInfo { &self.inner }
}







/// Returns information about a message, if it is available locally without sending network request. This is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageLocally {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat the message belongs to
  chat_id: i64,
  /// Identifier of the message to get
  message_id: i64,
  
}

impl RObject for GetMessageLocally {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMessageLocally" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetMessageLocally {}

impl GetMessageLocally {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetMessageLocallyBuilder {
    let mut inner = GetMessageLocally::default();
    inner.td_name = "getMessageLocally".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetMessageLocallyBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDGetMessageLocallyBuilder {
  inner: GetMessageLocally
}

impl RTDGetMessageLocallyBuilder {
  pub fn build(&self) -> GetMessageLocally { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<GetMessageLocally> for GetMessageLocally {
  fn as_ref(&self) -> &GetMessageLocally { self }
}

impl AsRef<GetMessageLocally> for RTDGetMessageLocallyBuilder {
  fn as_ref(&self) -> &GetMessageLocally { &self.inner }
}







/// Returns information about messages. If a message is not found, returns null on the corresponding position of the result
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat the messages belong to
  chat_id: i64,
  /// Identifiers of the messages to get
  message_ids: Vec<i64>,
  
}

impl RObject for GetMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetMessages {}

impl GetMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetMessagesBuilder {
    let mut inner = GetMessages::default();
    inner.td_name = "getMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_ids(&self) -> &Vec<i64> { &self.message_ids }

}

#[doc(hidden)]
pub struct RTDGetMessagesBuilder {
  inner: GetMessages
}

impl RTDGetMessagesBuilder {
  pub fn build(&self) -> GetMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }

}

impl AsRef<GetMessages> for GetMessages {
  fn as_ref(&self) -> &GetMessages { self }
}

impl AsRef<GetMessages> for RTDGetMessagesBuilder {
  fn as_ref(&self) -> &GetMessages { &self.inner }
}







/// Returns network data usage statistics. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetNetworkStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// If true, returns only data for the current library launch
  only_current: bool,
  
}

impl RObject for GetNetworkStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getNetworkStatistics" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetNetworkStatistics {}

impl GetNetworkStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetNetworkStatisticsBuilder {
    let mut inner = GetNetworkStatistics::default();
    inner.td_name = "getNetworkStatistics".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetNetworkStatisticsBuilder { inner }
  }

  pub fn only_current(&self) -> bool { self.only_current }

}

#[doc(hidden)]
pub struct RTDGetNetworkStatisticsBuilder {
  inner: GetNetworkStatistics
}

impl RTDGetNetworkStatisticsBuilder {
  pub fn build(&self) -> GetNetworkStatistics { self.inner.clone() }

   
  pub fn only_current(&mut self, only_current: bool) -> &mut Self {
    self.inner.only_current = only_current;
    self
  }

}

impl AsRef<GetNetworkStatistics> for GetNetworkStatistics {
  fn as_ref(&self) -> &GetNetworkStatistics { self }
}

impl AsRef<GetNetworkStatistics> for RTDGetNetworkStatisticsBuilder {
  fn as_ref(&self) -> &GetNetworkStatistics { &self.inner }
}







/// Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The name of the option
  name: String,
  
}

impl RObject for GetOption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getOption" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDOptionValue for GetOption {}

impl RFunction for GetOption {}

impl GetOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetOptionBuilder {
    let mut inner = GetOption::default();
    inner.td_name = "getOption".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetOptionBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

}

#[doc(hidden)]
pub struct RTDGetOptionBuilder {
  inner: GetOption
}

impl RTDGetOptionBuilder {
  pub fn build(&self) -> GetOption { self.inner.clone() }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

}

impl AsRef<GetOption> for GetOption {
  fn as_ref(&self) -> &GetOption { self }
}

impl AsRef<GetOption> for RTDGetOptionBuilder {
  fn as_ref(&self) -> &GetOption { &self.inner }
}







/// Returns a Telegram Passport authorization form for sharing data with a service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPassportAuthorizationForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the service's bot
  bot_user_id: i64,
  /// Telegram Passport element types requested by the service
  scope: String,
  /// Service's public_key
  public_key: String,
  /// Authorization form nonce provided by the service
  nonce: String,
  
}

impl RObject for GetPassportAuthorizationForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPassportAuthorizationForm" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPassportAuthorizationForm {}

impl GetPassportAuthorizationForm {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPassportAuthorizationFormBuilder {
    let mut inner = GetPassportAuthorizationForm::default();
    inner.td_name = "getPassportAuthorizationForm".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPassportAuthorizationFormBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 { self.bot_user_id }

  pub fn scope(&self) -> &String { &self.scope }

  pub fn public_key(&self) -> &String { &self.public_key }

  pub fn nonce(&self) -> &String { &self.nonce }

}

#[doc(hidden)]
pub struct RTDGetPassportAuthorizationFormBuilder {
  inner: GetPassportAuthorizationForm
}

impl RTDGetPassportAuthorizationFormBuilder {
  pub fn build(&self) -> GetPassportAuthorizationForm { self.inner.clone() }

   
  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

   
  pub fn scope<T: AsRef<str>>(&mut self, scope: T) -> &mut Self {
    self.inner.scope = scope.as_ref().to_string();
    self
  }

   
  pub fn public_key<T: AsRef<str>>(&mut self, public_key: T) -> &mut Self {
    self.inner.public_key = public_key.as_ref().to_string();
    self
  }

   
  pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
    self.inner.nonce = nonce.as_ref().to_string();
    self
  }

}

impl AsRef<GetPassportAuthorizationForm> for GetPassportAuthorizationForm {
  fn as_ref(&self) -> &GetPassportAuthorizationForm { self }
}

impl AsRef<GetPassportAuthorizationForm> for RTDGetPassportAuthorizationFormBuilder {
  fn as_ref(&self) -> &GetPassportAuthorizationForm { &self.inner }
}







/// Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPassportAuthorizationFormAvailableElements {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Authorization form identifier
  autorization_form_id: i64,
  /// Password of the current user
  password: String,
  
}

impl RObject for GetPassportAuthorizationFormAvailableElements {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPassportAuthorizationFormAvailableElements" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPassportAuthorizationFormAvailableElements {}

impl GetPassportAuthorizationFormAvailableElements {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPassportAuthorizationFormAvailableElementsBuilder {
    let mut inner = GetPassportAuthorizationFormAvailableElements::default();
    inner.td_name = "getPassportAuthorizationFormAvailableElements".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPassportAuthorizationFormAvailableElementsBuilder { inner }
  }

  pub fn autorization_form_id(&self) -> i64 { self.autorization_form_id }

  pub fn password(&self) -> &String { &self.password }

}

#[doc(hidden)]
pub struct RTDGetPassportAuthorizationFormAvailableElementsBuilder {
  inner: GetPassportAuthorizationFormAvailableElements
}

impl RTDGetPassportAuthorizationFormAvailableElementsBuilder {
  pub fn build(&self) -> GetPassportAuthorizationFormAvailableElements { self.inner.clone() }

   
  pub fn autorization_form_id(&mut self, autorization_form_id: i64) -> &mut Self {
    self.inner.autorization_form_id = autorization_form_id;
    self
  }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

}

impl AsRef<GetPassportAuthorizationFormAvailableElements> for GetPassportAuthorizationFormAvailableElements {
  fn as_ref(&self) -> &GetPassportAuthorizationFormAvailableElements { self }
}

impl AsRef<GetPassportAuthorizationFormAvailableElements> for RTDGetPassportAuthorizationFormAvailableElementsBuilder {
  fn as_ref(&self) -> &GetPassportAuthorizationFormAvailableElements { &self.inner }
}







/// Returns one of the available Telegram Passport elements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPassportElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Telegram Passport element type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: PassportElementType,
  /// Password of the current user
  password: String,
  
}

impl RObject for GetPassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPassportElement" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElement for GetPassportElement {}

impl RFunction for GetPassportElement {}

impl GetPassportElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPassportElementBuilder {
    let mut inner = GetPassportElement::default();
    inner.td_name = "getPassportElement".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPassportElementBuilder { inner }
  }

  pub fn type_(&self) -> &PassportElementType { &self.type_ }

  pub fn password(&self) -> &String { &self.password }

}

#[doc(hidden)]
pub struct RTDGetPassportElementBuilder {
  inner: GetPassportElement
}

impl RTDGetPassportElementBuilder {
  pub fn build(&self) -> GetPassportElement { self.inner.clone() }

   
  pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

}

impl AsRef<GetPassportElement> for GetPassportElement {
  fn as_ref(&self) -> &GetPassportElement { self }
}

impl AsRef<GetPassportElement> for RTDGetPassportElementBuilder {
  fn as_ref(&self) -> &GetPassportElement { &self.inner }
}







/// Returns the current state of 2-step verification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPasswordState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetPasswordState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPasswordState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPasswordState {}

impl GetPasswordState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPasswordStateBuilder {
    let mut inner = GetPasswordState::default();
    inner.td_name = "getPasswordState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPasswordStateBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetPasswordStateBuilder {
  inner: GetPasswordState
}

impl RTDGetPasswordStateBuilder {
  pub fn build(&self) -> GetPasswordState { self.inner.clone() }

}

impl AsRef<GetPasswordState> for GetPasswordState {
  fn as_ref(&self) -> &GetPasswordState { self }
}

impl AsRef<GetPasswordState> for RTDGetPasswordStateBuilder {
  fn as_ref(&self) -> &GetPasswordState { &self.inner }
}







/// Returns an invoice payment form. This method should be called when the user presses inlineKeyboardButtonBuy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPaymentForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the Invoice message
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  
}

impl RObject for GetPaymentForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPaymentForm" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPaymentForm {}

impl GetPaymentForm {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPaymentFormBuilder {
    let mut inner = GetPaymentForm::default();
    inner.td_name = "getPaymentForm".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPaymentFormBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDGetPaymentFormBuilder {
  inner: GetPaymentForm
}

impl RTDGetPaymentFormBuilder {
  pub fn build(&self) -> GetPaymentForm { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<GetPaymentForm> for GetPaymentForm {
  fn as_ref(&self) -> &GetPaymentForm { self }
}

impl AsRef<GetPaymentForm> for RTDGetPaymentFormBuilder {
  fn as_ref(&self) -> &GetPaymentForm { &self.inner }
}







/// Returns information about a successful payment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPaymentReceipt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the PaymentSuccessful message
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  
}

impl RObject for GetPaymentReceipt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPaymentReceipt" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPaymentReceipt {}

impl GetPaymentReceipt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPaymentReceiptBuilder {
    let mut inner = GetPaymentReceipt::default();
    inner.td_name = "getPaymentReceipt".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPaymentReceiptBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDGetPaymentReceiptBuilder {
  inner: GetPaymentReceipt
}

impl RTDGetPaymentReceiptBuilder {
  pub fn build(&self) -> GetPaymentReceipt { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<GetPaymentReceipt> for GetPaymentReceipt {
  fn as_ref(&self) -> &GetPaymentReceipt { self }
}

impl AsRef<GetPaymentReceipt> for RTDGetPaymentReceiptBuilder {
  fn as_ref(&self) -> &GetPaymentReceipt { &self.inner }
}







/// Returns users voted for the specified option in a non-anonymous polls. For the optimal performance the number of returned users is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPollVoters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to which the poll belongs
  chat_id: i64,
  /// Identifier of the message containing the poll
  message_id: i64,
  /// 0-based identifier of the answer option
  option_id: i64,
  /// Number of users to skip in the result; must be non-negative
  offset: i64,
  /// The maximum number of users to be returned; must be positive and can't be greater than 50. Fewer users may be returned than specified by the limit, even if the end of the voter list has not been reached
  limit: i64,
  
}

impl RObject for GetPollVoters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPollVoters" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPollVoters {}

impl GetPollVoters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPollVotersBuilder {
    let mut inner = GetPollVoters::default();
    inner.td_name = "getPollVoters".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPollVotersBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn option_id(&self) -> i64 { self.option_id }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetPollVotersBuilder {
  inner: GetPollVoters
}

impl RTDGetPollVotersBuilder {
  pub fn build(&self) -> GetPollVoters { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn option_id(&mut self, option_id: i64) -> &mut Self {
    self.inner.option_id = option_id;
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetPollVoters> for GetPollVoters {
  fn as_ref(&self) -> &GetPollVoters { self }
}

impl AsRef<GetPollVoters> for RTDGetPollVotersBuilder {
  fn as_ref(&self) -> &GetPollVoters { &self.inner }
}







/// Returns an IETF language tag of the language preferred in the country, which should be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPreferredCountryLanguage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A two-letter ISO 3166-1 alpha-2 country code
  country_code: String,
  
}

impl RObject for GetPreferredCountryLanguage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPreferredCountryLanguage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPreferredCountryLanguage {}

impl GetPreferredCountryLanguage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPreferredCountryLanguageBuilder {
    let mut inner = GetPreferredCountryLanguage::default();
    inner.td_name = "getPreferredCountryLanguage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPreferredCountryLanguageBuilder { inner }
  }

  pub fn country_code(&self) -> &String { &self.country_code }

}

#[doc(hidden)]
pub struct RTDGetPreferredCountryLanguageBuilder {
  inner: GetPreferredCountryLanguage
}

impl RTDGetPreferredCountryLanguageBuilder {
  pub fn build(&self) -> GetPreferredCountryLanguage { self.inner.clone() }

   
  pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
    self.inner.country_code = country_code.as_ref().to_string();
    self
  }

}

impl AsRef<GetPreferredCountryLanguage> for GetPreferredCountryLanguage {
  fn as_ref(&self) -> &GetPreferredCountryLanguage { self }
}

impl AsRef<GetPreferredCountryLanguage> for RTDGetPreferredCountryLanguageBuilder {
  fn as_ref(&self) -> &GetPreferredCountryLanguage { &self.inner }
}







/// Returns list of proxies that are currently set up. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetProxies {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetProxies {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getProxies" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetProxies {}

impl GetProxies {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetProxiesBuilder {
    let mut inner = GetProxies::default();
    inner.td_name = "getProxies".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetProxiesBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetProxiesBuilder {
  inner: GetProxies
}

impl RTDGetProxiesBuilder {
  pub fn build(&self) -> GetProxies { self.inner.clone() }

}

impl AsRef<GetProxies> for GetProxies {
  fn as_ref(&self) -> &GetProxies { self }
}

impl AsRef<GetProxies> for RTDGetProxiesBuilder {
  fn as_ref(&self) -> &GetProxies { &self.inner }
}







/// Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetProxyLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Proxy identifier
  proxy_id: i64,
  
}

impl RObject for GetProxyLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getProxyLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetProxyLink {}

impl GetProxyLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetProxyLinkBuilder {
    let mut inner = GetProxyLink::default();
    inner.td_name = "getProxyLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetProxyLinkBuilder { inner }
  }

  pub fn proxy_id(&self) -> i64 { self.proxy_id }

}

#[doc(hidden)]
pub struct RTDGetProxyLinkBuilder {
  inner: GetProxyLink
}

impl RTDGetProxyLinkBuilder {
  pub fn build(&self) -> GetProxyLink { self.inner.clone() }

   
  pub fn proxy_id(&mut self, proxy_id: i64) -> &mut Self {
    self.inner.proxy_id = proxy_id;
    self
  }

}

impl AsRef<GetProxyLink> for GetProxyLink {
  fn as_ref(&self) -> &GetProxyLink { self }
}

impl AsRef<GetProxyLink> for RTDGetProxyLinkBuilder {
  fn as_ref(&self) -> &GetProxyLink { &self.inner }
}







/// Returns a public HTTPS link to a message. Available only for messages in supergroups and channels with a username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPublicMessageLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to which the message belongs
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// Pass true if a link for a whole media album should be returned
  for_album: bool,
  
}

impl RObject for GetPublicMessageLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPublicMessageLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPublicMessageLink {}

impl GetPublicMessageLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPublicMessageLinkBuilder {
    let mut inner = GetPublicMessageLink::default();
    inner.td_name = "getPublicMessageLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPublicMessageLinkBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn for_album(&self) -> bool { self.for_album }

}

#[doc(hidden)]
pub struct RTDGetPublicMessageLinkBuilder {
  inner: GetPublicMessageLink
}

impl RTDGetPublicMessageLinkBuilder {
  pub fn build(&self) -> GetPublicMessageLink { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn for_album(&mut self, for_album: bool) -> &mut Self {
    self.inner.for_album = for_album;
    self
  }

}

impl AsRef<GetPublicMessageLink> for GetPublicMessageLink {
  fn as_ref(&self) -> &GetPublicMessageLink { self }
}

impl AsRef<GetPublicMessageLink> for RTDGetPublicMessageLinkBuilder {
  fn as_ref(&self) -> &GetPublicMessageLink { &self.inner }
}







/// Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPushReceiverId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// JSON-encoded push notification payload
  payload: String,
  
}

impl RObject for GetPushReceiverId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPushReceiverId" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetPushReceiverId {}

impl GetPushReceiverId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetPushReceiverIdBuilder {
    let mut inner = GetPushReceiverId::default();
    inner.td_name = "getPushReceiverId".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetPushReceiverIdBuilder { inner }
  }

  pub fn payload(&self) -> &String { &self.payload }

}

#[doc(hidden)]
pub struct RTDGetPushReceiverIdBuilder {
  inner: GetPushReceiverId
}

impl RTDGetPushReceiverIdBuilder {
  pub fn build(&self) -> GetPushReceiverId { self.inner.clone() }

   
  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

}

impl AsRef<GetPushReceiverId> for GetPushReceiverId {
  fn as_ref(&self) -> &GetPushReceiverId { self }
}

impl AsRef<GetPushReceiverId> for RTDGetPushReceiverIdBuilder {
  fn as_ref(&self) -> &GetPushReceiverId { &self.inner }
}







/// Returns up to 20 recently used inline bots in the order of their last usage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecentInlineBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetRecentInlineBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRecentInlineBots" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetRecentInlineBots {}

impl GetRecentInlineBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetRecentInlineBotsBuilder {
    let mut inner = GetRecentInlineBots::default();
    inner.td_name = "getRecentInlineBots".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetRecentInlineBotsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetRecentInlineBotsBuilder {
  inner: GetRecentInlineBots
}

impl RTDGetRecentInlineBotsBuilder {
  pub fn build(&self) -> GetRecentInlineBots { self.inner.clone() }

}

impl AsRef<GetRecentInlineBots> for GetRecentInlineBots {
  fn as_ref(&self) -> &GetRecentInlineBots { self }
}

impl AsRef<GetRecentInlineBots> for RTDGetRecentInlineBotsBuilder {
  fn as_ref(&self) -> &GetRecentInlineBots { &self.inner }
}







/// Returns a list of recently used stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecentStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers
  is_attached: bool,
  
}

impl RObject for GetRecentStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRecentStickers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetRecentStickers {}

impl GetRecentStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetRecentStickersBuilder {
    let mut inner = GetRecentStickers::default();
    inner.td_name = "getRecentStickers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetRecentStickersBuilder { inner }
  }

  pub fn is_attached(&self) -> bool { self.is_attached }

}

#[doc(hidden)]
pub struct RTDGetRecentStickersBuilder {
  inner: GetRecentStickers
}

impl RTDGetRecentStickersBuilder {
  pub fn build(&self) -> GetRecentStickers { self.inner.clone() }

   
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.is_attached = is_attached;
    self
  }

}

impl AsRef<GetRecentStickers> for GetRecentStickers {
  fn as_ref(&self) -> &GetRecentStickers { self }
}

impl AsRef<GetRecentStickers> for RTDGetRecentStickersBuilder {
  fn as_ref(&self) -> &GetRecentStickers { &self.inner }
}







/// Returns t.me URLs recently visited by a newly registered user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecentlyVisitedTMeUrls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Google Play referrer to identify the user
  referrer: String,
  
}

impl RObject for GetRecentlyVisitedTMeUrls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRecentlyVisitedTMeUrls" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetRecentlyVisitedTMeUrls {}

impl GetRecentlyVisitedTMeUrls {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetRecentlyVisitedTMeUrlsBuilder {
    let mut inner = GetRecentlyVisitedTMeUrls::default();
    inner.td_name = "getRecentlyVisitedTMeUrls".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetRecentlyVisitedTMeUrlsBuilder { inner }
  }

  pub fn referrer(&self) -> &String { &self.referrer }

}

#[doc(hidden)]
pub struct RTDGetRecentlyVisitedTMeUrlsBuilder {
  inner: GetRecentlyVisitedTMeUrls
}

impl RTDGetRecentlyVisitedTMeUrlsBuilder {
  pub fn build(&self) -> GetRecentlyVisitedTMeUrls { self.inner.clone() }

   
  pub fn referrer<T: AsRef<str>>(&mut self, referrer: T) -> &mut Self {
    self.inner.referrer = referrer.as_ref().to_string();
    self
  }

}

impl AsRef<GetRecentlyVisitedTMeUrls> for GetRecentlyVisitedTMeUrls {
  fn as_ref(&self) -> &GetRecentlyVisitedTMeUrls { self }
}

impl AsRef<GetRecentlyVisitedTMeUrls> for RTDGetRecentlyVisitedTMeUrlsBuilder {
  fn as_ref(&self) -> &GetRecentlyVisitedTMeUrls { &self.inner }
}







/// Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecoveryEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The password for the current user
  password: String,
  
}

impl RObject for GetRecoveryEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRecoveryEmailAddress" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetRecoveryEmailAddress {}

impl GetRecoveryEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetRecoveryEmailAddressBuilder {
    let mut inner = GetRecoveryEmailAddress::default();
    inner.td_name = "getRecoveryEmailAddress".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetRecoveryEmailAddressBuilder { inner }
  }

  pub fn password(&self) -> &String { &self.password }

}

#[doc(hidden)]
pub struct RTDGetRecoveryEmailAddressBuilder {
  inner: GetRecoveryEmailAddress
}

impl RTDGetRecoveryEmailAddressBuilder {
  pub fn build(&self) -> GetRecoveryEmailAddress { self.inner.clone() }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

}

impl AsRef<GetRecoveryEmailAddress> for GetRecoveryEmailAddress {
  fn as_ref(&self) -> &GetRecoveryEmailAddress { self }
}

impl AsRef<GetRecoveryEmailAddress> for RTDGetRecoveryEmailAddressBuilder {
  fn as_ref(&self) -> &GetRecoveryEmailAddress { &self.inner }
}







/// Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. Even the request succeeds, the file can be used only if it is still accessible to the user. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the client
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRemoteFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Remote identifier of the file to get
  remote_file_id: String,
  /// File type, if known
  file_type: FileType,
  
}

impl RObject for GetRemoteFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRemoteFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetRemoteFile {}

impl GetRemoteFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetRemoteFileBuilder {
    let mut inner = GetRemoteFile::default();
    inner.td_name = "getRemoteFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetRemoteFileBuilder { inner }
  }

  pub fn remote_file_id(&self) -> &String { &self.remote_file_id }

  pub fn file_type(&self) -> &FileType { &self.file_type }

}

#[doc(hidden)]
pub struct RTDGetRemoteFileBuilder {
  inner: GetRemoteFile
}

impl RTDGetRemoteFileBuilder {
  pub fn build(&self) -> GetRemoteFile { self.inner.clone() }

   
  pub fn remote_file_id<T: AsRef<str>>(&mut self, remote_file_id: T) -> &mut Self {
    self.inner.remote_file_id = remote_file_id.as_ref().to_string();
    self
  }

   
  pub fn file_type<T: AsRef<FileType>>(&mut self, file_type: T) -> &mut Self {
    self.inner.file_type = file_type.as_ref().clone();
    self
  }

}

impl AsRef<GetRemoteFile> for GetRemoteFile {
  fn as_ref(&self) -> &GetRemoteFile { self }
}

impl AsRef<GetRemoteFile> for RTDGetRemoteFileBuilder {
  fn as_ref(&self) -> &GetRemoteFile { &self.inner }
}







/// Returns information about a message that is replied by given message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRepliedMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat the message belongs to
  chat_id: i64,
  /// Identifier of the message reply to which get
  message_id: i64,
  
}

impl RObject for GetRepliedMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRepliedMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetRepliedMessage {}

impl GetRepliedMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetRepliedMessageBuilder {
    let mut inner = GetRepliedMessage::default();
    inner.td_name = "getRepliedMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetRepliedMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDGetRepliedMessageBuilder {
  inner: GetRepliedMessage
}

impl RTDGetRepliedMessageBuilder {
  pub fn build(&self) -> GetRepliedMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<GetRepliedMessage> for GetRepliedMessage {
  fn as_ref(&self) -> &GetRepliedMessage { self }
}

impl AsRef<GetRepliedMessage> for RTDGetRepliedMessageBuilder {
  fn as_ref(&self) -> &GetRepliedMessage { &self.inner }
}







/// Returns saved animations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSavedAnimations {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetSavedAnimations {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSavedAnimations" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetSavedAnimations {}

impl GetSavedAnimations {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetSavedAnimationsBuilder {
    let mut inner = GetSavedAnimations::default();
    inner.td_name = "getSavedAnimations".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetSavedAnimationsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetSavedAnimationsBuilder {
  inner: GetSavedAnimations
}

impl RTDGetSavedAnimationsBuilder {
  pub fn build(&self) -> GetSavedAnimations { self.inner.clone() }

}

impl AsRef<GetSavedAnimations> for GetSavedAnimations {
  fn as_ref(&self) -> &GetSavedAnimations { self }
}

impl AsRef<GetSavedAnimations> for RTDGetSavedAnimationsBuilder {
  fn as_ref(&self) -> &GetSavedAnimations { &self.inner }
}







/// Returns saved order info, if any
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSavedOrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetSavedOrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSavedOrderInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetSavedOrderInfo {}

impl GetSavedOrderInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetSavedOrderInfoBuilder {
    let mut inner = GetSavedOrderInfo::default();
    inner.td_name = "getSavedOrderInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetSavedOrderInfoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetSavedOrderInfoBuilder {
  inner: GetSavedOrderInfo
}

impl RTDGetSavedOrderInfoBuilder {
  pub fn build(&self) -> GetSavedOrderInfo { self.inner.clone() }

}

impl AsRef<GetSavedOrderInfo> for GetSavedOrderInfo {
  fn as_ref(&self) -> &GetSavedOrderInfo { self }
}

impl AsRef<GetSavedOrderInfo> for RTDGetSavedOrderInfoBuilder {
  fn as_ref(&self) -> &GetSavedOrderInfo { &self.inner }
}







/// Returns the notification settings for chats of a given type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetScopeNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Types of chats for which to return the notification settings information
  scope: NotificationSettingsScope,
  
}

impl RObject for GetScopeNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getScopeNotificationSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetScopeNotificationSettings {}

impl GetScopeNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetScopeNotificationSettingsBuilder {
    let mut inner = GetScopeNotificationSettings::default();
    inner.td_name = "getScopeNotificationSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetScopeNotificationSettingsBuilder { inner }
  }

  pub fn scope(&self) -> &NotificationSettingsScope { &self.scope }

}

#[doc(hidden)]
pub struct RTDGetScopeNotificationSettingsBuilder {
  inner: GetScopeNotificationSettings
}

impl RTDGetScopeNotificationSettingsBuilder {
  pub fn build(&self) -> GetScopeNotificationSettings { self.inner.clone() }

   
  pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
    self.inner.scope = scope.as_ref().clone();
    self
  }

}

impl AsRef<GetScopeNotificationSettings> for GetScopeNotificationSettings {
  fn as_ref(&self) -> &GetScopeNotificationSettings { self }
}

impl AsRef<GetScopeNotificationSettings> for RTDGetScopeNotificationSettingsBuilder {
  fn as_ref(&self) -> &GetScopeNotificationSettings { &self.inner }
}







/// Returns information about a secret chat by its identifier. This is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Secret chat identifier
  secret_chat_id: i64,
  
}

impl RObject for GetSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSecretChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetSecretChat {}

impl GetSecretChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetSecretChatBuilder {
    let mut inner = GetSecretChat::default();
    inner.td_name = "getSecretChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetSecretChatBuilder { inner }
  }

  pub fn secret_chat_id(&self) -> i64 { self.secret_chat_id }

}

#[doc(hidden)]
pub struct RTDGetSecretChatBuilder {
  inner: GetSecretChat
}

impl RTDGetSecretChatBuilder {
  pub fn build(&self) -> GetSecretChat { self.inner.clone() }

   
  pub fn secret_chat_id(&mut self, secret_chat_id: i64) -> &mut Self {
    self.inner.secret_chat_id = secret_chat_id;
    self
  }

}

impl AsRef<GetSecretChat> for GetSecretChat {
  fn as_ref(&self) -> &GetSecretChat { self }
}

impl AsRef<GetSecretChat> for RTDGetSecretChatBuilder {
  fn as_ref(&self) -> &GetSecretChat { &self.inner }
}







/// Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStickerEmojis {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker file identifier
  sticker: InputFile,
  
}

impl RObject for GetStickerEmojis {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStickerEmojis" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetStickerEmojis {}

impl GetStickerEmojis {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetStickerEmojisBuilder {
    let mut inner = GetStickerEmojis::default();
    inner.td_name = "getStickerEmojis".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetStickerEmojisBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

}

#[doc(hidden)]
pub struct RTDGetStickerEmojisBuilder {
  inner: GetStickerEmojis
}

impl RTDGetStickerEmojisBuilder {
  pub fn build(&self) -> GetStickerEmojis { self.inner.clone() }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<GetStickerEmojis> for GetStickerEmojis {
  fn as_ref(&self) -> &GetStickerEmojis { self }
}

impl AsRef<GetStickerEmojis> for RTDGetStickerEmojisBuilder {
  fn as_ref(&self) -> &GetStickerEmojis { &self.inner }
}







/// Returns information about a sticker set by its identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the sticker set
  set_id: isize,
  
}

impl RObject for GetStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStickerSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetStickerSet {}

impl GetStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetStickerSetBuilder {
    let mut inner = GetStickerSet::default();
    inner.td_name = "getStickerSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetStickerSetBuilder { inner }
  }

  pub fn set_id(&self) -> isize { self.set_id }

}

#[doc(hidden)]
pub struct RTDGetStickerSetBuilder {
  inner: GetStickerSet
}

impl RTDGetStickerSetBuilder {
  pub fn build(&self) -> GetStickerSet { self.inner.clone() }

   
  pub fn set_id(&mut self, set_id: isize) -> &mut Self {
    self.inner.set_id = set_id;
    self
  }

}

impl AsRef<GetStickerSet> for GetStickerSet {
  fn as_ref(&self) -> &GetStickerSet { self }
}

impl AsRef<GetStickerSet> for RTDGetStickerSetBuilder {
  fn as_ref(&self) -> &GetStickerSet { &self.inner }
}







/// Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is not empty, favorite and recently used stickers may also be returned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// String representation of emoji. If empty, returns all known installed stickers
  emoji: String,
  /// The maximum number of stickers to be returned
  limit: i64,
  
}

impl RObject for GetStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStickers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetStickers {}

impl GetStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetStickersBuilder {
    let mut inner = GetStickers::default();
    inner.td_name = "getStickers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetStickersBuilder { inner }
  }

  pub fn emoji(&self) -> &String { &self.emoji }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetStickersBuilder {
  inner: GetStickers
}

impl RTDGetStickersBuilder {
  pub fn build(&self) -> GetStickers { self.inner.clone() }

   
  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetStickers> for GetStickers {
  fn as_ref(&self) -> &GetStickers { self }
}

impl AsRef<GetStickers> for RTDGetStickersBuilder {
  fn as_ref(&self) -> &GetStickers { &self.inner }
}







/// Returns storage usage statistics. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStorageStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The maximum number of chats with the largest storage usage for which separate statistics should be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0
  chat_limit: i64,
  
}

impl RObject for GetStorageStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStorageStatistics" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetStorageStatistics {}

impl GetStorageStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetStorageStatisticsBuilder {
    let mut inner = GetStorageStatistics::default();
    inner.td_name = "getStorageStatistics".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetStorageStatisticsBuilder { inner }
  }

  pub fn chat_limit(&self) -> i64 { self.chat_limit }

}

#[doc(hidden)]
pub struct RTDGetStorageStatisticsBuilder {
  inner: GetStorageStatistics
}

impl RTDGetStorageStatisticsBuilder {
  pub fn build(&self) -> GetStorageStatistics { self.inner.clone() }

   
  pub fn chat_limit(&mut self, chat_limit: i64) -> &mut Self {
    self.inner.chat_limit = chat_limit;
    self
  }

}

impl AsRef<GetStorageStatistics> for GetStorageStatistics {
  fn as_ref(&self) -> &GetStorageStatistics { self }
}

impl AsRef<GetStorageStatistics> for RTDGetStorageStatisticsBuilder {
  fn as_ref(&self) -> &GetStorageStatistics { &self.inner }
}







/// Quickly returns approximate storage usage statistics. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStorageStatisticsFast {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetStorageStatisticsFast {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStorageStatisticsFast" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetStorageStatisticsFast {}

impl GetStorageStatisticsFast {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetStorageStatisticsFastBuilder {
    let mut inner = GetStorageStatisticsFast::default();
    inner.td_name = "getStorageStatisticsFast".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetStorageStatisticsFastBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetStorageStatisticsFastBuilder {
  inner: GetStorageStatisticsFast
}

impl RTDGetStorageStatisticsFastBuilder {
  pub fn build(&self) -> GetStorageStatisticsFast { self.inner.clone() }

}

impl AsRef<GetStorageStatisticsFast> for GetStorageStatisticsFast {
  fn as_ref(&self) -> &GetStorageStatisticsFast { self }
}

impl AsRef<GetStorageStatisticsFast> for RTDGetStorageStatisticsFastBuilder {
  fn as_ref(&self) -> &GetStorageStatisticsFast { &self.inner }
}







/// Returns a list of basic group and supergroup chats, which can be used as a discussion group for a channel. Basic group chats need to be first upgraded to supergroups before they can be set as a discussion group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSuitableDiscussionChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetSuitableDiscussionChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSuitableDiscussionChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetSuitableDiscussionChats {}

impl GetSuitableDiscussionChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetSuitableDiscussionChatsBuilder {
    let mut inner = GetSuitableDiscussionChats::default();
    inner.td_name = "getSuitableDiscussionChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetSuitableDiscussionChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetSuitableDiscussionChatsBuilder {
  inner: GetSuitableDiscussionChats
}

impl RTDGetSuitableDiscussionChatsBuilder {
  pub fn build(&self) -> GetSuitableDiscussionChats { self.inner.clone() }

}

impl AsRef<GetSuitableDiscussionChats> for GetSuitableDiscussionChats {
  fn as_ref(&self) -> &GetSuitableDiscussionChats { self }
}

impl AsRef<GetSuitableDiscussionChats> for RTDGetSuitableDiscussionChatsBuilder {
  fn as_ref(&self) -> &GetSuitableDiscussionChats { &self.inner }
}







/// Returns information about a supergroup or a channel by its identifier. This is an offline request if the current user is not a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup or channel identifier
  supergroup_id: i64,
  
}

impl RObject for GetSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSupergroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetSupergroup {}

impl GetSupergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetSupergroupBuilder {
    let mut inner = GetSupergroup::default();
    inner.td_name = "getSupergroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetSupergroupBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

}

#[doc(hidden)]
pub struct RTDGetSupergroupBuilder {
  inner: GetSupergroup
}

impl RTDGetSupergroupBuilder {
  pub fn build(&self) -> GetSupergroup { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

}

impl AsRef<GetSupergroup> for GetSupergroup {
  fn as_ref(&self) -> &GetSupergroup { self }
}

impl AsRef<GetSupergroup> for RTDGetSupergroupBuilder {
  fn as_ref(&self) -> &GetSupergroup { &self.inner }
}







/// Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupergroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup or channel identifier
  supergroup_id: i64,
  
}

impl RObject for GetSupergroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSupergroupFullInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetSupergroupFullInfo {}

impl GetSupergroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetSupergroupFullInfoBuilder {
    let mut inner = GetSupergroupFullInfo::default();
    inner.td_name = "getSupergroupFullInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetSupergroupFullInfoBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

}

#[doc(hidden)]
pub struct RTDGetSupergroupFullInfoBuilder {
  inner: GetSupergroupFullInfo
}

impl RTDGetSupergroupFullInfoBuilder {
  pub fn build(&self) -> GetSupergroupFullInfo { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

}

impl AsRef<GetSupergroupFullInfo> for GetSupergroupFullInfo {
  fn as_ref(&self) -> &GetSupergroupFullInfo { self }
}

impl AsRef<GetSupergroupFullInfo> for RTDGetSupergroupFullInfoBuilder {
  fn as_ref(&self) -> &GetSupergroupFullInfo { &self.inner }
}







/// Returns information about members or banned users in a supergroup or channel. Can be used only if SupergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupergroupMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the supergroup or channel
  supergroup_id: i64,
  /// The type of users to return. By default, supergroupMembersRecent
  filter: SupergroupMembersFilter,
  /// Number of users to skip
  offset: i64,
  /// The maximum number of users be returned; up to 200
  limit: i64,
  
}

impl RObject for GetSupergroupMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSupergroupMembers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetSupergroupMembers {}

impl GetSupergroupMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetSupergroupMembersBuilder {
    let mut inner = GetSupergroupMembers::default();
    inner.td_name = "getSupergroupMembers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetSupergroupMembersBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn filter(&self) -> &SupergroupMembersFilter { &self.filter }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetSupergroupMembersBuilder {
  inner: GetSupergroupMembers
}

impl RTDGetSupergroupMembersBuilder {
  pub fn build(&self) -> GetSupergroupMembers { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn filter<T: AsRef<SupergroupMembersFilter>>(&mut self, filter: T) -> &mut Self {
    self.inner.filter = filter.as_ref().clone();
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetSupergroupMembers> for GetSupergroupMembers {
  fn as_ref(&self) -> &GetSupergroupMembers { self }
}

impl AsRef<GetSupergroupMembers> for RTDGetSupergroupMembersBuilder {
  fn as_ref(&self) -> &GetSupergroupMembers { &self.inner }
}







/// Returns a user that can be contacted to get support
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupportUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetSupportUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSupportUser" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetSupportUser {}

impl GetSupportUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetSupportUserBuilder {
    let mut inner = GetSupportUser::default();
    inner.td_name = "getSupportUser".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetSupportUserBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetSupportUserBuilder {
  inner: GetSupportUser
}

impl RTDGetSupportUserBuilder {
  pub fn build(&self) -> GetSupportUser { self.inner.clone() }

}

impl AsRef<GetSupportUser> for GetSupportUser {
  fn as_ref(&self) -> &GetSupportUser { self }
}

impl AsRef<GetSupportUser> for RTDGetSupportUserBuilder {
  fn as_ref(&self) -> &GetSupportUser { &self.inner }
}







/// Returns information about the current temporary password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTemporaryPasswordState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetTemporaryPasswordState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getTemporaryPasswordState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetTemporaryPasswordState {}

impl GetTemporaryPasswordState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetTemporaryPasswordStateBuilder {
    let mut inner = GetTemporaryPasswordState::default();
    inner.td_name = "getTemporaryPasswordState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetTemporaryPasswordStateBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetTemporaryPasswordStateBuilder {
  inner: GetTemporaryPasswordState
}

impl RTDGetTemporaryPasswordStateBuilder {
  pub fn build(&self) -> GetTemporaryPasswordState { self.inner.clone() }

}

impl AsRef<GetTemporaryPasswordState> for GetTemporaryPasswordState {
  fn as_ref(&self) -> &GetTemporaryPasswordState { self }
}

impl AsRef<GetTemporaryPasswordState> for RTDGetTemporaryPasswordStateBuilder {
  fn as_ref(&self) -> &GetTemporaryPasswordState { &self.inner }
}







/// Returns all entities (mentions, hashtags, cashtags, bot commands, URLs, and email addresses) contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTextEntities {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The text in which to look for entites
  text: String,
  
}

impl RObject for GetTextEntities {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getTextEntities" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetTextEntities {}

impl GetTextEntities {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetTextEntitiesBuilder {
    let mut inner = GetTextEntities::default();
    inner.td_name = "getTextEntities".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetTextEntitiesBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

}

#[doc(hidden)]
pub struct RTDGetTextEntitiesBuilder {
  inner: GetTextEntities
}

impl RTDGetTextEntitiesBuilder {
  pub fn build(&self) -> GetTextEntities { self.inner.clone() }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

}

impl AsRef<GetTextEntities> for GetTextEntities {
  fn as_ref(&self) -> &GetTextEntities { self }
}

impl AsRef<GetTextEntities> for RTDGetTextEntitiesBuilder {
  fn as_ref(&self) -> &GetTextEntities { &self.inner }
}







/// Returns a list of frequently used chats. Supported only if the chat info database is enabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTopChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Category of chats to be returned
  category: TopChatCategory,
  /// The maximum number of chats to be returned; up to 30
  limit: i64,
  
}

impl RObject for GetTopChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getTopChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetTopChats {}

impl GetTopChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetTopChatsBuilder {
    let mut inner = GetTopChats::default();
    inner.td_name = "getTopChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetTopChatsBuilder { inner }
  }

  pub fn category(&self) -> &TopChatCategory { &self.category }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetTopChatsBuilder {
  inner: GetTopChats
}

impl RTDGetTopChatsBuilder {
  pub fn build(&self) -> GetTopChats { self.inner.clone() }

   
  pub fn category<T: AsRef<TopChatCategory>>(&mut self, category: T) -> &mut Self {
    self.inner.category = category.as_ref().clone();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetTopChats> for GetTopChats {
  fn as_ref(&self) -> &GetTopChats { self }
}

impl AsRef<GetTopChats> for RTDGetTopChatsBuilder {
  fn as_ref(&self) -> &GetTopChats { &self.inner }
}







/// Returns a list of trending sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTrendingStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for GetTrendingStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getTrendingStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetTrendingStickerSets {}

impl GetTrendingStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetTrendingStickerSetsBuilder {
    let mut inner = GetTrendingStickerSets::default();
    inner.td_name = "getTrendingStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetTrendingStickerSetsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDGetTrendingStickerSetsBuilder {
  inner: GetTrendingStickerSets
}

impl RTDGetTrendingStickerSetsBuilder {
  pub fn build(&self) -> GetTrendingStickerSets { self.inner.clone() }

}

impl AsRef<GetTrendingStickerSets> for GetTrendingStickerSets {
  fn as_ref(&self) -> &GetTrendingStickerSets { self }
}

impl AsRef<GetTrendingStickerSets> for RTDGetTrendingStickerSetsBuilder {
  fn as_ref(&self) -> &GetTrendingStickerSets { &self.inner }
}







/// Returns information about a user by their identifier. This is an offline request if the current user is not a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  
}

impl RObject for GetUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getUser" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetUser {}

impl GetUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetUserBuilder {
    let mut inner = GetUser::default();
    inner.td_name = "getUser".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetUserBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDGetUserBuilder {
  inner: GetUser
}

impl RTDGetUserBuilder {
  pub fn build(&self) -> GetUser { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<GetUser> for GetUser {
  fn as_ref(&self) -> &GetUser { self }
}

impl AsRef<GetUser> for RTDGetUserBuilder {
  fn as_ref(&self) -> &GetUser { &self.inner }
}







/// Returns full information about a user by their identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  
}

impl RObject for GetUserFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getUserFullInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetUserFullInfo {}

impl GetUserFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetUserFullInfoBuilder {
    let mut inner = GetUserFullInfo::default();
    inner.td_name = "getUserFullInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetUserFullInfoBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDGetUserFullInfoBuilder {
  inner: GetUserFullInfo
}

impl RTDGetUserFullInfoBuilder {
  pub fn build(&self) -> GetUserFullInfo { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<GetUserFullInfo> for GetUserFullInfo {
  fn as_ref(&self) -> &GetUserFullInfo { self }
}

impl AsRef<GetUserFullInfo> for RTDGetUserFullInfoBuilder {
  fn as_ref(&self) -> &GetUserFullInfo { &self.inner }
}







/// Returns the current privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserPrivacySettingRules {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The privacy setting
  setting: UserPrivacySetting,
  
}

impl RObject for GetUserPrivacySettingRules {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getUserPrivacySettingRules" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetUserPrivacySettingRules {}

impl GetUserPrivacySettingRules {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetUserPrivacySettingRulesBuilder {
    let mut inner = GetUserPrivacySettingRules::default();
    inner.td_name = "getUserPrivacySettingRules".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetUserPrivacySettingRulesBuilder { inner }
  }

  pub fn setting(&self) -> &UserPrivacySetting { &self.setting }

}

#[doc(hidden)]
pub struct RTDGetUserPrivacySettingRulesBuilder {
  inner: GetUserPrivacySettingRules
}

impl RTDGetUserPrivacySettingRulesBuilder {
  pub fn build(&self) -> GetUserPrivacySettingRules { self.inner.clone() }

   
  pub fn setting<T: AsRef<UserPrivacySetting>>(&mut self, setting: T) -> &mut Self {
    self.inner.setting = setting.as_ref().clone();
    self
  }

}

impl AsRef<GetUserPrivacySettingRules> for GetUserPrivacySettingRules {
  fn as_ref(&self) -> &GetUserPrivacySettingRules { self }
}

impl AsRef<GetUserPrivacySettingRules> for RTDGetUserPrivacySettingRulesBuilder {
  fn as_ref(&self) -> &GetUserPrivacySettingRules { &self.inner }
}







/// Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserProfilePhotos {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// The number of photos to skip; must be non-negative
  offset: i64,
  /// The maximum number of photos to be returned; up to 100
  limit: i64,
  
}

impl RObject for GetUserProfilePhotos {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getUserProfilePhotos" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetUserProfilePhotos {}

impl GetUserProfilePhotos {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetUserProfilePhotosBuilder {
    let mut inner = GetUserProfilePhotos::default();
    inner.td_name = "getUserProfilePhotos".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetUserProfilePhotosBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDGetUserProfilePhotosBuilder {
  inner: GetUserProfilePhotos
}

impl RTDGetUserProfilePhotosBuilder {
  pub fn build(&self) -> GetUserProfilePhotos { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<GetUserProfilePhotos> for GetUserProfilePhotos {
  fn as_ref(&self) -> &GetUserProfilePhotos { self }
}

impl AsRef<GetUserProfilePhotos> for RTDGetUserProfilePhotosBuilder {
  fn as_ref(&self) -> &GetUserProfilePhotos { &self.inner }
}







/// Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWebPageInstantView {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The web page URL
  url: String,
  /// If true, the full instant view for the web page will be returned
  force_full: bool,
  
}

impl RObject for GetWebPageInstantView {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getWebPageInstantView" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetWebPageInstantView {}

impl GetWebPageInstantView {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetWebPageInstantViewBuilder {
    let mut inner = GetWebPageInstantView::default();
    inner.td_name = "getWebPageInstantView".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetWebPageInstantViewBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn force_full(&self) -> bool { self.force_full }

}

#[doc(hidden)]
pub struct RTDGetWebPageInstantViewBuilder {
  inner: GetWebPageInstantView
}

impl RTDGetWebPageInstantViewBuilder {
  pub fn build(&self) -> GetWebPageInstantView { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn force_full(&mut self, force_full: bool) -> &mut Self {
    self.inner.force_full = force_full;
    self
  }

}

impl AsRef<GetWebPageInstantView> for GetWebPageInstantView {
  fn as_ref(&self) -> &GetWebPageInstantView { self }
}

impl AsRef<GetWebPageInstantView> for RTDGetWebPageInstantViewBuilder {
  fn as_ref(&self) -> &GetWebPageInstantView { &self.inner }
}







/// Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWebPagePreview {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message text with formatting
  text: FormattedText,
  
}

impl RObject for GetWebPagePreview {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getWebPagePreview" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for GetWebPagePreview {}

impl GetWebPagePreview {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGetWebPagePreviewBuilder {
    let mut inner = GetWebPagePreview::default();
    inner.td_name = "getWebPagePreview".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGetWebPagePreviewBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText { &self.text }

}

#[doc(hidden)]
pub struct RTDGetWebPagePreviewBuilder {
  inner: GetWebPagePreview
}

impl RTDGetWebPagePreviewBuilder {
  pub fn build(&self) -> GetWebPagePreview { self.inner.clone() }

   
  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

}

impl AsRef<GetWebPagePreview> for GetWebPagePreview {
  fn as_ref(&self) -> &GetWebPagePreview { self }
}

impl AsRef<GetWebPagePreview> for RTDGetWebPagePreviewBuilder {
  fn as_ref(&self) -> &GetWebPagePreview { &self.inner }
}







/// Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of contacts to import or edit; contacts' vCard are ignored and are not imported
  contacts: Vec<Contact>,
  
}

impl RObject for ImportContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "importContacts" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ImportContacts {}

impl ImportContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDImportContactsBuilder {
    let mut inner = ImportContacts::default();
    inner.td_name = "importContacts".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDImportContactsBuilder { inner }
  }

  pub fn contacts(&self) -> &Vec<Contact> { &self.contacts }

}

#[doc(hidden)]
pub struct RTDImportContactsBuilder {
  inner: ImportContacts
}

impl RTDImportContactsBuilder {
  pub fn build(&self) -> ImportContacts { self.inner.clone() }

   
  pub fn contacts(&mut self, contacts: Vec<Contact>) -> &mut Self {
    self.inner.contacts = contacts;
    self
  }

}

impl AsRef<ImportContacts> for ImportContacts {
  fn as_ref(&self) -> &ImportContacts { self }
}

impl AsRef<ImportContacts> for RTDImportContactsBuilder {
  fn as_ref(&self) -> &ImportContacts { &self.inner }
}







/// Adds current user as a new member to a chat. Private and secret chats can't be joined using this method
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JoinChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for JoinChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "joinChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for JoinChat {}

impl JoinChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJoinChatBuilder {
    let mut inner = JoinChat::default();
    inner.td_name = "joinChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJoinChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDJoinChatBuilder {
  inner: JoinChat
}

impl RTDJoinChatBuilder {
  pub fn build(&self) -> JoinChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<JoinChat> for JoinChat {
  fn as_ref(&self) -> &JoinChat { self }
}

impl AsRef<JoinChat> for RTDJoinChatBuilder {
  fn as_ref(&self) -> &JoinChat { &self.inner }
}







/// Uses an invite link to add the current user to the chat if possible. The new member will not be added until the chat state has been synchronized with the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JoinChatByInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Invite link to import; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/"
  invite_link: String,
  
}

impl RObject for JoinChatByInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "joinChatByInviteLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for JoinChatByInviteLink {}

impl JoinChatByInviteLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJoinChatByInviteLinkBuilder {
    let mut inner = JoinChatByInviteLink::default();
    inner.td_name = "joinChatByInviteLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJoinChatByInviteLinkBuilder { inner }
  }

  pub fn invite_link(&self) -> &String { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDJoinChatByInviteLinkBuilder {
  inner: JoinChatByInviteLink
}

impl RTDJoinChatByInviteLinkBuilder {
  pub fn build(&self) -> JoinChatByInviteLink { self.inner.clone() }

   
  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

}

impl AsRef<JoinChatByInviteLink> for JoinChatByInviteLink {
  fn as_ref(&self) -> &JoinChatByInviteLink { self }
}

impl AsRef<JoinChatByInviteLink> for RTDJoinChatByInviteLinkBuilder {
  fn as_ref(&self) -> &JoinChatByInviteLink { &self.inner }
}







/// Removes current user from chat members. Private and secret chats can't be left using this method
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for LeaveChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "leaveChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for LeaveChat {}

impl LeaveChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLeaveChatBuilder {
    let mut inner = LeaveChat::default();
    inner.td_name = "leaveChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLeaveChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDLeaveChatBuilder {
  inner: LeaveChat
}

impl RTDLeaveChatBuilder {
  pub fn build(&self) -> LeaveChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<LeaveChat> for LeaveChat {
  fn as_ref(&self) -> &LeaveChat { self }
}

impl AsRef<LeaveChat> for RTDLeaveChatBuilder {
  fn as_ref(&self) -> &LeaveChat { &self.inner }
}







/// Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogOut {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for LogOut {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logOut" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for LogOut {}

impl LogOut {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLogOutBuilder {
    let mut inner = LogOut::default();
    inner.td_name = "logOut".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLogOutBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDLogOutBuilder {
  inner: LogOut
}

impl RTDLogOutBuilder {
  pub fn build(&self) -> LogOut { self.inner.clone() }

}

impl AsRef<LogOut> for LogOut {
  fn as_ref(&self) -> &LogOut { self }
}

impl AsRef<LogOut> for RTDLogOutBuilder {
  fn as_ref(&self) -> &LogOut { &self.inner }
}







/// Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for OpenChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "openChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for OpenChat {}

impl OpenChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDOpenChatBuilder {
    let mut inner = OpenChat::default();
    inner.td_name = "openChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDOpenChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDOpenChatBuilder {
  inner: OpenChat
}

impl RTDOpenChatBuilder {
  pub fn build(&self) -> OpenChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<OpenChat> for OpenChat {
  fn as_ref(&self) -> &OpenChat { self }
}

impl AsRef<OpenChat> for RTDOpenChatBuilder {
  fn as_ref(&self) -> &OpenChat { &self.inner }
}







/// Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An updateMessageContentOpened update will be generated if something has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenMessageContent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the message
  chat_id: i64,
  /// Identifier of the message with the opened content
  message_id: i64,
  
}

impl RObject for OpenMessageContent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "openMessageContent" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for OpenMessageContent {}

impl OpenMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDOpenMessageContentBuilder {
    let mut inner = OpenMessageContent::default();
    inner.td_name = "openMessageContent".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDOpenMessageContentBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDOpenMessageContentBuilder {
  inner: OpenMessageContent
}

impl RTDOpenMessageContentBuilder {
  pub fn build(&self) -> OpenMessageContent { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<OpenMessageContent> for OpenMessageContent {
  fn as_ref(&self) -> &OpenMessageContent { self }
}

impl AsRef<OpenMessageContent> for RTDOpenMessageContentBuilder {
  fn as_ref(&self) -> &OpenMessageContent { &self.inner }
}







/// Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptimizeStorage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Limit on the total size of files after deletion. Pass 1 to use the default limit
  size: i64,
  /// Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass 1 to use the default limit
  ttl: i64,
  /// Limit on the total count of files after deletion. Pass 1 to use the default limit
  count: i64,
  /// The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass 1 to use the default value
  immunity_delay: i64,
  /// If not empty, only files with the given type(s) are considered. By default, all types except thumbnails, profile photos, stickers and wallpapers are deleted
  file_types: Vec<FileType>,
  /// If not empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos)
  chat_ids: Vec<i64>,
  /// If not empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos)
  exclude_chat_ids: Vec<i64>,
  /// Same as in getStorageStatistics. Affects only returned statistics
  chat_limit: i64,
  
}

impl RObject for OptimizeStorage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optimizeStorage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for OptimizeStorage {}

impl OptimizeStorage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDOptimizeStorageBuilder {
    let mut inner = OptimizeStorage::default();
    inner.td_name = "optimizeStorage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDOptimizeStorageBuilder { inner }
  }

  pub fn size(&self) -> i64 { self.size }

  pub fn ttl(&self) -> i64 { self.ttl }

  pub fn count(&self) -> i64 { self.count }

  pub fn immunity_delay(&self) -> i64 { self.immunity_delay }

  pub fn file_types(&self) -> &Vec<FileType> { &self.file_types }

  pub fn chat_ids(&self) -> &Vec<i64> { &self.chat_ids }

  pub fn exclude_chat_ids(&self) -> &Vec<i64> { &self.exclude_chat_ids }

  pub fn chat_limit(&self) -> i64 { self.chat_limit }

}

#[doc(hidden)]
pub struct RTDOptimizeStorageBuilder {
  inner: OptimizeStorage
}

impl RTDOptimizeStorageBuilder {
  pub fn build(&self) -> OptimizeStorage { self.inner.clone() }

   
  pub fn size(&mut self, size: i64) -> &mut Self {
    self.inner.size = size;
    self
  }

   
  pub fn ttl(&mut self, ttl: i64) -> &mut Self {
    self.inner.ttl = ttl;
    self
  }

   
  pub fn count(&mut self, count: i64) -> &mut Self {
    self.inner.count = count;
    self
  }

   
  pub fn immunity_delay(&mut self, immunity_delay: i64) -> &mut Self {
    self.inner.immunity_delay = immunity_delay;
    self
  }

   
  pub fn file_types(&mut self, file_types: Vec<FileType>) -> &mut Self {
    self.inner.file_types = file_types;
    self
  }

   
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }

   
  pub fn exclude_chat_ids(&mut self, exclude_chat_ids: Vec<i64>) -> &mut Self {
    self.inner.exclude_chat_ids = exclude_chat_ids;
    self
  }

   
  pub fn chat_limit(&mut self, chat_limit: i64) -> &mut Self {
    self.inner.chat_limit = chat_limit;
    self
  }

}

impl AsRef<OptimizeStorage> for OptimizeStorage {
  fn as_ref(&self) -> &OptimizeStorage { self }
}

impl AsRef<OptimizeStorage> for RTDOptimizeStorageBuilder {
  fn as_ref(&self) -> &OptimizeStorage { &self.inner }
}







/// Parses Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParseTextEntities {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The text which should be parsed
  text: String,
  /// Text parse mode
  parse_mode: TextParseMode,
  
}

impl RObject for ParseTextEntities {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "parseTextEntities" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ParseTextEntities {}

impl ParseTextEntities {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDParseTextEntitiesBuilder {
    let mut inner = ParseTextEntities::default();
    inner.td_name = "parseTextEntities".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDParseTextEntitiesBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

  pub fn parse_mode(&self) -> &TextParseMode { &self.parse_mode }

}

#[doc(hidden)]
pub struct RTDParseTextEntitiesBuilder {
  inner: ParseTextEntities
}

impl RTDParseTextEntitiesBuilder {
  pub fn build(&self) -> ParseTextEntities { self.inner.clone() }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

   
  pub fn parse_mode<T: AsRef<TextParseMode>>(&mut self, parse_mode: T) -> &mut Self {
    self.inner.parse_mode = parse_mode.as_ref().clone();
    self
  }

}

impl AsRef<ParseTextEntities> for ParseTextEntities {
  fn as_ref(&self) -> &ParseTextEntities { self }
}

impl AsRef<ParseTextEntities> for RTDParseTextEntitiesBuilder {
  fn as_ref(&self) -> &ParseTextEntities { &self.inner }
}







/// Pins a message in a chat; requires can_pin_messages rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PinChatMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat
  chat_id: i64,
  /// Identifier of the new pinned message
  message_id: i64,
  /// True, if there should be no notification about the pinned message
  disable_notification: bool,
  
}

impl RObject for PinChatMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pinChatMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for PinChatMessage {}

impl PinChatMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPinChatMessageBuilder {
    let mut inner = PinChatMessage::default();
    inner.td_name = "pinChatMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPinChatMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn disable_notification(&self) -> bool { self.disable_notification }

}

#[doc(hidden)]
pub struct RTDPinChatMessageBuilder {
  inner: PinChatMessage
}

impl RTDPinChatMessageBuilder {
  pub fn build(&self) -> PinChatMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.disable_notification = disable_notification;
    self
  }

}

impl AsRef<PinChatMessage> for PinChatMessage {
  fn as_ref(&self) -> &PinChatMessage { self }
}

impl AsRef<PinChatMessage> for RTDPinChatMessageBuilder {
  fn as_ref(&self) -> &PinChatMessage { &self.inner }
}







/// Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PingProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Proxy identifier. Use 0 to ping a Telegram server without a proxy
  proxy_id: i64,
  
}

impl RObject for PingProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pingProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for PingProxy {}

impl PingProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPingProxyBuilder {
    let mut inner = PingProxy::default();
    inner.td_name = "pingProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPingProxyBuilder { inner }
  }

  pub fn proxy_id(&self) -> i64 { self.proxy_id }

}

#[doc(hidden)]
pub struct RTDPingProxyBuilder {
  inner: PingProxy
}

impl RTDPingProxyBuilder {
  pub fn build(&self) -> PingProxy { self.inner.clone() }

   
  pub fn proxy_id(&mut self, proxy_id: i64) -> &mut Self {
    self.inner.proxy_id = proxy_id;
    self
  }

}

impl AsRef<PingProxy> for PingProxy {
  fn as_ref(&self) -> &PingProxy { self }
}

impl AsRef<PingProxy> for RTDPingProxyBuilder {
  fn as_ref(&self) -> &PingProxy { &self.inner }
}







/// Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessPushNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// JSON-encoded push notification payload with all fields sent by the server, and "google.sent_time" and "google.notification.sound" fields added
  payload: String,
  
}

impl RObject for ProcessPushNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "processPushNotification" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ProcessPushNotification {}

impl ProcessPushNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDProcessPushNotificationBuilder {
    let mut inner = ProcessPushNotification::default();
    inner.td_name = "processPushNotification".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDProcessPushNotificationBuilder { inner }
  }

  pub fn payload(&self) -> &String { &self.payload }

}

#[doc(hidden)]
pub struct RTDProcessPushNotificationBuilder {
  inner: ProcessPushNotification
}

impl RTDProcessPushNotificationBuilder {
  pub fn build(&self) -> ProcessPushNotification { self.inner.clone() }

   
  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

}

impl AsRef<ProcessPushNotification> for ProcessPushNotification {
  fn as_ref(&self) -> &ProcessPushNotification { self }
}

impl AsRef<ProcessPushNotification> for RTDProcessPushNotificationBuilder {
  fn as_ref(&self) -> &ProcessPushNotification { &self.inner }
}







/// Marks all mentions in a chat as read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadAllChatMentions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for ReadAllChatMentions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "readAllChatMentions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ReadAllChatMentions {}

impl ReadAllChatMentions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDReadAllChatMentionsBuilder {
    let mut inner = ReadAllChatMentions::default();
    inner.td_name = "readAllChatMentions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDReadAllChatMentionsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDReadAllChatMentionsBuilder {
  inner: ReadAllChatMentions
}

impl RTDReadAllChatMentionsBuilder {
  pub fn build(&self) -> ReadAllChatMentions { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<ReadAllChatMentions> for ReadAllChatMentions {
  fn as_ref(&self) -> &ReadAllChatMentions { self }
}

impl AsRef<ReadAllChatMentions> for RTDReadAllChatMentionsBuilder {
  fn as_ref(&self) -> &ReadAllChatMentions { &self.inner }
}







/// Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct read from the file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadFilePart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the file. The file must be located in the TDLib file cache
  file_id: i64,
  /// The offset from which to read the file
  offset: i64,
  /// Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position
  count: i64,
  
}

impl RObject for ReadFilePart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "readFilePart" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ReadFilePart {}

impl ReadFilePart {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDReadFilePartBuilder {
    let mut inner = ReadFilePart::default();
    inner.td_name = "readFilePart".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDReadFilePartBuilder { inner }
  }

  pub fn file_id(&self) -> i64 { self.file_id }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn count(&self) -> i64 { self.count }

}

#[doc(hidden)]
pub struct RTDReadFilePartBuilder {
  inner: ReadFilePart
}

impl RTDReadFilePartBuilder {
  pub fn build(&self) -> ReadFilePart { self.inner.clone() }

   
  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn count(&mut self, count: i64) -> &mut Self {
    self.inner.count = count;
    self
  }

}

impl AsRef<ReadFilePart> for ReadFilePart {
  fn as_ref(&self) -> &ReadFilePart { self }
}

impl AsRef<ReadFilePart> for RTDReadFilePartBuilder {
  fn as_ref(&self) -> &ReadFilePart { &self.inner }
}







/// Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoverAuthenticationPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Recovery code to check
  recovery_code: String,
  
}

impl RObject for RecoverAuthenticationPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "recoverAuthenticationPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RecoverAuthenticationPassword {}

impl RecoverAuthenticationPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRecoverAuthenticationPasswordBuilder {
    let mut inner = RecoverAuthenticationPassword::default();
    inner.td_name = "recoverAuthenticationPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRecoverAuthenticationPasswordBuilder { inner }
  }

  pub fn recovery_code(&self) -> &String { &self.recovery_code }

}

#[doc(hidden)]
pub struct RTDRecoverAuthenticationPasswordBuilder {
  inner: RecoverAuthenticationPassword
}

impl RTDRecoverAuthenticationPasswordBuilder {
  pub fn build(&self) -> RecoverAuthenticationPassword { self.inner.clone() }

   
  pub fn recovery_code<T: AsRef<str>>(&mut self, recovery_code: T) -> &mut Self {
    self.inner.recovery_code = recovery_code.as_ref().to_string();
    self
  }

}

impl AsRef<RecoverAuthenticationPassword> for RecoverAuthenticationPassword {
  fn as_ref(&self) -> &RecoverAuthenticationPassword { self }
}

impl AsRef<RecoverAuthenticationPassword> for RTDRecoverAuthenticationPasswordBuilder {
  fn as_ref(&self) -> &RecoverAuthenticationPassword { &self.inner }
}







/// Recovers the password using a recovery code sent to an email address that was previously set up
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoverPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Recovery code to check
  recovery_code: String,
  
}

impl RObject for RecoverPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "recoverPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RecoverPassword {}

impl RecoverPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRecoverPasswordBuilder {
    let mut inner = RecoverPassword::default();
    inner.td_name = "recoverPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRecoverPasswordBuilder { inner }
  }

  pub fn recovery_code(&self) -> &String { &self.recovery_code }

}

#[doc(hidden)]
pub struct RTDRecoverPasswordBuilder {
  inner: RecoverPassword
}

impl RTDRecoverPasswordBuilder {
  pub fn build(&self) -> RecoverPassword { self.inner.clone() }

   
  pub fn recovery_code<T: AsRef<str>>(&mut self, recovery_code: T) -> &mut Self {
    self.inner.recovery_code = recovery_code.as_ref().to_string();
    self
  }

}

impl AsRef<RecoverPassword> for RecoverPassword {
  fn as_ref(&self) -> &RecoverPassword { self }
}

impl AsRef<RecoverPassword> for RTDRecoverPasswordBuilder {
  fn as_ref(&self) -> &RecoverPassword { &self.inner }
}







/// Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterDevice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Device token
  device_token: DeviceToken,
  /// List of user identifiers of other users currently using the client
  other_user_ids: Vec<i64>,
  
}

impl RObject for RegisterDevice {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "registerDevice" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RegisterDevice {}

impl RegisterDevice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRegisterDeviceBuilder {
    let mut inner = RegisterDevice::default();
    inner.td_name = "registerDevice".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRegisterDeviceBuilder { inner }
  }

  pub fn device_token(&self) -> &DeviceToken { &self.device_token }

  pub fn other_user_ids(&self) -> &Vec<i64> { &self.other_user_ids }

}

#[doc(hidden)]
pub struct RTDRegisterDeviceBuilder {
  inner: RegisterDevice
}

impl RTDRegisterDeviceBuilder {
  pub fn build(&self) -> RegisterDevice { self.inner.clone() }

   
  pub fn device_token<T: AsRef<DeviceToken>>(&mut self, device_token: T) -> &mut Self {
    self.inner.device_token = device_token.as_ref().clone();
    self
  }

   
  pub fn other_user_ids(&mut self, other_user_ids: Vec<i64>) -> &mut Self {
    self.inner.other_user_ids = other_user_ids;
    self
  }

}

impl AsRef<RegisterDevice> for RegisterDevice {
  fn as_ref(&self) -> &RegisterDevice { self }
}

impl AsRef<RegisterDevice> for RTDRegisterDeviceBuilder {
  fn as_ref(&self) -> &RegisterDevice { &self.inner }
}







/// Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The first name of the user; 1-64 characters
  first_name: String,
  /// The last name of the user; 0-64 characters
  last_name: String,
  
}

impl RObject for RegisterUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "registerUser" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RegisterUser {}

impl RegisterUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRegisterUserBuilder {
    let mut inner = RegisterUser::default();
    inner.td_name = "registerUser".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRegisterUserBuilder { inner }
  }

  pub fn first_name(&self) -> &String { &self.first_name }

  pub fn last_name(&self) -> &String { &self.last_name }

}

#[doc(hidden)]
pub struct RTDRegisterUserBuilder {
  inner: RegisterUser
}

impl RTDRegisterUserBuilder {
  pub fn build(&self) -> RegisterUser { self.inner.clone() }

   
  pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
    self.inner.first_name = first_name.as_ref().to_string();
    self
  }

   
  pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
    self.inner.last_name = last_name.as_ref().to_string();
    self
  }

}

impl AsRef<RegisterUser> for RegisterUser {
  fn as_ref(&self) -> &RegisterUser { self }
}

impl AsRef<RegisterUser> for RTDRegisterUserBuilder {
  fn as_ref(&self) -> &RegisterUser { &self.inner }
}







/// Removes background from the list of installed backgrounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The background identifier
  background_id: isize,
  
}

impl RObject for RemoveBackground {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeBackground" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveBackground {}

impl RemoveBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveBackgroundBuilder {
    let mut inner = RemoveBackground::default();
    inner.td_name = "removeBackground".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveBackgroundBuilder { inner }
  }

  pub fn background_id(&self) -> isize { self.background_id }

}

#[doc(hidden)]
pub struct RTDRemoveBackgroundBuilder {
  inner: RemoveBackground
}

impl RTDRemoveBackgroundBuilder {
  pub fn build(&self) -> RemoveBackground { self.inner.clone() }

   
  pub fn background_id(&mut self, background_id: isize) -> &mut Self {
    self.inner.background_id = background_id;
    self
  }

}

impl AsRef<RemoveBackground> for RemoveBackground {
  fn as_ref(&self) -> &RemoveBackground { self }
}

impl AsRef<RemoveBackground> for RTDRemoveBackgroundBuilder {
  fn as_ref(&self) -> &RemoveBackground { &self.inner }
}







/// Removes a chat action bar without any other action
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveChatActionBar {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for RemoveChatActionBar {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeChatActionBar" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveChatActionBar {}

impl RemoveChatActionBar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveChatActionBarBuilder {
    let mut inner = RemoveChatActionBar::default();
    inner.td_name = "removeChatActionBar".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveChatActionBarBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDRemoveChatActionBarBuilder {
  inner: RemoveChatActionBar
}

impl RTDRemoveChatActionBarBuilder {
  pub fn build(&self) -> RemoveChatActionBar { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<RemoveChatActionBar> for RemoveChatActionBar {
  fn as_ref(&self) -> &RemoveChatActionBar { self }
}

impl AsRef<RemoveChatActionBar> for RTDRemoveChatActionBarBuilder {
  fn as_ref(&self) -> &RemoveChatActionBar { &self.inner }
}







/// Removes users from the contact list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifiers of users to be deleted
  user_ids: Vec<i64>,
  
}

impl RObject for RemoveContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeContacts" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveContacts {}

impl RemoveContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveContactsBuilder {
    let mut inner = RemoveContacts::default();
    inner.td_name = "removeContacts".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveContactsBuilder { inner }
  }

  pub fn user_ids(&self) -> &Vec<i64> { &self.user_ids }

}

#[doc(hidden)]
pub struct RTDRemoveContactsBuilder {
  inner: RemoveContacts
}

impl RTDRemoveContactsBuilder {
  pub fn build(&self) -> RemoveContacts { self.inner.clone() }

   
  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

}

impl AsRef<RemoveContacts> for RemoveContacts {
  fn as_ref(&self) -> &RemoveContacts { self }
}

impl AsRef<RemoveContacts> for RTDRemoveContactsBuilder {
  fn as_ref(&self) -> &RemoveContacts { &self.inner }
}







/// Removes a sticker from the list of favorite stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveFavoriteSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker file to delete from the list
  sticker: InputFile,
  
}

impl RObject for RemoveFavoriteSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeFavoriteSticker" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveFavoriteSticker {}

impl RemoveFavoriteSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveFavoriteStickerBuilder {
    let mut inner = RemoveFavoriteSticker::default();
    inner.td_name = "removeFavoriteSticker".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveFavoriteStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

}

#[doc(hidden)]
pub struct RTDRemoveFavoriteStickerBuilder {
  inner: RemoveFavoriteSticker
}

impl RTDRemoveFavoriteStickerBuilder {
  pub fn build(&self) -> RemoveFavoriteSticker { self.inner.clone() }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<RemoveFavoriteSticker> for RemoveFavoriteSticker {
  fn as_ref(&self) -> &RemoveFavoriteSticker { self }
}

impl AsRef<RemoveFavoriteSticker> for RTDRemoveFavoriteStickerBuilder {
  fn as_ref(&self) -> &RemoveFavoriteSticker { &self.inner }
}







/// Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of notification group to which the notification belongs
  notification_group_id: i64,
  /// Identifier of removed notification
  notification_id: i64,
  
}

impl RObject for RemoveNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeNotification" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveNotification {}

impl RemoveNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveNotificationBuilder {
    let mut inner = RemoveNotification::default();
    inner.td_name = "removeNotification".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveNotificationBuilder { inner }
  }

  pub fn notification_group_id(&self) -> i64 { self.notification_group_id }

  pub fn notification_id(&self) -> i64 { self.notification_id }

}

#[doc(hidden)]
pub struct RTDRemoveNotificationBuilder {
  inner: RemoveNotification
}

impl RTDRemoveNotificationBuilder {
  pub fn build(&self) -> RemoveNotification { self.inner.clone() }

   
  pub fn notification_group_id(&mut self, notification_group_id: i64) -> &mut Self {
    self.inner.notification_group_id = notification_group_id;
    self
  }

   
  pub fn notification_id(&mut self, notification_id: i64) -> &mut Self {
    self.inner.notification_id = notification_id;
    self
  }

}

impl AsRef<RemoveNotification> for RemoveNotification {
  fn as_ref(&self) -> &RemoveNotification { self }
}

impl AsRef<RemoveNotification> for RTDRemoveNotificationBuilder {
  fn as_ref(&self) -> &RemoveNotification { &self.inner }
}







/// Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveNotificationGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Notification group identifier
  notification_group_id: i64,
  /// The maximum identifier of removed notifications
  max_notification_id: i64,
  
}

impl RObject for RemoveNotificationGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeNotificationGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveNotificationGroup {}

impl RemoveNotificationGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveNotificationGroupBuilder {
    let mut inner = RemoveNotificationGroup::default();
    inner.td_name = "removeNotificationGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveNotificationGroupBuilder { inner }
  }

  pub fn notification_group_id(&self) -> i64 { self.notification_group_id }

  pub fn max_notification_id(&self) -> i64 { self.max_notification_id }

}

#[doc(hidden)]
pub struct RTDRemoveNotificationGroupBuilder {
  inner: RemoveNotificationGroup
}

impl RTDRemoveNotificationGroupBuilder {
  pub fn build(&self) -> RemoveNotificationGroup { self.inner.clone() }

   
  pub fn notification_group_id(&mut self, notification_group_id: i64) -> &mut Self {
    self.inner.notification_group_id = notification_group_id;
    self
  }

   
  pub fn max_notification_id(&mut self, max_notification_id: i64) -> &mut Self {
    self.inner.max_notification_id = max_notification_id;
    self
  }

}

impl AsRef<RemoveNotificationGroup> for RemoveNotificationGroup {
  fn as_ref(&self) -> &RemoveNotificationGroup { self }
}

impl AsRef<RemoveNotificationGroup> for RTDRemoveNotificationGroupBuilder {
  fn as_ref(&self) -> &RemoveNotificationGroup { &self.inner }
}







/// Removes a proxy server. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Proxy identifier
  proxy_id: i64,
  
}

impl RObject for RemoveProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveProxy {}

impl RemoveProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveProxyBuilder {
    let mut inner = RemoveProxy::default();
    inner.td_name = "removeProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveProxyBuilder { inner }
  }

  pub fn proxy_id(&self) -> i64 { self.proxy_id }

}

#[doc(hidden)]
pub struct RTDRemoveProxyBuilder {
  inner: RemoveProxy
}

impl RTDRemoveProxyBuilder {
  pub fn build(&self) -> RemoveProxy { self.inner.clone() }

   
  pub fn proxy_id(&mut self, proxy_id: i64) -> &mut Self {
    self.inner.proxy_id = proxy_id;
    self
  }

}

impl AsRef<RemoveProxy> for RemoveProxy {
  fn as_ref(&self) -> &RemoveProxy { self }
}

impl AsRef<RemoveProxy> for RTDRemoveProxyBuilder {
  fn as_ref(&self) -> &RemoveProxy { &self.inner }
}







/// Removes a hashtag from the list of recently used hashtags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveRecentHashtag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hashtag to delete
  hashtag: String,
  
}

impl RObject for RemoveRecentHashtag {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeRecentHashtag" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveRecentHashtag {}

impl RemoveRecentHashtag {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveRecentHashtagBuilder {
    let mut inner = RemoveRecentHashtag::default();
    inner.td_name = "removeRecentHashtag".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveRecentHashtagBuilder { inner }
  }

  pub fn hashtag(&self) -> &String { &self.hashtag }

}

#[doc(hidden)]
pub struct RTDRemoveRecentHashtagBuilder {
  inner: RemoveRecentHashtag
}

impl RTDRemoveRecentHashtagBuilder {
  pub fn build(&self) -> RemoveRecentHashtag { self.inner.clone() }

   
  pub fn hashtag<T: AsRef<str>>(&mut self, hashtag: T) -> &mut Self {
    self.inner.hashtag = hashtag.as_ref().to_string();
    self
  }

}

impl AsRef<RemoveRecentHashtag> for RemoveRecentHashtag {
  fn as_ref(&self) -> &RemoveRecentHashtag { self }
}

impl AsRef<RemoveRecentHashtag> for RTDRemoveRecentHashtagBuilder {
  fn as_ref(&self) -> &RemoveRecentHashtag { &self.inner }
}







/// Removes a sticker from the list of recently used stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveRecentSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers
  is_attached: bool,
  /// Sticker file to delete
  sticker: InputFile,
  
}

impl RObject for RemoveRecentSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeRecentSticker" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveRecentSticker {}

impl RemoveRecentSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveRecentStickerBuilder {
    let mut inner = RemoveRecentSticker::default();
    inner.td_name = "removeRecentSticker".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveRecentStickerBuilder { inner }
  }

  pub fn is_attached(&self) -> bool { self.is_attached }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

}

#[doc(hidden)]
pub struct RTDRemoveRecentStickerBuilder {
  inner: RemoveRecentSticker
}

impl RTDRemoveRecentStickerBuilder {
  pub fn build(&self) -> RemoveRecentSticker { self.inner.clone() }

   
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.is_attached = is_attached;
    self
  }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<RemoveRecentSticker> for RemoveRecentSticker {
  fn as_ref(&self) -> &RemoveRecentSticker { self }
}

impl AsRef<RemoveRecentSticker> for RTDRemoveRecentStickerBuilder {
  fn as_ref(&self) -> &RemoveRecentSticker { &self.inner }
}







/// Removes a chat from the list of recently found chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveRecentlyFoundChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to be removed
  chat_id: i64,
  
}

impl RObject for RemoveRecentlyFoundChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeRecentlyFoundChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveRecentlyFoundChat {}

impl RemoveRecentlyFoundChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveRecentlyFoundChatBuilder {
    let mut inner = RemoveRecentlyFoundChat::default();
    inner.td_name = "removeRecentlyFoundChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveRecentlyFoundChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDRemoveRecentlyFoundChatBuilder {
  inner: RemoveRecentlyFoundChat
}

impl RTDRemoveRecentlyFoundChatBuilder {
  pub fn build(&self) -> RemoveRecentlyFoundChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<RemoveRecentlyFoundChat> for RemoveRecentlyFoundChat {
  fn as_ref(&self) -> &RemoveRecentlyFoundChat { self }
}

impl AsRef<RemoveRecentlyFoundChat> for RTDRemoveRecentlyFoundChatBuilder {
  fn as_ref(&self) -> &RemoveRecentlyFoundChat { &self.inner }
}







/// Removes an animation from the list of saved animations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveSavedAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Animation file to be removed
  animation: InputFile,
  
}

impl RObject for RemoveSavedAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeSavedAnimation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveSavedAnimation {}

impl RemoveSavedAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveSavedAnimationBuilder {
    let mut inner = RemoveSavedAnimation::default();
    inner.td_name = "removeSavedAnimation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveSavedAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &InputFile { &self.animation }

}

#[doc(hidden)]
pub struct RTDRemoveSavedAnimationBuilder {
  inner: RemoveSavedAnimation
}

impl RTDRemoveSavedAnimationBuilder {
  pub fn build(&self) -> RemoveSavedAnimation { self.inner.clone() }

   
  pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

}

impl AsRef<RemoveSavedAnimation> for RemoveSavedAnimation {
  fn as_ref(&self) -> &RemoveSavedAnimation { self }
}

impl AsRef<RemoveSavedAnimation> for RTDRemoveSavedAnimationBuilder {
  fn as_ref(&self) -> &RemoveSavedAnimation { &self.inner }
}







/// Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveStickerFromSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker
  sticker: InputFile,
  
}

impl RObject for RemoveStickerFromSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeStickerFromSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveStickerFromSet {}

impl RemoveStickerFromSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveStickerFromSetBuilder {
    let mut inner = RemoveStickerFromSet::default();
    inner.td_name = "removeStickerFromSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveStickerFromSetBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

}

#[doc(hidden)]
pub struct RTDRemoveStickerFromSetBuilder {
  inner: RemoveStickerFromSet
}

impl RTDRemoveStickerFromSetBuilder {
  pub fn build(&self) -> RemoveStickerFromSet { self.inner.clone() }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<RemoveStickerFromSet> for RemoveStickerFromSet {
  fn as_ref(&self) -> &RemoveStickerFromSet { self }
}

impl AsRef<RemoveStickerFromSet> for RTDRemoveStickerFromSetBuilder {
  fn as_ref(&self) -> &RemoveStickerFromSet { &self.inner }
}







/// Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveTopChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Category of frequently used chats
  category: TopChatCategory,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for RemoveTopChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeTopChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RemoveTopChat {}

impl RemoveTopChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRemoveTopChatBuilder {
    let mut inner = RemoveTopChat::default();
    inner.td_name = "removeTopChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRemoveTopChatBuilder { inner }
  }

  pub fn category(&self) -> &TopChatCategory { &self.category }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDRemoveTopChatBuilder {
  inner: RemoveTopChat
}

impl RTDRemoveTopChatBuilder {
  pub fn build(&self) -> RemoveTopChat { self.inner.clone() }

   
  pub fn category<T: AsRef<TopChatCategory>>(&mut self, category: T) -> &mut Self {
    self.inner.category = category.as_ref().clone();
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<RemoveTopChat> for RemoveTopChat {
  fn as_ref(&self) -> &RemoveTopChat { self }
}

impl AsRef<RemoveTopChat> for RTDRemoveTopChatBuilder {
  fn as_ref(&self) -> &RemoveTopChat { &self.inner }
}







/// Changes the order of installed sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderInstalledStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to change the order of mask sticker sets; pass false to change the order of ordinary sticker sets
  is_masks: bool,
  /// Identifiers of installed sticker sets in the new correct order
  sticker_set_ids: Vec<isize>,
  
}

impl RObject for ReorderInstalledStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "reorderInstalledStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ReorderInstalledStickerSets {}

impl ReorderInstalledStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDReorderInstalledStickerSetsBuilder {
    let mut inner = ReorderInstalledStickerSets::default();
    inner.td_name = "reorderInstalledStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDReorderInstalledStickerSetsBuilder { inner }
  }

  pub fn is_masks(&self) -> bool { self.is_masks }

  pub fn sticker_set_ids(&self) -> &Vec<isize> { &self.sticker_set_ids }

}

#[doc(hidden)]
pub struct RTDReorderInstalledStickerSetsBuilder {
  inner: ReorderInstalledStickerSets
}

impl RTDReorderInstalledStickerSetsBuilder {
  pub fn build(&self) -> ReorderInstalledStickerSets { self.inner.clone() }

   
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.is_masks = is_masks;
    self
  }

   
  pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<isize>) -> &mut Self {
    self.inner.sticker_set_ids = sticker_set_ids;
    self
  }

}

impl AsRef<ReorderInstalledStickerSets> for ReorderInstalledStickerSets {
  fn as_ref(&self) -> &ReorderInstalledStickerSets { self }
}

impl AsRef<ReorderInstalledStickerSets> for RTDReorderInstalledStickerSetsBuilder {
  fn as_ref(&self) -> &ReorderInstalledStickerSets { &self.inner }
}







/// Reports a chat to the Telegram moderators. Supported only for supergroups, channels, or private chats with bots, since other chats can't be checked by moderators, or when the report is done from the chat action bar
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The reason for reporting the chat
  reason: ChatReportReason,
  /// Identifiers of reported messages, if any
  message_ids: Vec<i64>,
  
}

impl RObject for ReportChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "reportChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ReportChat {}

impl ReportChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDReportChatBuilder {
    let mut inner = ReportChat::default();
    inner.td_name = "reportChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDReportChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn reason(&self) -> &ChatReportReason { &self.reason }

  pub fn message_ids(&self) -> &Vec<i64> { &self.message_ids }

}

#[doc(hidden)]
pub struct RTDReportChatBuilder {
  inner: ReportChat
}

impl RTDReportChatBuilder {
  pub fn build(&self) -> ReportChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn reason<T: AsRef<ChatReportReason>>(&mut self, reason: T) -> &mut Self {
    self.inner.reason = reason.as_ref().clone();
    self
  }

   
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }

}

impl AsRef<ReportChat> for ReportChat {
  fn as_ref(&self) -> &ReportChat { self }
}

impl AsRef<ReportChat> for RTDReportChatBuilder {
  fn as_ref(&self) -> &ReportChat { &self.inner }
}







/// Reports some messages from a user in a supergroup as spam; requires administrator rights in the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSupergroupSpam {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup identifier
  supergroup_id: i64,
  /// User identifier
  user_id: i64,
  /// Identifiers of messages sent in the supergroup by the user. This list must be non-empty
  message_ids: Vec<i64>,
  
}

impl RObject for ReportSupergroupSpam {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "reportSupergroupSpam" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ReportSupergroupSpam {}

impl ReportSupergroupSpam {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDReportSupergroupSpamBuilder {
    let mut inner = ReportSupergroupSpam::default();
    inner.td_name = "reportSupergroupSpam".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDReportSupergroupSpamBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn message_ids(&self) -> &Vec<i64> { &self.message_ids }

}

#[doc(hidden)]
pub struct RTDReportSupergroupSpamBuilder {
  inner: ReportSupergroupSpam
}

impl RTDReportSupergroupSpamBuilder {
  pub fn build(&self) -> ReportSupergroupSpam { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }

}

impl AsRef<ReportSupergroupSpam> for ReportSupergroupSpam {
  fn as_ref(&self) -> &ReportSupergroupSpam { self }
}

impl AsRef<ReportSupergroupSpam> for RTDReportSupergroupSpamBuilder {
  fn as_ref(&self) -> &ReportSupergroupSpam { &self.inner }
}







/// Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestAuthenticationPasswordRecovery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for RequestAuthenticationPasswordRecovery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "requestAuthenticationPasswordRecovery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RequestAuthenticationPasswordRecovery {}

impl RequestAuthenticationPasswordRecovery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRequestAuthenticationPasswordRecoveryBuilder {
    let mut inner = RequestAuthenticationPasswordRecovery::default();
    inner.td_name = "requestAuthenticationPasswordRecovery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRequestAuthenticationPasswordRecoveryBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDRequestAuthenticationPasswordRecoveryBuilder {
  inner: RequestAuthenticationPasswordRecovery
}

impl RTDRequestAuthenticationPasswordRecoveryBuilder {
  pub fn build(&self) -> RequestAuthenticationPasswordRecovery { self.inner.clone() }

}

impl AsRef<RequestAuthenticationPasswordRecovery> for RequestAuthenticationPasswordRecovery {
  fn as_ref(&self) -> &RequestAuthenticationPasswordRecovery { self }
}

impl AsRef<RequestAuthenticationPasswordRecovery> for RTDRequestAuthenticationPasswordRecoveryBuilder {
  fn as_ref(&self) -> &RequestAuthenticationPasswordRecovery { &self.inner }
}







/// Requests to send a password recovery code to an email address that was previously set up
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestPasswordRecovery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for RequestPasswordRecovery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "requestPasswordRecovery" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RequestPasswordRecovery {}

impl RequestPasswordRecovery {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRequestPasswordRecoveryBuilder {
    let mut inner = RequestPasswordRecovery::default();
    inner.td_name = "requestPasswordRecovery".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRequestPasswordRecoveryBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDRequestPasswordRecoveryBuilder {
  inner: RequestPasswordRecovery
}

impl RTDRequestPasswordRecoveryBuilder {
  pub fn build(&self) -> RequestPasswordRecovery { self.inner.clone() }

}

impl AsRef<RequestPasswordRecovery> for RequestPasswordRecovery {
  fn as_ref(&self) -> &RequestPasswordRecovery { self }
}

impl AsRef<RequestPasswordRecovery> for RTDRequestPasswordRecoveryBuilder {
  fn as_ref(&self) -> &RequestPasswordRecovery { &self.inner }
}







/// Requests QR code authentication by scanning a QR code on another logged in device. Works only when the current authorization state is authorizationStateWaitPhoneNumber
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestQrCodeAuthentication {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of user identifiers of other users currently using the client
  other_user_ids: Vec<i64>,
  
}

impl RObject for RequestQrCodeAuthentication {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "requestQrCodeAuthentication" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for RequestQrCodeAuthentication {}

impl RequestQrCodeAuthentication {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRequestQrCodeAuthenticationBuilder {
    let mut inner = RequestQrCodeAuthentication::default();
    inner.td_name = "requestQrCodeAuthentication".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRequestQrCodeAuthenticationBuilder { inner }
  }

  pub fn other_user_ids(&self) -> &Vec<i64> { &self.other_user_ids }

}

#[doc(hidden)]
pub struct RTDRequestQrCodeAuthenticationBuilder {
  inner: RequestQrCodeAuthentication
}

impl RTDRequestQrCodeAuthenticationBuilder {
  pub fn build(&self) -> RequestQrCodeAuthentication { self.inner.clone() }

   
  pub fn other_user_ids(&mut self, other_user_ids: Vec<i64>) -> &mut Self {
    self.inner.other_user_ids = other_user_ids;
    self
  }

}

impl AsRef<RequestQrCodeAuthentication> for RequestQrCodeAuthentication {
  fn as_ref(&self) -> &RequestQrCodeAuthentication { self }
}

impl AsRef<RequestQrCodeAuthentication> for RTDRequestQrCodeAuthenticationBuilder {
  fn as_ref(&self) -> &RequestQrCodeAuthentication { &self.inner }
}







/// Re-sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode and the next_code_type of the result is not null
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendAuthenticationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResendAuthenticationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendAuthenticationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResendAuthenticationCode {}

impl ResendAuthenticationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResendAuthenticationCodeBuilder {
    let mut inner = ResendAuthenticationCode::default();
    inner.td_name = "resendAuthenticationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResendAuthenticationCodeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResendAuthenticationCodeBuilder {
  inner: ResendAuthenticationCode
}

impl RTDResendAuthenticationCodeBuilder {
  pub fn build(&self) -> ResendAuthenticationCode { self.inner.clone() }

}

impl AsRef<ResendAuthenticationCode> for ResendAuthenticationCode {
  fn as_ref(&self) -> &ResendAuthenticationCode { self }
}

impl AsRef<ResendAuthenticationCode> for RTDResendAuthenticationCodeBuilder {
  fn as_ref(&self) -> &ResendAuthenticationCode { &self.inner }
}







/// Re-sends the authentication code sent to confirm a new phone number for the user. Works only if the previously received authenticationCodeInfo next_code_type was not null
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendChangePhoneNumberCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResendChangePhoneNumberCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendChangePhoneNumberCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResendChangePhoneNumberCode {}

impl ResendChangePhoneNumberCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResendChangePhoneNumberCodeBuilder {
    let mut inner = ResendChangePhoneNumberCode::default();
    inner.td_name = "resendChangePhoneNumberCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResendChangePhoneNumberCodeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResendChangePhoneNumberCodeBuilder {
  inner: ResendChangePhoneNumberCode
}

impl RTDResendChangePhoneNumberCodeBuilder {
  pub fn build(&self) -> ResendChangePhoneNumberCode { self.inner.clone() }

}

impl AsRef<ResendChangePhoneNumberCode> for ResendChangePhoneNumberCode {
  fn as_ref(&self) -> &ResendChangePhoneNumberCode { self }
}

impl AsRef<ResendChangePhoneNumberCode> for RTDResendChangePhoneNumberCodeBuilder {
  fn as_ref(&self) -> &ResendChangePhoneNumberCode { &self.inner }
}







/// Re-sends the code to verify an email address to be added to a user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendEmailAddressVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResendEmailAddressVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendEmailAddressVerificationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResendEmailAddressVerificationCode {}

impl ResendEmailAddressVerificationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResendEmailAddressVerificationCodeBuilder {
    let mut inner = ResendEmailAddressVerificationCode::default();
    inner.td_name = "resendEmailAddressVerificationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResendEmailAddressVerificationCodeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResendEmailAddressVerificationCodeBuilder {
  inner: ResendEmailAddressVerificationCode
}

impl RTDResendEmailAddressVerificationCodeBuilder {
  pub fn build(&self) -> ResendEmailAddressVerificationCode { self.inner.clone() }

}

impl AsRef<ResendEmailAddressVerificationCode> for ResendEmailAddressVerificationCode {
  fn as_ref(&self) -> &ResendEmailAddressVerificationCode { self }
}

impl AsRef<ResendEmailAddressVerificationCode> for RTDResendEmailAddressVerificationCodeBuilder {
  fn as_ref(&self) -> &ResendEmailAddressVerificationCode { &self.inner }
}







/// Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed. If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to send messages
  chat_id: i64,
  /// Identifiers of the messages to resend. Message identifiers must be in a strictly increasing order
  message_ids: Vec<i64>,
  
}

impl RObject for ResendMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResendMessages {}

impl ResendMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResendMessagesBuilder {
    let mut inner = ResendMessages::default();
    inner.td_name = "resendMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResendMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_ids(&self) -> &Vec<i64> { &self.message_ids }

}

#[doc(hidden)]
pub struct RTDResendMessagesBuilder {
  inner: ResendMessages
}

impl RTDResendMessagesBuilder {
  pub fn build(&self) -> ResendMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }

}

impl AsRef<ResendMessages> for ResendMessages {
  fn as_ref(&self) -> &ResendMessages { self }
}

impl AsRef<ResendMessages> for RTDResendMessagesBuilder {
  fn as_ref(&self) -> &ResendMessages { &self.inner }
}







/// Resends phone number confirmation code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendPhoneNumberConfirmationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResendPhoneNumberConfirmationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendPhoneNumberConfirmationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResendPhoneNumberConfirmationCode {}

impl ResendPhoneNumberConfirmationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResendPhoneNumberConfirmationCodeBuilder {
    let mut inner = ResendPhoneNumberConfirmationCode::default();
    inner.td_name = "resendPhoneNumberConfirmationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResendPhoneNumberConfirmationCodeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResendPhoneNumberConfirmationCodeBuilder {
  inner: ResendPhoneNumberConfirmationCode
}

impl RTDResendPhoneNumberConfirmationCodeBuilder {
  pub fn build(&self) -> ResendPhoneNumberConfirmationCode { self.inner.clone() }

}

impl AsRef<ResendPhoneNumberConfirmationCode> for ResendPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &ResendPhoneNumberConfirmationCode { self }
}

impl AsRef<ResendPhoneNumberConfirmationCode> for RTDResendPhoneNumberConfirmationCodeBuilder {
  fn as_ref(&self) -> &ResendPhoneNumberConfirmationCode { &self.inner }
}







/// Re-sends the code to verify a phone number to be added to a user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendPhoneNumberVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResendPhoneNumberVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendPhoneNumberVerificationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResendPhoneNumberVerificationCode {}

impl ResendPhoneNumberVerificationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResendPhoneNumberVerificationCodeBuilder {
    let mut inner = ResendPhoneNumberVerificationCode::default();
    inner.td_name = "resendPhoneNumberVerificationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResendPhoneNumberVerificationCodeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResendPhoneNumberVerificationCodeBuilder {
  inner: ResendPhoneNumberVerificationCode
}

impl RTDResendPhoneNumberVerificationCodeBuilder {
  pub fn build(&self) -> ResendPhoneNumberVerificationCode { self.inner.clone() }

}

impl AsRef<ResendPhoneNumberVerificationCode> for ResendPhoneNumberVerificationCode {
  fn as_ref(&self) -> &ResendPhoneNumberVerificationCode { self }
}

impl AsRef<ResendPhoneNumberVerificationCode> for RTDResendPhoneNumberVerificationCodeBuilder {
  fn as_ref(&self) -> &ResendPhoneNumberVerificationCode { &self.inner }
}







/// Resends the 2-step verification recovery email address verification code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendRecoveryEmailAddressCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResendRecoveryEmailAddressCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendRecoveryEmailAddressCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResendRecoveryEmailAddressCode {}

impl ResendRecoveryEmailAddressCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResendRecoveryEmailAddressCodeBuilder {
    let mut inner = ResendRecoveryEmailAddressCode::default();
    inner.td_name = "resendRecoveryEmailAddressCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResendRecoveryEmailAddressCodeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResendRecoveryEmailAddressCodeBuilder {
  inner: ResendRecoveryEmailAddressCode
}

impl RTDResendRecoveryEmailAddressCodeBuilder {
  pub fn build(&self) -> ResendRecoveryEmailAddressCode { self.inner.clone() }

}

impl AsRef<ResendRecoveryEmailAddressCode> for ResendRecoveryEmailAddressCode {
  fn as_ref(&self) -> &ResendRecoveryEmailAddressCode { self }
}

impl AsRef<ResendRecoveryEmailAddressCode> for RTDResendRecoveryEmailAddressCodeBuilder {
  fn as_ref(&self) -> &ResendRecoveryEmailAddressCode { &self.inner }
}







/// Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to "default" and message previews are shown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetAllNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResetAllNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resetAllNotificationSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResetAllNotificationSettings {}

impl ResetAllNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResetAllNotificationSettingsBuilder {
    let mut inner = ResetAllNotificationSettings::default();
    inner.td_name = "resetAllNotificationSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResetAllNotificationSettingsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResetAllNotificationSettingsBuilder {
  inner: ResetAllNotificationSettings
}

impl RTDResetAllNotificationSettingsBuilder {
  pub fn build(&self) -> ResetAllNotificationSettings { self.inner.clone() }

}

impl AsRef<ResetAllNotificationSettings> for ResetAllNotificationSettings {
  fn as_ref(&self) -> &ResetAllNotificationSettings { self }
}

impl AsRef<ResetAllNotificationSettings> for RTDResetAllNotificationSettingsBuilder {
  fn as_ref(&self) -> &ResetAllNotificationSettings { &self.inner }
}







/// Resets list of installed backgrounds to its default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetBackgrounds {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResetBackgrounds {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resetBackgrounds" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResetBackgrounds {}

impl ResetBackgrounds {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResetBackgroundsBuilder {
    let mut inner = ResetBackgrounds::default();
    inner.td_name = "resetBackgrounds".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResetBackgroundsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResetBackgroundsBuilder {
  inner: ResetBackgrounds
}

impl RTDResetBackgroundsBuilder {
  pub fn build(&self) -> ResetBackgrounds { self.inner.clone() }

}

impl AsRef<ResetBackgrounds> for ResetBackgrounds {
  fn as_ref(&self) -> &ResetBackgrounds { self }
}

impl AsRef<ResetBackgrounds> for RTDResetBackgroundsBuilder {
  fn as_ref(&self) -> &ResetBackgrounds { &self.inner }
}







/// Resets all network data usage statistics to zero. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetNetworkStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ResetNetworkStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resetNetworkStatistics" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ResetNetworkStatistics {}

impl ResetNetworkStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDResetNetworkStatisticsBuilder {
    let mut inner = ResetNetworkStatistics::default();
    inner.td_name = "resetNetworkStatistics".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDResetNetworkStatisticsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDResetNetworkStatisticsBuilder {
  inner: ResetNetworkStatistics
}

impl RTDResetNetworkStatisticsBuilder {
  pub fn build(&self) -> ResetNetworkStatistics { self.inner.clone() }

}

impl AsRef<ResetNetworkStatistics> for ResetNetworkStatistics {
  fn as_ref(&self) -> &ResetNetworkStatistics { self }
}

impl AsRef<ResetNetworkStatistics> for RTDResetNetworkStatisticsBuilder {
  fn as_ref(&self) -> &ResetNetworkStatistics { &self.inner }
}







/// Saves application log event on the server. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SaveApplicationLogEvent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Event type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: String,
  /// Optional chat identifier, associated with the event
  chat_id: i64,
  /// The log event data
  data: JsonValue,
  
}

impl RObject for SaveApplicationLogEvent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "saveApplicationLogEvent" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SaveApplicationLogEvent {}

impl SaveApplicationLogEvent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSaveApplicationLogEventBuilder {
    let mut inner = SaveApplicationLogEvent::default();
    inner.td_name = "saveApplicationLogEvent".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSaveApplicationLogEventBuilder { inner }
  }

  pub fn type_(&self) -> &String { &self.type_ }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn data(&self) -> &JsonValue { &self.data }

}

#[doc(hidden)]
pub struct RTDSaveApplicationLogEventBuilder {
  inner: SaveApplicationLogEvent
}

impl RTDSaveApplicationLogEventBuilder {
  pub fn build(&self) -> SaveApplicationLogEvent { self.inner.clone() }

   
  pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().to_string();
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn data<T: AsRef<JsonValue>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().clone();
    self
  }

}

impl AsRef<SaveApplicationLogEvent> for SaveApplicationLogEvent {
  fn as_ref(&self) -> &SaveApplicationLogEvent { self }
}

impl AsRef<SaveApplicationLogEvent> for RTDSaveApplicationLogEventBuilder {
  fn as_ref(&self) -> &SaveApplicationLogEvent { &self.inner }
}







/// Searches for a background by its name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The name of the background
  name: String,
  
}

impl RObject for SearchBackground {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchBackground" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchBackground {}

impl SearchBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchBackgroundBuilder {
    let mut inner = SearchBackground::default();
    inner.td_name = "searchBackground".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchBackgroundBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

}

#[doc(hidden)]
pub struct RTDSearchBackgroundBuilder {
  inner: SearchBackground
}

impl RTDSearchBackgroundBuilder {
  pub fn build(&self) -> SearchBackground { self.inner.clone() }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

}

impl AsRef<SearchBackground> for SearchBackground {
  fn as_ref(&self) -> &SearchBackground { self }
}

impl AsRef<SearchBackground> for RTDSearchBackgroundBuilder {
  fn as_ref(&self) -> &SearchBackground { &self.inner }
}







/// Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchCallMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the message from which to search; use 0 to get results from the last message
  from_message_id: i64,
  /// The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached
  limit: i64,
  /// If true, returns only messages with missed calls
  only_missed: bool,
  
}

impl RObject for SearchCallMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchCallMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchCallMessages {}

impl SearchCallMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchCallMessagesBuilder {
    let mut inner = SearchCallMessages::default();
    inner.td_name = "searchCallMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchCallMessagesBuilder { inner }
  }

  pub fn from_message_id(&self) -> i64 { self.from_message_id }

  pub fn limit(&self) -> i64 { self.limit }

  pub fn only_missed(&self) -> bool { self.only_missed }

}

#[doc(hidden)]
pub struct RTDSearchCallMessagesBuilder {
  inner: SearchCallMessages
}

impl RTDSearchCallMessagesBuilder {
  pub fn build(&self) -> SearchCallMessages { self.inner.clone() }

   
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
    self.inner.from_message_id = from_message_id;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

   
  pub fn only_missed(&mut self, only_missed: bool) -> &mut Self {
    self.inner.only_missed = only_missed;
    self
  }

}

impl AsRef<SearchCallMessages> for SearchCallMessages {
  fn as_ref(&self) -> &SearchCallMessages { self }
}

impl AsRef<SearchCallMessages> for RTDSearchCallMessagesBuilder {
  fn as_ref(&self) -> &SearchCallMessages { &self.inner }
}







/// Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Query to search for
  query: String,
  /// The maximum number of users to be returned
  limit: i64,
  /// The type of users to return. By default, chatMembersFilterMembers
  filter: ChatMembersFilter,
  
}

impl RObject for SearchChatMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatMembers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchChatMembers {}

impl SearchChatMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchChatMembersBuilder {
    let mut inner = SearchChatMembers::default();
    inner.td_name = "searchChatMembers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchChatMembersBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn query(&self) -> &String { &self.query }

  pub fn limit(&self) -> i64 { self.limit }

  pub fn filter(&self) -> &ChatMembersFilter { &self.filter }

}

#[doc(hidden)]
pub struct RTDSearchChatMembersBuilder {
  inner: SearchChatMembers
}

impl RTDSearchChatMembersBuilder {
  pub fn build(&self) -> SearchChatMembers { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

   
  pub fn filter<T: AsRef<ChatMembersFilter>>(&mut self, filter: T) -> &mut Self {
    self.inner.filter = filter.as_ref().clone();
    self
  }

}

impl AsRef<SearchChatMembers> for SearchChatMembers {
  fn as_ref(&self) -> &SearchChatMembers { self }
}

impl AsRef<SearchChatMembers> for RTDSearchChatMembersBuilder {
  fn as_ref(&self) -> &SearchChatMembers { &self.inner }
}







/// Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query (searchSecretMessages should be used instead), or without an enabled message database. For optimal performance the number of returned messages is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat in which to search messages
  chat_id: i64,
  /// Query to search for
  query: String,
  /// If not 0, only messages sent by the specified user will be returned. Not supported in secret chats
  sender_user_id: i64,
  /// Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
  from_message_id: i64,
  /// Specify 0 to get results from exactly the from_message_id or a negative offset to get the specified message and some newer messages
  offset: i64,
  /// The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached
  limit: i64,
  /// Filter for message content in the search results
  filter: SearchMessagesFilter,
  
}

impl RObject for SearchChatMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchChatMessages {}

impl SearchChatMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchChatMessagesBuilder {
    let mut inner = SearchChatMessages::default();
    inner.td_name = "searchChatMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchChatMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn query(&self) -> &String { &self.query }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn from_message_id(&self) -> i64 { self.from_message_id }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn limit(&self) -> i64 { self.limit }

  pub fn filter(&self) -> &SearchMessagesFilter { &self.filter }

}

#[doc(hidden)]
pub struct RTDSearchChatMessagesBuilder {
  inner: SearchChatMessages
}

impl RTDSearchChatMessagesBuilder {
  pub fn build(&self) -> SearchChatMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
    self.inner.from_message_id = from_message_id;
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

   
  pub fn filter<T: AsRef<SearchMessagesFilter>>(&mut self, filter: T) -> &mut Self {
    self.inner.filter = filter.as_ref().clone();
    self
  }

}

impl AsRef<SearchChatMessages> for SearchChatMessages {
  fn as_ref(&self) -> &SearchChatMessages { self }
}

impl AsRef<SearchChatMessages> for RTDSearchChatMessagesBuilder {
  fn as_ref(&self) -> &SearchChatMessages { &self.inner }
}







/// Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatRecentLocationMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The maximum number of messages to be returned
  limit: i64,
  
}

impl RObject for SearchChatRecentLocationMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatRecentLocationMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchChatRecentLocationMessages {}

impl SearchChatRecentLocationMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchChatRecentLocationMessagesBuilder {
    let mut inner = SearchChatRecentLocationMessages::default();
    inner.td_name = "searchChatRecentLocationMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchChatRecentLocationMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDSearchChatRecentLocationMessagesBuilder {
  inner: SearchChatRecentLocationMessages
}

impl RTDSearchChatRecentLocationMessagesBuilder {
  pub fn build(&self) -> SearchChatRecentLocationMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<SearchChatRecentLocationMessages> for SearchChatRecentLocationMessages {
  fn as_ref(&self) -> &SearchChatRecentLocationMessages { self }
}

impl AsRef<SearchChatRecentLocationMessages> for RTDSearchChatRecentLocationMessagesBuilder {
  fn as_ref(&self) -> &SearchChatRecentLocationMessages { &self.inner }
}







/// Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Query to search for. If the query is empty, returns up to 20 recently found chats
  query: String,
  /// The maximum number of chats to be returned
  limit: i64,
  
}

impl RObject for SearchChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchChats {}

impl SearchChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchChatsBuilder {
    let mut inner = SearchChats::default();
    inner.td_name = "searchChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchChatsBuilder { inner }
  }

  pub fn query(&self) -> &String { &self.query }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDSearchChatsBuilder {
  inner: SearchChats
}

impl RTDSearchChatsBuilder {
  pub fn build(&self) -> SearchChats { self.inner.clone() }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<SearchChats> for SearchChats {
  fn as_ref(&self) -> &SearchChats { self }
}

impl AsRef<SearchChats> for RTDSearchChatsBuilder {
  fn as_ref(&self) -> &SearchChats { &self.inner }
}







/// Returns a list of users and location-based supergroups nearby. The list of users nearby will be updated for 60 seconds after the request by the updates updateUsersNearby. The request should be sent again every 25 seconds with adjusted location to not miss new chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatsNearby {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Current user location
  location: Location,
  
}

impl RObject for SearchChatsNearby {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatsNearby" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchChatsNearby {}

impl SearchChatsNearby {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchChatsNearbyBuilder {
    let mut inner = SearchChatsNearby::default();
    inner.td_name = "searchChatsNearby".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchChatsNearbyBuilder { inner }
  }

  pub fn location(&self) -> &Location { &self.location }

}

#[doc(hidden)]
pub struct RTDSearchChatsNearbyBuilder {
  inner: SearchChatsNearby
}

impl RTDSearchChatsNearbyBuilder {
  pub fn build(&self) -> SearchChatsNearby { self.inner.clone() }

   
  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

}

impl AsRef<SearchChatsNearby> for SearchChatsNearby {
  fn as_ref(&self) -> &SearchChatsNearby { self }
}

impl AsRef<SearchChatsNearby> for RTDSearchChatsNearbyBuilder {
  fn as_ref(&self) -> &SearchChatsNearby { &self.inner }
}







/// Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchChatsOnServer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Query to search for
  query: String,
  /// The maximum number of chats to be returned
  limit: i64,
  
}

impl RObject for SearchChatsOnServer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchChatsOnServer" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchChatsOnServer {}

impl SearchChatsOnServer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchChatsOnServerBuilder {
    let mut inner = SearchChatsOnServer::default();
    inner.td_name = "searchChatsOnServer".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchChatsOnServerBuilder { inner }
  }

  pub fn query(&self) -> &String { &self.query }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDSearchChatsOnServerBuilder {
  inner: SearchChatsOnServer
}

impl RTDSearchChatsOnServerBuilder {
  pub fn build(&self) -> SearchChatsOnServer { self.inner.clone() }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<SearchChatsOnServer> for SearchChatsOnServer {
  fn as_ref(&self) -> &SearchChatsOnServer { self }
}

impl AsRef<SearchChatsOnServer> for RTDSearchChatsOnServerBuilder {
  fn as_ref(&self) -> &SearchChatsOnServer { &self.inner }
}







/// Searches for the specified query in the first names, last names and usernames of the known user contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Query to search for; may be empty to return all contacts
  query: String,
  /// The maximum number of users to be returned
  limit: i64,
  
}

impl RObject for SearchContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchContacts" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchContacts {}

impl SearchContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchContactsBuilder {
    let mut inner = SearchContacts::default();
    inner.td_name = "searchContacts".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchContactsBuilder { inner }
  }

  pub fn query(&self) -> &String { &self.query }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDSearchContactsBuilder {
  inner: SearchContacts
}

impl RTDSearchContactsBuilder {
  pub fn build(&self) -> SearchContacts { self.inner.clone() }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<SearchContacts> for SearchContacts {
  fn as_ref(&self) -> &SearchContacts { self }
}

impl AsRef<SearchContacts> for RTDSearchContactsBuilder {
  fn as_ref(&self) -> &SearchContacts { &self.inner }
}







/// Searches for emojis by keywords. Supported only if the file database is enabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchEmojis {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text to search for
  text: String,
  /// True, if only emojis, which exactly match text needs to be returned
  exact_match: bool,
  /// IETF language tag of the user's input language; may be empty if unknown
  input_language_code: String,
  
}

impl RObject for SearchEmojis {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchEmojis" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchEmojis {}

impl SearchEmojis {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchEmojisBuilder {
    let mut inner = SearchEmojis::default();
    inner.td_name = "searchEmojis".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchEmojisBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

  pub fn exact_match(&self) -> bool { self.exact_match }

  pub fn input_language_code(&self) -> &String { &self.input_language_code }

}

#[doc(hidden)]
pub struct RTDSearchEmojisBuilder {
  inner: SearchEmojis
}

impl RTDSearchEmojisBuilder {
  pub fn build(&self) -> SearchEmojis { self.inner.clone() }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

   
  pub fn exact_match(&mut self, exact_match: bool) -> &mut Self {
    self.inner.exact_match = exact_match;
    self
  }

   
  pub fn input_language_code<T: AsRef<str>>(&mut self, input_language_code: T) -> &mut Self {
    self.inner.input_language_code = input_language_code.as_ref().to_string();
    self
  }

}

impl AsRef<SearchEmojis> for SearchEmojis {
  fn as_ref(&self) -> &SearchEmojis { self }
}

impl AsRef<SearchEmojis> for RTDSearchEmojisBuilder {
  fn as_ref(&self) -> &SearchEmojis { &self.inner }
}







/// Searches for recently used hashtags by their prefix
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchHashtags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hashtag prefix to search for
  prefix: String,
  /// The maximum number of hashtags to be returned
  limit: i64,
  
}

impl RObject for SearchHashtags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchHashtags" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchHashtags {}

impl SearchHashtags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchHashtagsBuilder {
    let mut inner = SearchHashtags::default();
    inner.td_name = "searchHashtags".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchHashtagsBuilder { inner }
  }

  pub fn prefix(&self) -> &String { &self.prefix }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDSearchHashtagsBuilder {
  inner: SearchHashtags
}

impl RTDSearchHashtagsBuilder {
  pub fn build(&self) -> SearchHashtags { self.inner.clone() }

   
  pub fn prefix<T: AsRef<str>>(&mut self, prefix: T) -> &mut Self {
    self.inner.prefix = prefix.as_ref().to_string();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<SearchHashtags> for SearchHashtags {
  fn as_ref(&self) -> &SearchHashtags { self }
}

impl AsRef<SearchHashtags> for RTDSearchHashtagsBuilder {
  fn as_ref(&self) -> &SearchHashtags { &self.inner }
}







/// Searches for installed sticker sets by looking for specified query in their title and name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchInstalledStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to return mask sticker sets; pass false to return ordinary sticker sets
  is_masks: bool,
  /// Query to search for
  query: String,
  /// The maximum number of sticker sets to return
  limit: i64,
  
}

impl RObject for SearchInstalledStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchInstalledStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchInstalledStickerSets {}

impl SearchInstalledStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchInstalledStickerSetsBuilder {
    let mut inner = SearchInstalledStickerSets::default();
    inner.td_name = "searchInstalledStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchInstalledStickerSetsBuilder { inner }
  }

  pub fn is_masks(&self) -> bool { self.is_masks }

  pub fn query(&self) -> &String { &self.query }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDSearchInstalledStickerSetsBuilder {
  inner: SearchInstalledStickerSets
}

impl RTDSearchInstalledStickerSetsBuilder {
  pub fn build(&self) -> SearchInstalledStickerSets { self.inner.clone() }

   
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.is_masks = is_masks;
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<SearchInstalledStickerSets> for SearchInstalledStickerSets {
  fn as_ref(&self) -> &SearchInstalledStickerSets { self }
}

impl AsRef<SearchInstalledStickerSets> for RTDSearchInstalledStickerSetsBuilder {
  fn as_ref(&self) -> &SearchInstalledStickerSets { &self.inner }
}







/// Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance the number of returned messages is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat list in which to search messages; pass null to search in all chats regardless of their chat list
  chat_list: ChatList,
  /// Query to search for
  query: String,
  /// The date of the message starting from which the results should be fetched. Use 0 or any date in the future to get results from the last message
  offset_date: i64,
  /// The chat identifier of the last found message, or 0 for the first request
  offset_chat_id: i64,
  /// The message identifier of the last found message, or 0 for the first request
  offset_message_id: i64,
  /// The maximum number of messages to be returned, up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached
  limit: i64,
  
}

impl RObject for SearchMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchMessages {}

impl SearchMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesBuilder {
    let mut inner = SearchMessages::default();
    inner.td_name = "searchMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchMessagesBuilder { inner }
  }

  pub fn chat_list(&self) -> &ChatList { &self.chat_list }

  pub fn query(&self) -> &String { &self.query }

  pub fn offset_date(&self) -> i64 { self.offset_date }

  pub fn offset_chat_id(&self) -> i64 { self.offset_chat_id }

  pub fn offset_message_id(&self) -> i64 { self.offset_message_id }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDSearchMessagesBuilder {
  inner: SearchMessages
}

impl RTDSearchMessagesBuilder {
  pub fn build(&self) -> SearchMessages { self.inner.clone() }

   
  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = chat_list.as_ref().clone();
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn offset_date(&mut self, offset_date: i64) -> &mut Self {
    self.inner.offset_date = offset_date;
    self
  }

   
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
    self.inner.offset_chat_id = offset_chat_id;
    self
  }

   
  pub fn offset_message_id(&mut self, offset_message_id: i64) -> &mut Self {
    self.inner.offset_message_id = offset_message_id;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<SearchMessages> for SearchMessages {
  fn as_ref(&self) -> &SearchMessages { self }
}

impl AsRef<SearchMessages> for RTDSearchMessagesBuilder {
  fn as_ref(&self) -> &SearchMessages { &self.inner }
}







/// Searches a public chat by its username. Currently only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchPublicChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Username to be resolved
  username: String,
  
}

impl RObject for SearchPublicChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchPublicChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchPublicChat {}

impl SearchPublicChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchPublicChatBuilder {
    let mut inner = SearchPublicChat::default();
    inner.td_name = "searchPublicChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchPublicChatBuilder { inner }
  }

  pub fn username(&self) -> &String { &self.username }

}

#[doc(hidden)]
pub struct RTDSearchPublicChatBuilder {
  inner: SearchPublicChat
}

impl RTDSearchPublicChatBuilder {
  pub fn build(&self) -> SearchPublicChat { self.inner.clone() }

   
  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

}

impl AsRef<SearchPublicChat> for SearchPublicChat {
  fn as_ref(&self) -> &SearchPublicChat { self }
}

impl AsRef<SearchPublicChat> for RTDSearchPublicChatBuilder {
  fn as_ref(&self) -> &SearchPublicChat { &self.inner }
}







/// Searches public chats by looking for specified query in their username and title. Currently only private chats, supergroups and channels can be public. Returns a meaningful number of results. Returns nothing if the length of the searched username prefix is less than 5. Excludes private chats with contacts and chats from the chat list from the results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchPublicChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Query to search for
  query: String,
  
}

impl RObject for SearchPublicChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchPublicChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchPublicChats {}

impl SearchPublicChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchPublicChatsBuilder {
    let mut inner = SearchPublicChats::default();
    inner.td_name = "searchPublicChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchPublicChatsBuilder { inner }
  }

  pub fn query(&self) -> &String { &self.query }

}

#[doc(hidden)]
pub struct RTDSearchPublicChatsBuilder {
  inner: SearchPublicChats
}

impl RTDSearchPublicChatsBuilder {
  pub fn build(&self) -> SearchPublicChats { self.inner.clone() }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

}

impl AsRef<SearchPublicChats> for SearchPublicChats {
  fn as_ref(&self) -> &SearchPublicChats { self }
}

impl AsRef<SearchPublicChats> for RTDSearchPublicChatsBuilder {
  fn as_ref(&self) -> &SearchPublicChats { &self.inner }
}







/// Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance the number of returned messages is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchSecretMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat in which to search. Specify 0 to search in all secret chats
  chat_id: i64,
  /// Query to search for. If empty, searchChatMessages should be used instead
  query: String,
  /// The identifier from the result of a previous request, use 0 to get results from the last message
  from_search_id: isize,
  /// The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached
  limit: i64,
  /// A filter for the content of messages in the search results
  filter: SearchMessagesFilter,
  
}

impl RObject for SearchSecretMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchSecretMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchSecretMessages {}

impl SearchSecretMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchSecretMessagesBuilder {
    let mut inner = SearchSecretMessages::default();
    inner.td_name = "searchSecretMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchSecretMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn query(&self) -> &String { &self.query }

  pub fn from_search_id(&self) -> isize { self.from_search_id }

  pub fn limit(&self) -> i64 { self.limit }

  pub fn filter(&self) -> &SearchMessagesFilter { &self.filter }

}

#[doc(hidden)]
pub struct RTDSearchSecretMessagesBuilder {
  inner: SearchSecretMessages
}

impl RTDSearchSecretMessagesBuilder {
  pub fn build(&self) -> SearchSecretMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn from_search_id(&mut self, from_search_id: isize) -> &mut Self {
    self.inner.from_search_id = from_search_id;
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

   
  pub fn filter<T: AsRef<SearchMessagesFilter>>(&mut self, filter: T) -> &mut Self {
    self.inner.filter = filter.as_ref().clone();
    self
  }

}

impl AsRef<SearchSecretMessages> for SearchSecretMessages {
  fn as_ref(&self) -> &SearchSecretMessages { self }
}

impl AsRef<SearchSecretMessages> for RTDSearchSecretMessagesBuilder {
  fn as_ref(&self) -> &SearchSecretMessages { &self.inner }
}







/// Searches for a sticker set by its name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the sticker set
  name: String,
  
}

impl RObject for SearchStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchStickerSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchStickerSet {}

impl SearchStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchStickerSetBuilder {
    let mut inner = SearchStickerSet::default();
    inner.td_name = "searchStickerSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchStickerSetBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

}

#[doc(hidden)]
pub struct RTDSearchStickerSetBuilder {
  inner: SearchStickerSet
}

impl RTDSearchStickerSetBuilder {
  pub fn build(&self) -> SearchStickerSet { self.inner.clone() }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

}

impl AsRef<SearchStickerSet> for SearchStickerSet {
  fn as_ref(&self) -> &SearchStickerSet { self }
}

impl AsRef<SearchStickerSet> for RTDSearchStickerSetBuilder {
  fn as_ref(&self) -> &SearchStickerSet { &self.inner }
}







/// Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Query to search for
  query: String,
  
}

impl RObject for SearchStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchStickerSets {}

impl SearchStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchStickerSetsBuilder {
    let mut inner = SearchStickerSets::default();
    inner.td_name = "searchStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchStickerSetsBuilder { inner }
  }

  pub fn query(&self) -> &String { &self.query }

}

#[doc(hidden)]
pub struct RTDSearchStickerSetsBuilder {
  inner: SearchStickerSets
}

impl RTDSearchStickerSetsBuilder {
  pub fn build(&self) -> SearchStickerSets { self.inner.clone() }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

}

impl AsRef<SearchStickerSets> for SearchStickerSets {
  fn as_ref(&self) -> &SearchStickerSets { self }
}

impl AsRef<SearchStickerSets> for RTDSearchStickerSetsBuilder {
  fn as_ref(&self) -> &SearchStickerSets { &self.inner }
}







/// Searches for stickers from public sticker sets that correspond to a given emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// String representation of emoji; must be non-empty
  emoji: String,
  /// The maximum number of stickers to be returned
  limit: i64,
  
}

impl RObject for SearchStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchStickers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SearchStickers {}

impl SearchStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchStickersBuilder {
    let mut inner = SearchStickers::default();
    inner.td_name = "searchStickers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSearchStickersBuilder { inner }
  }

  pub fn emoji(&self) -> &String { &self.emoji }

  pub fn limit(&self) -> i64 { self.limit }

}

#[doc(hidden)]
pub struct RTDSearchStickersBuilder {
  inner: SearchStickers
}

impl RTDSearchStickersBuilder {
  pub fn build(&self) -> SearchStickers { self.inner.clone() }

   
  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

   
  pub fn limit(&mut self, limit: i64) -> &mut Self {
    self.inner.limit = limit;
    self
  }

}

impl AsRef<SearchStickers> for SearchStickers {
  fn as_ref(&self) -> &SearchStickers { self }
}

impl AsRef<SearchStickers> for RTDSearchStickersBuilder {
  fn as_ref(&self) -> &SearchStickers { &self.inner }
}







/// Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendBotStartMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the bot
  bot_user_id: i64,
  /// Identifier of the target chat
  chat_id: i64,
  /// A hidden parameter sent to the bot for deep linking purposes (https://core.telegram.org/bots#deep-linking)
  parameter: String,
  
}

impl RObject for SendBotStartMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendBotStartMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendBotStartMessage {}

impl SendBotStartMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendBotStartMessageBuilder {
    let mut inner = SendBotStartMessage::default();
    inner.td_name = "sendBotStartMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendBotStartMessageBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 { self.bot_user_id }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn parameter(&self) -> &String { &self.parameter }

}

#[doc(hidden)]
pub struct RTDSendBotStartMessageBuilder {
  inner: SendBotStartMessage
}

impl RTDSendBotStartMessageBuilder {
  pub fn build(&self) -> SendBotStartMessage { self.inner.clone() }

   
  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn parameter<T: AsRef<str>>(&mut self, parameter: T) -> &mut Self {
    self.inner.parameter = parameter.as_ref().to_string();
    self
  }

}

impl AsRef<SendBotStartMessage> for SendBotStartMessage {
  fn as_ref(&self) -> &SendBotStartMessage { self }
}

impl AsRef<SendBotStartMessage> for RTDSendBotStartMessageBuilder {
  fn as_ref(&self) -> &SendBotStartMessage { &self.inner }
}







/// Sends debug information for a call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCallDebugInformation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Call identifier
  call_id: i64,
  /// Debug information in application-specific format
  debug_information: String,
  
}

impl RObject for SendCallDebugInformation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendCallDebugInformation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendCallDebugInformation {}

impl SendCallDebugInformation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendCallDebugInformationBuilder {
    let mut inner = SendCallDebugInformation::default();
    inner.td_name = "sendCallDebugInformation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendCallDebugInformationBuilder { inner }
  }

  pub fn call_id(&self) -> i64 { self.call_id }

  pub fn debug_information(&self) -> &String { &self.debug_information }

}

#[doc(hidden)]
pub struct RTDSendCallDebugInformationBuilder {
  inner: SendCallDebugInformation
}

impl RTDSendCallDebugInformationBuilder {
  pub fn build(&self) -> SendCallDebugInformation { self.inner.clone() }

   
  pub fn call_id(&mut self, call_id: i64) -> &mut Self {
    self.inner.call_id = call_id;
    self
  }

   
  pub fn debug_information<T: AsRef<str>>(&mut self, debug_information: T) -> &mut Self {
    self.inner.debug_information = debug_information.as_ref().to_string();
    self
  }

}

impl AsRef<SendCallDebugInformation> for SendCallDebugInformation {
  fn as_ref(&self) -> &SendCallDebugInformation { self }
}

impl AsRef<SendCallDebugInformation> for RTDSendCallDebugInformationBuilder {
  fn as_ref(&self) -> &SendCallDebugInformation { &self.inner }
}







/// Sends a call rating
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCallRating {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Call identifier
  call_id: i64,
  /// Call rating; 1-5
  rating: i64,
  /// An optional user comment if the rating is less than 5
  comment: String,
  /// List of the exact types of problems with the call, specified by the user
  problems: Vec<CallProblem>,
  
}

impl RObject for SendCallRating {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendCallRating" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendCallRating {}

impl SendCallRating {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendCallRatingBuilder {
    let mut inner = SendCallRating::default();
    inner.td_name = "sendCallRating".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendCallRatingBuilder { inner }
  }

  pub fn call_id(&self) -> i64 { self.call_id }

  pub fn rating(&self) -> i64 { self.rating }

  pub fn comment(&self) -> &String { &self.comment }

  pub fn problems(&self) -> &Vec<CallProblem> { &self.problems }

}

#[doc(hidden)]
pub struct RTDSendCallRatingBuilder {
  inner: SendCallRating
}

impl RTDSendCallRatingBuilder {
  pub fn build(&self) -> SendCallRating { self.inner.clone() }

   
  pub fn call_id(&mut self, call_id: i64) -> &mut Self {
    self.inner.call_id = call_id;
    self
  }

   
  pub fn rating(&mut self, rating: i64) -> &mut Self {
    self.inner.rating = rating;
    self
  }

   
  pub fn comment<T: AsRef<str>>(&mut self, comment: T) -> &mut Self {
    self.inner.comment = comment.as_ref().to_string();
    self
  }

   
  pub fn problems(&mut self, problems: Vec<CallProblem>) -> &mut Self {
    self.inner.problems = problems;
    self
  }

}

impl AsRef<SendCallRating> for SendCallRating {
  fn as_ref(&self) -> &SendCallRating { self }
}

impl AsRef<SendCallRating> for RTDSendCallRatingBuilder {
  fn as_ref(&self) -> &SendCallRating { &self.inner }
}







/// Sends a notification about user activity in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendChatAction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The action description
  action: ChatAction,
  
}

impl RObject for SendChatAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendChatAction" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendChatAction {}

impl SendChatAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendChatActionBuilder {
    let mut inner = SendChatAction::default();
    inner.td_name = "sendChatAction".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendChatActionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn action(&self) -> &ChatAction { &self.action }

}

#[doc(hidden)]
pub struct RTDSendChatActionBuilder {
  inner: SendChatAction
}

impl RTDSendChatActionBuilder {
  pub fn build(&self) -> SendChatAction { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn action<T: AsRef<ChatAction>>(&mut self, action: T) -> &mut Self {
    self.inner.action = action.as_ref().clone();
    self
  }

}

impl AsRef<SendChatAction> for SendChatAction {
  fn as_ref(&self) -> &SendChatAction { self }
}

impl AsRef<SendChatAction> for RTDSendChatActionBuilder {
  fn as_ref(&self) -> &SendChatAction { &self.inner }
}







/// Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendChatScreenshotTakenNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for SendChatScreenshotTakenNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendChatScreenshotTakenNotification" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendChatScreenshotTakenNotification {}

impl SendChatScreenshotTakenNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendChatScreenshotTakenNotificationBuilder {
    let mut inner = SendChatScreenshotTakenNotification::default();
    inner.td_name = "sendChatScreenshotTakenNotification".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendChatScreenshotTakenNotificationBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDSendChatScreenshotTakenNotificationBuilder {
  inner: SendChatScreenshotTakenNotification
}

impl RTDSendChatScreenshotTakenNotificationBuilder {
  pub fn build(&self) -> SendChatScreenshotTakenNotification { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<SendChatScreenshotTakenNotification> for SendChatScreenshotTakenNotification {
  fn as_ref(&self) -> &SendChatScreenshotTakenNotification { self }
}

impl AsRef<SendChatScreenshotTakenNotification> for RTDSendChatScreenshotTakenNotificationBuilder {
  fn as_ref(&self) -> &SendChatScreenshotTakenNotification { &self.inner }
}







/// Changes the current TTL setting (sets a new self-destruct timer) in a secret chat and sends the corresponding message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendChatSetTtlMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New TTL value, in seconds
  ttl: i64,
  
}

impl RObject for SendChatSetTtlMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendChatSetTtlMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendChatSetTtlMessage {}

impl SendChatSetTtlMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendChatSetTtlMessageBuilder {
    let mut inner = SendChatSetTtlMessage::default();
    inner.td_name = "sendChatSetTtlMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendChatSetTtlMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn ttl(&self) -> i64 { self.ttl }

}

#[doc(hidden)]
pub struct RTDSendChatSetTtlMessageBuilder {
  inner: SendChatSetTtlMessage
}

impl RTDSendChatSetTtlMessageBuilder {
  pub fn build(&self) -> SendChatSetTtlMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn ttl(&mut self, ttl: i64) -> &mut Self {
    self.inner.ttl = ttl;
    self
  }

}

impl AsRef<SendChatSetTtlMessage> for SendChatSetTtlMessage {
  fn as_ref(&self) -> &SendChatSetTtlMessage { self }
}

impl AsRef<SendChatSetTtlMessage> for RTDSendChatSetTtlMessageBuilder {
  fn as_ref(&self) -> &SendChatSetTtlMessage { &self.inner }
}







/// Sends a custom request; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCustomRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The method name
  method: String,
  /// JSON-serialized method parameters
  parameters: String,
  
}

impl RObject for SendCustomRequest {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendCustomRequest" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendCustomRequest {}

impl SendCustomRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendCustomRequestBuilder {
    let mut inner = SendCustomRequest::default();
    inner.td_name = "sendCustomRequest".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendCustomRequestBuilder { inner }
  }

  pub fn method(&self) -> &String { &self.method }

  pub fn parameters(&self) -> &String { &self.parameters }

}

#[doc(hidden)]
pub struct RTDSendCustomRequestBuilder {
  inner: SendCustomRequest
}

impl RTDSendCustomRequestBuilder {
  pub fn build(&self) -> SendCustomRequest { self.inner.clone() }

   
  pub fn method<T: AsRef<str>>(&mut self, method: T) -> &mut Self {
    self.inner.method = method.as_ref().to_string();
    self
  }

   
  pub fn parameters<T: AsRef<str>>(&mut self, parameters: T) -> &mut Self {
    self.inner.parameters = parameters.as_ref().to_string();
    self
  }

}

impl AsRef<SendCustomRequest> for SendCustomRequest {
  fn as_ref(&self) -> &SendCustomRequest { self }
}

impl AsRef<SendCustomRequest> for RTDSendCustomRequestBuilder {
  fn as_ref(&self) -> &SendCustomRequest { &self.inner }
}







/// Sends a code to verify an email address to be added to a user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendEmailAddressVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Email address
  email_address: String,
  
}

impl RObject for SendEmailAddressVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendEmailAddressVerificationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendEmailAddressVerificationCode {}

impl SendEmailAddressVerificationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendEmailAddressVerificationCodeBuilder {
    let mut inner = SendEmailAddressVerificationCode::default();
    inner.td_name = "sendEmailAddressVerificationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendEmailAddressVerificationCodeBuilder { inner }
  }

  pub fn email_address(&self) -> &String { &self.email_address }

}

#[doc(hidden)]
pub struct RTDSendEmailAddressVerificationCodeBuilder {
  inner: SendEmailAddressVerificationCode
}

impl RTDSendEmailAddressVerificationCodeBuilder {
  pub fn build(&self) -> SendEmailAddressVerificationCode { self.inner.clone() }

   
  pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
    self.inner.email_address = email_address.as_ref().to_string();
    self
  }

}

impl AsRef<SendEmailAddressVerificationCode> for SendEmailAddressVerificationCode {
  fn as_ref(&self) -> &SendEmailAddressVerificationCode { self }
}

impl AsRef<SendEmailAddressVerificationCode> for RTDSendEmailAddressVerificationCodeBuilder {
  fn as_ref(&self) -> &SendEmailAddressVerificationCode { &self.inner }
}







/// Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendInlineQueryResultMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Target chat
  chat_id: i64,
  /// Identifier of a message to reply to or 0
  reply_to_message_id: i64,
  /// Options to be used to send the message
  options: SendMessageOptions,
  /// Identifier of the inline query
  query_id: isize,
  /// Identifier of the inline result
  result_id: String,
  /// If true, there will be no mention of a bot, via which the message is sent. Can be used only for bots GetOption("animation_search_bot_username"), GetOption("photo_search_bot_username") and GetOption("venue_search_bot_username")
  hide_via_bot: bool,
  
}

impl RObject for SendInlineQueryResultMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendInlineQueryResultMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendInlineQueryResultMessage {}

impl SendInlineQueryResultMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendInlineQueryResultMessageBuilder {
    let mut inner = SendInlineQueryResultMessage::default();
    inner.td_name = "sendInlineQueryResultMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendInlineQueryResultMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn reply_to_message_id(&self) -> i64 { self.reply_to_message_id }

  pub fn options(&self) -> &SendMessageOptions { &self.options }

  pub fn query_id(&self) -> isize { self.query_id }

  pub fn result_id(&self) -> &String { &self.result_id }

  pub fn hide_via_bot(&self) -> bool { self.hide_via_bot }

}

#[doc(hidden)]
pub struct RTDSendInlineQueryResultMessageBuilder {
  inner: SendInlineQueryResultMessage
}

impl RTDSendInlineQueryResultMessageBuilder {
  pub fn build(&self) -> SendInlineQueryResultMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.reply_to_message_id = reply_to_message_id;
    self
  }

   
  pub fn options<T: AsRef<SendMessageOptions>>(&mut self, options: T) -> &mut Self {
    self.inner.options = options.as_ref().clone();
    self
  }

   
  pub fn query_id(&mut self, query_id: isize) -> &mut Self {
    self.inner.query_id = query_id;
    self
  }

   
  pub fn result_id<T: AsRef<str>>(&mut self, result_id: T) -> &mut Self {
    self.inner.result_id = result_id.as_ref().to_string();
    self
  }

   
  pub fn hide_via_bot(&mut self, hide_via_bot: bool) -> &mut Self {
    self.inner.hide_via_bot = hide_via_bot;
    self
  }

}

impl AsRef<SendInlineQueryResultMessage> for SendInlineQueryResultMessage {
  fn as_ref(&self) -> &SendInlineQueryResultMessage { self }
}

impl AsRef<SendInlineQueryResultMessage> for RTDSendInlineQueryResultMessageBuilder {
  fn as_ref(&self) -> &SendInlineQueryResultMessage { &self.inner }
}







/// Sends a message. Returns the sent message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Target chat
  chat_id: i64,
  /// Identifier of the message to reply to or 0
  reply_to_message_id: i64,
  /// Options to be used to send the message
  options: SendMessageOptions,
  /// Markup for replying to the message; for bots only
  reply_markup: ReplyMarkup,
  /// The content of the message to be sent
  input_message_content: InputMessageContent,
  
}

impl RObject for SendMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendMessage {}

impl SendMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendMessageBuilder {
    let mut inner = SendMessage::default();
    inner.td_name = "sendMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn reply_to_message_id(&self) -> i64 { self.reply_to_message_id }

  pub fn options(&self) -> &SendMessageOptions { &self.options }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

  pub fn input_message_content(&self) -> &InputMessageContent { &self.input_message_content }

}

#[doc(hidden)]
pub struct RTDSendMessageBuilder {
  inner: SendMessage
}

impl RTDSendMessageBuilder {
  pub fn build(&self) -> SendMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.reply_to_message_id = reply_to_message_id;
    self
  }

   
  pub fn options<T: AsRef<SendMessageOptions>>(&mut self, options: T) -> &mut Self {
    self.inner.options = options.as_ref().clone();
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

   
  pub fn input_message_content<T: AsRef<InputMessageContent>>(&mut self, input_message_content: T) -> &mut Self {
    self.inner.input_message_content = input_message_content.as_ref().clone();
    self
  }

}

impl AsRef<SendMessage> for SendMessage {
  fn as_ref(&self) -> &SendMessage { self }
}

impl AsRef<SendMessage> for RTDSendMessageBuilder {
  fn as_ref(&self) -> &SendMessage { &self.inner }
}







/// Sends messages grouped together into an album. Currently only photo and video messages can be grouped into an album. Returns sent messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageAlbum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Target chat
  chat_id: i64,
  /// Identifier of a message to reply to or 0
  reply_to_message_id: i64,
  /// Options to be used to send the messages
  options: SendMessageOptions,
  /// Contents of messages to be sent
  input_message_contents: Vec<InputMessageContent>,
  
}

impl RObject for SendMessageAlbum {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendMessageAlbum" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendMessageAlbum {}

impl SendMessageAlbum {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendMessageAlbumBuilder {
    let mut inner = SendMessageAlbum::default();
    inner.td_name = "sendMessageAlbum".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendMessageAlbumBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn reply_to_message_id(&self) -> i64 { self.reply_to_message_id }

  pub fn options(&self) -> &SendMessageOptions { &self.options }

  pub fn input_message_contents(&self) -> &Vec<InputMessageContent> { &self.input_message_contents }

}

#[doc(hidden)]
pub struct RTDSendMessageAlbumBuilder {
  inner: SendMessageAlbum
}

impl RTDSendMessageAlbumBuilder {
  pub fn build(&self) -> SendMessageAlbum { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.reply_to_message_id = reply_to_message_id;
    self
  }

   
  pub fn options<T: AsRef<SendMessageOptions>>(&mut self, options: T) -> &mut Self {
    self.inner.options = options.as_ref().clone();
    self
  }

   
  pub fn input_message_contents(&mut self, input_message_contents: Vec<InputMessageContent>) -> &mut Self {
    self.inner.input_message_contents = input_message_contents;
    self
  }

}

impl AsRef<SendMessageAlbum> for SendMessageAlbum {
  fn as_ref(&self) -> &SendMessageAlbum { self }
}

impl AsRef<SendMessageAlbum> for RTDSendMessageAlbumBuilder {
  fn as_ref(&self) -> &SendMessageAlbum { &self.inner }
}







/// Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements need to be used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPassportAuthorizationForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Authorization form identifier
  autorization_form_id: i64,
  /// Types of Telegram Passport elements chosen by user to complete the authorization form
  types: Vec<PassportElementType>,
  
}

impl RObject for SendPassportAuthorizationForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendPassportAuthorizationForm" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendPassportAuthorizationForm {}

impl SendPassportAuthorizationForm {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendPassportAuthorizationFormBuilder {
    let mut inner = SendPassportAuthorizationForm::default();
    inner.td_name = "sendPassportAuthorizationForm".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendPassportAuthorizationFormBuilder { inner }
  }

  pub fn autorization_form_id(&self) -> i64 { self.autorization_form_id }

  pub fn types(&self) -> &Vec<PassportElementType> { &self.types }

}

#[doc(hidden)]
pub struct RTDSendPassportAuthorizationFormBuilder {
  inner: SendPassportAuthorizationForm
}

impl RTDSendPassportAuthorizationFormBuilder {
  pub fn build(&self) -> SendPassportAuthorizationForm { self.inner.clone() }

   
  pub fn autorization_form_id(&mut self, autorization_form_id: i64) -> &mut Self {
    self.inner.autorization_form_id = autorization_form_id;
    self
  }

   
  pub fn types(&mut self, types: Vec<PassportElementType>) -> &mut Self {
    self.inner.types = types;
    self
  }

}

impl AsRef<SendPassportAuthorizationForm> for SendPassportAuthorizationForm {
  fn as_ref(&self) -> &SendPassportAuthorizationForm { self }
}

impl AsRef<SendPassportAuthorizationForm> for RTDSendPassportAuthorizationFormBuilder {
  fn as_ref(&self) -> &SendPassportAuthorizationForm { &self.inner }
}







/// Sends a filled-out payment form to the bot for final verification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPaymentForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the Invoice message
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  /// Identifier returned by ValidateOrderInfo, or an empty string
  order_info_id: String,
  /// Identifier of a chosen shipping option, if applicable
  shipping_option_id: String,
  /// The credentials chosen by user for payment
  credentials: InputCredentials,
  
}

impl RObject for SendPaymentForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendPaymentForm" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendPaymentForm {}

impl SendPaymentForm {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendPaymentFormBuilder {
    let mut inner = SendPaymentForm::default();
    inner.td_name = "sendPaymentForm".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendPaymentFormBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn order_info_id(&self) -> &String { &self.order_info_id }

  pub fn shipping_option_id(&self) -> &String { &self.shipping_option_id }

  pub fn credentials(&self) -> &InputCredentials { &self.credentials }

}

#[doc(hidden)]
pub struct RTDSendPaymentFormBuilder {
  inner: SendPaymentForm
}

impl RTDSendPaymentFormBuilder {
  pub fn build(&self) -> SendPaymentForm { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn order_info_id<T: AsRef<str>>(&mut self, order_info_id: T) -> &mut Self {
    self.inner.order_info_id = order_info_id.as_ref().to_string();
    self
  }

   
  pub fn shipping_option_id<T: AsRef<str>>(&mut self, shipping_option_id: T) -> &mut Self {
    self.inner.shipping_option_id = shipping_option_id.as_ref().to_string();
    self
  }

   
  pub fn credentials<T: AsRef<InputCredentials>>(&mut self, credentials: T) -> &mut Self {
    self.inner.credentials = credentials.as_ref().clone();
    self
  }

}

impl AsRef<SendPaymentForm> for SendPaymentForm {
  fn as_ref(&self) -> &SendPaymentForm { self }
}

impl AsRef<SendPaymentForm> for RTDSendPaymentFormBuilder {
  fn as_ref(&self) -> &SendPaymentForm { &self.inner }
}







/// Sends phone number confirmation code. Should be called when user presses "https://t.me/confirmphone?phone=*******&hash=**********" or "tg://confirmphone?phone=*******&hash=**********" link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPhoneNumberConfirmationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Value of the "hash" parameter from the link
  hash: String,
  /// Value of the "phone" parameter from the link
  phone_number: String,
  /// Settings for the authentication of the user's phone number
  settings: PhoneNumberAuthenticationSettings,
  
}

impl RObject for SendPhoneNumberConfirmationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendPhoneNumberConfirmationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendPhoneNumberConfirmationCode {}

impl SendPhoneNumberConfirmationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendPhoneNumberConfirmationCodeBuilder {
    let mut inner = SendPhoneNumberConfirmationCode::default();
    inner.td_name = "sendPhoneNumberConfirmationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendPhoneNumberConfirmationCodeBuilder { inner }
  }

  pub fn hash(&self) -> &String { &self.hash }

  pub fn phone_number(&self) -> &String { &self.phone_number }

  pub fn settings(&self) -> &PhoneNumberAuthenticationSettings { &self.settings }

}

#[doc(hidden)]
pub struct RTDSendPhoneNumberConfirmationCodeBuilder {
  inner: SendPhoneNumberConfirmationCode
}

impl RTDSendPhoneNumberConfirmationCodeBuilder {
  pub fn build(&self) -> SendPhoneNumberConfirmationCode { self.inner.clone() }

   
  pub fn hash<T: AsRef<str>>(&mut self, hash: T) -> &mut Self {
    self.inner.hash = hash.as_ref().to_string();
    self
  }

   
  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

   
  pub fn settings<T: AsRef<PhoneNumberAuthenticationSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = settings.as_ref().clone();
    self
  }

}

impl AsRef<SendPhoneNumberConfirmationCode> for SendPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &SendPhoneNumberConfirmationCode { self }
}

impl AsRef<SendPhoneNumberConfirmationCode> for RTDSendPhoneNumberConfirmationCodeBuilder {
  fn as_ref(&self) -> &SendPhoneNumberConfirmationCode { &self.inner }
}







/// Sends a code to verify a phone number to be added to a user's Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPhoneNumberVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The phone number of the user, in international format
  phone_number: String,
  /// Settings for the authentication of the user's phone number
  settings: PhoneNumberAuthenticationSettings,
  
}

impl RObject for SendPhoneNumberVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendPhoneNumberVerificationCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SendPhoneNumberVerificationCode {}

impl SendPhoneNumberVerificationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendPhoneNumberVerificationCodeBuilder {
    let mut inner = SendPhoneNumberVerificationCode::default();
    inner.td_name = "sendPhoneNumberVerificationCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSendPhoneNumberVerificationCodeBuilder { inner }
  }

  pub fn phone_number(&self) -> &String { &self.phone_number }

  pub fn settings(&self) -> &PhoneNumberAuthenticationSettings { &self.settings }

}

#[doc(hidden)]
pub struct RTDSendPhoneNumberVerificationCodeBuilder {
  inner: SendPhoneNumberVerificationCode
}

impl RTDSendPhoneNumberVerificationCodeBuilder {
  pub fn build(&self) -> SendPhoneNumberVerificationCode { self.inner.clone() }

   
  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

   
  pub fn settings<T: AsRef<PhoneNumberAuthenticationSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = settings.as_ref().clone();
    self
  }

}

impl AsRef<SendPhoneNumberVerificationCode> for SendPhoneNumberVerificationCode {
  fn as_ref(&self) -> &SendPhoneNumberVerificationCode { self }
}

impl AsRef<SendPhoneNumberVerificationCode> for RTDSendPhoneNumberVerificationCodeBuilder {
  fn as_ref(&self) -> &SendPhoneNumberVerificationCode { &self.inner }
}







/// Changes the period of inactivity after which the account of the current user will automatically be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAccountTtl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New account TTL
  ttl: AccountTtl,
  
}

impl RObject for SetAccountTtl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setAccountTtl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetAccountTtl {}

impl SetAccountTtl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetAccountTtlBuilder {
    let mut inner = SetAccountTtl::default();
    inner.td_name = "setAccountTtl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetAccountTtlBuilder { inner }
  }

  pub fn ttl(&self) -> &AccountTtl { &self.ttl }

}

#[doc(hidden)]
pub struct RTDSetAccountTtlBuilder {
  inner: SetAccountTtl
}

impl RTDSetAccountTtlBuilder {
  pub fn build(&self) -> SetAccountTtl { self.inner.clone() }

   
  pub fn ttl<T: AsRef<AccountTtl>>(&mut self, ttl: T) -> &mut Self {
    self.inner.ttl = ttl.as_ref().clone();
    self
  }

}

impl AsRef<SetAccountTtl> for SetAccountTtl {
  fn as_ref(&self) -> &SetAccountTtl { self }
}

impl AsRef<SetAccountTtl> for RTDSetAccountTtlBuilder {
  fn as_ref(&self) -> &SetAccountTtl { &self.inner }
}







/// Succeeds after a specified amount of time has passed. Can be called before authorization. Can be called before initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAlarm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of seconds before the function returns
  seconds: f32,
  
}

impl RObject for SetAlarm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setAlarm" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetAlarm {}

impl SetAlarm {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetAlarmBuilder {
    let mut inner = SetAlarm::default();
    inner.td_name = "setAlarm".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetAlarmBuilder { inner }
  }

  pub fn seconds(&self) -> f32 { self.seconds }

}

#[doc(hidden)]
pub struct RTDSetAlarmBuilder {
  inner: SetAlarm
}

impl RTDSetAlarmBuilder {
  pub fn build(&self) -> SetAlarm { self.inner.clone() }

   
  pub fn seconds(&mut self, seconds: f32) -> &mut Self {
    self.inner.seconds = seconds;
    self
  }

}

impl AsRef<SetAlarm> for SetAlarm {
  fn as_ref(&self) -> &SetAlarm { self }
}

impl AsRef<SetAlarm> for RTDSetAlarmBuilder {
  fn as_ref(&self) -> &SetAlarm { &self.inner }
}







/// Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAuthenticationPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The phone number of the user, in international format
  phone_number: String,
  /// Settings for the authentication of the user's phone number
  settings: PhoneNumberAuthenticationSettings,
  
}

impl RObject for SetAuthenticationPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setAuthenticationPhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetAuthenticationPhoneNumber {}

impl SetAuthenticationPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetAuthenticationPhoneNumberBuilder {
    let mut inner = SetAuthenticationPhoneNumber::default();
    inner.td_name = "setAuthenticationPhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetAuthenticationPhoneNumberBuilder { inner }
  }

  pub fn phone_number(&self) -> &String { &self.phone_number }

  pub fn settings(&self) -> &PhoneNumberAuthenticationSettings { &self.settings }

}

#[doc(hidden)]
pub struct RTDSetAuthenticationPhoneNumberBuilder {
  inner: SetAuthenticationPhoneNumber
}

impl RTDSetAuthenticationPhoneNumberBuilder {
  pub fn build(&self) -> SetAuthenticationPhoneNumber { self.inner.clone() }

   
  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

   
  pub fn settings<T: AsRef<PhoneNumberAuthenticationSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = settings.as_ref().clone();
    self
  }

}

impl AsRef<SetAuthenticationPhoneNumber> for SetAuthenticationPhoneNumber {
  fn as_ref(&self) -> &SetAuthenticationPhoneNumber { self }
}

impl AsRef<SetAuthenticationPhoneNumber> for RTDSetAuthenticationPhoneNumberBuilder {
  fn as_ref(&self) -> &SetAuthenticationPhoneNumber { &self.inner }
}







/// Sets auto-download settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAutoDownloadSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New user auto-download settings
  settings: AutoDownloadSettings,
  /// Type of the network for which the new settings are applied
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: NetworkType,
  
}

impl RObject for SetAutoDownloadSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setAutoDownloadSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetAutoDownloadSettings {}

impl SetAutoDownloadSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetAutoDownloadSettingsBuilder {
    let mut inner = SetAutoDownloadSettings::default();
    inner.td_name = "setAutoDownloadSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetAutoDownloadSettingsBuilder { inner }
  }

  pub fn settings(&self) -> &AutoDownloadSettings { &self.settings }

  pub fn type_(&self) -> &NetworkType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDSetAutoDownloadSettingsBuilder {
  inner: SetAutoDownloadSettings
}

impl RTDSetAutoDownloadSettingsBuilder {
  pub fn build(&self) -> SetAutoDownloadSettings { self.inner.clone() }

   
  pub fn settings<T: AsRef<AutoDownloadSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = settings.as_ref().clone();
    self
  }

   
  pub fn type_<T: AsRef<NetworkType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<SetAutoDownloadSettings> for SetAutoDownloadSettings {
  fn as_ref(&self) -> &SetAutoDownloadSettings { self }
}

impl AsRef<SetAutoDownloadSettings> for RTDSetAutoDownloadSettingsBuilder {
  fn as_ref(&self) -> &SetAutoDownloadSettings { &self.inner }
}







/// Changes the background selected by the user; adds background to the list of installed backgrounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The input background to use, null for filled backgrounds
  background: InputBackground,
  /// Background type; null for default background. The method will return error 404 if type is null
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: BackgroundType,
  /// True, if the background is chosen for dark theme
  for_dark_theme: bool,
  
}

impl RObject for SetBackground {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setBackground" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetBackground {}

impl SetBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetBackgroundBuilder {
    let mut inner = SetBackground::default();
    inner.td_name = "setBackground".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetBackgroundBuilder { inner }
  }

  pub fn background(&self) -> &InputBackground { &self.background }

  pub fn type_(&self) -> &BackgroundType { &self.type_ }

  pub fn for_dark_theme(&self) -> bool { self.for_dark_theme }

}

#[doc(hidden)]
pub struct RTDSetBackgroundBuilder {
  inner: SetBackground
}

impl RTDSetBackgroundBuilder {
  pub fn build(&self) -> SetBackground { self.inner.clone() }

   
  pub fn background<T: AsRef<InputBackground>>(&mut self, background: T) -> &mut Self {
    self.inner.background = background.as_ref().clone();
    self
  }

   
  pub fn type_<T: AsRef<BackgroundType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn for_dark_theme(&mut self, for_dark_theme: bool) -> &mut Self {
    self.inner.for_dark_theme = for_dark_theme;
    self
  }

}

impl AsRef<SetBackground> for SetBackground {
  fn as_ref(&self) -> &SetBackground { self }
}

impl AsRef<SetBackground> for RTDSetBackgroundBuilder {
  fn as_ref(&self) -> &SetBackground { &self.inner }
}







/// Changes the bio of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new value of the user bio; 0-70 characters without line feeds
  bio: String,
  
}

impl RObject for SetBio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setBio" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetBio {}

impl SetBio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetBioBuilder {
    let mut inner = SetBio::default();
    inner.td_name = "setBio".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetBioBuilder { inner }
  }

  pub fn bio(&self) -> &String { &self.bio }

}

#[doc(hidden)]
pub struct RTDSetBioBuilder {
  inner: SetBio
}

impl RTDSetBioBuilder {
  pub fn build(&self) -> SetBio { self.inner.clone() }

   
  pub fn bio<T: AsRef<str>>(&mut self, bio: T) -> &mut Self {
    self.inner.bio = bio.as_ref().to_string();
    self
  }

}

impl AsRef<SetBio> for SetBio {
  fn as_ref(&self) -> &SetBio { self }
}

impl AsRef<SetBio> for RTDSetBioBuilder {
  fn as_ref(&self) -> &SetBio { &self.inner }
}







/// Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBotUpdatesStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of pending updates
  pending_update_count: i64,
  /// The last error message
  error_message: String,
  
}

impl RObject for SetBotUpdatesStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setBotUpdatesStatus" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetBotUpdatesStatus {}

impl SetBotUpdatesStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetBotUpdatesStatusBuilder {
    let mut inner = SetBotUpdatesStatus::default();
    inner.td_name = "setBotUpdatesStatus".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetBotUpdatesStatusBuilder { inner }
  }

  pub fn pending_update_count(&self) -> i64 { self.pending_update_count }

  pub fn error_message(&self) -> &String { &self.error_message }

}

#[doc(hidden)]
pub struct RTDSetBotUpdatesStatusBuilder {
  inner: SetBotUpdatesStatus
}

impl RTDSetBotUpdatesStatusBuilder {
  pub fn build(&self) -> SetBotUpdatesStatus { self.inner.clone() }

   
  pub fn pending_update_count(&mut self, pending_update_count: i64) -> &mut Self {
    self.inner.pending_update_count = pending_update_count;
    self
  }

   
  pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
    self.inner.error_message = error_message.as_ref().to_string();
    self
  }

}

impl AsRef<SetBotUpdatesStatus> for SetBotUpdatesStatus {
  fn as_ref(&self) -> &SetBotUpdatesStatus { self }
}

impl AsRef<SetBotUpdatesStatus> for RTDSetBotUpdatesStatusBuilder {
  fn as_ref(&self) -> &SetBotUpdatesStatus { &self.inner }
}







/// Moves a chat to a different chat list. Current chat list of the chat must ne non-null
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatChatList {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New chat list of the chat
  chat_list: ChatList,
  
}

impl RObject for SetChatChatList {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatChatList" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatChatList {}

impl SetChatChatList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatChatListBuilder {
    let mut inner = SetChatChatList::default();
    inner.td_name = "setChatChatList".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatChatListBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn chat_list(&self) -> &ChatList { &self.chat_list }

}

#[doc(hidden)]
pub struct RTDSetChatChatListBuilder {
  inner: SetChatChatList
}

impl RTDSetChatChatListBuilder {
  pub fn build(&self) -> SetChatChatList { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = chat_list.as_ref().clone();
    self
  }

}

impl AsRef<SetChatChatList> for SetChatChatList {
  fn as_ref(&self) -> &SetChatChatList { self }
}

impl AsRef<SetChatChatList> for RTDSetChatChatListBuilder {
  fn as_ref(&self) -> &SetChatChatList { &self.inner }
}







/// Changes client data associated with a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatClientData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of client_data
  client_data: String,
  
}

impl RObject for SetChatClientData {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatClientData" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatClientData {}

impl SetChatClientData {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatClientDataBuilder {
    let mut inner = SetChatClientData::default();
    inner.td_name = "setChatClientData".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatClientDataBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn client_data(&self) -> &String { &self.client_data }

}

#[doc(hidden)]
pub struct RTDSetChatClientDataBuilder {
  inner: SetChatClientData
}

impl RTDSetChatClientDataBuilder {
  pub fn build(&self) -> SetChatClientData { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn client_data<T: AsRef<str>>(&mut self, client_data: T) -> &mut Self {
    self.inner.client_data = client_data.as_ref().to_string();
    self
  }

}

impl AsRef<SetChatClientData> for SetChatClientData {
  fn as_ref(&self) -> &SetChatClientData { self }
}

impl AsRef<SetChatClientData> for RTDSetChatClientDataBuilder {
  fn as_ref(&self) -> &SetChatClientData { &self.inner }
}







/// Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatDescription {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat
  chat_id: i64,
  /// Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info rights
  description: String,
  
}

impl RObject for SetChatDescription {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatDescription" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatDescription {}

impl SetChatDescription {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatDescriptionBuilder {
    let mut inner = SetChatDescription::default();
    inner.td_name = "setChatDescription".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatDescriptionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn description(&self) -> &String { &self.description }

}

#[doc(hidden)]
pub struct RTDSetChatDescriptionBuilder {
  inner: SetChatDescription
}

impl RTDSetChatDescriptionBuilder {
  pub fn build(&self) -> SetChatDescription { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

}

impl AsRef<SetChatDescription> for SetChatDescription {
  fn as_ref(&self) -> &SetChatDescription { self }
}

impl AsRef<SetChatDescription> for RTDSetChatDescriptionBuilder {
  fn as_ref(&self) -> &SetChatDescription { &self.inner }
}







/// Changes the discussion group of a channel chat; requires can_change_info rights in the channel if it is specified
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatDiscussionGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel chat. Pass 0 to remove a link from the supergroup passed in the second argument to a linked channel chat (requires can_pin_messages rights in the supergroup)
  chat_id: i64,
  /// Identifier of a new channel's discussion group. Use 0 to remove the discussion group. Use the method getSuitableDiscussionChats to find all suitable groups. Basic group chats needs to be first upgraded to supergroup chats. If new chat members don't have access to old messages in the supergroup, then toggleSupergroupIsAllHistoryAvailable needs to be used first to change that
  discussion_chat_id: i64,
  
}

impl RObject for SetChatDiscussionGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatDiscussionGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatDiscussionGroup {}

impl SetChatDiscussionGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatDiscussionGroupBuilder {
    let mut inner = SetChatDiscussionGroup::default();
    inner.td_name = "setChatDiscussionGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatDiscussionGroupBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn discussion_chat_id(&self) -> i64 { self.discussion_chat_id }

}

#[doc(hidden)]
pub struct RTDSetChatDiscussionGroupBuilder {
  inner: SetChatDiscussionGroup
}

impl RTDSetChatDiscussionGroupBuilder {
  pub fn build(&self) -> SetChatDiscussionGroup { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn discussion_chat_id(&mut self, discussion_chat_id: i64) -> &mut Self {
    self.inner.discussion_chat_id = discussion_chat_id;
    self
  }

}

impl AsRef<SetChatDiscussionGroup> for SetChatDiscussionGroup {
  fn as_ref(&self) -> &SetChatDiscussionGroup { self }
}

impl AsRef<SetChatDiscussionGroup> for RTDSetChatDiscussionGroupBuilder {
  fn as_ref(&self) -> &SetChatDiscussionGroup { &self.inner }
}







/// Changes the draft message in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatDraftMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New draft message; may be null
  draft_message: Option<DraftMessage>,
  
}

impl RObject for SetChatDraftMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatDraftMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatDraftMessage {}

impl SetChatDraftMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatDraftMessageBuilder {
    let mut inner = SetChatDraftMessage::default();
    inner.td_name = "setChatDraftMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatDraftMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn draft_message(&self) -> &Option<DraftMessage> { &self.draft_message }

}

#[doc(hidden)]
pub struct RTDSetChatDraftMessageBuilder {
  inner: SetChatDraftMessage
}

impl RTDSetChatDraftMessageBuilder {
  pub fn build(&self) -> SetChatDraftMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
    self.inner.draft_message = Some(draft_message.as_ref().clone());
    self
  }

}

impl AsRef<SetChatDraftMessage> for SetChatDraftMessage {
  fn as_ref(&self) -> &SetChatDraftMessage { self }
}

impl AsRef<SetChatDraftMessage> for RTDSetChatDraftMessageBuilder {
  fn as_ref(&self) -> &SetChatDraftMessage { &self.inner }
}







/// Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New location for the chat; must be valid and not null
  location: ChatLocation,
  
}

impl RObject for SetChatLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatLocation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatLocation {}

impl SetChatLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatLocationBuilder {
    let mut inner = SetChatLocation::default();
    inner.td_name = "setChatLocation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatLocationBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn location(&self) -> &ChatLocation { &self.location }

}

#[doc(hidden)]
pub struct RTDSetChatLocationBuilder {
  inner: SetChatLocation
}

impl RTDSetChatLocationBuilder {
  pub fn build(&self) -> SetChatLocation { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn location<T: AsRef<ChatLocation>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

}

impl AsRef<SetChatLocation> for SetChatLocation {
  fn as_ref(&self) -> &SetChatLocation { self }
}

impl AsRef<SetChatLocation> for RTDSetChatLocationBuilder {
  fn as_ref(&self) -> &SetChatLocation { &self.inner }
}







/// Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for adding new members to the chat and transferring chat ownership; instead, use addChatMember or transferChatOwnership. The chat member status will not be changed until it has been synchronized with the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatMemberStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// User identifier
  user_id: i64,
  /// The new status of the member in the chat
  status: ChatMemberStatus,
  
}

impl RObject for SetChatMemberStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatMemberStatus" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatMemberStatus {}

impl SetChatMemberStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatMemberStatusBuilder {
    let mut inner = SetChatMemberStatus::default();
    inner.td_name = "setChatMemberStatus".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatMemberStatusBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn status(&self) -> &ChatMemberStatus { &self.status }

}

#[doc(hidden)]
pub struct RTDSetChatMemberStatusBuilder {
  inner: SetChatMemberStatus
}

impl RTDSetChatMemberStatusBuilder {
  pub fn build(&self) -> SetChatMemberStatus { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

}

impl AsRef<SetChatMemberStatus> for SetChatMemberStatus {
  fn as_ref(&self) -> &SetChatMemberStatus { self }
}

impl AsRef<SetChatMemberStatus> for RTDSetChatMemberStatusBuilder {
  fn as_ref(&self) -> &SetChatMemberStatus { &self.inner }
}







/// Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New notification settings for the chat. If the chat is muted for more than 1 week, it is considered to be muted forever
  notification_settings: ChatNotificationSettings,
  
}

impl RObject for SetChatNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatNotificationSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatNotificationSettings {}

impl SetChatNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatNotificationSettingsBuilder {
    let mut inner = SetChatNotificationSettings::default();
    inner.td_name = "setChatNotificationSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatNotificationSettingsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn notification_settings(&self) -> &ChatNotificationSettings { &self.notification_settings }

}

#[doc(hidden)]
pub struct RTDSetChatNotificationSettingsBuilder {
  inner: SetChatNotificationSettings
}

impl RTDSetChatNotificationSettingsBuilder {
  pub fn build(&self) -> SetChatNotificationSettings { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(&mut self, notification_settings: T) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }

}

impl AsRef<SetChatNotificationSettings> for SetChatNotificationSettings {
  fn as_ref(&self) -> &SetChatNotificationSettings { self }
}

impl AsRef<SetChatNotificationSettings> for RTDSetChatNotificationSettingsBuilder {
  fn as_ref(&self) -> &SetChatNotificationSettings { &self.inner }
}







/// Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatPermissions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New non-administrator members permissions in the chat
  permissions: ChatPermissions,
  
}

impl RObject for SetChatPermissions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatPermissions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatPermissions {}

impl SetChatPermissions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatPermissionsBuilder {
    let mut inner = SetChatPermissions::default();
    inner.td_name = "setChatPermissions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatPermissionsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn permissions(&self) -> &ChatPermissions { &self.permissions }

}

#[doc(hidden)]
pub struct RTDSetChatPermissionsBuilder {
  inner: SetChatPermissions
}

impl RTDSetChatPermissionsBuilder {
  pub fn build(&self) -> SetChatPermissions { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
    self.inner.permissions = permissions.as_ref().clone();
    self
  }

}

impl AsRef<SetChatPermissions> for SetChatPermissions {
  fn as_ref(&self) -> &SetChatPermissions { self }
}

impl AsRef<SetChatPermissions> for RTDSetChatPermissionsBuilder {
  fn as_ref(&self) -> &SetChatPermissions { &self.inner }
}







/// Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info rights. The photo will not be changed before request to the server has been completed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New chat photo. You can use a zero InputFileId to delete the chat photo. Files that are accessible only by HTTP URL are not acceptable
  photo: InputFile,
  
}

impl RObject for SetChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatPhoto" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatPhoto {}

impl SetChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatPhotoBuilder {
    let mut inner = SetChatPhoto::default();
    inner.td_name = "setChatPhoto".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatPhotoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn photo(&self) -> &InputFile { &self.photo }

}

#[doc(hidden)]
pub struct RTDSetChatPhotoBuilder {
  inner: SetChatPhoto
}

impl RTDSetChatPhotoBuilder {
  pub fn build(&self) -> SetChatPhoto { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

}

impl AsRef<SetChatPhoto> for SetChatPhoto {
  fn as_ref(&self) -> &SetChatPhoto { self }
}

impl AsRef<SetChatPhoto> for RTDSetChatPhotoBuilder {
  fn as_ref(&self) -> &SetChatPhoto { &self.inner }
}







/// Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatSlowModeDelay {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New slow mode delay for the chat; must be one of 0, 10, 30, 60, 300, 900, 3600
  slow_mode_delay: i64,
  
}

impl RObject for SetChatSlowModeDelay {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatSlowModeDelay" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatSlowModeDelay {}

impl SetChatSlowModeDelay {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatSlowModeDelayBuilder {
    let mut inner = SetChatSlowModeDelay::default();
    inner.td_name = "setChatSlowModeDelay".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatSlowModeDelayBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn slow_mode_delay(&self) -> i64 { self.slow_mode_delay }

}

#[doc(hidden)]
pub struct RTDSetChatSlowModeDelayBuilder {
  inner: SetChatSlowModeDelay
}

impl RTDSetChatSlowModeDelayBuilder {
  pub fn build(&self) -> SetChatSlowModeDelay { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn slow_mode_delay(&mut self, slow_mode_delay: i64) -> &mut Self {
    self.inner.slow_mode_delay = slow_mode_delay;
    self
  }

}

impl AsRef<SetChatSlowModeDelay> for SetChatSlowModeDelay {
  fn as_ref(&self) -> &SetChatSlowModeDelay { self }
}

impl AsRef<SetChatSlowModeDelay> for RTDSetChatSlowModeDelayBuilder {
  fn as_ref(&self) -> &SetChatSlowModeDelay { &self.inner }
}







/// Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info rights. The title will not be changed until the request to the server has been completed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New title of the chat; 1-128 characters
  title: String,
  
}

impl RObject for SetChatTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatTitle" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetChatTitle {}

impl SetChatTitle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetChatTitleBuilder {
    let mut inner = SetChatTitle::default();
    inner.td_name = "setChatTitle".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetChatTitleBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDSetChatTitleBuilder {
  inner: SetChatTitle
}

impl RTDSetChatTitleBuilder {
  pub fn build(&self) -> SetChatTitle { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<SetChatTitle> for SetChatTitle {
  fn as_ref(&self) -> &SetChatTitle { self }
}

impl AsRef<SetChatTitle> for RTDSetChatTitleBuilder {
  fn as_ref(&self) -> &SetChatTitle { &self.inner }
}







/// Adds or changes a custom local language pack to the current localization target
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCustomLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the language pack. Language pack ID must start with 'X', consist only of English letters, digits and hyphens, and must not exceed 64 characters. Can be called before authorization
  info: LanguagePackInfo,
  /// Strings of the new language pack
  strings: Vec<LanguagePackString>,
  
}

impl RObject for SetCustomLanguagePack {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setCustomLanguagePack" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetCustomLanguagePack {}

impl SetCustomLanguagePack {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetCustomLanguagePackBuilder {
    let mut inner = SetCustomLanguagePack::default();
    inner.td_name = "setCustomLanguagePack".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetCustomLanguagePackBuilder { inner }
  }

  pub fn info(&self) -> &LanguagePackInfo { &self.info }

  pub fn strings(&self) -> &Vec<LanguagePackString> { &self.strings }

}

#[doc(hidden)]
pub struct RTDSetCustomLanguagePackBuilder {
  inner: SetCustomLanguagePack
}

impl RTDSetCustomLanguagePackBuilder {
  pub fn build(&self) -> SetCustomLanguagePack { self.inner.clone() }

   
  pub fn info<T: AsRef<LanguagePackInfo>>(&mut self, info: T) -> &mut Self {
    self.inner.info = info.as_ref().clone();
    self
  }

   
  pub fn strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self {
    self.inner.strings = strings;
    self
  }

}

impl AsRef<SetCustomLanguagePack> for SetCustomLanguagePack {
  fn as_ref(&self) -> &SetCustomLanguagePack { self }
}

impl AsRef<SetCustomLanguagePack> for RTDSetCustomLanguagePackBuilder {
  fn as_ref(&self) -> &SetCustomLanguagePack { &self.inner }
}







/// Adds, edits or deletes a string in a custom local language pack. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCustomLanguagePackString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a previously added custom local language pack in the current localization target
  language_pack_id: String,
  /// New language pack string
  new_string: LanguagePackString,
  
}

impl RObject for SetCustomLanguagePackString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setCustomLanguagePackString" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetCustomLanguagePackString {}

impl SetCustomLanguagePackString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetCustomLanguagePackStringBuilder {
    let mut inner = SetCustomLanguagePackString::default();
    inner.td_name = "setCustomLanguagePackString".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetCustomLanguagePackStringBuilder { inner }
  }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

  pub fn new_string(&self) -> &LanguagePackString { &self.new_string }

}

#[doc(hidden)]
pub struct RTDSetCustomLanguagePackStringBuilder {
  inner: SetCustomLanguagePackString
}

impl RTDSetCustomLanguagePackStringBuilder {
  pub fn build(&self) -> SetCustomLanguagePackString { self.inner.clone() }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

   
  pub fn new_string<T: AsRef<LanguagePackString>>(&mut self, new_string: T) -> &mut Self {
    self.inner.new_string = new_string.as_ref().clone();
    self
  }

}

impl AsRef<SetCustomLanguagePackString> for SetCustomLanguagePackString {
  fn as_ref(&self) -> &SetCustomLanguagePackString { self }
}

impl AsRef<SetCustomLanguagePackString> for RTDSetCustomLanguagePackStringBuilder {
  fn as_ref(&self) -> &SetCustomLanguagePackString { &self.inner }
}







/// Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetDatabaseEncryptionKey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New encryption key
  new_encryption_key: String,
  
}

impl RObject for SetDatabaseEncryptionKey {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setDatabaseEncryptionKey" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetDatabaseEncryptionKey {}

impl SetDatabaseEncryptionKey {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetDatabaseEncryptionKeyBuilder {
    let mut inner = SetDatabaseEncryptionKey::default();
    inner.td_name = "setDatabaseEncryptionKey".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetDatabaseEncryptionKeyBuilder { inner }
  }

  pub fn new_encryption_key(&self) -> &String { &self.new_encryption_key }

}

#[doc(hidden)]
pub struct RTDSetDatabaseEncryptionKeyBuilder {
  inner: SetDatabaseEncryptionKey
}

impl RTDSetDatabaseEncryptionKeyBuilder {
  pub fn build(&self) -> SetDatabaseEncryptionKey { self.inner.clone() }

   
  pub fn new_encryption_key<T: AsRef<str>>(&mut self, new_encryption_key: T) -> &mut Self {
    self.inner.new_encryption_key = new_encryption_key.as_ref().to_string();
    self
  }

}

impl AsRef<SetDatabaseEncryptionKey> for SetDatabaseEncryptionKey {
  fn as_ref(&self) -> &SetDatabaseEncryptionKey { self }
}

impl AsRef<SetDatabaseEncryptionKey> for RTDSetDatabaseEncryptionKeyBuilder {
  fn as_ref(&self) -> &SetDatabaseEncryptionKey { &self.inner }
}







/// Informs TDLib on a file generation progress
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetFileGenerationProgress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the generation process
  generation_id: isize,
  /// Expected size of the generated file, in bytes; 0 if unknown
  expected_size: i64,
  /// The number of bytes already generated
  local_prefix_size: i64,
  
}

impl RObject for SetFileGenerationProgress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setFileGenerationProgress" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetFileGenerationProgress {}

impl SetFileGenerationProgress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetFileGenerationProgressBuilder {
    let mut inner = SetFileGenerationProgress::default();
    inner.td_name = "setFileGenerationProgress".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetFileGenerationProgressBuilder { inner }
  }

  pub fn generation_id(&self) -> isize { self.generation_id }

  pub fn expected_size(&self) -> i64 { self.expected_size }

  pub fn local_prefix_size(&self) -> i64 { self.local_prefix_size }

}

#[doc(hidden)]
pub struct RTDSetFileGenerationProgressBuilder {
  inner: SetFileGenerationProgress
}

impl RTDSetFileGenerationProgressBuilder {
  pub fn build(&self) -> SetFileGenerationProgress { self.inner.clone() }

   
  pub fn generation_id(&mut self, generation_id: isize) -> &mut Self {
    self.inner.generation_id = generation_id;
    self
  }

   
  pub fn expected_size(&mut self, expected_size: i64) -> &mut Self {
    self.inner.expected_size = expected_size;
    self
  }

   
  pub fn local_prefix_size(&mut self, local_prefix_size: i64) -> &mut Self {
    self.inner.local_prefix_size = local_prefix_size;
    self
  }

}

impl AsRef<SetFileGenerationProgress> for SetFileGenerationProgress {
  fn as_ref(&self) -> &SetFileGenerationProgress { self }
}

impl AsRef<SetFileGenerationProgress> for RTDSetFileGenerationProgressBuilder {
  fn as_ref(&self) -> &SetFileGenerationProgress { &self.inner }
}







/// Updates the game score of the specified user in the game; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetGameScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat to which the message with the game belongs
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
  /// True, if the message should be edited
  edit_message: bool,
  /// User identifier
  user_id: i64,
  /// The new score
  score: i64,
  /// Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table
  force: bool,
  
}

impl RObject for SetGameScore {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setGameScore" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetGameScore {}

impl SetGameScore {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetGameScoreBuilder {
    let mut inner = SetGameScore::default();
    inner.td_name = "setGameScore".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetGameScoreBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn edit_message(&self) -> bool { self.edit_message }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn score(&self) -> i64 { self.score }

  pub fn force(&self) -> bool { self.force }

}

#[doc(hidden)]
pub struct RTDSetGameScoreBuilder {
  inner: SetGameScore
}

impl RTDSetGameScoreBuilder {
  pub fn build(&self) -> SetGameScore { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn edit_message(&mut self, edit_message: bool) -> &mut Self {
    self.inner.edit_message = edit_message;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn score(&mut self, score: i64) -> &mut Self {
    self.inner.score = score;
    self
  }

   
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.force = force;
    self
  }

}

impl AsRef<SetGameScore> for SetGameScore {
  fn as_ref(&self) -> &SetGameScore { self }
}

impl AsRef<SetGameScore> for RTDSetGameScoreBuilder {
  fn as_ref(&self) -> &SetGameScore { &self.inner }
}







/// Updates the game score of the specified user in a game; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetInlineGameScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Inline message identifier
  inline_message_id: String,
  /// True, if the message should be edited
  edit_message: bool,
  /// User identifier
  user_id: i64,
  /// The new score
  score: i64,
  /// Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table
  force: bool,
  
}

impl RObject for SetInlineGameScore {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setInlineGameScore" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetInlineGameScore {}

impl SetInlineGameScore {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetInlineGameScoreBuilder {
    let mut inner = SetInlineGameScore::default();
    inner.td_name = "setInlineGameScore".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetInlineGameScoreBuilder { inner }
  }

  pub fn inline_message_id(&self) -> &String { &self.inline_message_id }

  pub fn edit_message(&self) -> bool { self.edit_message }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn score(&self) -> i64 { self.score }

  pub fn force(&self) -> bool { self.force }

}

#[doc(hidden)]
pub struct RTDSetInlineGameScoreBuilder {
  inner: SetInlineGameScore
}

impl RTDSetInlineGameScoreBuilder {
  pub fn build(&self) -> SetInlineGameScore { self.inner.clone() }

   
  pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
    self.inner.inline_message_id = inline_message_id.as_ref().to_string();
    self
  }

   
  pub fn edit_message(&mut self, edit_message: bool) -> &mut Self {
    self.inner.edit_message = edit_message;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn score(&mut self, score: i64) -> &mut Self {
    self.inner.score = score;
    self
  }

   
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.force = force;
    self
  }

}

impl AsRef<SetInlineGameScore> for SetInlineGameScore {
  fn as_ref(&self) -> &SetInlineGameScore { self }
}

impl AsRef<SetInlineGameScore> for RTDSetInlineGameScoreBuilder {
  fn as_ref(&self) -> &SetInlineGameScore { &self.inner }
}







/// Sets new log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetLogStream {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New log stream
  log_stream: LogStream,
  
}

impl RObject for SetLogStream {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setLogStream" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetLogStream {}

impl SetLogStream {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetLogStreamBuilder {
    let mut inner = SetLogStream::default();
    inner.td_name = "setLogStream".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetLogStreamBuilder { inner }
  }

  pub fn log_stream(&self) -> &LogStream { &self.log_stream }

}

#[doc(hidden)]
pub struct RTDSetLogStreamBuilder {
  inner: SetLogStream
}

impl RTDSetLogStreamBuilder {
  pub fn build(&self) -> SetLogStream { self.inner.clone() }

   
  pub fn log_stream<T: AsRef<LogStream>>(&mut self, log_stream: T) -> &mut Self {
    self.inner.log_stream = log_stream.as_ref().clone();
    self
  }

}

impl AsRef<SetLogStream> for SetLogStream {
  fn as_ref(&self) -> &SetLogStream { self }
}

impl AsRef<SetLogStream> for RTDSetLogStreamBuilder {
  fn as_ref(&self) -> &SetLogStream { &self.inner }
}







/// Sets the verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetLogTagVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Logging tag to change verbosity level
  tag: String,
  /// New verbosity level; 1-1024
  new_verbosity_level: i64,
  
}

impl RObject for SetLogTagVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setLogTagVerbosityLevel" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetLogTagVerbosityLevel {}

impl SetLogTagVerbosityLevel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetLogTagVerbosityLevelBuilder {
    let mut inner = SetLogTagVerbosityLevel::default();
    inner.td_name = "setLogTagVerbosityLevel".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetLogTagVerbosityLevelBuilder { inner }
  }

  pub fn tag(&self) -> &String { &self.tag }

  pub fn new_verbosity_level(&self) -> i64 { self.new_verbosity_level }

}

#[doc(hidden)]
pub struct RTDSetLogTagVerbosityLevelBuilder {
  inner: SetLogTagVerbosityLevel
}

impl RTDSetLogTagVerbosityLevelBuilder {
  pub fn build(&self) -> SetLogTagVerbosityLevel { self.inner.clone() }

   
  pub fn tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.tag = tag.as_ref().to_string();
    self
  }

   
  pub fn new_verbosity_level(&mut self, new_verbosity_level: i64) -> &mut Self {
    self.inner.new_verbosity_level = new_verbosity_level;
    self
  }

}

impl AsRef<SetLogTagVerbosityLevel> for SetLogTagVerbosityLevel {
  fn as_ref(&self) -> &SetLogTagVerbosityLevel { self }
}

impl AsRef<SetLogTagVerbosityLevel> for RTDSetLogTagVerbosityLevelBuilder {
  fn as_ref(&self) -> &SetLogTagVerbosityLevel { &self.inner }
}







/// Sets the verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetLogVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings, value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging
  new_verbosity_level: i64,
  
}

impl RObject for SetLogVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setLogVerbosityLevel" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetLogVerbosityLevel {}

impl SetLogVerbosityLevel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetLogVerbosityLevelBuilder {
    let mut inner = SetLogVerbosityLevel::default();
    inner.td_name = "setLogVerbosityLevel".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetLogVerbosityLevelBuilder { inner }
  }

  pub fn new_verbosity_level(&self) -> i64 { self.new_verbosity_level }

}

#[doc(hidden)]
pub struct RTDSetLogVerbosityLevelBuilder {
  inner: SetLogVerbosityLevel
}

impl RTDSetLogVerbosityLevelBuilder {
  pub fn build(&self) -> SetLogVerbosityLevel { self.inner.clone() }

   
  pub fn new_verbosity_level(&mut self, new_verbosity_level: i64) -> &mut Self {
    self.inner.new_verbosity_level = new_verbosity_level;
    self
  }

}

impl AsRef<SetLogVerbosityLevel> for SetLogVerbosityLevel {
  fn as_ref(&self) -> &SetLogVerbosityLevel { self }
}

impl AsRef<SetLogVerbosityLevel> for RTDSetLogVerbosityLevelBuilder {
  fn as_ref(&self) -> &SetLogVerbosityLevel { &self.inner }
}







/// Changes the first and last name of the current user. If something changes, updateUser will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new value of the first name for the user; 1-64 characters
  first_name: String,
  /// The new value of the optional last name for the user; 0-64 characters
  last_name: String,
  
}

impl RObject for SetName {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setName" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetName {}

impl SetName {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetNameBuilder {
    let mut inner = SetName::default();
    inner.td_name = "setName".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetNameBuilder { inner }
  }

  pub fn first_name(&self) -> &String { &self.first_name }

  pub fn last_name(&self) -> &String { &self.last_name }

}

#[doc(hidden)]
pub struct RTDSetNameBuilder {
  inner: SetName
}

impl RTDSetNameBuilder {
  pub fn build(&self) -> SetName { self.inner.clone() }

   
  pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
    self.inner.first_name = first_name.as_ref().to_string();
    self
  }

   
  pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
    self.inner.last_name = last_name.as_ref().to_string();
    self
  }

}

impl AsRef<SetName> for SetName {
  fn as_ref(&self) -> &SetName { self }
}

impl AsRef<SetName> for RTDSetNameBuilder {
  fn as_ref(&self) -> &SetName { &self.inner }
}







/// Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it should be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetNetworkType {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new network type. By default, networkTypeOther
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: NetworkType,
  
}

impl RObject for SetNetworkType {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setNetworkType" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetNetworkType {}

impl SetNetworkType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetNetworkTypeBuilder {
    let mut inner = SetNetworkType::default();
    inner.td_name = "setNetworkType".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetNetworkTypeBuilder { inner }
  }

  pub fn type_(&self) -> &NetworkType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDSetNetworkTypeBuilder {
  inner: SetNetworkType
}

impl RTDSetNetworkTypeBuilder {
  pub fn build(&self) -> SetNetworkType { self.inner.clone() }

   
  pub fn type_<T: AsRef<NetworkType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<SetNetworkType> for SetNetworkType {
  fn as_ref(&self) -> &SetNetworkType { self }
}

impl AsRef<SetNetworkType> for RTDSetNetworkTypeBuilder {
  fn as_ref(&self) -> &SetNetworkType { &self.inner }
}







/// Sets the value of an option. (Check the list of available options on https://core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The name of the option
  name: String,
  /// The new value of the option
  value: OptionValue,
  
}

impl RObject for SetOption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setOption" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetOption {}

impl SetOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetOptionBuilder {
    let mut inner = SetOption::default();
    inner.td_name = "setOption".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetOptionBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

  pub fn value(&self) -> &OptionValue { &self.value }

}

#[doc(hidden)]
pub struct RTDSetOptionBuilder {
  inner: SetOption
}

impl RTDSetOptionBuilder {
  pub fn build(&self) -> SetOption { self.inner.clone() }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn value<T: AsRef<OptionValue>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().clone();
    self
  }

}

impl AsRef<SetOption> for SetOption {
  fn as_ref(&self) -> &SetOption { self }
}

impl AsRef<SetOption> for RTDSetOptionBuilder {
  fn as_ref(&self) -> &SetOption { &self.inner }
}







/// Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPassportElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Input Telegram Passport element
  element: InputPassportElement,
  /// Password of the current user
  password: String,
  
}

impl RObject for SetPassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPassportElement" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElement for SetPassportElement {}

impl RFunction for SetPassportElement {}

impl SetPassportElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetPassportElementBuilder {
    let mut inner = SetPassportElement::default();
    inner.td_name = "setPassportElement".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetPassportElementBuilder { inner }
  }

  pub fn element(&self) -> &InputPassportElement { &self.element }

  pub fn password(&self) -> &String { &self.password }

}

#[doc(hidden)]
pub struct RTDSetPassportElementBuilder {
  inner: SetPassportElement
}

impl RTDSetPassportElementBuilder {
  pub fn build(&self) -> SetPassportElement { self.inner.clone() }

   
  pub fn element<T: AsRef<InputPassportElement>>(&mut self, element: T) -> &mut Self {
    self.inner.element = element.as_ref().clone();
    self
  }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

}

impl AsRef<SetPassportElement> for SetPassportElement {
  fn as_ref(&self) -> &SetPassportElement { self }
}

impl AsRef<SetPassportElement> for RTDSetPassportElementBuilder {
  fn as_ref(&self) -> &SetPassportElement { &self.inner }
}







/// Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPassportElementErrors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// The errors
  errors: Vec<InputPassportElementError>,
  
}

impl RObject for SetPassportElementErrors {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPassportElementErrors" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetPassportElementErrors {}

impl SetPassportElementErrors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetPassportElementErrorsBuilder {
    let mut inner = SetPassportElementErrors::default();
    inner.td_name = "setPassportElementErrors".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetPassportElementErrorsBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn errors(&self) -> &Vec<InputPassportElementError> { &self.errors }

}

#[doc(hidden)]
pub struct RTDSetPassportElementErrorsBuilder {
  inner: SetPassportElementErrors
}

impl RTDSetPassportElementErrorsBuilder {
  pub fn build(&self) -> SetPassportElementErrors { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn errors(&mut self, errors: Vec<InputPassportElementError>) -> &mut Self {
    self.inner.errors = errors;
    self
  }

}

impl AsRef<SetPassportElementErrors> for SetPassportElementErrors {
  fn as_ref(&self) -> &SetPassportElementErrors { self }
}

impl AsRef<SetPassportElementErrors> for RTDSetPassportElementErrorsBuilder {
  fn as_ref(&self) -> &SetPassportElementErrors { &self.inner }
}







/// Changes the password for the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous password of the user
  old_password: String,
  /// New password of the user; may be empty to remove the password
  new_password: String,
  /// New password hint; may be empty
  new_hint: String,
  /// Pass true if the recovery email address should be changed
  set_recovery_email_address: bool,
  /// New recovery email address; may be empty
  new_recovery_email_address: String,
  
}

impl RObject for SetPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetPassword {}

impl SetPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetPasswordBuilder {
    let mut inner = SetPassword::default();
    inner.td_name = "setPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetPasswordBuilder { inner }
  }

  pub fn old_password(&self) -> &String { &self.old_password }

  pub fn new_password(&self) -> &String { &self.new_password }

  pub fn new_hint(&self) -> &String { &self.new_hint }

  pub fn set_recovery_email_address(&self) -> bool { self.set_recovery_email_address }

  pub fn new_recovery_email_address(&self) -> &String { &self.new_recovery_email_address }

}

#[doc(hidden)]
pub struct RTDSetPasswordBuilder {
  inner: SetPassword
}

impl RTDSetPasswordBuilder {
  pub fn build(&self) -> SetPassword { self.inner.clone() }

   
  pub fn old_password<T: AsRef<str>>(&mut self, old_password: T) -> &mut Self {
    self.inner.old_password = old_password.as_ref().to_string();
    self
  }

   
  pub fn new_password<T: AsRef<str>>(&mut self, new_password: T) -> &mut Self {
    self.inner.new_password = new_password.as_ref().to_string();
    self
  }

   
  pub fn new_hint<T: AsRef<str>>(&mut self, new_hint: T) -> &mut Self {
    self.inner.new_hint = new_hint.as_ref().to_string();
    self
  }

   
  pub fn set_recovery_email_address(&mut self, set_recovery_email_address: bool) -> &mut Self {
    self.inner.set_recovery_email_address = set_recovery_email_address;
    self
  }

   
  pub fn new_recovery_email_address<T: AsRef<str>>(&mut self, new_recovery_email_address: T) -> &mut Self {
    self.inner.new_recovery_email_address = new_recovery_email_address.as_ref().to_string();
    self
  }

}

impl AsRef<SetPassword> for SetPassword {
  fn as_ref(&self) -> &SetPassword { self }
}

impl AsRef<SetPassword> for RTDSetPasswordBuilder {
  fn as_ref(&self) -> &SetPassword { &self.inner }
}







/// Changes the order of pinned chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPinnedChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat list in which to change the order of pinned chats
  chat_list: ChatList,
  /// The new list of pinned chats
  chat_ids: Vec<i64>,
  
}

impl RObject for SetPinnedChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPinnedChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetPinnedChats {}

impl SetPinnedChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetPinnedChatsBuilder {
    let mut inner = SetPinnedChats::default();
    inner.td_name = "setPinnedChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetPinnedChatsBuilder { inner }
  }

  pub fn chat_list(&self) -> &ChatList { &self.chat_list }

  pub fn chat_ids(&self) -> &Vec<i64> { &self.chat_ids }

}

#[doc(hidden)]
pub struct RTDSetPinnedChatsBuilder {
  inner: SetPinnedChats
}

impl RTDSetPinnedChatsBuilder {
  pub fn build(&self) -> SetPinnedChats { self.inner.clone() }

   
  pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
    self.inner.chat_list = chat_list.as_ref().clone();
    self
  }

   
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }

}

impl AsRef<SetPinnedChats> for SetPinnedChats {
  fn as_ref(&self) -> &SetPinnedChats { self }
}

impl AsRef<SetPinnedChats> for RTDSetPinnedChatsBuilder {
  fn as_ref(&self) -> &SetPinnedChats { &self.inner }
}







/// Changes the user answer to a poll. A poll in quiz mode can be answered only once
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPollAnswer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to which the poll belongs
  chat_id: i64,
  /// Identifier of the message containing the poll
  message_id: i64,
  /// 0-based identifiers of answer options, chosen by the user. User can choose more than 1 answer option only is the poll allows multiple answers
  option_ids: Vec<i64>,
  
}

impl RObject for SetPollAnswer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPollAnswer" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetPollAnswer {}

impl SetPollAnswer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetPollAnswerBuilder {
    let mut inner = SetPollAnswer::default();
    inner.td_name = "setPollAnswer".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetPollAnswerBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn option_ids(&self) -> &Vec<i64> { &self.option_ids }

}

#[doc(hidden)]
pub struct RTDSetPollAnswerBuilder {
  inner: SetPollAnswer
}

impl RTDSetPollAnswerBuilder {
  pub fn build(&self) -> SetPollAnswer { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn option_ids(&mut self, option_ids: Vec<i64>) -> &mut Self {
    self.inner.option_ids = option_ids;
    self
  }

}

impl AsRef<SetPollAnswer> for SetPollAnswer {
  fn as_ref(&self) -> &SetPollAnswer { self }
}

impl AsRef<SetPollAnswer> for RTDSetPollAnswerBuilder {
  fn as_ref(&self) -> &SetPollAnswer { &self.inner }
}







/// Uploads a new profile photo for the current user. If something changes, updateUser will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Profile photo to set. inputFileId and inputFileRemote may still be unsupported
  photo: InputFile,
  
}

impl RObject for SetProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setProfilePhoto" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetProfilePhoto {}

impl SetProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetProfilePhotoBuilder {
    let mut inner = SetProfilePhoto::default();
    inner.td_name = "setProfilePhoto".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetProfilePhotoBuilder { inner }
  }

  pub fn photo(&self) -> &InputFile { &self.photo }

}

#[doc(hidden)]
pub struct RTDSetProfilePhotoBuilder {
  inner: SetProfilePhoto
}

impl RTDSetProfilePhotoBuilder {
  pub fn build(&self) -> SetProfilePhoto { self.inner.clone() }

   
  pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

}

impl AsRef<SetProfilePhoto> for SetProfilePhoto {
  fn as_ref(&self) -> &SetProfilePhoto { self }
}

impl AsRef<SetProfilePhoto> for RTDSetProfilePhotoBuilder {
  fn as_ref(&self) -> &SetProfilePhoto { &self.inner }
}







/// Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed. If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetRecoveryEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Password of the current user
  password: String,
  /// New recovery email address
  new_recovery_email_address: String,
  
}

impl RObject for SetRecoveryEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setRecoveryEmailAddress" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetRecoveryEmailAddress {}

impl SetRecoveryEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetRecoveryEmailAddressBuilder {
    let mut inner = SetRecoveryEmailAddress::default();
    inner.td_name = "setRecoveryEmailAddress".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetRecoveryEmailAddressBuilder { inner }
  }

  pub fn password(&self) -> &String { &self.password }

  pub fn new_recovery_email_address(&self) -> &String { &self.new_recovery_email_address }

}

#[doc(hidden)]
pub struct RTDSetRecoveryEmailAddressBuilder {
  inner: SetRecoveryEmailAddress
}

impl RTDSetRecoveryEmailAddressBuilder {
  pub fn build(&self) -> SetRecoveryEmailAddress { self.inner.clone() }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

   
  pub fn new_recovery_email_address<T: AsRef<str>>(&mut self, new_recovery_email_address: T) -> &mut Self {
    self.inner.new_recovery_email_address = new_recovery_email_address.as_ref().to_string();
    self
  }

}

impl AsRef<SetRecoveryEmailAddress> for SetRecoveryEmailAddress {
  fn as_ref(&self) -> &SetRecoveryEmailAddress { self }
}

impl AsRef<SetRecoveryEmailAddress> for RTDSetRecoveryEmailAddressBuilder {
  fn as_ref(&self) -> &SetRecoveryEmailAddress { &self.inner }
}







/// Changes notification settings for chats of a given type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetScopeNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Types of chats for which to change the notification settings
  scope: NotificationSettingsScope,
  /// The new notification settings for the given scope
  notification_settings: ScopeNotificationSettings,
  
}

impl RObject for SetScopeNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setScopeNotificationSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetScopeNotificationSettings {}

impl SetScopeNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetScopeNotificationSettingsBuilder {
    let mut inner = SetScopeNotificationSettings::default();
    inner.td_name = "setScopeNotificationSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetScopeNotificationSettingsBuilder { inner }
  }

  pub fn scope(&self) -> &NotificationSettingsScope { &self.scope }

  pub fn notification_settings(&self) -> &ScopeNotificationSettings { &self.notification_settings }

}

#[doc(hidden)]
pub struct RTDSetScopeNotificationSettingsBuilder {
  inner: SetScopeNotificationSettings
}

impl RTDSetScopeNotificationSettingsBuilder {
  pub fn build(&self) -> SetScopeNotificationSettings { self.inner.clone() }

   
  pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
    self.inner.scope = scope.as_ref().clone();
    self
  }

   
  pub fn notification_settings<T: AsRef<ScopeNotificationSettings>>(&mut self, notification_settings: T) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }

}

impl AsRef<SetScopeNotificationSettings> for SetScopeNotificationSettings {
  fn as_ref(&self) -> &SetScopeNotificationSettings { self }
}

impl AsRef<SetScopeNotificationSettings> for RTDSetScopeNotificationSettingsBuilder {
  fn as_ref(&self) -> &SetScopeNotificationSettings { &self.inner }
}







/// Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStickerPositionInSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker
  sticker: InputFile,
  /// New position of the sticker in the set, zero-based
  position: i64,
  
}

impl RObject for SetStickerPositionInSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setStickerPositionInSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetStickerPositionInSet {}

impl SetStickerPositionInSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetStickerPositionInSetBuilder {
    let mut inner = SetStickerPositionInSet::default();
    inner.td_name = "setStickerPositionInSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetStickerPositionInSetBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

  pub fn position(&self) -> i64 { self.position }

}

#[doc(hidden)]
pub struct RTDSetStickerPositionInSetBuilder {
  inner: SetStickerPositionInSet
}

impl RTDSetStickerPositionInSetBuilder {
  pub fn build(&self) -> SetStickerPositionInSet { self.inner.clone() }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

   
  pub fn position(&mut self, position: i64) -> &mut Self {
    self.inner.position = position;
    self
  }

}

impl AsRef<SetStickerPositionInSet> for SetStickerPositionInSet {
  fn as_ref(&self) -> &SetStickerPositionInSet { self }
}

impl AsRef<SetStickerPositionInSet> for RTDSetStickerPositionInSetBuilder {
  fn as_ref(&self) -> &SetStickerPositionInSet { &self.inner }
}







/// Changes the sticker set of a supergroup; requires can_change_info rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSupergroupStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the supergroup
  supergroup_id: i64,
  /// New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set
  sticker_set_id: isize,
  
}

impl RObject for SetSupergroupStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setSupergroupStickerSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetSupergroupStickerSet {}

impl SetSupergroupStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetSupergroupStickerSetBuilder {
    let mut inner = SetSupergroupStickerSet::default();
    inner.td_name = "setSupergroupStickerSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetSupergroupStickerSetBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn sticker_set_id(&self) -> isize { self.sticker_set_id }

}

#[doc(hidden)]
pub struct RTDSetSupergroupStickerSetBuilder {
  inner: SetSupergroupStickerSet
}

impl RTDSetSupergroupStickerSetBuilder {
  pub fn build(&self) -> SetSupergroupStickerSet { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn sticker_set_id(&mut self, sticker_set_id: isize) -> &mut Self {
    self.inner.sticker_set_id = sticker_set_id;
    self
  }

}

impl AsRef<SetSupergroupStickerSet> for SetSupergroupStickerSet {
  fn as_ref(&self) -> &SetSupergroupStickerSet { self }
}

impl AsRef<SetSupergroupStickerSet> for RTDSetSupergroupStickerSetBuilder {
  fn as_ref(&self) -> &SetSupergroupStickerSet { &self.inner }
}







/// Changes the username of a supergroup or channel, requires owner privileges in the supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSupergroupUsername {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the supergroup or channel
  supergroup_id: i64,
  /// New value of the username. Use an empty string to remove the username
  username: String,
  
}

impl RObject for SetSupergroupUsername {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setSupergroupUsername" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetSupergroupUsername {}

impl SetSupergroupUsername {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetSupergroupUsernameBuilder {
    let mut inner = SetSupergroupUsername::default();
    inner.td_name = "setSupergroupUsername".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetSupergroupUsernameBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn username(&self) -> &String { &self.username }

}

#[doc(hidden)]
pub struct RTDSetSupergroupUsernameBuilder {
  inner: SetSupergroupUsername
}

impl RTDSetSupergroupUsernameBuilder {
  pub fn build(&self) -> SetSupergroupUsername { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

}

impl AsRef<SetSupergroupUsername> for SetSupergroupUsername {
  fn as_ref(&self) -> &SetSupergroupUsername { self }
}

impl AsRef<SetSupergroupUsername> for RTDSetSupergroupUsernameBuilder {
  fn as_ref(&self) -> &SetSupergroupUsername { &self.inner }
}







/// Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetTdlibParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Parameters
  parameters: TdlibParameters,
  
}

impl RObject for SetTdlibParameters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setTdlibParameters" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetTdlibParameters {}

impl SetTdlibParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetTdlibParametersBuilder {
    let mut inner = SetTdlibParameters::default();
    inner.td_name = "setTdlibParameters".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetTdlibParametersBuilder { inner }
  }

  pub fn parameters(&self) -> &TdlibParameters { &self.parameters }

}

#[doc(hidden)]
pub struct RTDSetTdlibParametersBuilder {
  inner: SetTdlibParameters
}

impl RTDSetTdlibParametersBuilder {
  pub fn build(&self) -> SetTdlibParameters { self.inner.clone() }

   
  pub fn parameters<T: AsRef<TdlibParameters>>(&mut self, parameters: T) -> &mut Self {
    self.inner.parameters = parameters.as_ref().clone();
    self
  }

}

impl AsRef<SetTdlibParameters> for SetTdlibParameters {
  fn as_ref(&self) -> &SetTdlibParameters { self }
}

impl AsRef<SetTdlibParameters> for RTDSetTdlibParametersBuilder {
  fn as_ref(&self) -> &SetTdlibParameters { &self.inner }
}







/// Changes user privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetUserPrivacySettingRules {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The privacy setting
  setting: UserPrivacySetting,
  /// The new privacy rules
  rules: UserPrivacySettingRules,
  
}

impl RObject for SetUserPrivacySettingRules {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setUserPrivacySettingRules" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetUserPrivacySettingRules {}

impl SetUserPrivacySettingRules {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetUserPrivacySettingRulesBuilder {
    let mut inner = SetUserPrivacySettingRules::default();
    inner.td_name = "setUserPrivacySettingRules".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetUserPrivacySettingRulesBuilder { inner }
  }

  pub fn setting(&self) -> &UserPrivacySetting { &self.setting }

  pub fn rules(&self) -> &UserPrivacySettingRules { &self.rules }

}

#[doc(hidden)]
pub struct RTDSetUserPrivacySettingRulesBuilder {
  inner: SetUserPrivacySettingRules
}

impl RTDSetUserPrivacySettingRulesBuilder {
  pub fn build(&self) -> SetUserPrivacySettingRules { self.inner.clone() }

   
  pub fn setting<T: AsRef<UserPrivacySetting>>(&mut self, setting: T) -> &mut Self {
    self.inner.setting = setting.as_ref().clone();
    self
  }

   
  pub fn rules<T: AsRef<UserPrivacySettingRules>>(&mut self, rules: T) -> &mut Self {
    self.inner.rules = rules.as_ref().clone();
    self
  }

}

impl AsRef<SetUserPrivacySettingRules> for SetUserPrivacySettingRules {
  fn as_ref(&self) -> &SetUserPrivacySettingRules { self }
}

impl AsRef<SetUserPrivacySettingRules> for RTDSetUserPrivacySettingRulesBuilder {
  fn as_ref(&self) -> &SetUserPrivacySettingRules { &self.inner }
}







/// Changes the username of the current user. If something changes, updateUser will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetUsername {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The new value of the username. Use an empty string to remove the username
  username: String,
  
}

impl RObject for SetUsername {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setUsername" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SetUsername {}

impl SetUsername {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSetUsernameBuilder {
    let mut inner = SetUsername::default();
    inner.td_name = "setUsername".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSetUsernameBuilder { inner }
  }

  pub fn username(&self) -> &String { &self.username }

}

#[doc(hidden)]
pub struct RTDSetUsernameBuilder {
  inner: SetUsername
}

impl RTDSetUsernameBuilder {
  pub fn build(&self) -> SetUsername { self.inner.clone() }

   
  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

}

impl AsRef<SetUsername> for SetUsername {
  fn as_ref(&self) -> &SetUsername { self }
}

impl AsRef<SetUsername> for RTDSetUsernameBuilder {
  fn as_ref(&self) -> &SetUsername { &self.inner }
}







/// Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SharePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user with whom to share the phone number. The user must be a mutual contact
  user_id: i64,
  
}

impl RObject for SharePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sharePhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SharePhoneNumber {}

impl SharePhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSharePhoneNumberBuilder {
    let mut inner = SharePhoneNumber::default();
    inner.td_name = "sharePhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSharePhoneNumberBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDSharePhoneNumberBuilder {
  inner: SharePhoneNumber
}

impl RTDSharePhoneNumberBuilder {
  pub fn build(&self) -> SharePhoneNumber { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<SharePhoneNumber> for SharePhoneNumber {
  fn as_ref(&self) -> &SharePhoneNumber { self }
}

impl AsRef<SharePhoneNumber> for RTDSharePhoneNumberBuilder {
  fn as_ref(&self) -> &SharePhoneNumber { &self.inner }
}







/// Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StopPoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to which the poll belongs
  chat_id: i64,
  /// Identifier of the message containing the poll
  message_id: i64,
  /// The new message reply markup; for bots only
  reply_markup: ReplyMarkup,
  
}

impl RObject for StopPoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stopPoll" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for StopPoll {}

impl StopPoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStopPollBuilder {
    let mut inner = StopPoll::default();
    inner.td_name = "stopPoll".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStopPollBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn reply_markup(&self) -> &ReplyMarkup { &self.reply_markup }

}

#[doc(hidden)]
pub struct RTDStopPollBuilder {
  inner: StopPoll
}

impl RTDStopPollBuilder {
  pub fn build(&self) -> StopPoll { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = reply_markup.as_ref().clone();
    self
  }

}

impl AsRef<StopPoll> for StopPoll {
  fn as_ref(&self) -> &StopPoll { self }
}

impl AsRef<StopPoll> for RTDStopPollBuilder {
  fn as_ref(&self) -> &StopPoll { &self.inner }
}







/// Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SynchronizeLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Language pack identifier
  language_pack_id: String,
  
}

impl RObject for SynchronizeLanguagePack {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "synchronizeLanguagePack" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for SynchronizeLanguagePack {}

impl SynchronizeLanguagePack {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSynchronizeLanguagePackBuilder {
    let mut inner = SynchronizeLanguagePack::default();
    inner.td_name = "synchronizeLanguagePack".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSynchronizeLanguagePackBuilder { inner }
  }

  pub fn language_pack_id(&self) -> &String { &self.language_pack_id }

}

#[doc(hidden)]
pub struct RTDSynchronizeLanguagePackBuilder {
  inner: SynchronizeLanguagePack
}

impl RTDSynchronizeLanguagePackBuilder {
  pub fn build(&self) -> SynchronizeLanguagePack { self.inner.clone() }

   
  pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
    self.inner.language_pack_id = language_pack_id.as_ref().to_string();
    self
  }

}

impl AsRef<SynchronizeLanguagePack> for SynchronizeLanguagePack {
  fn as_ref(&self) -> &SynchronizeLanguagePack { self }
}

impl AsRef<SynchronizeLanguagePack> for RTDSynchronizeLanguagePackBuilder {
  fn as_ref(&self) -> &SynchronizeLanguagePack { &self.inner }
}







/// Terminates all other sessions of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminateAllOtherSessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TerminateAllOtherSessions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "terminateAllOtherSessions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TerminateAllOtherSessions {}

impl TerminateAllOtherSessions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTerminateAllOtherSessionsBuilder {
    let mut inner = TerminateAllOtherSessions::default();
    inner.td_name = "terminateAllOtherSessions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTerminateAllOtherSessionsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTerminateAllOtherSessionsBuilder {
  inner: TerminateAllOtherSessions
}

impl RTDTerminateAllOtherSessionsBuilder {
  pub fn build(&self) -> TerminateAllOtherSessions { self.inner.clone() }

}

impl AsRef<TerminateAllOtherSessions> for TerminateAllOtherSessions {
  fn as_ref(&self) -> &TerminateAllOtherSessions { self }
}

impl AsRef<TerminateAllOtherSessions> for RTDTerminateAllOtherSessionsBuilder {
  fn as_ref(&self) -> &TerminateAllOtherSessions { &self.inner }
}







/// Terminates a session of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminateSession {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Session identifier
  session_id: isize,
  
}

impl RObject for TerminateSession {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "terminateSession" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TerminateSession {}

impl TerminateSession {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTerminateSessionBuilder {
    let mut inner = TerminateSession::default();
    inner.td_name = "terminateSession".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTerminateSessionBuilder { inner }
  }

  pub fn session_id(&self) -> isize { self.session_id }

}

#[doc(hidden)]
pub struct RTDTerminateSessionBuilder {
  inner: TerminateSession
}

impl RTDTerminateSessionBuilder {
  pub fn build(&self) -> TerminateSession { self.inner.clone() }

   
  pub fn session_id(&mut self, session_id: isize) -> &mut Self {
    self.inner.session_id = session_id;
    self
  }

}

impl AsRef<TerminateSession> for TerminateSession {
  fn as_ref(&self) -> &TerminateSession { self }
}

impl AsRef<TerminateSession> for RTDTerminateSessionBuilder {
  fn as_ref(&self) -> &TerminateSession { &self.inner }
}







/// Returns the received bytes; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallBytes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Bytes to return
  x: String,
  
}

impl RObject for TestCallBytes {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallBytes" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestCallBytes {}

impl TestCallBytes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestCallBytesBuilder {
    let mut inner = TestCallBytes::default();
    inner.td_name = "testCallBytes".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestCallBytesBuilder { inner }
  }

  pub fn x(&self) -> &String { &self.x }

}

#[doc(hidden)]
pub struct RTDTestCallBytesBuilder {
  inner: TestCallBytes
}

impl RTDTestCallBytesBuilder {
  pub fn build(&self) -> TestCallBytes { self.inner.clone() }

   
  pub fn x<T: AsRef<str>>(&mut self, x: T) -> &mut Self {
    self.inner.x = x.as_ref().to_string();
    self
  }

}

impl AsRef<TestCallBytes> for TestCallBytes {
  fn as_ref(&self) -> &TestCallBytes { self }
}

impl AsRef<TestCallBytes> for RTDTestCallBytesBuilder {
  fn as_ref(&self) -> &TestCallBytes { &self.inner }
}







/// Does nothing; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TestCallEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallEmpty" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestCallEmpty {}

impl TestCallEmpty {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestCallEmptyBuilder {
    let mut inner = TestCallEmpty::default();
    inner.td_name = "testCallEmpty".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestCallEmptyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTestCallEmptyBuilder {
  inner: TestCallEmpty
}

impl RTDTestCallEmptyBuilder {
  pub fn build(&self) -> TestCallEmpty { self.inner.clone() }

}

impl AsRef<TestCallEmpty> for TestCallEmpty {
  fn as_ref(&self) -> &TestCallEmpty { self }
}

impl AsRef<TestCallEmpty> for RTDTestCallEmptyBuilder {
  fn as_ref(&self) -> &TestCallEmpty { &self.inner }
}







/// Returns the received string; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// String to return
  x: String,
  
}

impl RObject for TestCallString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallString" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestCallString {}

impl TestCallString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestCallStringBuilder {
    let mut inner = TestCallString::default();
    inner.td_name = "testCallString".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestCallStringBuilder { inner }
  }

  pub fn x(&self) -> &String { &self.x }

}

#[doc(hidden)]
pub struct RTDTestCallStringBuilder {
  inner: TestCallString
}

impl RTDTestCallStringBuilder {
  pub fn build(&self) -> TestCallString { self.inner.clone() }

   
  pub fn x<T: AsRef<str>>(&mut self, x: T) -> &mut Self {
    self.inner.x = x.as_ref().to_string();
    self
  }

}

impl AsRef<TestCallString> for TestCallString {
  fn as_ref(&self) -> &TestCallString { self }
}

impl AsRef<TestCallString> for RTDTestCallStringBuilder {
  fn as_ref(&self) -> &TestCallString { &self.inner }
}







/// Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallVectorInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Vector of numbers to return
  x: Vec<i64>,
  
}

impl RObject for TestCallVectorInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallVectorInt" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestCallVectorInt {}

impl TestCallVectorInt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestCallVectorIntBuilder {
    let mut inner = TestCallVectorInt::default();
    inner.td_name = "testCallVectorInt".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestCallVectorIntBuilder { inner }
  }

  pub fn x(&self) -> &Vec<i64> { &self.x }

}

#[doc(hidden)]
pub struct RTDTestCallVectorIntBuilder {
  inner: TestCallVectorInt
}

impl RTDTestCallVectorIntBuilder {
  pub fn build(&self) -> TestCallVectorInt { self.inner.clone() }

   
  pub fn x(&mut self, x: Vec<i64>) -> &mut Self {
    self.inner.x = x;
    self
  }

}

impl AsRef<TestCallVectorInt> for TestCallVectorInt {
  fn as_ref(&self) -> &TestCallVectorInt { self }
}

impl AsRef<TestCallVectorInt> for RTDTestCallVectorIntBuilder {
  fn as_ref(&self) -> &TestCallVectorInt { &self.inner }
}







/// Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallVectorIntObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Vector of objects to return
  x: Vec<TestInt>,
  
}

impl RObject for TestCallVectorIntObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallVectorIntObject" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestCallVectorIntObject {}

impl TestCallVectorIntObject {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestCallVectorIntObjectBuilder {
    let mut inner = TestCallVectorIntObject::default();
    inner.td_name = "testCallVectorIntObject".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestCallVectorIntObjectBuilder { inner }
  }

  pub fn x(&self) -> &Vec<TestInt> { &self.x }

}

#[doc(hidden)]
pub struct RTDTestCallVectorIntObjectBuilder {
  inner: TestCallVectorIntObject
}

impl RTDTestCallVectorIntObjectBuilder {
  pub fn build(&self) -> TestCallVectorIntObject { self.inner.clone() }

   
  pub fn x(&mut self, x: Vec<TestInt>) -> &mut Self {
    self.inner.x = x;
    self
  }

}

impl AsRef<TestCallVectorIntObject> for TestCallVectorIntObject {
  fn as_ref(&self) -> &TestCallVectorIntObject { self }
}

impl AsRef<TestCallVectorIntObject> for RTDTestCallVectorIntObjectBuilder {
  fn as_ref(&self) -> &TestCallVectorIntObject { &self.inner }
}







/// Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallVectorString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Vector of strings to return
  x: Vec<String>,
  
}

impl RObject for TestCallVectorString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallVectorString" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestCallVectorString {}

impl TestCallVectorString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestCallVectorStringBuilder {
    let mut inner = TestCallVectorString::default();
    inner.td_name = "testCallVectorString".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestCallVectorStringBuilder { inner }
  }

  pub fn x(&self) -> &Vec<String> { &self.x }

}

#[doc(hidden)]
pub struct RTDTestCallVectorStringBuilder {
  inner: TestCallVectorString
}

impl RTDTestCallVectorStringBuilder {
  pub fn build(&self) -> TestCallVectorString { self.inner.clone() }

   
  pub fn x(&mut self, x: Vec<String>) -> &mut Self {
    self.inner.x = x;
    self
  }

}

impl AsRef<TestCallVectorString> for TestCallVectorString {
  fn as_ref(&self) -> &TestCallVectorString { self }
}

impl AsRef<TestCallVectorString> for RTDTestCallVectorStringBuilder {
  fn as_ref(&self) -> &TestCallVectorString { &self.inner }
}







/// Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallVectorStringObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Vector of objects to return
  x: Vec<TestString>,
  
}

impl RObject for TestCallVectorStringObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testCallVectorStringObject" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestCallVectorStringObject {}

impl TestCallVectorStringObject {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestCallVectorStringObjectBuilder {
    let mut inner = TestCallVectorStringObject::default();
    inner.td_name = "testCallVectorStringObject".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestCallVectorStringObjectBuilder { inner }
  }

  pub fn x(&self) -> &Vec<TestString> { &self.x }

}

#[doc(hidden)]
pub struct RTDTestCallVectorStringObjectBuilder {
  inner: TestCallVectorStringObject
}

impl RTDTestCallVectorStringObjectBuilder {
  pub fn build(&self) -> TestCallVectorStringObject { self.inner.clone() }

   
  pub fn x(&mut self, x: Vec<TestString>) -> &mut Self {
    self.inner.x = x;
    self
  }

}

impl AsRef<TestCallVectorStringObject> for TestCallVectorStringObject {
  fn as_ref(&self) -> &TestCallVectorStringObject { self }
}

impl AsRef<TestCallVectorStringObject> for RTDTestCallVectorStringObjectBuilder {
  fn as_ref(&self) -> &TestCallVectorStringObject { &self.inner }
}







/// Forces an updates.getDifference call to the Telegram servers; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestGetDifference {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TestGetDifference {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testGetDifference" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestGetDifference {}

impl TestGetDifference {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestGetDifferenceBuilder {
    let mut inner = TestGetDifference::default();
    inner.td_name = "testGetDifference".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestGetDifferenceBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTestGetDifferenceBuilder {
  inner: TestGetDifference
}

impl RTDTestGetDifferenceBuilder {
  pub fn build(&self) -> TestGetDifference { self.inner.clone() }

}

impl AsRef<TestGetDifference> for TestGetDifference {
  fn as_ref(&self) -> &TestGetDifference { self }
}

impl AsRef<TestGetDifference> for RTDTestGetDifferenceBuilder {
  fn as_ref(&self) -> &TestGetDifference { &self.inner }
}







/// Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestNetwork {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TestNetwork {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testNetwork" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestNetwork {}

impl TestNetwork {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestNetworkBuilder {
    let mut inner = TestNetwork::default();
    inner.td_name = "testNetwork".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestNetworkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTestNetworkBuilder {
  inner: TestNetwork
}

impl RTDTestNetworkBuilder {
  pub fn build(&self) -> TestNetwork { self.inner.clone() }

}

impl AsRef<TestNetwork> for TestNetwork {
  fn as_ref(&self) -> &TestNetwork { self }
}

impl AsRef<TestNetwork> for RTDTestNetworkBuilder {
  fn as_ref(&self) -> &TestNetwork { &self.inner }
}







/// Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Proxy server IP address
  server: String,
  /// Proxy server port
  port: i64,
  /// Proxy type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: ProxyType,
  /// Identifier of a datacenter, with which to test connection
  dc_id: i64,
  /// The maximum overall timeout for the request
  timeout: f32,
  
}

impl RObject for TestProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testProxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestProxy {}

impl TestProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestProxyBuilder {
    let mut inner = TestProxy::default();
    inner.td_name = "testProxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestProxyBuilder { inner }
  }

  pub fn server(&self) -> &String { &self.server }

  pub fn port(&self) -> i64 { self.port }

  pub fn type_(&self) -> &ProxyType { &self.type_ }

  pub fn dc_id(&self) -> i64 { self.dc_id }

  pub fn timeout(&self) -> f32 { self.timeout }

}

#[doc(hidden)]
pub struct RTDTestProxyBuilder {
  inner: TestProxy
}

impl RTDTestProxyBuilder {
  pub fn build(&self) -> TestProxy { self.inner.clone() }

   
  pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
    self.inner.server = server.as_ref().to_string();
    self
  }

   
  pub fn port(&mut self, port: i64) -> &mut Self {
    self.inner.port = port;
    self
  }

   
  pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn dc_id(&mut self, dc_id: i64) -> &mut Self {
    self.inner.dc_id = dc_id;
    self
  }

   
  pub fn timeout(&mut self, timeout: f32) -> &mut Self {
    self.inner.timeout = timeout;
    self
  }

}

impl AsRef<TestProxy> for TestProxy {
  fn as_ref(&self) -> &TestProxy { self }
}

impl AsRef<TestProxy> for RTDTestProxyBuilder {
  fn as_ref(&self) -> &TestProxy { &self.inner }
}







/// Returns the specified error and ensures that the Error object is used; for testing only. This is an offline method. Can be called before authorization. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestReturnError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The error to be returned
  error: Error,
  
}

impl RObject for TestReturnError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testReturnError" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestReturnError {}

impl TestReturnError {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestReturnErrorBuilder {
    let mut inner = TestReturnError::default();
    inner.td_name = "testReturnError".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestReturnErrorBuilder { inner }
  }

  pub fn error(&self) -> &Error { &self.error }

}

#[doc(hidden)]
pub struct RTDTestReturnErrorBuilder {
  inner: TestReturnError
}

impl RTDTestReturnErrorBuilder {
  pub fn build(&self) -> TestReturnError { self.inner.clone() }

   
  pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
    self.inner.error = error.as_ref().clone();
    self
  }

}

impl AsRef<TestReturnError> for TestReturnError {
  fn as_ref(&self) -> &TestReturnError { self }
}

impl AsRef<TestReturnError> for RTDTestReturnErrorBuilder {
  fn as_ref(&self) -> &TestReturnError { &self.inner }
}







/// Returns the squared received number; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestSquareInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number to square
  x: i64,
  
}

impl RObject for TestSquareInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testSquareInt" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TestSquareInt {}

impl TestSquareInt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestSquareIntBuilder {
    let mut inner = TestSquareInt::default();
    inner.td_name = "testSquareInt".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestSquareIntBuilder { inner }
  }

  pub fn x(&self) -> i64 { self.x }

}

#[doc(hidden)]
pub struct RTDTestSquareIntBuilder {
  inner: TestSquareInt
}

impl RTDTestSquareIntBuilder {
  pub fn build(&self) -> TestSquareInt { self.inner.clone() }

   
  pub fn x(&mut self, x: i64) -> &mut Self {
    self.inner.x = x;
    self
  }

}

impl AsRef<TestSquareInt> for TestSquareInt {
  fn as_ref(&self) -> &TestSquareInt { self }
}

impl AsRef<TestSquareInt> for RTDTestSquareIntBuilder {
  fn as_ref(&self) -> &TestSquareInt { &self.inner }
}







/// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestUseUpdate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TestUseUpdate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testUseUpdate" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUpdate for TestUseUpdate {}

impl RFunction for TestUseUpdate {}

impl TestUseUpdate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestUseUpdateBuilder {
    let mut inner = TestUseUpdate::default();
    inner.td_name = "testUseUpdate".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestUseUpdateBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTestUseUpdateBuilder {
  inner: TestUseUpdate
}

impl RTDTestUseUpdateBuilder {
  pub fn build(&self) -> TestUseUpdate { self.inner.clone() }

}

impl AsRef<TestUseUpdate> for TestUseUpdate {
  fn as_ref(&self) -> &TestUseUpdate { self }
}

impl AsRef<TestUseUpdate> for RTDTestUseUpdateBuilder {
  fn as_ref(&self) -> &TestUseUpdate { &self.inner }
}







/// Changes the value of the default disable_notification parameter, used when a message is sent to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleChatDefaultDisableNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of default_disable_notification
  default_disable_notification: bool,
  
}

impl RObject for ToggleChatDefaultDisableNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleChatDefaultDisableNotification" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ToggleChatDefaultDisableNotification {}

impl ToggleChatDefaultDisableNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDToggleChatDefaultDisableNotificationBuilder {
    let mut inner = ToggleChatDefaultDisableNotification::default();
    inner.td_name = "toggleChatDefaultDisableNotification".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDToggleChatDefaultDisableNotificationBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn default_disable_notification(&self) -> bool { self.default_disable_notification }

}

#[doc(hidden)]
pub struct RTDToggleChatDefaultDisableNotificationBuilder {
  inner: ToggleChatDefaultDisableNotification
}

impl RTDToggleChatDefaultDisableNotificationBuilder {
  pub fn build(&self) -> ToggleChatDefaultDisableNotification { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn default_disable_notification(&mut self, default_disable_notification: bool) -> &mut Self {
    self.inner.default_disable_notification = default_disable_notification;
    self
  }

}

impl AsRef<ToggleChatDefaultDisableNotification> for ToggleChatDefaultDisableNotification {
  fn as_ref(&self) -> &ToggleChatDefaultDisableNotification { self }
}

impl AsRef<ToggleChatDefaultDisableNotification> for RTDToggleChatDefaultDisableNotificationBuilder {
  fn as_ref(&self) -> &ToggleChatDefaultDisableNotification { &self.inner }
}







/// Changes the marked as unread state of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleChatIsMarkedAsUnread {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of is_marked_as_unread
  is_marked_as_unread: bool,
  
}

impl RObject for ToggleChatIsMarkedAsUnread {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleChatIsMarkedAsUnread" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ToggleChatIsMarkedAsUnread {}

impl ToggleChatIsMarkedAsUnread {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDToggleChatIsMarkedAsUnreadBuilder {
    let mut inner = ToggleChatIsMarkedAsUnread::default();
    inner.td_name = "toggleChatIsMarkedAsUnread".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDToggleChatIsMarkedAsUnreadBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn is_marked_as_unread(&self) -> bool { self.is_marked_as_unread }

}

#[doc(hidden)]
pub struct RTDToggleChatIsMarkedAsUnreadBuilder {
  inner: ToggleChatIsMarkedAsUnread
}

impl RTDToggleChatIsMarkedAsUnreadBuilder {
  pub fn build(&self) -> ToggleChatIsMarkedAsUnread { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
    self.inner.is_marked_as_unread = is_marked_as_unread;
    self
  }

}

impl AsRef<ToggleChatIsMarkedAsUnread> for ToggleChatIsMarkedAsUnread {
  fn as_ref(&self) -> &ToggleChatIsMarkedAsUnread { self }
}

impl AsRef<ToggleChatIsMarkedAsUnread> for RTDToggleChatIsMarkedAsUnreadBuilder {
  fn as_ref(&self) -> &ToggleChatIsMarkedAsUnread { &self.inner }
}







/// Changes the pinned state of a chat. You can pin up to GetOption("pinned_chat_count_max")/GetOption("pinned_archived_chat_count_max") non-secret chats and the same number of secret chats in the main/archive chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleChatIsPinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// New value of is_pinned
  is_pinned: bool,
  
}

impl RObject for ToggleChatIsPinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleChatIsPinned" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ToggleChatIsPinned {}

impl ToggleChatIsPinned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDToggleChatIsPinnedBuilder {
    let mut inner = ToggleChatIsPinned::default();
    inner.td_name = "toggleChatIsPinned".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDToggleChatIsPinnedBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDToggleChatIsPinnedBuilder {
  inner: ToggleChatIsPinned
}

impl RTDToggleChatIsPinnedBuilder {
  pub fn build(&self) -> ToggleChatIsPinned { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

}

impl AsRef<ToggleChatIsPinned> for ToggleChatIsPinned {
  fn as_ref(&self) -> &ToggleChatIsPinned { self }
}

impl AsRef<ToggleChatIsPinned> for RTDToggleChatIsPinnedBuilder {
  fn as_ref(&self) -> &ToggleChatIsPinned { &self.inner }
}







/// Toggles whether the message history of a supergroup is available to new members; requires can_change_info rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupIsAllHistoryAvailable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the supergroup
  supergroup_id: i64,
  /// The new value of is_all_history_available
  is_all_history_available: bool,
  
}

impl RObject for ToggleSupergroupIsAllHistoryAvailable {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleSupergroupIsAllHistoryAvailable" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ToggleSupergroupIsAllHistoryAvailable {}

impl ToggleSupergroupIsAllHistoryAvailable {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDToggleSupergroupIsAllHistoryAvailableBuilder {
    let mut inner = ToggleSupergroupIsAllHistoryAvailable::default();
    inner.td_name = "toggleSupergroupIsAllHistoryAvailable".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDToggleSupergroupIsAllHistoryAvailableBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn is_all_history_available(&self) -> bool { self.is_all_history_available }

}

#[doc(hidden)]
pub struct RTDToggleSupergroupIsAllHistoryAvailableBuilder {
  inner: ToggleSupergroupIsAllHistoryAvailable
}

impl RTDToggleSupergroupIsAllHistoryAvailableBuilder {
  pub fn build(&self) -> ToggleSupergroupIsAllHistoryAvailable { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
    self.inner.is_all_history_available = is_all_history_available;
    self
  }

}

impl AsRef<ToggleSupergroupIsAllHistoryAvailable> for ToggleSupergroupIsAllHistoryAvailable {
  fn as_ref(&self) -> &ToggleSupergroupIsAllHistoryAvailable { self }
}

impl AsRef<ToggleSupergroupIsAllHistoryAvailable> for RTDToggleSupergroupIsAllHistoryAvailableBuilder {
  fn as_ref(&self) -> &ToggleSupergroupIsAllHistoryAvailable { &self.inner }
}







/// Toggles sender signatures messages sent in a channel; requires can_change_info rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupSignMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel
  supergroup_id: i64,
  /// New value of sign_messages
  sign_messages: bool,
  
}

impl RObject for ToggleSupergroupSignMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "toggleSupergroupSignMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ToggleSupergroupSignMessages {}

impl ToggleSupergroupSignMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDToggleSupergroupSignMessagesBuilder {
    let mut inner = ToggleSupergroupSignMessages::default();
    inner.td_name = "toggleSupergroupSignMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDToggleSupergroupSignMessagesBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn sign_messages(&self) -> bool { self.sign_messages }

}

#[doc(hidden)]
pub struct RTDToggleSupergroupSignMessagesBuilder {
  inner: ToggleSupergroupSignMessages
}

impl RTDToggleSupergroupSignMessagesBuilder {
  pub fn build(&self) -> ToggleSupergroupSignMessages { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
    self.inner.sign_messages = sign_messages;
    self
  }

}

impl AsRef<ToggleSupergroupSignMessages> for ToggleSupergroupSignMessages {
  fn as_ref(&self) -> &ToggleSupergroupSignMessages { self }
}

impl AsRef<ToggleSupergroupSignMessages> for RTDToggleSupergroupSignMessagesBuilder {
  fn as_ref(&self) -> &ToggleSupergroupSignMessages { &self.inner }
}







/// Changes the owner of a chat. The current user must be a current owner of the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session. Available only for supergroups and channel chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferChatOwnership {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Identifier of the user to which transfer the ownership. The ownership can't be transferred to a bot or to a deleted user
  user_id: i64,
  /// The password of the current user
  password: String,
  
}

impl RObject for TransferChatOwnership {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "transferChatOwnership" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for TransferChatOwnership {}

impl TransferChatOwnership {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTransferChatOwnershipBuilder {
    let mut inner = TransferChatOwnership::default();
    inner.td_name = "transferChatOwnership".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTransferChatOwnershipBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn password(&self) -> &String { &self.password }

}

#[doc(hidden)]
pub struct RTDTransferChatOwnershipBuilder {
  inner: TransferChatOwnership
}

impl RTDTransferChatOwnershipBuilder {
  pub fn build(&self) -> TransferChatOwnership { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
    self.inner.password = password.as_ref().to_string();
    self
  }

}

impl AsRef<TransferChatOwnership> for TransferChatOwnership {
  fn as_ref(&self) -> &TransferChatOwnership { self }
}

impl AsRef<TransferChatOwnership> for RTDTransferChatOwnershipBuilder {
  fn as_ref(&self) -> &TransferChatOwnership { &self.inner }
}







/// Removes a user from the blacklist
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnblockUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  
}

impl RObject for UnblockUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "unblockUser" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for UnblockUser {}

impl UnblockUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUnblockUserBuilder {
    let mut inner = UnblockUser::default();
    inner.td_name = "unblockUser".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUnblockUserBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDUnblockUserBuilder {
  inner: UnblockUser
}

impl RTDUnblockUserBuilder {
  pub fn build(&self) -> UnblockUser { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<UnblockUser> for UnblockUser {
  fn as_ref(&self) -> &UnblockUser { self }
}

impl AsRef<UnblockUser> for RTDUnblockUserBuilder {
  fn as_ref(&self) -> &UnblockUser { &self.inner }
}







/// Removes the pinned message from a chat; requires can_pin_messages rights in the group or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnpinChatMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat
  chat_id: i64,
  
}

impl RObject for UnpinChatMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "unpinChatMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for UnpinChatMessage {}

impl UnpinChatMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUnpinChatMessageBuilder {
    let mut inner = UnpinChatMessage::default();
    inner.td_name = "unpinChatMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUnpinChatMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDUnpinChatMessageBuilder {
  inner: UnpinChatMessage
}

impl RTDUnpinChatMessageBuilder {
  pub fn build(&self) -> UnpinChatMessage { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<UnpinChatMessage> for UnpinChatMessage {
  fn as_ref(&self) -> &UnpinChatMessage { self }
}

impl AsRef<UnpinChatMessage> for RTDUnpinChatMessageBuilder {
  fn as_ref(&self) -> &UnpinChatMessage { &self.inner }
}







/// Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires creator privileges. Deactivates the original basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeBasicGroupChatToSupergroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to upgrade
  chat_id: i64,
  
}

impl RObject for UpgradeBasicGroupChatToSupergroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "upgradeBasicGroupChatToSupergroupChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for UpgradeBasicGroupChatToSupergroupChat {}

impl UpgradeBasicGroupChatToSupergroupChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUpgradeBasicGroupChatToSupergroupChatBuilder {
    let mut inner = UpgradeBasicGroupChatToSupergroupChat::default();
    inner.td_name = "upgradeBasicGroupChatToSupergroupChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUpgradeBasicGroupChatToSupergroupChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDUpgradeBasicGroupChatToSupergroupChatBuilder {
  inner: UpgradeBasicGroupChatToSupergroupChat
}

impl RTDUpgradeBasicGroupChatToSupergroupChatBuilder {
  pub fn build(&self) -> UpgradeBasicGroupChatToSupergroupChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<UpgradeBasicGroupChatToSupergroupChat> for UpgradeBasicGroupChatToSupergroupChat {
  fn as_ref(&self) -> &UpgradeBasicGroupChatToSupergroupChat { self }
}

impl AsRef<UpgradeBasicGroupChatToSupergroupChat> for RTDUpgradeBasicGroupChatToSupergroupChatBuilder {
  fn as_ref(&self) -> &UpgradeBasicGroupChatToSupergroupChat { &self.inner }
}







/// Asynchronously uploads a file to the cloud without sending it in a message. updateFile will be used to notify about upload progress and successful completion of the upload. The file will not have a persistent remote identifier until it will be sent in a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File to upload
  file: InputFile,
  /// File type
  file_type: FileType,
  /// Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which uploadFile was called will be uploaded first
  priority: i64,
  
}

impl RObject for UploadFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "uploadFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for UploadFile {}

impl UploadFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUploadFileBuilder {
    let mut inner = UploadFile::default();
    inner.td_name = "uploadFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUploadFileBuilder { inner }
  }

  pub fn file(&self) -> &InputFile { &self.file }

  pub fn file_type(&self) -> &FileType { &self.file_type }

  pub fn priority(&self) -> i64 { self.priority }

}

#[doc(hidden)]
pub struct RTDUploadFileBuilder {
  inner: UploadFile
}

impl RTDUploadFileBuilder {
  pub fn build(&self) -> UploadFile { self.inner.clone() }

   
  pub fn file<T: AsRef<InputFile>>(&mut self, file: T) -> &mut Self {
    self.inner.file = file.as_ref().clone();
    self
  }

   
  pub fn file_type<T: AsRef<FileType>>(&mut self, file_type: T) -> &mut Self {
    self.inner.file_type = file_type.as_ref().clone();
    self
  }

   
  pub fn priority(&mut self, priority: i64) -> &mut Self {
    self.inner.priority = priority;
    self
  }

}

impl AsRef<UploadFile> for UploadFile {
  fn as_ref(&self) -> &UploadFile { self }
}

impl AsRef<UploadFile> for RTDUploadFileBuilder {
  fn as_ref(&self) -> &UploadFile { &self.inner }
}







/// Uploads a PNG image with a sticker; for bots only; returns the uploaded file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadStickerFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker file owner
  user_id: i64,
  /// PNG image with the sticker; must be up to 512 kB in size and fit in 512x512 square
  png_sticker: InputFile,
  
}

impl RObject for UploadStickerFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "uploadStickerFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for UploadStickerFile {}

impl UploadStickerFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUploadStickerFileBuilder {
    let mut inner = UploadStickerFile::default();
    inner.td_name = "uploadStickerFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUploadStickerFileBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn png_sticker(&self) -> &InputFile { &self.png_sticker }

}

#[doc(hidden)]
pub struct RTDUploadStickerFileBuilder {
  inner: UploadStickerFile
}

impl RTDUploadStickerFileBuilder {
  pub fn build(&self) -> UploadStickerFile { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn png_sticker<T: AsRef<InputFile>>(&mut self, png_sticker: T) -> &mut Self {
    self.inner.png_sticker = png_sticker.as_ref().clone();
    self
  }

}

impl AsRef<UploadStickerFile> for UploadStickerFile {
  fn as_ref(&self) -> &UploadStickerFile { self }
}

impl AsRef<UploadStickerFile> for RTDUploadStickerFileBuilder {
  fn as_ref(&self) -> &UploadStickerFile { &self.inner }
}







/// Validates the order information provided by a user and returns the available shipping options for a flexible invoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidateOrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the Invoice message
  chat_id: i64,
  /// Message identifier
  message_id: i64,
  /// The order information, provided by the user
  order_info: OrderInfo,
  /// True, if the order information can be saved
  allow_save: bool,
  
}

impl RObject for ValidateOrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "validateOrderInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ValidateOrderInfo {}

impl ValidateOrderInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDValidateOrderInfoBuilder {
    let mut inner = ValidateOrderInfo::default();
    inner.td_name = "validateOrderInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDValidateOrderInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn order_info(&self) -> &OrderInfo { &self.order_info }

  pub fn allow_save(&self) -> bool { self.allow_save }

}

#[doc(hidden)]
pub struct RTDValidateOrderInfoBuilder {
  inner: ValidateOrderInfo
}

impl RTDValidateOrderInfoBuilder {
  pub fn build(&self) -> ValidateOrderInfo { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn order_info<T: AsRef<OrderInfo>>(&mut self, order_info: T) -> &mut Self {
    self.inner.order_info = order_info.as_ref().clone();
    self
  }

   
  pub fn allow_save(&mut self, allow_save: bool) -> &mut Self {
    self.inner.allow_save = allow_save;
    self
  }

}

impl AsRef<ValidateOrderInfo> for ValidateOrderInfo {
  fn as_ref(&self) -> &ValidateOrderInfo { self }
}

impl AsRef<ValidateOrderInfo> for RTDValidateOrderInfoBuilder {
  fn as_ref(&self) -> &ValidateOrderInfo { &self.inner }
}







/// Informs TDLib that messages are being viewed by the user. Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// The identifiers of the messages being viewed
  message_ids: Vec<i64>,
  /// True, if messages in closed chats should be marked as read
  force_read: bool,
  
}

impl RObject for ViewMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "viewMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ViewMessages {}

impl ViewMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDViewMessagesBuilder {
    let mut inner = ViewMessages::default();
    inner.td_name = "viewMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDViewMessagesBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_ids(&self) -> &Vec<i64> { &self.message_ids }

  pub fn force_read(&self) -> bool { self.force_read }

}

#[doc(hidden)]
pub struct RTDViewMessagesBuilder {
  inner: ViewMessages
}

impl RTDViewMessagesBuilder {
  pub fn build(&self) -> ViewMessages { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.message_ids = message_ids;
    self
  }

   
  pub fn force_read(&mut self, force_read: bool) -> &mut Self {
    self.inner.force_read = force_read;
    self
  }

}

impl AsRef<ViewMessages> for ViewMessages {
  fn as_ref(&self) -> &ViewMessages { self }
}

impl AsRef<ViewMessages> for RTDViewMessagesBuilder {
  fn as_ref(&self) -> &ViewMessages { &self.inner }
}







/// Informs the server that some trending sticker sets have been viewed by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewTrendingStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifiers of viewed trending sticker sets
  sticker_set_ids: Vec<isize>,
  
}

impl RObject for ViewTrendingStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "viewTrendingStickerSets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for ViewTrendingStickerSets {}

impl ViewTrendingStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDViewTrendingStickerSetsBuilder {
    let mut inner = ViewTrendingStickerSets::default();
    inner.td_name = "viewTrendingStickerSets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDViewTrendingStickerSetsBuilder { inner }
  }

  pub fn sticker_set_ids(&self) -> &Vec<isize> { &self.sticker_set_ids }

}

#[doc(hidden)]
pub struct RTDViewTrendingStickerSetsBuilder {
  inner: ViewTrendingStickerSets
}

impl RTDViewTrendingStickerSetsBuilder {
  pub fn build(&self) -> ViewTrendingStickerSets { self.inner.clone() }

   
  pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<isize>) -> &mut Self {
    self.inner.sticker_set_ids = sticker_set_ids;
    self
  }

}

impl AsRef<ViewTrendingStickerSets> for ViewTrendingStickerSets {
  fn as_ref(&self) -> &ViewTrendingStickerSets { self }
}

impl AsRef<ViewTrendingStickerSets> for RTDViewTrendingStickerSetsBuilder {
  fn as_ref(&self) -> &ViewTrendingStickerSets { &self.inner }
}







/// Writes a part of a generated file. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WriteGeneratedFilePart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the generation process
  generation_id: isize,
  /// The offset from which to write the data to the file
  offset: i64,
  /// The data to write
  data: String,
  
}

impl RObject for WriteGeneratedFilePart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "writeGeneratedFilePart" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}




impl RFunction for WriteGeneratedFilePart {}

impl WriteGeneratedFilePart {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDWriteGeneratedFilePartBuilder {
    let mut inner = WriteGeneratedFilePart::default();
    inner.td_name = "writeGeneratedFilePart".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDWriteGeneratedFilePartBuilder { inner }
  }

  pub fn generation_id(&self) -> isize { self.generation_id }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDWriteGeneratedFilePartBuilder {
  inner: WriteGeneratedFilePart
}

impl RTDWriteGeneratedFilePartBuilder {
  pub fn build(&self) -> WriteGeneratedFilePart { self.inner.clone() }

   
  pub fn generation_id(&mut self, generation_id: isize) -> &mut Self {
    self.inner.generation_id = generation_id;
    self
  }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<WriteGeneratedFilePart> for WriteGeneratedFilePart {
  fn as_ref(&self) -> &WriteGeneratedFilePart { self }
}

impl AsRef<WriteGeneratedFilePart> for RTDWriteGeneratedFilePartBuilder {
  fn as_ref(&self) -> &WriteGeneratedFilePart { &self.inner }
}



