use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks whether a name can be used for a new sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckStickerSetName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name to be checked

    #[serde(default)]
    name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckStickerSetName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckStickerSetNameResult for CheckStickerSetName {}

impl RFunction for CheckStickerSetName {}

impl CheckStickerSetName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckStickerSetNameBuilder {
        let mut inner = CheckStickerSetName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkStickerSetName".to_string();

        CheckStickerSetNameBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct CheckStickerSetNameBuilder {
    inner: CheckStickerSetName,
}

#[deprecated]
pub type RTDCheckStickerSetNameBuilder = CheckStickerSetNameBuilder;

impl CheckStickerSetNameBuilder {
    pub fn build(&self) -> CheckStickerSetName {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<CheckStickerSetName> for CheckStickerSetName {
    fn as_ref(&self) -> &CheckStickerSetName {
        self
    }
}

impl AsRef<CheckStickerSetName> for CheckStickerSetNameBuilder {
    fn as_ref(&self) -> &CheckStickerSetName {
        &self.inner
    }
}
