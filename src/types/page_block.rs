use crate::errors::Result;
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
    #[serde(rename = "pageBlockAnchor")]
    Anchor(PageBlockAnchor),
    /// An animation
    #[serde(rename = "pageBlockAnimation")]
    Animation(PageBlockAnimation),
    /// An audio file
    #[serde(rename = "pageBlockAudio")]
    Audio(PageBlockAudio),
    /// The author and publishing date of a page
    #[serde(rename = "pageBlockAuthorDate")]
    AuthorDate(PageBlockAuthorDate),
    /// A block quote
    #[serde(rename = "pageBlockBlockQuote")]
    BlockQuote(PageBlockBlockQuote),
    /// A link to a chat
    #[serde(rename = "pageBlockChatLink")]
    ChatLink(PageBlockChatLink),
    /// A collage
    #[serde(rename = "pageBlockCollage")]
    Collage(PageBlockCollage),
    /// A page cover
    #[serde(rename = "pageBlockCover")]
    Cover(PageBlockCover),
    /// A collapsible block
    #[serde(rename = "pageBlockDetails")]
    Details(PageBlockDetails),
    /// An empty block separating a page
    #[serde(rename = "pageBlockDivider")]
    Divider(PageBlockDivider),
    /// An embedded web page
    #[serde(rename = "pageBlockEmbedded")]
    Embedded(PageBlockEmbedded),
    /// An embedded post
    #[serde(rename = "pageBlockEmbeddedPost")]
    EmbeddedPost(PageBlockEmbeddedPost),
    /// The footer of a page
    #[serde(rename = "pageBlockFooter")]
    Footer(PageBlockFooter),
    /// A header
    #[serde(rename = "pageBlockHeader")]
    Header(PageBlockHeader),
    /// A kicker
    #[serde(rename = "pageBlockKicker")]
    Kicker(PageBlockKicker),
    /// A list of data blocks
    #[serde(rename = "pageBlockList")]
    List(PageBlockList),
    /// A map
    #[serde(rename = "pageBlockMap")]
    Map(PageBlockMap),
    /// A text paragraph
    #[serde(rename = "pageBlockParagraph")]
    Paragraph(PageBlockParagraph),
    /// A photo
    #[serde(rename = "pageBlockPhoto")]
    Photo(PageBlockPhoto),
    /// A preformatted text paragraph
    #[serde(rename = "pageBlockPreformatted")]
    Preformatted(PageBlockPreformatted),
    /// A pull quote
    #[serde(rename = "pageBlockPullQuote")]
    PullQuote(PageBlockPullQuote),
    /// Related articles
    #[serde(rename = "pageBlockRelatedArticles")]
    RelatedArticles(PageBlockRelatedArticles),
    /// A slideshow
    #[serde(rename = "pageBlockSlideshow")]
    Slideshow(PageBlockSlideshow),
    /// A subheader
    #[serde(rename = "pageBlockSubheader")]
    Subheader(PageBlockSubheader),
    /// The subtitle of a page
    #[serde(rename = "pageBlockSubtitle")]
    Subtitle(PageBlockSubtitle),
    /// A table
    #[serde(rename = "pageBlockTable")]
    Table(PageBlockTable),
    /// The title of a page
    #[serde(rename = "pageBlockTitle")]
    Title(PageBlockTitle),
    /// A video
    #[serde(rename = "pageBlockVideo")]
    Video(PageBlockVideo),
    /// A voice note
    #[serde(rename = "pageBlockVoiceNote")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockAnchorBuilder {
        let mut inner = PageBlockAnchor::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockAnchorBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct PageBlockAnchorBuilder {
    inner: PageBlockAnchor,
}

#[deprecated]
pub type RTDPageBlockAnchorBuilder = PageBlockAnchorBuilder;

impl PageBlockAnchorBuilder {
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

impl AsRef<PageBlockAnchor> for PageBlockAnchorBuilder {
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
    /// True, if the animation must be played automatically

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockAnimationBuilder {
        let mut inner = PageBlockAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockAnimationBuilder { inner }
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
pub struct PageBlockAnimationBuilder {
    inner: PageBlockAnimation,
}

#[deprecated]
pub type RTDPageBlockAnimationBuilder = PageBlockAnimationBuilder;

impl PageBlockAnimationBuilder {
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

impl AsRef<PageBlockAnimation> for PageBlockAnimationBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockAudioBuilder {
        let mut inner = PageBlockAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockAudioBuilder { inner }
    }

    pub fn audio(&self) -> &Option<Audio> {
        &self.audio
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct PageBlockAudioBuilder {
    inner: PageBlockAudio,
}

#[deprecated]
pub type RTDPageBlockAudioBuilder = PageBlockAudioBuilder;

impl PageBlockAudioBuilder {
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

impl AsRef<PageBlockAudio> for PageBlockAudioBuilder {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockAuthorDateBuilder {
        let mut inner = PageBlockAuthorDate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockAuthorDateBuilder { inner }
    }

    pub fn author(&self) -> &RichText {
        &self.author
    }

    pub fn publish_date(&self) -> i32 {
        self.publish_date
    }
}

#[doc(hidden)]
pub struct PageBlockAuthorDateBuilder {
    inner: PageBlockAuthorDate,
}

#[deprecated]
pub type RTDPageBlockAuthorDateBuilder = PageBlockAuthorDateBuilder;

impl PageBlockAuthorDateBuilder {
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

impl AsRef<PageBlockAuthorDate> for PageBlockAuthorDateBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockBlockQuoteBuilder {
        let mut inner = PageBlockBlockQuote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockBlockQuoteBuilder { inner }
    }

    pub fn text(&self) -> &RichText {
        &self.text
    }

    pub fn credit(&self) -> &RichText {
        &self.credit
    }
}

#[doc(hidden)]
pub struct PageBlockBlockQuoteBuilder {
    inner: PageBlockBlockQuote,
}

#[deprecated]
pub type RTDPageBlockBlockQuoteBuilder = PageBlockBlockQuoteBuilder;

impl PageBlockBlockQuoteBuilder {
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

impl AsRef<PageBlockBlockQuote> for PageBlockBlockQuoteBuilder {
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

    #[serde(default)]
    title: String,
    /// Chat photo; may be null
    photo: Option<ChatPhotoInfo>,
    /// Chat username, by which all other information about the chat can be resolved

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockChatLinkBuilder {
        let mut inner = PageBlockChatLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockChatLinkBuilder { inner }
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
pub struct PageBlockChatLinkBuilder {
    inner: PageBlockChatLink,
}

#[deprecated]
pub type RTDPageBlockChatLinkBuilder = PageBlockChatLinkBuilder;

impl PageBlockChatLinkBuilder {
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

impl AsRef<PageBlockChatLink> for PageBlockChatLinkBuilder {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockCollageBuilder {
        let mut inner = PageBlockCollage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockCollageBuilder { inner }
    }

    pub fn page_blocks(&self) -> &Vec<PageBlock> {
        &self.page_blocks
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct PageBlockCollageBuilder {
    inner: PageBlockCollage,
}

#[deprecated]
pub type RTDPageBlockCollageBuilder = PageBlockCollageBuilder;

impl PageBlockCollageBuilder {
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

impl AsRef<PageBlockCollage> for PageBlockCollageBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockCoverBuilder {
        let mut inner = PageBlockCover::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockCoverBuilder { inner }
    }

    pub fn cover(&self) -> &Box<PageBlock> {
        &self.cover
    }
}

#[doc(hidden)]
pub struct PageBlockCoverBuilder {
    inner: PageBlockCover,
}

#[deprecated]
pub type RTDPageBlockCoverBuilder = PageBlockCoverBuilder;

impl PageBlockCoverBuilder {
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

impl AsRef<PageBlockCover> for PageBlockCoverBuilder {
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

    #[serde(default)]
    page_blocks: Vec<PageBlock>,
    /// True, if the block is open by default

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockDetailsBuilder {
        let mut inner = PageBlockDetails::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockDetailsBuilder { inner }
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
pub struct PageBlockDetailsBuilder {
    inner: PageBlockDetails,
}

#[deprecated]
pub type RTDPageBlockDetailsBuilder = PageBlockDetailsBuilder;

impl PageBlockDetailsBuilder {
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

impl AsRef<PageBlockDetails> for PageBlockDetailsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockDividerBuilder {
        let mut inner = PageBlockDivider::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockDividerBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PageBlockDividerBuilder {
    inner: PageBlockDivider,
}

#[deprecated]
pub type RTDPageBlockDividerBuilder = PageBlockDividerBuilder;

impl PageBlockDividerBuilder {
    pub fn build(&self) -> PageBlockDivider {
        self.inner.clone()
    }
}

impl AsRef<PageBlockDivider> for PageBlockDivider {
    fn as_ref(&self) -> &PageBlockDivider {
        self
    }
}

impl AsRef<PageBlockDivider> for PageBlockDividerBuilder {
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

    #[serde(default)]
    url: String,
    /// HTML-markup of the embedded page

    #[serde(default)]
    html: String,
    /// Poster photo, if available; may be null
    poster_photo: Option<Photo>,
    /// Block width; 0 if unknown

    #[serde(default)]
    width: i32,
    /// Block height; 0 if unknown

    #[serde(default)]
    height: i32,
    /// Block caption
    caption: PageBlockCaption,
    /// True, if the block must be full width

    #[serde(default)]
    is_full_width: bool,
    /// True, if scrolling needs to be allowed

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockEmbeddedBuilder {
        let mut inner = PageBlockEmbedded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockEmbeddedBuilder { inner }
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
pub struct PageBlockEmbeddedBuilder {
    inner: PageBlockEmbedded,
}

#[deprecated]
pub type RTDPageBlockEmbeddedBuilder = PageBlockEmbeddedBuilder;

impl PageBlockEmbeddedBuilder {
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

impl AsRef<PageBlockEmbedded> for PageBlockEmbeddedBuilder {
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

    #[serde(default)]
    url: String,
    /// Post author

    #[serde(default)]
    author: String,
    /// Post author photo; may be null
    author_photo: Option<Photo>,
    /// Point in time (Unix timestamp) when the post was created; 0 if unknown

    #[serde(default)]
    date: i32,
    /// Post content

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockEmbeddedPostBuilder {
        let mut inner = PageBlockEmbeddedPost::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockEmbeddedPostBuilder { inner }
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
pub struct PageBlockEmbeddedPostBuilder {
    inner: PageBlockEmbeddedPost,
}

#[deprecated]
pub type RTDPageBlockEmbeddedPostBuilder = PageBlockEmbeddedPostBuilder;

impl PageBlockEmbeddedPostBuilder {
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

impl AsRef<PageBlockEmbeddedPost> for PageBlockEmbeddedPostBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockFooterBuilder {
        let mut inner = PageBlockFooter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockFooterBuilder { inner }
    }

    pub fn footer(&self) -> &RichText {
        &self.footer
    }
}

#[doc(hidden)]
pub struct PageBlockFooterBuilder {
    inner: PageBlockFooter,
}

#[deprecated]
pub type RTDPageBlockFooterBuilder = PageBlockFooterBuilder;

impl PageBlockFooterBuilder {
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

impl AsRef<PageBlockFooter> for PageBlockFooterBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockHeaderBuilder {
        let mut inner = PageBlockHeader::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockHeaderBuilder { inner }
    }

    pub fn header(&self) -> &RichText {
        &self.header
    }
}

#[doc(hidden)]
pub struct PageBlockHeaderBuilder {
    inner: PageBlockHeader,
}

#[deprecated]
pub type RTDPageBlockHeaderBuilder = PageBlockHeaderBuilder;

impl PageBlockHeaderBuilder {
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

impl AsRef<PageBlockHeader> for PageBlockHeaderBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockKickerBuilder {
        let mut inner = PageBlockKicker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockKickerBuilder { inner }
    }

    pub fn kicker(&self) -> &RichText {
        &self.kicker
    }
}

#[doc(hidden)]
pub struct PageBlockKickerBuilder {
    inner: PageBlockKicker,
}

#[deprecated]
pub type RTDPageBlockKickerBuilder = PageBlockKickerBuilder;

impl PageBlockKickerBuilder {
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

impl AsRef<PageBlockKicker> for PageBlockKickerBuilder {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockListBuilder {
        let mut inner = PageBlockList::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockListBuilder { inner }
    }

    pub fn items(&self) -> &Vec<PageBlockListItem> {
        &self.items
    }
}

#[doc(hidden)]
pub struct PageBlockListBuilder {
    inner: PageBlockList,
}

#[deprecated]
pub type RTDPageBlockListBuilder = PageBlockListBuilder;

impl PageBlockListBuilder {
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

impl AsRef<PageBlockList> for PageBlockListBuilder {
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

    #[serde(default)]
    zoom: i32,
    /// Map width

    #[serde(default)]
    width: i32,
    /// Map height

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockMapBuilder {
        let mut inner = PageBlockMap::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockMapBuilder { inner }
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
pub struct PageBlockMapBuilder {
    inner: PageBlockMap,
}

#[deprecated]
pub type RTDPageBlockMapBuilder = PageBlockMapBuilder;

impl PageBlockMapBuilder {
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

impl AsRef<PageBlockMap> for PageBlockMapBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockParagraphBuilder {
        let mut inner = PageBlockParagraph::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockParagraphBuilder { inner }
    }

    pub fn text(&self) -> &RichText {
        &self.text
    }
}

#[doc(hidden)]
pub struct PageBlockParagraphBuilder {
    inner: PageBlockParagraph,
}

#[deprecated]
pub type RTDPageBlockParagraphBuilder = PageBlockParagraphBuilder;

impl PageBlockParagraphBuilder {
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

impl AsRef<PageBlockParagraph> for PageBlockParagraphBuilder {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockPhotoBuilder {
        let mut inner = PageBlockPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockPhotoBuilder { inner }
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
pub struct PageBlockPhotoBuilder {
    inner: PageBlockPhoto,
}

#[deprecated]
pub type RTDPageBlockPhotoBuilder = PageBlockPhotoBuilder;

impl PageBlockPhotoBuilder {
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

impl AsRef<PageBlockPhoto> for PageBlockPhotoBuilder {
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
    /// Programming language for which the text needs to be formatted

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockPreformattedBuilder {
        let mut inner = PageBlockPreformatted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockPreformattedBuilder { inner }
    }

    pub fn text(&self) -> &RichText {
        &self.text
    }

    pub fn language(&self) -> &String {
        &self.language
    }
}

#[doc(hidden)]
pub struct PageBlockPreformattedBuilder {
    inner: PageBlockPreformatted,
}

#[deprecated]
pub type RTDPageBlockPreformattedBuilder = PageBlockPreformattedBuilder;

impl PageBlockPreformattedBuilder {
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

impl AsRef<PageBlockPreformatted> for PageBlockPreformattedBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockPullQuoteBuilder {
        let mut inner = PageBlockPullQuote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockPullQuoteBuilder { inner }
    }

    pub fn text(&self) -> &RichText {
        &self.text
    }

    pub fn credit(&self) -> &RichText {
        &self.credit
    }
}

#[doc(hidden)]
pub struct PageBlockPullQuoteBuilder {
    inner: PageBlockPullQuote,
}

#[deprecated]
pub type RTDPageBlockPullQuoteBuilder = PageBlockPullQuoteBuilder;

impl PageBlockPullQuoteBuilder {
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

impl AsRef<PageBlockPullQuote> for PageBlockPullQuoteBuilder {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockRelatedArticlesBuilder {
        let mut inner = PageBlockRelatedArticles::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockRelatedArticlesBuilder { inner }
    }

    pub fn header(&self) -> &RichText {
        &self.header
    }

    pub fn articles(&self) -> &Vec<PageBlockRelatedArticle> {
        &self.articles
    }
}

#[doc(hidden)]
pub struct PageBlockRelatedArticlesBuilder {
    inner: PageBlockRelatedArticles,
}

#[deprecated]
pub type RTDPageBlockRelatedArticlesBuilder = PageBlockRelatedArticlesBuilder;

impl PageBlockRelatedArticlesBuilder {
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

impl AsRef<PageBlockRelatedArticles> for PageBlockRelatedArticlesBuilder {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockSlideshowBuilder {
        let mut inner = PageBlockSlideshow::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockSlideshowBuilder { inner }
    }

    pub fn page_blocks(&self) -> &Vec<PageBlock> {
        &self.page_blocks
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct PageBlockSlideshowBuilder {
    inner: PageBlockSlideshow,
}

#[deprecated]
pub type RTDPageBlockSlideshowBuilder = PageBlockSlideshowBuilder;

impl PageBlockSlideshowBuilder {
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

impl AsRef<PageBlockSlideshow> for PageBlockSlideshowBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockSubheaderBuilder {
        let mut inner = PageBlockSubheader::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockSubheaderBuilder { inner }
    }

    pub fn subheader(&self) -> &RichText {
        &self.subheader
    }
}

#[doc(hidden)]
pub struct PageBlockSubheaderBuilder {
    inner: PageBlockSubheader,
}

#[deprecated]
pub type RTDPageBlockSubheaderBuilder = PageBlockSubheaderBuilder;

impl PageBlockSubheaderBuilder {
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

impl AsRef<PageBlockSubheader> for PageBlockSubheaderBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockSubtitleBuilder {
        let mut inner = PageBlockSubtitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockSubtitleBuilder { inner }
    }

    pub fn subtitle(&self) -> &RichText {
        &self.subtitle
    }
}

#[doc(hidden)]
pub struct PageBlockSubtitleBuilder {
    inner: PageBlockSubtitle,
}

#[deprecated]
pub type RTDPageBlockSubtitleBuilder = PageBlockSubtitleBuilder;

impl PageBlockSubtitleBuilder {
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

impl AsRef<PageBlockSubtitle> for PageBlockSubtitleBuilder {
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

    #[serde(default)]
    cells: Vec<Vec<PageBlockTableCell>>,
    /// True, if the table is bordered

    #[serde(default)]
    is_bordered: bool,
    /// True, if the table is striped

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockTableBuilder {
        let mut inner = PageBlockTable::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockTableBuilder { inner }
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
pub struct PageBlockTableBuilder {
    inner: PageBlockTable,
}

#[deprecated]
pub type RTDPageBlockTableBuilder = PageBlockTableBuilder;

impl PageBlockTableBuilder {
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

impl AsRef<PageBlockTable> for PageBlockTableBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockTitleBuilder {
        let mut inner = PageBlockTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockTitleBuilder { inner }
    }

    pub fn title(&self) -> &RichText {
        &self.title
    }
}

#[doc(hidden)]
pub struct PageBlockTitleBuilder {
    inner: PageBlockTitle,
}

#[deprecated]
pub type RTDPageBlockTitleBuilder = PageBlockTitleBuilder;

impl PageBlockTitleBuilder {
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

impl AsRef<PageBlockTitle> for PageBlockTitleBuilder {
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
    /// True, if the video must be played automatically

    #[serde(default)]
    need_autoplay: bool,
    /// True, if the video must be looped

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockVideoBuilder {
        let mut inner = PageBlockVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockVideoBuilder { inner }
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
pub struct PageBlockVideoBuilder {
    inner: PageBlockVideo,
}

#[deprecated]
pub type RTDPageBlockVideoBuilder = PageBlockVideoBuilder;

impl PageBlockVideoBuilder {
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

impl AsRef<PageBlockVideo> for PageBlockVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockVoiceNoteBuilder {
        let mut inner = PageBlockVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockVoiceNoteBuilder { inner }
    }

    pub fn voice_note(&self) -> &Option<VoiceNote> {
        &self.voice_note
    }

    pub fn caption(&self) -> &PageBlockCaption {
        &self.caption
    }
}

#[doc(hidden)]
pub struct PageBlockVoiceNoteBuilder {
    inner: PageBlockVoiceNote,
}

#[deprecated]
pub type RTDPageBlockVoiceNoteBuilder = PageBlockVoiceNoteBuilder;

impl PageBlockVoiceNoteBuilder {
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

impl AsRef<PageBlockVoiceNote> for PageBlockVoiceNoteBuilder {
    fn as_ref(&self) -> &PageBlockVoiceNote {
        &self.inner
    }
}
