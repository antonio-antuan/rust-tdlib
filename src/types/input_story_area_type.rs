use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of a clickable rectangle area on a story media to be added
pub trait TDInputStoryAreaType: Debug + RObject {}

/// Describes type of a clickable rectangle area on a story media to be added
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum InputStoryAreaType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// An area pointing to a venue found by the bot getOption("venue_search_bot_username")
    #[serde(rename = "inputStoryAreaTypeFoundVenue")]
    FoundVenue(InputStoryAreaTypeFoundVenue),
    /// An area pointing to a location
    #[serde(rename = "inputStoryAreaTypeLocation")]
    Location(InputStoryAreaTypeLocation),
    /// An area pointing to a venue already added to the story
    #[serde(rename = "inputStoryAreaTypePreviousVenue")]
    PreviousVenue(InputStoryAreaTypePreviousVenue),
    /// An area pointing to a suggested reaction
    #[serde(rename = "inputStoryAreaTypeSuggestedReaction")]
    SuggestedReaction(InputStoryAreaTypeSuggestedReaction),
}

impl RObject for InputStoryAreaType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputStoryAreaType::FoundVenue(t) => t.extra(),
            InputStoryAreaType::Location(t) => t.extra(),
            InputStoryAreaType::PreviousVenue(t) => t.extra(),
            InputStoryAreaType::SuggestedReaction(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputStoryAreaType::FoundVenue(t) => t.client_id(),
            InputStoryAreaType::Location(t) => t.client_id(),
            InputStoryAreaType::PreviousVenue(t) => t.client_id(),
            InputStoryAreaType::SuggestedReaction(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputStoryAreaType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputStoryAreaType::_Default)
    }
}

impl AsRef<InputStoryAreaType> for InputStoryAreaType {
    fn as_ref(&self) -> &InputStoryAreaType {
        self
    }
}

/// An area pointing to a venue found by the bot getOption("venue_search_bot_username")
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeFoundVenue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the inline query, used to found the venue

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    query_id: i64,
    /// Identifier of the inline query result

    #[serde(default)]
    result_id: String,
}

impl RObject for InputStoryAreaTypeFoundVenue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputStoryAreaType for InputStoryAreaTypeFoundVenue {}

impl InputStoryAreaTypeFoundVenue {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStoryAreaTypeFoundVenueBuilder {
        let mut inner = InputStoryAreaTypeFoundVenue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStoryAreaTypeFoundVenueBuilder { inner }
    }

    pub fn query_id(&self) -> i64 {
        self.query_id
    }

    pub fn result_id(&self) -> &String {
        &self.result_id
    }
}

#[doc(hidden)]
pub struct InputStoryAreaTypeFoundVenueBuilder {
    inner: InputStoryAreaTypeFoundVenue,
}

#[deprecated]
pub type RTDInputStoryAreaTypeFoundVenueBuilder = InputStoryAreaTypeFoundVenueBuilder;

impl InputStoryAreaTypeFoundVenueBuilder {
    pub fn build(&self) -> InputStoryAreaTypeFoundVenue {
        self.inner.clone()
    }

    pub fn query_id(&mut self, query_id: i64) -> &mut Self {
        self.inner.query_id = query_id;
        self
    }

    pub fn result_id<T: AsRef<str>>(&mut self, result_id: T) -> &mut Self {
        self.inner.result_id = result_id.as_ref().to_string();
        self
    }
}

impl AsRef<InputStoryAreaTypeFoundVenue> for InputStoryAreaTypeFoundVenue {
    fn as_ref(&self) -> &InputStoryAreaTypeFoundVenue {
        self
    }
}

impl AsRef<InputStoryAreaTypeFoundVenue> for InputStoryAreaTypeFoundVenueBuilder {
    fn as_ref(&self) -> &InputStoryAreaTypeFoundVenue {
        &self.inner
    }
}

/// An area pointing to a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The location
    location: Location,
}

impl RObject for InputStoryAreaTypeLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputStoryAreaType for InputStoryAreaTypeLocation {}

impl InputStoryAreaTypeLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStoryAreaTypeLocationBuilder {
        let mut inner = InputStoryAreaTypeLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStoryAreaTypeLocationBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

#[doc(hidden)]
pub struct InputStoryAreaTypeLocationBuilder {
    inner: InputStoryAreaTypeLocation,
}

#[deprecated]
pub type RTDInputStoryAreaTypeLocationBuilder = InputStoryAreaTypeLocationBuilder;

impl InputStoryAreaTypeLocationBuilder {
    pub fn build(&self) -> InputStoryAreaTypeLocation {
        self.inner.clone()
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }
}

impl AsRef<InputStoryAreaTypeLocation> for InputStoryAreaTypeLocation {
    fn as_ref(&self) -> &InputStoryAreaTypeLocation {
        self
    }
}

impl AsRef<InputStoryAreaTypeLocation> for InputStoryAreaTypeLocationBuilder {
    fn as_ref(&self) -> &InputStoryAreaTypeLocation {
        &self.inner
    }
}

/// An area pointing to a venue already added to the story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypePreviousVenue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Provider of the venue

    #[serde(default)]
    venue_provider: String,
    /// Identifier of the venue in the provider database

    #[serde(default)]
    venue_id: String,
}

impl RObject for InputStoryAreaTypePreviousVenue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputStoryAreaType for InputStoryAreaTypePreviousVenue {}

impl InputStoryAreaTypePreviousVenue {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStoryAreaTypePreviousVenueBuilder {
        let mut inner = InputStoryAreaTypePreviousVenue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStoryAreaTypePreviousVenueBuilder { inner }
    }

    pub fn venue_provider(&self) -> &String {
        &self.venue_provider
    }

    pub fn venue_id(&self) -> &String {
        &self.venue_id
    }
}

#[doc(hidden)]
pub struct InputStoryAreaTypePreviousVenueBuilder {
    inner: InputStoryAreaTypePreviousVenue,
}

#[deprecated]
pub type RTDInputStoryAreaTypePreviousVenueBuilder = InputStoryAreaTypePreviousVenueBuilder;

impl InputStoryAreaTypePreviousVenueBuilder {
    pub fn build(&self) -> InputStoryAreaTypePreviousVenue {
        self.inner.clone()
    }

    pub fn venue_provider<T: AsRef<str>>(&mut self, venue_provider: T) -> &mut Self {
        self.inner.venue_provider = venue_provider.as_ref().to_string();
        self
    }

    pub fn venue_id<T: AsRef<str>>(&mut self, venue_id: T) -> &mut Self {
        self.inner.venue_id = venue_id.as_ref().to_string();
        self
    }
}

impl AsRef<InputStoryAreaTypePreviousVenue> for InputStoryAreaTypePreviousVenue {
    fn as_ref(&self) -> &InputStoryAreaTypePreviousVenue {
        self
    }
}

impl AsRef<InputStoryAreaTypePreviousVenue> for InputStoryAreaTypePreviousVenueBuilder {
    fn as_ref(&self) -> &InputStoryAreaTypePreviousVenue {
        &self.inner
    }
}

/// An area pointing to a suggested reaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreaTypeSuggestedReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the reaction

    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    reaction_type: ReactionType,
    /// True, if reaction has a dark background

    #[serde(default)]
    is_dark: bool,
    /// True, if reaction corner is flipped

    #[serde(default)]
    is_flipped: bool,
}

impl RObject for InputStoryAreaTypeSuggestedReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputStoryAreaType for InputStoryAreaTypeSuggestedReaction {}

impl InputStoryAreaTypeSuggestedReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStoryAreaTypeSuggestedReactionBuilder {
        let mut inner = InputStoryAreaTypeSuggestedReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStoryAreaTypeSuggestedReactionBuilder { inner }
    }

    pub fn reaction_type(&self) -> &ReactionType {
        &self.reaction_type
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    pub fn is_flipped(&self) -> bool {
        self.is_flipped
    }
}

#[doc(hidden)]
pub struct InputStoryAreaTypeSuggestedReactionBuilder {
    inner: InputStoryAreaTypeSuggestedReaction,
}

#[deprecated]
pub type RTDInputStoryAreaTypeSuggestedReactionBuilder = InputStoryAreaTypeSuggestedReactionBuilder;

impl InputStoryAreaTypeSuggestedReactionBuilder {
    pub fn build(&self) -> InputStoryAreaTypeSuggestedReaction {
        self.inner.clone()
    }

    pub fn reaction_type<T: AsRef<ReactionType>>(&mut self, reaction_type: T) -> &mut Self {
        self.inner.reaction_type = reaction_type.as_ref().clone();
        self
    }

    pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
        self.inner.is_dark = is_dark;
        self
    }

    pub fn is_flipped(&mut self, is_flipped: bool) -> &mut Self {
        self.inner.is_flipped = is_flipped;
        self
    }
}

impl AsRef<InputStoryAreaTypeSuggestedReaction> for InputStoryAreaTypeSuggestedReaction {
    fn as_ref(&self) -> &InputStoryAreaTypeSuggestedReaction {
        self
    }
}

impl AsRef<InputStoryAreaTypeSuggestedReaction> for InputStoryAreaTypeSuggestedReactionBuilder {
    fn as_ref(&self) -> &InputStoryAreaTypeSuggestedReaction {
        &self.inner
    }
}
