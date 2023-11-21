use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns viewers of a story. The method can be called only for stories posted on behalf of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStoryViewers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Story identifier

    #[serde(default)]
    story_id: i32,
    /// Query to search for in names and usernames of the viewers; may be empty to get all relevant viewers

    #[serde(default)]
    query: String,
    /// Pass true to get only contacts; pass false to get all relevant viewers

    #[serde(default)]
    only_contacts: bool,
    /// Pass true to get viewers with reaction first; pass false to get viewers sorted just by view_date

    #[serde(default)]
    prefer_with_reaction: bool,
    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results

    #[serde(default)]
    offset: String,
    /// The maximum number of story viewers to return

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStoryViewers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStoryViewers {}

impl GetStoryViewers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetStoryViewersBuilder {
        let mut inner = GetStoryViewers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStoryViewers".to_string();

        GetStoryViewersBuilder { inner }
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn only_contacts(&self) -> bool {
        self.only_contacts
    }

    pub fn prefer_with_reaction(&self) -> bool {
        self.prefer_with_reaction
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetStoryViewersBuilder {
    inner: GetStoryViewers,
}

#[deprecated]
pub type RTDGetStoryViewersBuilder = GetStoryViewersBuilder;

impl GetStoryViewersBuilder {
    pub fn build(&self) -> GetStoryViewers {
        self.inner.clone()
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn only_contacts(&mut self, only_contacts: bool) -> &mut Self {
        self.inner.only_contacts = only_contacts;
        self
    }

    pub fn prefer_with_reaction(&mut self, prefer_with_reaction: bool) -> &mut Self {
        self.inner.prefer_with_reaction = prefer_with_reaction;
        self
    }

    pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
        self.inner.offset = offset.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetStoryViewers> for GetStoryViewers {
    fn as_ref(&self) -> &GetStoryViewers {
        self
    }
}

impl AsRef<GetStoryViewers> for GetStoryViewersBuilder {
    fn as_ref(&self) -> &GetStoryViewers {
        &self.inner
    }
}
