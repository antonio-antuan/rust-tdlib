use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of stickers
    stickers: Vec<Sticker>,
}

impl RObject for Stickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Stickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStickersBuilder {
        let mut inner = Stickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStickersBuilder { inner }
    }

    pub fn stickers(&self) -> &Vec<Sticker> {
        &self.stickers
    }
}

#[doc(hidden)]
pub struct RTDStickersBuilder {
    inner: Stickers,
}

impl RTDStickersBuilder {
    pub fn build(&self) -> Stickers {
        self.inner.clone()
    }

    pub fn stickers(&mut self, stickers: Vec<Sticker>) -> &mut Self {
        self.inner.stickers = stickers;
        self
    }
}

impl AsRef<Stickers> for Stickers {
    fn as_ref(&self) -> &Stickers {
        self
    }
}

impl AsRef<Stickers> for RTDStickersBuilder {
    fn as_ref(&self) -> &Stickers {
        &self.inner
    }
}
