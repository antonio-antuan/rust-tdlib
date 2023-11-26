use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns default list of custom emoji stickers for placing on a profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDefaultProfilePhotoCustomEmojiStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetDefaultProfilePhotoCustomEmojiStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetDefaultProfilePhotoCustomEmojiStickers {}

impl GetDefaultProfilePhotoCustomEmojiStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetDefaultProfilePhotoCustomEmojiStickersBuilder {
        let mut inner = GetDefaultProfilePhotoCustomEmojiStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getDefaultProfilePhotoCustomEmojiStickers".to_string();

        GetDefaultProfilePhotoCustomEmojiStickersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetDefaultProfilePhotoCustomEmojiStickersBuilder {
    inner: GetDefaultProfilePhotoCustomEmojiStickers,
}

#[deprecated]
pub type RTDGetDefaultProfilePhotoCustomEmojiStickersBuilder =
    GetDefaultProfilePhotoCustomEmojiStickersBuilder;

impl GetDefaultProfilePhotoCustomEmojiStickersBuilder {
    pub fn build(&self) -> GetDefaultProfilePhotoCustomEmojiStickers {
        self.inner.clone()
    }
}

impl AsRef<GetDefaultProfilePhotoCustomEmojiStickers>
    for GetDefaultProfilePhotoCustomEmojiStickers
{
    fn as_ref(&self) -> &GetDefaultProfilePhotoCustomEmojiStickers {
        self
    }
}

impl AsRef<GetDefaultProfilePhotoCustomEmojiStickers>
    for GetDefaultProfilePhotoCustomEmojiStickersBuilder
{
    fn as_ref(&self) -> &GetDefaultProfilePhotoCustomEmojiStickers {
        &self.inner
    }
}
