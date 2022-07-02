use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a related article
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockRelatedArticle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Related article URL

    #[serde(default)]
    url: String,
    /// Article title; may be empty

    #[serde(default)]
    title: String,
    /// Contains information about a related article

    #[serde(default)]
    description: String,
    /// Article photo; may be null
    photo: Option<Photo>,
    /// Article author; may be empty

    #[serde(default)]
    author: String,
    /// Point in time (Unix timestamp) when the article was published; 0 if unknown

    #[serde(default)]
    publish_date: i32,
}

impl RObject for PageBlockRelatedArticle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PageBlockRelatedArticle {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockRelatedArticleBuilder {
        let mut inner = PageBlockRelatedArticle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockRelatedArticleBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn photo(&self) -> &Option<Photo> {
        &self.photo
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn publish_date(&self) -> i32 {
        self.publish_date
    }
}

#[doc(hidden)]
pub struct PageBlockRelatedArticleBuilder {
    inner: PageBlockRelatedArticle,
}

#[deprecated]
pub type RTDPageBlockRelatedArticleBuilder = PageBlockRelatedArticleBuilder;

impl PageBlockRelatedArticleBuilder {
    pub fn build(&self) -> PageBlockRelatedArticle {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
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

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn author<T: AsRef<str>>(&mut self, author: T) -> &mut Self {
        self.inner.author = author.as_ref().to_string();
        self
    }

    pub fn publish_date(&mut self, publish_date: i32) -> &mut Self {
        self.inner.publish_date = publish_date;
        self
    }
}

impl AsRef<PageBlockRelatedArticle> for PageBlockRelatedArticle {
    fn as_ref(&self) -> &PageBlockRelatedArticle {
        self
    }
}

impl AsRef<PageBlockRelatedArticle> for PageBlockRelatedArticleBuilder {
    fn as_ref(&self) -> &PageBlockRelatedArticle {
        &self.inner
    }
}
