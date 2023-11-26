use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets a sticker set title; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStickerSetTitle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker set name

    #[serde(default)]
    name: String,
    /// New sticker set title

    #[serde(default)]
    title: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetStickerSetTitle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetStickerSetTitle {}

impl SetStickerSetTitle {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetStickerSetTitleBuilder {
        let mut inner = SetStickerSetTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setStickerSetTitle".to_string();

        SetStickerSetTitleBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct SetStickerSetTitleBuilder {
    inner: SetStickerSetTitle,
}

#[deprecated]
pub type RTDSetStickerSetTitleBuilder = SetStickerSetTitleBuilder;

impl SetStickerSetTitleBuilder {
    pub fn build(&self) -> SetStickerSetTitle {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<SetStickerSetTitle> for SetStickerSetTitle {
    fn as_ref(&self) -> &SetStickerSetTitle {
        self
    }
}

impl AsRef<SetStickerSetTitle> for SetStickerSetTitleBuilder {
    fn as_ref(&self) -> &SetStickerSetTitle {
        &self.inner
    }
}
