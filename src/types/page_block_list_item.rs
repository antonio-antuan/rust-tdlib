use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an item of a list page block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockListItem {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Item label
    label: String,
    /// Item blocks
    page_blocks: Vec<PageBlock>,
}

impl RObject for PageBlockListItem {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PageBlockListItem {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockListItemBuilder {
        let mut inner = PageBlockListItem::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockListItemBuilder { inner }
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn page_blocks(&self) -> &Vec<PageBlock> {
        &self.page_blocks
    }
}

#[doc(hidden)]
pub struct RTDPageBlockListItemBuilder {
    inner: PageBlockListItem,
}

impl RTDPageBlockListItemBuilder {
    pub fn build(&self) -> PageBlockListItem {
        self.inner.clone()
    }

    pub fn label<T: AsRef<str>>(&mut self, label: T) -> &mut Self {
        self.inner.label = label.as_ref().to_string();
        self
    }

    pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
        self.inner.page_blocks = page_blocks;
        self
    }
}

impl AsRef<PageBlockListItem> for PageBlockListItem {
    fn as_ref(&self) -> &PageBlockListItem {
        self
    }
}

impl AsRef<PageBlockListItem> for RTDPageBlockListItemBuilder {
    fn as_ref(&self) -> &PageBlockListItem {
        &self.inner
    }
}
