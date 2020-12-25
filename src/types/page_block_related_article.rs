
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a related article
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockRelatedArticle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Related article URL
  url: String,
  /// Article title; may be empty
  title: String,
  /// Contains information about a related article
  description: String,
  /// Article photo; may be null
  photo: Option<Photo>,
  /// Article author; may be empty
  author: String,
  /// Point in time (Unix timestamp) when the article was published; 0 if unknown
  publish_date: i64,
  
}

impl RObject for PageBlockRelatedArticle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockRelatedArticle" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PageBlockRelatedArticle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockRelatedArticleBuilder {
    let mut inner = PageBlockRelatedArticle::default();
    inner.td_name = "pageBlockRelatedArticle".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockRelatedArticleBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn title(&self) -> &String { &self.title }

  pub fn description(&self) -> &String { &self.description }

  pub fn photo(&self) -> &Option<Photo> { &self.photo }

  pub fn author(&self) -> &String { &self.author }

  pub fn publish_date(&self) -> i64 { self.publish_date }

}

#[doc(hidden)]
pub struct RTDPageBlockRelatedArticleBuilder {
  inner: PageBlockRelatedArticle
}

impl RTDPageBlockRelatedArticleBuilder {
  pub fn build(&self) -> PageBlockRelatedArticle { self.inner.clone() }

   
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

   
  pub fn publish_date(&mut self, publish_date: i64) -> &mut Self {
    self.inner.publish_date = publish_date;
    self
  }

}

impl AsRef<PageBlockRelatedArticle> for PageBlockRelatedArticle {
  fn as_ref(&self) -> &PageBlockRelatedArticle { self }
}

impl AsRef<PageBlockRelatedArticle> for RTDPageBlockRelatedArticleBuilder {
  fn as_ref(&self) -> &PageBlockRelatedArticle { &self.inner }
}



