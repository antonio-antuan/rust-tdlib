use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the block list of a message sender. Currently, only users and supergroup chats can be blocked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetMessageSenderBlockList {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a message sender to block/unblock

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// New block list for the message sender; pass null to unblock the message sender

    #[serde(skip_serializing_if = "BlockList::_is_default")]
    block_list: BlockList,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetMessageSenderBlockList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetMessageSenderBlockList {}

impl SetMessageSenderBlockList {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetMessageSenderBlockListBuilder {
        let mut inner = SetMessageSenderBlockList::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setMessageSenderBlockList".to_string();

        SetMessageSenderBlockListBuilder { inner }
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn block_list(&self) -> &BlockList {
        &self.block_list
    }
}

#[doc(hidden)]
pub struct SetMessageSenderBlockListBuilder {
    inner: SetMessageSenderBlockList,
}

#[deprecated]
pub type RTDSetMessageSenderBlockListBuilder = SetMessageSenderBlockListBuilder;

impl SetMessageSenderBlockListBuilder {
    pub fn build(&self) -> SetMessageSenderBlockList {
        self.inner.clone()
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }

    pub fn block_list<T: AsRef<BlockList>>(&mut self, block_list: T) -> &mut Self {
        self.inner.block_list = block_list.as_ref().clone();
        self
    }
}

impl AsRef<SetMessageSenderBlockList> for SetMessageSenderBlockList {
    fn as_ref(&self) -> &SetMessageSenderBlockList {
        self
    }
}

impl AsRef<SetMessageSenderBlockList> for SetMessageSenderBlockListBuilder {
    fn as_ref(&self) -> &SetMessageSenderBlockList {
        &self.inner
    }
}
