use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns examples of premium stickers for demonstration purposes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPremiumStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPremiumStickersBuilder {
        let mut inner = GetPremiumStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPremiumStickers".to_string();

        RTDGetPremiumStickersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetPremiumStickersBuilder {
    inner: GetPremiumStickers,
}

impl RTDGetPremiumStickersBuilder {
    pub fn build(&self) -> GetPremiumStickers {
        self.inner.clone()
    }
}

impl AsRef<GetPremiumStickers> for GetPremiumStickers {
    fn as_ref(&self) -> &GetPremiumStickers {
        self
    }
}

impl AsRef<GetPremiumStickers> for RTDGetPremiumStickersBuilder {
    fn as_ref(&self) -> &GetPremiumStickers {
        &self.inner
    }
}
