use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits the content of a live location in an inline message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageLiveLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Inline message identifier

    #[serde(default)]
    inline_message_id: String,
    /// The new message reply markup; pass null if none

    #[serde(skip_serializing_if = "ReplyMarkup::_is_default")]
    reply_markup: ReplyMarkup,
    /// New location content of the message; pass null to stop sharing the live location
    location: Location,
    /// The new direction in which the location moves, in degrees; 1-360. Pass 0 if unknown

    #[serde(default)]
    heading: i32,
    /// The new maximum distance for proximity alerts, in meters (0-100000). Pass 0 if the notification is disabled

    #[serde(default)]
    proximity_alert_radius: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditInlineMessageLiveLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditInlineMessageLiveLocation {}

impl EditInlineMessageLiveLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditInlineMessageLiveLocationBuilder {
        let mut inner = EditInlineMessageLiveLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editInlineMessageLiveLocation".to_string();

        EditInlineMessageLiveLocationBuilder { inner }
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn location(&self) -> &Location {
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
pub struct EditInlineMessageLiveLocationBuilder {
    inner: EditInlineMessageLiveLocation,
}

#[deprecated]
pub type RTDEditInlineMessageLiveLocationBuilder = EditInlineMessageLiveLocationBuilder;

impl EditInlineMessageLiveLocationBuilder {
    pub fn build(&self) -> EditInlineMessageLiveLocation {
        self.inner.clone()
    }

    pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
        self.inner.inline_message_id = inline_message_id.as_ref().to_string();
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
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

impl AsRef<EditInlineMessageLiveLocation> for EditInlineMessageLiveLocation {
    fn as_ref(&self) -> &EditInlineMessageLiveLocation {
        self
    }
}

impl AsRef<EditInlineMessageLiveLocation> for EditInlineMessageLiveLocationBuilder {
    fn as_ref(&self) -> &EditInlineMessageLiveLocation {
        &self.inner
    }
}
