use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns favorite stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFavoriteStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetFavoriteStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetFavoriteStickers {}

impl GetFavoriteStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetFavoriteStickersBuilder {
        let mut inner = GetFavoriteStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getFavoriteStickers".to_string();

        GetFavoriteStickersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetFavoriteStickersBuilder {
    inner: GetFavoriteStickers,
}

#[deprecated]
pub type RTDGetFavoriteStickersBuilder = GetFavoriteStickersBuilder;

impl GetFavoriteStickersBuilder {
    pub fn build(&self) -> GetFavoriteStickers {
        self.inner.clone()
    }
}

impl AsRef<GetFavoriteStickers> for GetFavoriteStickers {
    fn as_ref(&self) -> &GetFavoriteStickers {
        self
    }
}

impl AsRef<GetFavoriteStickers> for GetFavoriteStickersBuilder {
    fn as_ref(&self) -> &GetFavoriteStickers {
        &self.inner
    }
}
