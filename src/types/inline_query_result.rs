use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a single result of an inline query
pub trait TDInlineQueryResult: Debug + RObject {}

/// Represents a single result of an inline query
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResult {
    #[doc(hidden)]
    _Default,
    /// Represents an animation file
    #[serde(rename(
        serialize = "inlineQueryResultAnimation",
        deserialize = "inlineQueryResultAnimation"
    ))]
    Animation(InlineQueryResultAnimation),
    /// Represents a link to an article or web page
    #[serde(rename(
        serialize = "inlineQueryResultArticle",
        deserialize = "inlineQueryResultArticle"
    ))]
    Article(InlineQueryResultArticle),
    /// Represents an audio file
    #[serde(rename(
        serialize = "inlineQueryResultAudio",
        deserialize = "inlineQueryResultAudio"
    ))]
    Audio(InlineQueryResultAudio),
    /// Represents a user contact
    #[serde(rename(
        serialize = "inlineQueryResultContact",
        deserialize = "inlineQueryResultContact"
    ))]
    Contact(InlineQueryResultContact),
    /// Represents a document
    #[serde(rename(
        serialize = "inlineQueryResultDocument",
        deserialize = "inlineQueryResultDocument"
    ))]
    Document(InlineQueryResultDocument),
    /// Represents information about a game
    #[serde(rename(
        serialize = "inlineQueryResultGame",
        deserialize = "inlineQueryResultGame"
    ))]
    Game(InlineQueryResultGame),
    /// Represents a point on the map
    #[serde(rename(
        serialize = "inlineQueryResultLocation",
        deserialize = "inlineQueryResultLocation"
    ))]
    Location(InlineQueryResultLocation),
    /// Represents a photo
    #[serde(rename(
        serialize = "inlineQueryResultPhoto",
        deserialize = "inlineQueryResultPhoto"
    ))]
    Photo(InlineQueryResultPhoto),
    /// Represents a sticker
    #[serde(rename(
        serialize = "inlineQueryResultSticker",
        deserialize = "inlineQueryResultSticker"
    ))]
    Sticker(InlineQueryResultSticker),
    /// Represents information about a venue
    #[serde(rename(
        serialize = "inlineQueryResultVenue",
        deserialize = "inlineQueryResultVenue"
    ))]
    Venue(InlineQueryResultVenue),
    /// Represents a video
    #[serde(rename(
        serialize = "inlineQueryResultVideo",
        deserialize = "inlineQueryResultVideo"
    ))]
    Video(InlineQueryResultVideo),
    /// Represents a voice note
    #[serde(rename(
        serialize = "inlineQueryResultVoiceNote",
        deserialize = "inlineQueryResultVoiceNote"
    ))]
    VoiceNote(InlineQueryResultVoiceNote),
}

impl Default for InlineQueryResult {
    fn default() -> Self {
        InlineQueryResult::_Default
    }
}

impl RObject for InlineQueryResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InlineQueryResult::Animation(t) => t.extra(),
            InlineQueryResult::Article(t) => t.extra(),
            InlineQueryResult::Audio(t) => t.extra(),
            InlineQueryResult::Contact(t) => t.extra(),
            InlineQueryResult::Document(t) => t.extra(),
            InlineQueryResult::Game(t) => t.extra(),
            InlineQueryResult::Location(t) => t.extra(),
            InlineQueryResult::Photo(t) => t.extra(),
            InlineQueryResult::Sticker(t) => t.extra(),
            InlineQueryResult::Venue(t) => t.extra(),
            InlineQueryResult::Video(t) => t.extra(),
            InlineQueryResult::VoiceNote(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InlineQueryResult::Animation(t) => t.client_id(),
            InlineQueryResult::Article(t) => t.client_id(),
            InlineQueryResult::Audio(t) => t.client_id(),
            InlineQueryResult::Contact(t) => t.client_id(),
            InlineQueryResult::Document(t) => t.client_id(),
            InlineQueryResult::Game(t) => t.client_id(),
            InlineQueryResult::Location(t) => t.client_id(),
            InlineQueryResult::Photo(t) => t.client_id(),
            InlineQueryResult::Sticker(t) => t.client_id(),
            InlineQueryResult::Venue(t) => t.client_id(),
            InlineQueryResult::Video(t) => t.client_id(),
            InlineQueryResult::VoiceNote(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InlineQueryResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InlineQueryResult::_Default)
    }
}

impl AsRef<InlineQueryResult> for InlineQueryResult {
    fn as_ref(&self) -> &InlineQueryResult {
        self
    }
}

/// Represents an animation file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Animation file
    animation: Animation,
    /// Animation title
    title: String,
}

impl RObject for InlineQueryResultAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultAnimation {}

impl InlineQueryResultAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultAnimationBuilder {
        let mut inner = InlineQueryResultAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultAnimationBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn animation(&self) -> &Animation {
        &self.animation
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultAnimationBuilder {
    inner: InlineQueryResultAnimation,
}

impl RTDInlineQueryResultAnimationBuilder {
    pub fn build(&self) -> InlineQueryResultAnimation {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = animation.as_ref().clone();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<InlineQueryResultAnimation> for InlineQueryResultAnimation {
    fn as_ref(&self) -> &InlineQueryResultAnimation {
        self
    }
}

impl AsRef<InlineQueryResultAnimation> for RTDInlineQueryResultAnimationBuilder {
    fn as_ref(&self) -> &InlineQueryResultAnimation {
        &self.inner
    }
}

/// Represents a link to an article or web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// URL of the result, if it exists
    url: String,
    /// True, if the URL must be not shown
    hide_url: bool,
    /// Title of the result
    title: String,
    /// Represents a link to an article or web page
    description: String,
    /// Result thumbnail in JPEG format; may be null
    thumbnail: Option<Thumbnail>,
}

impl RObject for InlineQueryResultArticle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultArticle {}

impl InlineQueryResultArticle {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultArticleBuilder {
        let mut inner = InlineQueryResultArticle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultArticleBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn hide_url(&self) -> bool {
        self.hide_url
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultArticleBuilder {
    inner: InlineQueryResultArticle,
}

impl RTDInlineQueryResultArticleBuilder {
    pub fn build(&self) -> InlineQueryResultArticle {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn hide_url(&mut self, hide_url: bool) -> &mut Self {
        self.inner.hide_url = hide_url;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }
}

impl AsRef<InlineQueryResultArticle> for InlineQueryResultArticle {
    fn as_ref(&self) -> &InlineQueryResultArticle {
        self
    }
}

impl AsRef<InlineQueryResultArticle> for RTDInlineQueryResultArticleBuilder {
    fn as_ref(&self) -> &InlineQueryResultArticle {
        &self.inner
    }
}

/// Represents an audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Audio file
    audio: Audio,
}

impl RObject for InlineQueryResultAudio {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultAudio {}

impl InlineQueryResultAudio {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultAudioBuilder {
        let mut inner = InlineQueryResultAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultAudioBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn audio(&self) -> &Audio {
        &self.audio
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultAudioBuilder {
    inner: InlineQueryResultAudio,
}

impl RTDInlineQueryResultAudioBuilder {
    pub fn build(&self) -> InlineQueryResultAudio {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
        self.inner.audio = audio.as_ref().clone();
        self
    }
}

impl AsRef<InlineQueryResultAudio> for InlineQueryResultAudio {
    fn as_ref(&self) -> &InlineQueryResultAudio {
        self
    }
}

impl AsRef<InlineQueryResultAudio> for RTDInlineQueryResultAudioBuilder {
    fn as_ref(&self) -> &InlineQueryResultAudio {
        &self.inner
    }
}

/// Represents a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// A user contact
    contact: Contact,
    /// Result thumbnail in JPEG format; may be null
    thumbnail: Option<Thumbnail>,
}

impl RObject for InlineQueryResultContact {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultContact {}

impl InlineQueryResultContact {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultContactBuilder {
        let mut inner = InlineQueryResultContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultContactBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn contact(&self) -> &Contact {
        &self.contact
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultContactBuilder {
    inner: InlineQueryResultContact,
}

impl RTDInlineQueryResultContactBuilder {
    pub fn build(&self) -> InlineQueryResultContact {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
        self.inner.contact = contact.as_ref().clone();
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }
}

impl AsRef<InlineQueryResultContact> for InlineQueryResultContact {
    fn as_ref(&self) -> &InlineQueryResultContact {
        self
    }
}

impl AsRef<InlineQueryResultContact> for RTDInlineQueryResultContactBuilder {
    fn as_ref(&self) -> &InlineQueryResultContact {
        &self.inner
    }
}

/// Represents a document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Document
    document: Document,
    /// Document title
    title: String,
    /// Represents a document
    description: String,
}

impl RObject for InlineQueryResultDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultDocument {}

impl InlineQueryResultDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultDocumentBuilder {
        let mut inner = InlineQueryResultDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultDocumentBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultDocumentBuilder {
    inner: InlineQueryResultDocument,
}

impl RTDInlineQueryResultDocumentBuilder {
    pub fn build(&self) -> InlineQueryResultDocument {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
        self.inner.document = document.as_ref().clone();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<InlineQueryResultDocument> for InlineQueryResultDocument {
    fn as_ref(&self) -> &InlineQueryResultDocument {
        self
    }
}

impl AsRef<InlineQueryResultDocument> for RTDInlineQueryResultDocumentBuilder {
    fn as_ref(&self) -> &InlineQueryResultDocument {
        &self.inner
    }
}

/// Represents information about a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Game result
    game: Game,
}

impl RObject for InlineQueryResultGame {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultGame {}

impl InlineQueryResultGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultGameBuilder {
        let mut inner = InlineQueryResultGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultGameBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn game(&self) -> &Game {
        &self.game
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultGameBuilder {
    inner: InlineQueryResultGame,
}

impl RTDInlineQueryResultGameBuilder {
    pub fn build(&self) -> InlineQueryResultGame {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn game<T: AsRef<Game>>(&mut self, game: T) -> &mut Self {
        self.inner.game = game.as_ref().clone();
        self
    }
}

impl AsRef<InlineQueryResultGame> for InlineQueryResultGame {
    fn as_ref(&self) -> &InlineQueryResultGame {
        self
    }
}

impl AsRef<InlineQueryResultGame> for RTDInlineQueryResultGameBuilder {
    fn as_ref(&self) -> &InlineQueryResultGame {
        &self.inner
    }
}

/// Represents a point on the map
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Location result
    location: Location,
    /// Title of the result
    title: String,
    /// Result thumbnail in JPEG format; may be null
    thumbnail: Option<Thumbnail>,
}

impl RObject for InlineQueryResultLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultLocation {}

impl InlineQueryResultLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultLocationBuilder {
        let mut inner = InlineQueryResultLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultLocationBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultLocationBuilder {
    inner: InlineQueryResultLocation,
}

impl RTDInlineQueryResultLocationBuilder {
    pub fn build(&self) -> InlineQueryResultLocation {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }
}

impl AsRef<InlineQueryResultLocation> for InlineQueryResultLocation {
    fn as_ref(&self) -> &InlineQueryResultLocation {
        self
    }
}

impl AsRef<InlineQueryResultLocation> for RTDInlineQueryResultLocationBuilder {
    fn as_ref(&self) -> &InlineQueryResultLocation {
        &self.inner
    }
}

/// Represents a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Photo
    photo: Photo,
    /// Title of the result, if known
    title: String,
    /// Represents a photo
    description: String,
}

impl RObject for InlineQueryResultPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultPhoto {}

impl InlineQueryResultPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultPhotoBuilder {
        let mut inner = InlineQueryResultPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultPhotoBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn photo(&self) -> &Photo {
        &self.photo
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultPhotoBuilder {
    inner: InlineQueryResultPhoto,
}

impl RTDInlineQueryResultPhotoBuilder {
    pub fn build(&self) -> InlineQueryResultPhoto {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<InlineQueryResultPhoto> for InlineQueryResultPhoto {
    fn as_ref(&self) -> &InlineQueryResultPhoto {
        self
    }
}

impl AsRef<InlineQueryResultPhoto> for RTDInlineQueryResultPhotoBuilder {
    fn as_ref(&self) -> &InlineQueryResultPhoto {
        &self.inner
    }
}

/// Represents a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Sticker
    sticker: Sticker,
}

impl RObject for InlineQueryResultSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultSticker {}

impl InlineQueryResultSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultStickerBuilder {
        let mut inner = InlineQueryResultSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultStickerBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn sticker(&self) -> &Sticker {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultStickerBuilder {
    inner: InlineQueryResultSticker,
}

impl RTDInlineQueryResultStickerBuilder {
    pub fn build(&self) -> InlineQueryResultSticker {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<InlineQueryResultSticker> for InlineQueryResultSticker {
    fn as_ref(&self) -> &InlineQueryResultSticker {
        self
    }
}

impl AsRef<InlineQueryResultSticker> for RTDInlineQueryResultStickerBuilder {
    fn as_ref(&self) -> &InlineQueryResultSticker {
        &self.inner
    }
}

/// Represents information about a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Venue result
    venue: Venue,
    /// Result thumbnail in JPEG format; may be null
    thumbnail: Option<Thumbnail>,
}

impl RObject for InlineQueryResultVenue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultVenue {}

impl InlineQueryResultVenue {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultVenueBuilder {
        let mut inner = InlineQueryResultVenue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultVenueBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn venue(&self) -> &Venue {
        &self.venue
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultVenueBuilder {
    inner: InlineQueryResultVenue,
}

impl RTDInlineQueryResultVenueBuilder {
    pub fn build(&self) -> InlineQueryResultVenue {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
        self.inner.venue = venue.as_ref().clone();
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }
}

impl AsRef<InlineQueryResultVenue> for InlineQueryResultVenue {
    fn as_ref(&self) -> &InlineQueryResultVenue {
        self
    }
}

impl AsRef<InlineQueryResultVenue> for RTDInlineQueryResultVenueBuilder {
    fn as_ref(&self) -> &InlineQueryResultVenue {
        &self.inner
    }
}

/// Represents a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Video
    video: Video,
    /// Title of the video
    title: String,
    /// Represents a video
    description: String,
}

impl RObject for InlineQueryResultVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultVideo {}

impl InlineQueryResultVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultVideoBuilder {
        let mut inner = InlineQueryResultVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultVideoBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn video(&self) -> &Video {
        &self.video
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultVideoBuilder {
    inner: InlineQueryResultVideo,
}

impl RTDInlineQueryResultVideoBuilder {
    pub fn build(&self) -> InlineQueryResultVideo {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
        self.inner.video = video.as_ref().clone();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<InlineQueryResultVideo> for InlineQueryResultVideo {
    fn as_ref(&self) -> &InlineQueryResultVideo {
        self
    }
}

impl AsRef<InlineQueryResultVideo> for RTDInlineQueryResultVideoBuilder {
    fn as_ref(&self) -> &InlineQueryResultVideo {
        &self.inner
    }
}

/// Represents a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the query result
    id: String,
    /// Voice note
    voice_note: VoiceNote,
    /// Title of the voice note
    title: String,
}

impl RObject for InlineQueryResultVoiceNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineQueryResult for InlineQueryResultVoiceNote {}

impl InlineQueryResultVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineQueryResultVoiceNoteBuilder {
        let mut inner = InlineQueryResultVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineQueryResultVoiceNoteBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn voice_note(&self) -> &VoiceNote {
        &self.voice_note
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultVoiceNoteBuilder {
    inner: InlineQueryResultVoiceNote,
}

impl RTDInlineQueryResultVoiceNoteBuilder {
    pub fn build(&self) -> InlineQueryResultVoiceNote {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn voice_note<T: AsRef<VoiceNote>>(&mut self, voice_note: T) -> &mut Self {
        self.inner.voice_note = voice_note.as_ref().clone();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<InlineQueryResultVoiceNote> for InlineQueryResultVoiceNote {
    fn as_ref(&self) -> &InlineQueryResultVoiceNote {
        self
    }
}

impl AsRef<InlineQueryResultVoiceNote> for RTDInlineQueryResultVoiceNoteBuilder {
    fn as_ref(&self) -> &InlineQueryResultVoiceNote {
        &self.inner
    }
}
