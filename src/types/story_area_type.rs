use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of a clickable rectangle area on a story media
pub trait TDStoryAreaType: Debug + RObject {}

/// Describes type of a clickable rectangle area on a story media
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum StoryAreaType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// An area pointing to a location
    #[serde(rename = "storyAreaTypeLocation")]
    Location(StoryAreaTypeLocation),
    /// An area pointing to a suggested reaction. App needs to show a clickable reaction on the area and call setStoryReaction when the are is clicked
    #[serde(rename = "storyAreaTypeSuggestedReaction")]
    SuggestedReaction(StoryAreaTypeSuggestedReaction),
    /// An area pointing to a venue
    #[serde(rename = "storyAreaTypeVenue")]
    Venue(StoryAreaTypeVenue),
}

impl RObject for StoryAreaType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StoryAreaType::Location(t) => t.extra(),
            StoryAreaType::SuggestedReaction(t) => t.extra(),
            StoryAreaType::Venue(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StoryAreaType::Location(t) => t.client_id(),
            StoryAreaType::SuggestedReaction(t) => t.client_id(),
            StoryAreaType::Venue(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StoryAreaType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StoryAreaType::_Default)
    }
}

impl AsRef<StoryAreaType> for StoryAreaType {
    fn as_ref(&self) -> &StoryAreaType {
        self
    }
}

/// An area pointing to a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The location
    location: Location,
}

impl RObject for StoryAreaTypeLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryAreaType for StoryAreaTypeLocation {}

impl StoryAreaTypeLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryAreaTypeLocationBuilder {
        let mut inner = StoryAreaTypeLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryAreaTypeLocationBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

#[doc(hidden)]
pub struct StoryAreaTypeLocationBuilder {
    inner: StoryAreaTypeLocation,
}

#[deprecated]
pub type RTDStoryAreaTypeLocationBuilder = StoryAreaTypeLocationBuilder;

impl StoryAreaTypeLocationBuilder {
    pub fn build(&self) -> StoryAreaTypeLocation {
        self.inner.clone()
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }
}

impl AsRef<StoryAreaTypeLocation> for StoryAreaTypeLocation {
    fn as_ref(&self) -> &StoryAreaTypeLocation {
        self
    }
}

impl AsRef<StoryAreaTypeLocation> for StoryAreaTypeLocationBuilder {
    fn as_ref(&self) -> &StoryAreaTypeLocation {
        &self.inner
    }
}

/// An area pointing to a suggested reaction. App needs to show a clickable reaction on the area and call setStoryReaction when the are is clicked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeSuggestedReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the reaction

    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    reaction_type: ReactionType,
    /// Number of times the reaction was added

    #[serde(default)]
    total_count: i32,
    /// True, if reaction has a dark background

    #[serde(default)]
    is_dark: bool,
    /// True, if reaction corner is flipped

    #[serde(default)]
    is_flipped: bool,
}

impl RObject for StoryAreaTypeSuggestedReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryAreaType for StoryAreaTypeSuggestedReaction {}

impl StoryAreaTypeSuggestedReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryAreaTypeSuggestedReactionBuilder {
        let mut inner = StoryAreaTypeSuggestedReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryAreaTypeSuggestedReactionBuilder { inner }
    }

    pub fn reaction_type(&self) -> &ReactionType {
        &self.reaction_type
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    pub fn is_flipped(&self) -> bool {
        self.is_flipped
    }
}

#[doc(hidden)]
pub struct StoryAreaTypeSuggestedReactionBuilder {
    inner: StoryAreaTypeSuggestedReaction,
}

#[deprecated]
pub type RTDStoryAreaTypeSuggestedReactionBuilder = StoryAreaTypeSuggestedReactionBuilder;

impl StoryAreaTypeSuggestedReactionBuilder {
    pub fn build(&self) -> StoryAreaTypeSuggestedReaction {
        self.inner.clone()
    }

    pub fn reaction_type<T: AsRef<ReactionType>>(&mut self, reaction_type: T) -> &mut Self {
        self.inner.reaction_type = reaction_type.as_ref().clone();
        self
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
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

impl AsRef<StoryAreaTypeSuggestedReaction> for StoryAreaTypeSuggestedReaction {
    fn as_ref(&self) -> &StoryAreaTypeSuggestedReaction {
        self
    }
}

impl AsRef<StoryAreaTypeSuggestedReaction> for StoryAreaTypeSuggestedReactionBuilder {
    fn as_ref(&self) -> &StoryAreaTypeSuggestedReaction {
        &self.inner
    }
}

/// An area pointing to a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaTypeVenue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Information about the venue
    venue: Venue,
}

impl RObject for StoryAreaTypeVenue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryAreaType for StoryAreaTypeVenue {}

impl StoryAreaTypeVenue {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryAreaTypeVenueBuilder {
        let mut inner = StoryAreaTypeVenue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryAreaTypeVenueBuilder { inner }
    }

    pub fn venue(&self) -> &Venue {
        &self.venue
    }
}

#[doc(hidden)]
pub struct StoryAreaTypeVenueBuilder {
    inner: StoryAreaTypeVenue,
}

#[deprecated]
pub type RTDStoryAreaTypeVenueBuilder = StoryAreaTypeVenueBuilder;

impl StoryAreaTypeVenueBuilder {
    pub fn build(&self) -> StoryAreaTypeVenue {
        self.inner.clone()
    }

    pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
        self.inner.venue = venue.as_ref().clone();
        self
    }
}

impl AsRef<StoryAreaTypeVenue> for StoryAreaTypeVenue {
    fn as_ref(&self) -> &StoryAreaTypeVenue {
        self
    }
}

impl AsRef<StoryAreaTypeVenue> for StoryAreaTypeVenueBuilder {
    fn as_ref(&self) -> &StoryAreaTypeVenue {
        &self.inner
    }
}
