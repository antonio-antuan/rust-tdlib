
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a list of stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of stickers
  stickers: Vec<Sticker>,
  
}

impl RObject for Stickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Stickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStickersBuilder {
    let mut inner = Stickers::default();
    inner.td_name = "stickers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStickersBuilder { inner }
  }

  pub fn stickers(&self) -> &Vec<Sticker> { &self.stickers }

}

#[doc(hidden)]
pub struct RTDStickersBuilder {
  inner: Stickers
}

impl RTDStickersBuilder {
  pub fn build(&self) -> Stickers { self.inner.clone() }

   
  pub fn stickers(&mut self, stickers: Vec<Sticker>) -> &mut Self {
    self.inner.stickers = stickers;
    self
  }

}

impl AsRef<Stickers> for Stickers {
  fn as_ref(&self) -> &Stickers { self }
}

impl AsRef<Stickers> for RTDStickersBuilder {
  fn as_ref(&self) -> &Stickers { &self.inner }
}



