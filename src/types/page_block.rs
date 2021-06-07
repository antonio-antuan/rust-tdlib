use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a block of an instant view web page
pub trait TDPageBlock: Debug + RObject {}

/// Describes a block of an instant view web page
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlock {
    #[doc(hidden)]
    _Default,
    /// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor
    #[serde(rename(serialize = "pageBlockAnchor", deserialize = "pageBlockAnchor"))]
    Anchor(PageBlockAnchor),
    /// An animation
    #[serde(rename(serialize = "pageBlockAnimation", deserialize = "pageBlockAnimation"))]
    Animation(PageBlockAnimation),
    /// An audio file
    #[serde(rename(serialize = "pageBlockAudio", deserialize = "pageBlockAudio"))]
    Audio(PageBlockAudio),
    /// The author and publishing date of a page
    #[serde(rename(serialize = "pageBlockAuthorDate", deserialize = "pageBlockAuthorDate"))]
    AuthorDate(PageBlockAuthorDate),
    /// A block quote
    #[serde(rename(serialize = "pageBlockBlockQuote", deserialize = "pageBlockBlockQuote"))]
    BlockQuote(PageBlockBlockQuote),
    /// A link to a chat
    #[serde(rename(serialize = "pageBlockChatLink", deserialize = "pageBlockChatLink"))]
    ChatLink(PageBlockChatLink),
    /// A collage
    #[serde(rename(serialize = "pageBlockCollage", deserialize = "pageBlockCollage"))]
    Collage(PageBlockCollage),
    /// A page cover
    #[serde(rename(serialize = "pageBlockCover", deserialize = "pageBlockCover"))]
    Cover(PageBlockCover),
    /// A collapsible block
    #[serde(rename(serialize = "pageBlockDetails", deserialize = "pageBlockDetails"))]
    Details(PageBlockDetails),
    /// An empty block separating a page
    #[serde(rename(serialize = "pageBlockDivider", deserialize = "pageBlockDivider"))]
    Divider(PageBlockDivider),
    /// An embedded web page
    #[serde(rename(serialize = "pageBlockEmbedded", deserialize = "pageBlockEmbedded"))]
    Embedded(PageBlockEmbedded),
    /// An embedded post
    #[serde(rename(
        serialize = "pageBlockEmbeddedPost",
        deserialize = "pageBlockEmbeddedPost"
    ))]
    EmbeddedPost(PageBlockEmbeddedPost),
    /// The footer of a page
    #[serde(rename(serialize = "pageBlockFooter", deserialize = "pageBlockFooter"))]
    Footer(PageBlockFooter),
    /// A header
    #[serde(rename(serialize = "pageBlockHeader", deserialize = "pageBlockHeader"))]
    Header(PageBlockHeader),
    /// A kicker
    #[serde(rename(serialize = "pageBlockKicker", deserialize = "pageBlockKicker"))]
    Kicker(PageBlockKicker),
    /// A list of data blocks
    #[serde(rename(serialize = "pageBlockList", deserialize = "pageBlockList"))]
    List(PageBlockList),
    /// A map
    #[serde(rename(serialize = "pageBlockMap", deserialize = "pageBlockMap"))]
    Map(PageBlockMap),
    /// A text paragraph
    #[serde(rename(serialize = "pageBlockParagraph", deserialize = "pageBlockParagraph"))]
    Paragraph(PageBlockParagraph),
    /// A photo
    #[serde(rename(serialize = "pageBlockPhoto", deserialize = "pageBlockPhoto"))]
    Photo(PageBlockPhoto),
    /// A preformatted text paragraph
    #[serde(rename(
        serialize = "pageBlockPreformatted",
        deserialize = "pageBlockPreformatted"
    ))]
    Preformatted(PageBlockPreformatted),
    /// A pull quote
    #[serde(rename(serialize = "pageBlockPullQuote", deserialize = "pageBlockPullQuote"))]
    PullQuote(PageBlockPullQuote),
    /// Related articles
    #[serde(rename(
        serialize = "pageBlockRelatedArticles",
        deserialize = "pageBlockRelatedArticles"
    ))]
    RelatedArticles(PageBlockRelatedArticles),
    /// A slideshow
    #[serde(rename(serialize = "pageBlockSlideshow", deserialize = "pageBlockSlideshow"))]
    Slideshow(PageBlockSlideshow),
    /// A subheader
    #[serde(rename(serialize = "pageBlockSubheader", deserialize = "pageBlockSubheader"))]
    Subheader(PageBlockSubheader),
    /// The subtitle of a page
    #[serde(rename(serialize = "pageBlockSubtitle", deserialize = "pageBlockSubtitle"))]
    Subtitle(PageBlockSubtitle),
    /// A table
    #[serde(rename(serialize = "pageBlockTable", deserialize = "pageBlockTable"))]
    Table(PageBlockTable),
    /// The title of a page
    #[serde(rename(serialize = "pageBlockTitle", deserialize = "pageBlockTitle"))]
    Title(PageBlockTitle),
    /// A video
    #[serde(rename(serialize = "pageBlockVideo", deserialize = "pageBlockVideo"))]
    Video(PageBlockVideo),
    /// A voice note
    #[serde(rename(serialize = "pageBlockVoiceNote", deserialize = "pageBlockVoiceNote"))]
    VoiceNote(PageBlockVoiceNote),
}

impl Default for PageBlock {
    fn default() -> Self {
        PageBlock::_Default
    }
}

impl RObject for PageBlock {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PageBlock::Anchor(t) => t.extra(),
            PageBlock::Animation(t) => t.extra(),
            PageBlock::Audio(t) => t.extra(),
            PageBlock::AuthorDate(t) => t.extra(),
            PageBlock::BlockQuote(t) => t.extra(),
            PageBlock::ChatLink(t) => t.extra(),
            PageBlock::Collage(t) => t.extra(),
            PageBlock::Cover(t) => t.extra(),
            PageBlock::Details(t) => t.extra(),
            PageBlock::Divider(t) => t.extra(),
            PageBlock::Embedded(t) => t.extra(),
            PageBlock::EmbeddedPost(t) => t.extra(),
            PageBlock::Footer(t) => t.extra(),
            PageBlock::Header(t) => t.extra(),
            PageBlock::Kicker(t) => t.extra(),
            PageBlock::List(t) => t.extra(),
            PageBlock::Map(t) => t.extra(),
            PageBlock::Paragraph(t) => t.extra(),
            PageBlock::Photo(t) => t.extra(),
            PageBlock::Preformatted(t) => t.extra(),
            PageBlock::PullQuote(t) => t.extra(),
            PageBlock::RelatedArticles(t) => t.extra(),
            PageBlock::Slideshow(t) => t.extra(),
            PageBlock::Subheader(t) => t.extra(),
            PageBlock::Subtitle(t) => t.extra(),
            PageBlock::Table(t) => t.extra(),
            PageBlock::Title(t) => t.extra(),
            PageBlock::Video(t) => t.extra(),
            PageBlock::VoiceNote(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PageBlock::Anchor(t) => t.client_id(),
            PageBlock::Animation(t) => t.client_id(),
            PageBlock::Audio(t) => t.client_id(),
            PageBlock::AuthorDate(t) => t.client_id(),
            PageBlock::BlockQuote(t) => t.client_id(),
            PageBlock::ChatLink(t) => t.client_id(),
            PageBlock::Collage(t) => t.client_id(),
            PageBlock::Cover(t) => t.client_id(),
            PageBlock::Details(t) => t.client_id(),
            PageBlock::Divider(t) => t.client_id(),
            PageBlock::Embedded(t) => t.client_id(),
            PageBlock::EmbeddedPost(t) => t.client_id(),
            PageBlock::Footer(t) => t.client_id(),
            PageBlock::Header(t) => t.client_id(),
            PageBlock::Kicker(t) => t.client_id(),
            PageBlock::List(t) => t.client_id(),
            PageBlock::Map(t) => t.client_id(),
            PageBlock::Paragraph(t) => t.client_id(),
            PageBlock::Photo(t) => t.client_id(),
            PageBlock::Preformatted(t) => t.client_id(),
            PageBlock::PullQuote(t) => t.client_id(),
            PageBlock::RelatedArticles(t) => t.client_id(),
            PageBlock::Slideshow(t) => t.client_id(),
            PageBlock::Subheader(t) => t.client_id(),
            PageBlock::Subtitle(t) => t.client_id(),
            PageBlock::Table(t) => t.client_id(),
            PageBlock::Title(t) => t.client_id(),
            PageBlock::Video(t) => t.client_id(),
            PageBlock::VoiceNote(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PageBlock {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PageBlock::_Default)
    }
}

impl AsRef<PageBlock> for PageBlock {
    fn as_ref(&self) -> &PageBlock {
        self
    }
}

/// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockAnchor {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the anchor
    name: String,
}

impl RObject for PageBlockAnchor {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockAnchor {}

impl PageBlockAnchor {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockAnchorBuilder {
        let mut inner = PageBlockAnchor::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockAnchorBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct RTDPageBlockAnchorBuilder {
    inner: PageBlockAnchor,
}

impl RTDPageBlockAnchorBuilder {
    pub fn build(&self) -> PageBlockAnchor {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<PageBlockAnchor> for PageBlockAnchor {
    fn as_ref(&self) -> &PageBlockAnchor {
        self
    }
}

impl AsRef<PageBlockAnchor> for RTDPageBlockAnchorBuilder {
    fn as_ref(&self) -> &PageBlockAnchor {
        &self.inner
    }
}

/// An animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Animation file; may be null
    animation: Option<Animation>,
    /// Animation caption
    caption: PageBlockCaption,
    /// True, if the animation should be played automatically
    need_autoplay: bool,
}

impl RObject for PageBlockAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockAnimation {}

impl PageBlockAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockAnimationBuilder {
        let mut inner = PageBlockAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockAnimationBuilder { inner }
    }

    pub fn animation(&self) -> &Option<Animation> {
        &self.animation
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }

    pub fn need_autoplay(&self) -> bool {
        self.need_autoplay
    }
}

#[doc(hidden)]
pub struct RTDPageBlockAnimationBuilder {
    inner: PageBlockAnimation,
}

impl RTDPageBlockAnimationBuilder {
    pub fn build(&self) -> PageBlockAnimation {
        self.inner.clone()
    }

    pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = Some(animation.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }

    pub fn need_autoplay(&mut self, need_autoplay: bool) -> &mut Self {
        self.inner.need_autoplay = need_autoplay;
        self
    }
}

impl AsRef<PageBlockAnimation> for PageBlockAnimation {
    fn as_ref(&self) -> &PageBlockAnimation {
        self
    }
}

impl AsRef<PageBlockAnimation> for RTDPageBlockAnimationBuilder {
    fn as_ref(&self) -> &PageBlockAnimation {
        &self.inner
    }
}

/// An audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockAudio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Audio file; may be null
    audio: Option<Audio>,
    /// Audio file caption
    caption: PageBlockCaption,
}

impl RObject for PageBlockAudio {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockAudio {}

impl PageBlockAudio {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockAudioBuilder {
        let mut inner = PageBlockAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockAudioBuilder { inner }
    }

    pub fn audio(&self) -> &Option<Audio> {
        &self.audio
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDPageBlockAudioBuilder {
    inner: PageBlockAudio,
}

impl RTDPageBlockAudioBuilder {
    pub fn build(&self) -> PageBlockAudio {
        self.inner.clone()
    }

    pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
        self.inner.audio = Some(audio.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockAudio> for PageBlockAudio {
    fn as_ref(&self) -> &PageBlockAudio {
        self
    }
}

impl AsRef<PageBlockAudio> for RTDPageBlockAudioBuilder {
    fn as_ref(&self) -> &PageBlockAudio {
        &self.inner
    }
}

/// The author and publishing date of a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockAuthorDate {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Author

    #[serde(skip_serializing_if = "RichText::_is_default")]
    author: RichText,
    /// Point in time (Unix timestamp) when the article was published; 0 if unknown
    publish_date: i32,
}

impl RObject for PageBlockAuthorDate {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockAuthorDate {}

impl PageBlockAuthorDate {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockAuthorDateBuilder {
        let mut inner = PageBlockAuthorDate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockAuthorDateBuilder { inner }
    }

    pub fn author(&self) -> &RichText {
        &self.author
    }

    pub fn publish_date(&self) -> i32 {
        self.publish_date
    }
}

#[doc(hidden)]
pub struct RTDPageBlockAuthorDateBuilder {
    inner: PageBlockAuthorDate,
}

impl RTDPageBlockAuthorDateBuilder {
    pub fn build(&self) -> PageBlockAuthorDate {
        self.inner.clone()
    }

    pub fn author<T: AsRef<RichText>>(&mut self, author: T) -> &mut Self {
        self.inner.author = author.as_ref().clone();
        self
    }

    pub fn publish_date(&mut self, publish_date: i32) -> &mut Self {
        self.inner.publish_date = publish_date;
        self
    }
}

impl AsRef<PageBlockAuthorDate> for PageBlockAuthorDate {
    fn as_ref(&self) -> &PageBlockAuthorDate {
        self
    }
}

impl AsRef<PageBlockAuthorDate> for RTDPageBlockAuthorDateBuilder {
    fn as_ref(&self) -> &PageBlockAuthorDate {
        &self.inner
    }
}

/// A block quote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockBlockQuote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Quote text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: RichText,
    /// Quote credit

    #[serde(skip_serializing_if = "RichText::_is_default")]
    credit: RichText,
}

impl RObject for PageBlockBlockQuote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockBlockQuote {}

impl PageBlockBlockQuote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockBlockQuoteBuilder {
        let mut inner = PageBlockBlockQuote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockBlockQuoteBuilder { inner }
    }

    pub fn text(&self) -> &RichText {
        &self.text
    }

    pub fn credit(&self) -> &RichText {
        &self.credit
    }
}

#[doc(hidden)]
pub struct RTDPageBlockBlockQuoteBuilder {
    inner: PageBlockBlockQuote,
}

impl RTDPageBlockBlockQuoteBuilder {
    pub fn build(&self) -> PageBlockBlockQuote {
        self.inner.clone()
    }

    pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn credit<T: AsRef<RichText>>(&mut self, credit: T) -> &mut Self {
        self.inner.credit = credit.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockBlockQuote> for PageBlockBlockQuote {
    fn as_ref(&self) -> &PageBlockBlockQuote {
        self
    }
}

impl AsRef<PageBlockBlockQuote> for RTDPageBlockBlockQuoteBuilder {
    fn as_ref(&self) -> &PageBlockBlockQuote {
        &self.inner
    }
}

/// A link to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockChatLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat title
    title: String,
    /// Chat photo; may be null
    photo: Option<ChatPhotoInfo>,
    /// Chat username, by which all other information about the chat should be resolved
    username: String,
}

impl RObject for PageBlockChatLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockChatLink {}

impl PageBlockChatLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockChatLinkBuilder {
        let mut inner = PageBlockChatLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockChatLinkBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn photo(&self) -> &Option<ChatPhotoInfo> {
        &self.photo
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

#[doc(hidden)]
pub struct RTDPageBlockChatLinkBuilder {
    inner: PageBlockChatLink,
}

impl RTDPageBlockChatLinkBuilder {
    pub fn build(&self) -> PageBlockChatLink {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn photo<T: AsRef<ChatPhotoInfo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }
}

impl AsRef<PageBlockChatLink> for PageBlockChatLink {
    fn as_ref(&self) -> &PageBlockChatLink {
        self
    }
}

impl AsRef<PageBlockChatLink> for RTDPageBlockChatLinkBuilder {
    fn as_ref(&self) -> &PageBlockChatLink {
        &self.inner
    }
}

/// A collage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockCollage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Collage item contents
    page_blocks: Vec<PageBlock>,
    /// Block caption
    caption: PageBlockCaption,
}

impl RObject for PageBlockCollage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockCollage {}

impl PageBlockCollage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockCollageBuilder {
        let mut inner = PageBlockCollage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockCollageBuilder { inner }
    }

    pub fn page_blocks(&self) -> &Vec<PageBlock> {
        &self.page_blocks
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDPageBlockCollageBuilder {
    inner: PageBlockCollage,
}

impl RTDPageBlockCollageBuilder {
    pub fn build(&self) -> PageBlockCollage {
        self.inner.clone()
    }

    pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
        self.inner.page_blocks = page_blocks;
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockCollage> for PageBlockCollage {
    fn as_ref(&self) -> &PageBlockCollage {
        self
    }
}

impl AsRef<PageBlockCollage> for RTDPageBlockCollageBuilder {
    fn as_ref(&self) -> &PageBlockCollage {
        &self.inner
    }
}

/// A page cover
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockCover {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Cover

    #[serde(skip_serializing_if = "PageBlock::_is_default")]
    cover: Box<PageBlock>,
}

impl RObject for PageBlockCover {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockCover {}

impl PageBlockCover {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockCoverBuilder {
        let mut inner = PageBlockCover::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockCoverBuilder { inner }
    }

    pub fn cover(&self) -> &Box<PageBlock> {
        &self.cover
    }
}

#[doc(hidden)]
pub struct RTDPageBlockCoverBuilder {
    inner: PageBlockCover,
}

impl RTDPageBlockCoverBuilder {
    pub fn build(&self) -> PageBlockCover {
        self.inner.clone()
    }

    pub fn cover<T: AsRef<Box<PageBlock>>>(&mut self, cover: T) -> &mut Self {
        self.inner.cover = cover.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockCover> for PageBlockCover {
    fn as_ref(&self) -> &PageBlockCover {
        self
    }
}

impl AsRef<PageBlockCover> for RTDPageBlockCoverBuilder {
    fn as_ref(&self) -> &PageBlockCover {
        &self.inner
    }
}

/// A collapsible block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockDetails {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Always visible heading for the block

    #[serde(skip_serializing_if = "RichText::_is_default")]
    header: RichText,
    /// Block contents
    page_blocks: Vec<PageBlock>,
    /// True, if the block is open by default
    is_open: bool,
}

impl RObject for PageBlockDetails {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockDetails {}

impl PageBlockDetails {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockDetailsBuilder {
        let mut inner = PageBlockDetails::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockDetailsBuilder { inner }
    }

    pub fn header(&self) -> &RichText {
        &self.header
    }

    pub fn page_blocks(&self) -> &Vec<PageBlock> {
        &self.page_blocks
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

#[doc(hidden)]
pub struct RTDPageBlockDetailsBuilder {
    inner: PageBlockDetails,
}

impl RTDPageBlockDetailsBuilder {
    pub fn build(&self) -> PageBlockDetails {
        self.inner.clone()
    }

    pub fn header<T: AsRef<RichText>>(&mut self, header: T) -> &mut Self {
        self.inner.header = header.as_ref().clone();
        self
    }

    pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
        self.inner.page_blocks = page_blocks;
        self
    }

    pub fn is_open(&mut self, is_open: bool) -> &mut Self {
        self.inner.is_open = is_open;
        self
    }
}

impl AsRef<PageBlockDetails> for PageBlockDetails {
    fn as_ref(&self) -> &PageBlockDetails {
        self
    }
}

impl AsRef<PageBlockDetails> for RTDPageBlockDetailsBuilder {
    fn as_ref(&self) -> &PageBlockDetails {
        &self.inner
    }
}

/// An empty block separating a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockDivider {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockDivider {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockDivider {}

impl PageBlockDivider {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockDividerBuilder {
        let mut inner = PageBlockDivider::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockDividerBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPageBlockDividerBuilder {
    inner: PageBlockDivider,
}

impl RTDPageBlockDividerBuilder {
    pub fn build(&self) -> PageBlockDivider {
        self.inner.clone()
    }
}

impl AsRef<PageBlockDivider> for PageBlockDivider {
    fn as_ref(&self) -> &PageBlockDivider {
        self
    }
}

impl AsRef<PageBlockDivider> for RTDPageBlockDividerBuilder {
    fn as_ref(&self) -> &PageBlockDivider {
        &self.inner
    }
}

/// An embedded web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockEmbedded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Web page URL, if available
    url: String,
    /// HTML-markup of the embedded page
    html: String,
    /// Poster photo, if available; may be null
    poster_photo: Option<Photo>,
    /// Block width; 0 if unknown
    width: i32,
    /// Block height; 0 if unknown
    height: i32,
    /// Block caption
    caption: PageBlockCaption,
    /// True, if the block should be full width
    is_full_width: bool,
    /// True, if scrolling should be allowed
    allow_scrolling: bool,
}

impl RObject for PageBlockEmbedded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockEmbedded {}

impl PageBlockEmbedded {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockEmbeddedBuilder {
        let mut inner = PageBlockEmbedded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockEmbeddedBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn html(&self) -> &String {
        &self.html
    }

    pub fn poster_photo(&self) -> &Option<Photo> {
        &self.poster_photo
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }

    pub fn is_full_width(&self) -> bool {
        self.is_full_width
    }

    pub fn allow_scrolling(&self) -> bool {
        self.allow_scrolling
    }
}

#[doc(hidden)]
pub struct RTDPageBlockEmbeddedBuilder {
    inner: PageBlockEmbedded,
}

impl RTDPageBlockEmbeddedBuilder {
    pub fn build(&self) -> PageBlockEmbedded {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn html<T: AsRef<str>>(&mut self, html: T) -> &mut Self {
        self.inner.html = html.as_ref().to_string();
        self
    }

    pub fn poster_photo<T: AsRef<Photo>>(&mut self, poster_photo: T) -> &mut Self {
        self.inner.poster_photo = Some(poster_photo.as_ref().clone());
        self
    }

    pub fn width(&mut self, width: i32) -> &mut Self {
        self.inner.width = width;
        self
    }

    pub fn height(&mut self, height: i32) -> &mut Self {
        self.inner.height = height;
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }

    pub fn is_full_width(&mut self, is_full_width: bool) -> &mut Self {
        self.inner.is_full_width = is_full_width;
        self
    }

    pub fn allow_scrolling(&mut self, allow_scrolling: bool) -> &mut Self {
        self.inner.allow_scrolling = allow_scrolling;
        self
    }
}

impl AsRef<PageBlockEmbedded> for PageBlockEmbedded {
    fn as_ref(&self) -> &PageBlockEmbedded {
        self
    }
}

impl AsRef<PageBlockEmbedded> for RTDPageBlockEmbeddedBuilder {
    fn as_ref(&self) -> &PageBlockEmbedded {
        &self.inner
    }
}

/// An embedded post
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockEmbeddedPost {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Web page URL
    url: String,
    /// Post author
    author: String,
    /// Post author photo; may be null
    author_photo: Option<Photo>,
    /// Point in time (Unix timestamp) when the post was created; 0 if unknown
    date: i32,
    /// Post content
    page_blocks: Vec<PageBlock>,
    /// Post caption
    caption: PageBlockCaption,
}

impl RObject for PageBlockEmbeddedPost {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockEmbeddedPost {}

impl PageBlockEmbeddedPost {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockEmbeddedPostBuilder {
        let mut inner = PageBlockEmbeddedPost::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockEmbeddedPostBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn author_photo(&self) -> &Option<Photo> {
        &self.author_photo
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn page_blocks(&self) -> &Vec<PageBlock> {
        &self.page_blocks
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDPageBlockEmbeddedPostBuilder {
    inner: PageBlockEmbeddedPost,
}

impl RTDPageBlockEmbeddedPostBuilder {
    pub fn build(&self) -> PageBlockEmbeddedPost {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn author<T: AsRef<str>>(&mut self, author: T) -> &mut Self {
        self.inner.author = author.as_ref().to_string();
        self
    }

    pub fn author_photo<T: AsRef<Photo>>(&mut self, author_photo: T) -> &mut Self {
        self.inner.author_photo = Some(author_photo.as_ref().clone());
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
        self.inner.page_blocks = page_blocks;
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockEmbeddedPost> for PageBlockEmbeddedPost {
    fn as_ref(&self) -> &PageBlockEmbeddedPost {
        self
    }
}

impl AsRef<PageBlockEmbeddedPost> for RTDPageBlockEmbeddedPostBuilder {
    fn as_ref(&self) -> &PageBlockEmbeddedPost {
        &self.inner
    }
}

/// The footer of a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockFooter {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Footer

    #[serde(skip_serializing_if = "RichText::_is_default")]
    footer: RichText,
}

impl RObject for PageBlockFooter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockFooter {}

impl PageBlockFooter {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockFooterBuilder {
        let mut inner = PageBlockFooter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockFooterBuilder { inner }
    }

    pub fn footer(&self) -> &RichText {
        &self.footer
    }
}

#[doc(hidden)]
pub struct RTDPageBlockFooterBuilder {
    inner: PageBlockFooter,
}

impl RTDPageBlockFooterBuilder {
    pub fn build(&self) -> PageBlockFooter {
        self.inner.clone()
    }

    pub fn footer<T: AsRef<RichText>>(&mut self, footer: T) -> &mut Self {
        self.inner.footer = footer.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockFooter> for PageBlockFooter {
    fn as_ref(&self) -> &PageBlockFooter {
        self
    }
}

impl AsRef<PageBlockFooter> for RTDPageBlockFooterBuilder {
    fn as_ref(&self) -> &PageBlockFooter {
        &self.inner
    }
}

/// A header
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockHeader {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Header

    #[serde(skip_serializing_if = "RichText::_is_default")]
    header: RichText,
}

impl RObject for PageBlockHeader {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockHeader {}

impl PageBlockHeader {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockHeaderBuilder {
        let mut inner = PageBlockHeader::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockHeaderBuilder { inner }
    }

    pub fn header(&self) -> &RichText {
        &self.header
    }
}

#[doc(hidden)]
pub struct RTDPageBlockHeaderBuilder {
    inner: PageBlockHeader,
}

impl RTDPageBlockHeaderBuilder {
    pub fn build(&self) -> PageBlockHeader {
        self.inner.clone()
    }

    pub fn header<T: AsRef<RichText>>(&mut self, header: T) -> &mut Self {
        self.inner.header = header.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockHeader> for PageBlockHeader {
    fn as_ref(&self) -> &PageBlockHeader {
        self
    }
}

impl AsRef<PageBlockHeader> for RTDPageBlockHeaderBuilder {
    fn as_ref(&self) -> &PageBlockHeader {
        &self.inner
    }
}

/// A kicker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockKicker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Kicker

    #[serde(skip_serializing_if = "RichText::_is_default")]
    kicker: RichText,
}

impl RObject for PageBlockKicker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockKicker {}

impl PageBlockKicker {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockKickerBuilder {
        let mut inner = PageBlockKicker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockKickerBuilder { inner }
    }

    pub fn kicker(&self) -> &RichText {
        &self.kicker
    }
}

#[doc(hidden)]
pub struct RTDPageBlockKickerBuilder {
    inner: PageBlockKicker,
}

impl RTDPageBlockKickerBuilder {
    pub fn build(&self) -> PageBlockKicker {
        self.inner.clone()
    }

    pub fn kicker<T: AsRef<RichText>>(&mut self, kicker: T) -> &mut Self {
        self.inner.kicker = kicker.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockKicker> for PageBlockKicker {
    fn as_ref(&self) -> &PageBlockKicker {
        self
    }
}

impl AsRef<PageBlockKicker> for RTDPageBlockKickerBuilder {
    fn as_ref(&self) -> &PageBlockKicker {
        &self.inner
    }
}

/// A list of data blocks
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockList {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The items of the list
    items: Vec<PageBlockListItem>,
}

impl RObject for PageBlockList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockList {}

impl PageBlockList {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockListBuilder {
        let mut inner = PageBlockList::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockListBuilder { inner }
    }

    pub fn items(&self) -> &Vec<PageBlockListItem> {
        &self.items
    }
}

#[doc(hidden)]
pub struct RTDPageBlockListBuilder {
    inner: PageBlockList,
}

impl RTDPageBlockListBuilder {
    pub fn build(&self) -> PageBlockList {
        self.inner.clone()
    }

    pub fn items(&mut self, items: Vec<PageBlockListItem>) -> &mut Self {
        self.inner.items = items;
        self
    }
}

impl AsRef<PageBlockList> for PageBlockList {
    fn as_ref(&self) -> &PageBlockList {
        self
    }
}

impl AsRef<PageBlockList> for RTDPageBlockListBuilder {
    fn as_ref(&self) -> &PageBlockList {
        &self.inner
    }
}

/// A map
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockMap {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Location of the map center
    location: Location,
    /// Map zoom level
    zoom: i32,
    /// Map width
    width: i32,
    /// Map height
    height: i32,
    /// Block caption
    caption: PageBlockCaption,
}

impl RObject for PageBlockMap {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockMap {}

impl PageBlockMap {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockMapBuilder {
        let mut inner = PageBlockMap::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockMapBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn zoom(&self) -> i32 {
        self.zoom
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDPageBlockMapBuilder {
    inner: PageBlockMap,
}

impl RTDPageBlockMapBuilder {
    pub fn build(&self) -> PageBlockMap {
        self.inner.clone()
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }

    pub fn zoom(&mut self, zoom: i32) -> &mut Self {
        self.inner.zoom = zoom;
        self
    }

    pub fn width(&mut self, width: i32) -> &mut Self {
        self.inner.width = width;
        self
    }

    pub fn height(&mut self, height: i32) -> &mut Self {
        self.inner.height = height;
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockMap> for PageBlockMap {
    fn as_ref(&self) -> &PageBlockMap {
        self
    }
}

impl AsRef<PageBlockMap> for RTDPageBlockMapBuilder {
    fn as_ref(&self) -> &PageBlockMap {
        &self.inner
    }
}

/// A text paragraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockParagraph {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Paragraph text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: RichText,
}

impl RObject for PageBlockParagraph {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockParagraph {}

impl PageBlockParagraph {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockParagraphBuilder {
        let mut inner = PageBlockParagraph::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockParagraphBuilder { inner }
    }

    pub fn text(&self) -> &RichText {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDPageBlockParagraphBuilder {
    inner: PageBlockParagraph,
}

impl RTDPageBlockParagraphBuilder {
    pub fn build(&self) -> PageBlockParagraph {
        self.inner.clone()
    }

    pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockParagraph> for PageBlockParagraph {
    fn as_ref(&self) -> &PageBlockParagraph {
        self
    }
}

impl AsRef<PageBlockParagraph> for RTDPageBlockParagraphBuilder {
    fn as_ref(&self) -> &PageBlockParagraph {
        &self.inner
    }
}

/// A photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Photo file; may be null
    photo: Option<Photo>,
    /// Photo caption
    caption: PageBlockCaption,
    /// URL that needs to be opened when the photo is clicked
    url: String,
}

impl RObject for PageBlockPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockPhoto {}

impl PageBlockPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockPhotoBuilder {
        let mut inner = PageBlockPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockPhotoBuilder { inner }
    }

    pub fn photo(&self) -> &Option<Photo> {
        &self.photo
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDPageBlockPhotoBuilder {
    inner: PageBlockPhoto,
}

impl RTDPageBlockPhotoBuilder {
    pub fn build(&self) -> PageBlockPhoto {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<PageBlockPhoto> for PageBlockPhoto {
    fn as_ref(&self) -> &PageBlockPhoto {
        self
    }
}

impl AsRef<PageBlockPhoto> for RTDPageBlockPhotoBuilder {
    fn as_ref(&self) -> &PageBlockPhoto {
        &self.inner
    }
}

/// A preformatted text paragraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockPreformatted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Paragraph text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: RichText,
    /// Programming language for which the text should be formatted
    language: String,
}

impl RObject for PageBlockPreformatted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockPreformatted {}

impl PageBlockPreformatted {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockPreformattedBuilder {
        let mut inner = PageBlockPreformatted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockPreformattedBuilder { inner }
    }

    pub fn text(&self) -> &RichText {
        &self.text
    }

    pub fn language(&self) -> &String {
        &self.language
    }
}

#[doc(hidden)]
pub struct RTDPageBlockPreformattedBuilder {
    inner: PageBlockPreformatted,
}

impl RTDPageBlockPreformattedBuilder {
    pub fn build(&self) -> PageBlockPreformatted {
        self.inner.clone()
    }

    pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn language<T: AsRef<str>>(&mut self, language: T) -> &mut Self {
        self.inner.language = language.as_ref().to_string();
        self
    }
}

impl AsRef<PageBlockPreformatted> for PageBlockPreformatted {
    fn as_ref(&self) -> &PageBlockPreformatted {
        self
    }
}

impl AsRef<PageBlockPreformatted> for RTDPageBlockPreformattedBuilder {
    fn as_ref(&self) -> &PageBlockPreformatted {
        &self.inner
    }
}

/// A pull quote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockPullQuote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Quote text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: RichText,
    /// Quote credit

    #[serde(skip_serializing_if = "RichText::_is_default")]
    credit: RichText,
}

impl RObject for PageBlockPullQuote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockPullQuote {}

impl PageBlockPullQuote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockPullQuoteBuilder {
        let mut inner = PageBlockPullQuote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockPullQuoteBuilder { inner }
    }

    pub fn text(&self) -> &RichText {
        &self.text
    }

    pub fn credit(&self) -> &RichText {
        &self.credit
    }
}

#[doc(hidden)]
pub struct RTDPageBlockPullQuoteBuilder {
    inner: PageBlockPullQuote,
}

impl RTDPageBlockPullQuoteBuilder {
    pub fn build(&self) -> PageBlockPullQuote {
        self.inner.clone()
    }

    pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn credit<T: AsRef<RichText>>(&mut self, credit: T) -> &mut Self {
        self.inner.credit = credit.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockPullQuote> for PageBlockPullQuote {
    fn as_ref(&self) -> &PageBlockPullQuote {
        self
    }
}

impl AsRef<PageBlockPullQuote> for RTDPageBlockPullQuoteBuilder {
    fn as_ref(&self) -> &PageBlockPullQuote {
        &self.inner
    }
}

/// Related articles
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockRelatedArticles {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Block header

    #[serde(skip_serializing_if = "RichText::_is_default")]
    header: RichText,
    /// List of related articles
    articles: Vec<PageBlockRelatedArticle>,
}

impl RObject for PageBlockRelatedArticles {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockRelatedArticles {}

impl PageBlockRelatedArticles {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockRelatedArticlesBuilder {
        let mut inner = PageBlockRelatedArticles::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockRelatedArticlesBuilder { inner }
    }

    pub fn header(&self) -> &RichText {
        &self.header
    }

    pub fn articles(&self) -> &Vec<PageBlockRelatedArticle> {
        &self.articles
    }
}

#[doc(hidden)]
pub struct RTDPageBlockRelatedArticlesBuilder {
    inner: PageBlockRelatedArticles,
}

impl RTDPageBlockRelatedArticlesBuilder {
    pub fn build(&self) -> PageBlockRelatedArticles {
        self.inner.clone()
    }

    pub fn header<T: AsRef<RichText>>(&mut self, header: T) -> &mut Self {
        self.inner.header = header.as_ref().clone();
        self
    }

    pub fn articles(&mut self, articles: Vec<PageBlockRelatedArticle>) -> &mut Self {
        self.inner.articles = articles;
        self
    }
}

impl AsRef<PageBlockRelatedArticles> for PageBlockRelatedArticles {
    fn as_ref(&self) -> &PageBlockRelatedArticles {
        self
    }
}

impl AsRef<PageBlockRelatedArticles> for RTDPageBlockRelatedArticlesBuilder {
    fn as_ref(&self) -> &PageBlockRelatedArticles {
        &self.inner
    }
}

/// A slideshow
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockSlideshow {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Slideshow item contents
    page_blocks: Vec<PageBlock>,
    /// Block caption
    caption: PageBlockCaption,
}

impl RObject for PageBlockSlideshow {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockSlideshow {}

impl PageBlockSlideshow {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockSlideshowBuilder {
        let mut inner = PageBlockSlideshow::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockSlideshowBuilder { inner }
    }

    pub fn page_blocks(&self) -> &Vec<PageBlock> {
        &self.page_blocks
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDPageBlockSlideshowBuilder {
    inner: PageBlockSlideshow,
}

impl RTDPageBlockSlideshowBuilder {
    pub fn build(&self) -> PageBlockSlideshow {
        self.inner.clone()
    }

    pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
        self.inner.page_blocks = page_blocks;
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockSlideshow> for PageBlockSlideshow {
    fn as_ref(&self) -> &PageBlockSlideshow {
        self
    }
}

impl AsRef<PageBlockSlideshow> for RTDPageBlockSlideshowBuilder {
    fn as_ref(&self) -> &PageBlockSlideshow {
        &self.inner
    }
}

/// A subheader
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockSubheader {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Subheader

    #[serde(skip_serializing_if = "RichText::_is_default")]
    subheader: RichText,
}

impl RObject for PageBlockSubheader {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockSubheader {}

impl PageBlockSubheader {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockSubheaderBuilder {
        let mut inner = PageBlockSubheader::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockSubheaderBuilder { inner }
    }

    pub fn subheader(&self) -> &RichText {
        &self.subheader
    }
}

#[doc(hidden)]
pub struct RTDPageBlockSubheaderBuilder {
    inner: PageBlockSubheader,
}

impl RTDPageBlockSubheaderBuilder {
    pub fn build(&self) -> PageBlockSubheader {
        self.inner.clone()
    }

    pub fn subheader<T: AsRef<RichText>>(&mut self, subheader: T) -> &mut Self {
        self.inner.subheader = subheader.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockSubheader> for PageBlockSubheader {
    fn as_ref(&self) -> &PageBlockSubheader {
        self
    }
}

impl AsRef<PageBlockSubheader> for RTDPageBlockSubheaderBuilder {
    fn as_ref(&self) -> &PageBlockSubheader {
        &self.inner
    }
}

/// The subtitle of a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockSubtitle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Subtitle

    #[serde(skip_serializing_if = "RichText::_is_default")]
    subtitle: RichText,
}

impl RObject for PageBlockSubtitle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockSubtitle {}

impl PageBlockSubtitle {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockSubtitleBuilder {
        let mut inner = PageBlockSubtitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockSubtitleBuilder { inner }
    }

    pub fn subtitle(&self) -> &RichText {
        &self.subtitle
    }
}

#[doc(hidden)]
pub struct RTDPageBlockSubtitleBuilder {
    inner: PageBlockSubtitle,
}

impl RTDPageBlockSubtitleBuilder {
    pub fn build(&self) -> PageBlockSubtitle {
        self.inner.clone()
    }

    pub fn subtitle<T: AsRef<RichText>>(&mut self, subtitle: T) -> &mut Self {
        self.inner.subtitle = subtitle.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockSubtitle> for PageBlockSubtitle {
    fn as_ref(&self) -> &PageBlockSubtitle {
        self
    }
}

impl AsRef<PageBlockSubtitle> for RTDPageBlockSubtitleBuilder {
    fn as_ref(&self) -> &PageBlockSubtitle {
        &self.inner
    }
}

/// A table
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockTable {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Table caption

    #[serde(skip_serializing_if = "RichText::_is_default")]
    caption: RichText,
    /// Table cells
    cells: Vec<Vec<PageBlockTableCell>>,
    /// True, if the table is bordered
    is_bordered: bool,
    /// True, if the table is striped
    is_striped: bool,
}

impl RObject for PageBlockTable {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockTable {}

impl PageBlockTable {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockTableBuilder {
        let mut inner = PageBlockTable::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockTableBuilder { inner }
    }

    pub fn caption(&self) -> &RichText {
        &self.caption
    }

    pub fn cells(&self) -> &Vec<Vec<PageBlockTableCell>> {
        &self.cells
    }

    pub fn is_bordered(&self) -> bool {
        self.is_bordered
    }

    pub fn is_striped(&self) -> bool {
        self.is_striped
    }
}

#[doc(hidden)]
pub struct RTDPageBlockTableBuilder {
    inner: PageBlockTable,
}

impl RTDPageBlockTableBuilder {
    pub fn build(&self) -> PageBlockTable {
        self.inner.clone()
    }

    pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }

    pub fn cells(&mut self, cells: Vec<Vec<PageBlockTableCell>>) -> &mut Self {
        self.inner.cells = cells;
        self
    }

    pub fn is_bordered(&mut self, is_bordered: bool) -> &mut Self {
        self.inner.is_bordered = is_bordered;
        self
    }

    pub fn is_striped(&mut self, is_striped: bool) -> &mut Self {
        self.inner.is_striped = is_striped;
        self
    }
}

impl AsRef<PageBlockTable> for PageBlockTable {
    fn as_ref(&self) -> &PageBlockTable {
        self
    }
}

impl AsRef<PageBlockTable> for RTDPageBlockTableBuilder {
    fn as_ref(&self) -> &PageBlockTable {
        &self.inner
    }
}

/// The title of a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockTitle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Title

    #[serde(skip_serializing_if = "RichText::_is_default")]
    title: RichText,
}

impl RObject for PageBlockTitle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockTitle {}

impl PageBlockTitle {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockTitleBuilder {
        let mut inner = PageBlockTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockTitleBuilder { inner }
    }

    pub fn title(&self) -> &RichText {
        &self.title
    }
}

#[doc(hidden)]
pub struct RTDPageBlockTitleBuilder {
    inner: PageBlockTitle,
}

impl RTDPageBlockTitleBuilder {
    pub fn build(&self) -> PageBlockTitle {
        self.inner.clone()
    }

    pub fn title<T: AsRef<RichText>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockTitle> for PageBlockTitle {
    fn as_ref(&self) -> &PageBlockTitle {
        self
    }
}

impl AsRef<PageBlockTitle> for RTDPageBlockTitleBuilder {
    fn as_ref(&self) -> &PageBlockTitle {
        &self.inner
    }
}

/// A video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Video file; may be null
    video: Option<Video>,
    /// Video caption
    caption: PageBlockCaption,
    /// True, if the video should be played automatically
    need_autoplay: bool,
    /// True, if the video should be looped
    is_looped: bool,
}

impl RObject for PageBlockVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockVideo {}

impl PageBlockVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockVideoBuilder {
        let mut inner = PageBlockVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockVideoBuilder { inner }
    }

    pub fn video(&self) -> &Option<Video> {
        &self.video
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }

    pub fn need_autoplay(&self) -> bool {
        self.need_autoplay
    }

    pub fn is_looped(&self) -> bool {
        self.is_looped
    }
}

#[doc(hidden)]
pub struct RTDPageBlockVideoBuilder {
    inner: PageBlockVideo,
}

impl RTDPageBlockVideoBuilder {
    pub fn build(&self) -> PageBlockVideo {
        self.inner.clone()
    }

    pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
        self.inner.video = Some(video.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }

    pub fn need_autoplay(&mut self, need_autoplay: bool) -> &mut Self {
        self.inner.need_autoplay = need_autoplay;
        self
    }

    pub fn is_looped(&mut self, is_looped: bool) -> &mut Self {
        self.inner.is_looped = is_looped;
        self
    }
}

impl AsRef<PageBlockVideo> for PageBlockVideo {
    fn as_ref(&self) -> &PageBlockVideo {
        self
    }
}

impl AsRef<PageBlockVideo> for RTDPageBlockVideoBuilder {
    fn as_ref(&self) -> &PageBlockVideo {
        &self.inner
    }
}

/// A voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Voice note; may be null
    voice_note: Option<VoiceNote>,
    /// Voice note caption
    caption: PageBlockCaption,
}

impl RObject for PageBlockVoiceNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlock for PageBlockVoiceNote {}

impl PageBlockVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockVoiceNoteBuilder {
        let mut inner = PageBlockVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockVoiceNoteBuilder { inner }
    }

    pub fn voice_note(&self) -> &Option<VoiceNote> {
        &self.voice_note
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDPageBlockVoiceNoteBuilder {
    inner: PageBlockVoiceNote,
}

impl RTDPageBlockVoiceNoteBuilder {
    pub fn build(&self) -> PageBlockVoiceNote {
        self.inner.clone()
    }

    pub fn voice_note<T: AsRef<VoiceNote>>(&mut self, voice_note: T) -> &mut Self {
        self.inner.voice_note = Some(voice_note.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockVoiceNote> for PageBlockVoiceNote {
    fn as_ref(&self) -> &PageBlockVoiceNote {
        self
    }
}

impl AsRef<PageBlockVoiceNote> for RTDPageBlockVoiceNoteBuilder {
    fn as_ref(&self) -> &PageBlockVoiceNote {
        &self.inner
    }
}
