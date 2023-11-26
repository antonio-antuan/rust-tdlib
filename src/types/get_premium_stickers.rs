use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns premium stickers from regular sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPremiumStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The maximum number of stickers to be returned; 0-100

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPremiumStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPremiumStickers {}

impl GetPremiumStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPremiumStickersBuilder {
        let mut inner = GetPremiumStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPremiumStickers".to_string();

        GetPremiumStickersBuilder { inner }
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetPremiumStickersBuilder {
    inner: GetPremiumStickers,
}

#[deprecated]
pub type RTDGetPremiumStickersBuilder = GetPremiumStickersBuilder;

impl GetPremiumStickersBuilder {
    pub fn build(&self) -> GetPremiumStickers {
        self.inner.clone()
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetPremiumStickers> for GetPremiumStickers {
    fn as_ref(&self) -> &GetPremiumStickers {
        self
    }
}

impl AsRef<GetPremiumStickers> for GetPremiumStickersBuilder {
    fn as_ref(&self) -> &GetPremiumStickers {
        &self.inner
    }
}
