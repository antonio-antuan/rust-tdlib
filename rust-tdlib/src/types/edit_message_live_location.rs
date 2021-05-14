use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageLiveLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat the message belongs to
    chat_id: i64,
    /// Identifier of the message
    message_id: i64,
    /// The new message reply markup; for bots only

    #[serde(skip_serializing_if = "ReplyMarkup::_is_default")]
    reply_markup: ReplyMarkup,
    /// New location content of the message; may be null. Pass null to stop sharing the live location
    location: Option<Location>,
    /// The new direction in which the location moves, in degrees; 1-360. Pass 0 if unknown
    heading: i32,
    /// The new maximum distance for proximity alerts, in meters (0-100000). Pass 0 if the notification is disabled
    proximity_alert_radius: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditMessageLiveLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditMessageLiveLocation {}

impl EditMessageLiveLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDEditMessageLiveLocationBuilder {
        let mut inner = EditMessageLiveLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editMessageLiveLocation".to_string();

        RTDEditMessageLiveLocationBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn location(&self) -> &Option<Location> {
        &self.location
    }

    pub fn heading(&self) -> i32 {
        self.heading
    }

    pub fn proximity_alert_radius(&self) -> i32 {
        self.proximity_alert_radius
    }
}

#[doc(hidden)]
pub struct RTDEditMessageLiveLocationBuilder {
    inner: EditMessageLiveLocation,
}

impl RTDEditMessageLiveLocationBuilder {
    pub fn build(&self) -> EditMessageLiveLocation {
        self.inner.clone()
    }

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

    pub fn heading(&mut self, heading: i32) -> &mut Self {
        self.inner.heading = heading;
        self
    }

    pub fn proximity_alert_radius(&mut self, proximity_alert_radius: i32) -> &mut Self {
        self.inner.proximity_alert_radius = proximity_alert_radius;
        self
    }
}

impl AsRef<EditMessageLiveLocation> for EditMessageLiveLocation {
    fn as_ref(&self) -> &EditMessageLiveLocation {
        self
    }
}

impl AsRef<EditMessageLiveLocation> for RTDEditMessageLiveLocationBuilder {
    fn as_ref(&self) -> &EditMessageLiveLocation {
        &self.inner
    }
}
