use super::Client;
use super::{observer::OBSERVER, tdlib_client::TdLibClient};
use crate::{
    errors::{RTDError, RTDResult},
    types::*,
};

const CLOSED_RECEIVER_ERROR: RTDError = RTDError::Internal("receiver already closed");
const INVALID_RESPONSE_ERROR: RTDError = RTDError::Internal("receive invalid response");
const NO_EXTRA: RTDError =
    RTDError::Internal("invalid tdlib response type, not have `extra` field");

impl<R> Client<R>
where
    R: TdLibClient + Clone,
{
    // Accepts an incoming call
    pub async fn accept_call<C: AsRef<AcceptCall>>(&self, accept_call: C) -> RTDResult<Ok> {
        let extra = accept_call.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, accept_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Accepts Telegram terms of services
    pub async fn accept_terms_of_service<C: AsRef<AcceptTermsOfService>>(
        &self,
        accept_terms_of_service: C,
    ) -> RTDResult<Ok> {
        let extra = accept_terms_of_service.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, accept_terms_of_service.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a new member to a chat. Members can't be added to private or secret chats
    pub async fn add_chat_member<C: AsRef<AddChatMember>>(
        &self,
        add_chat_member: C,
    ) -> RTDResult<Ok> {
        let extra = add_chat_member.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_chat_member.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds multiple new members to a chat. Currently, this method is only available for supergroups and channels. This method can't be used to join a chat. Members can't be added to a channel if it has more than 200 members
    pub async fn add_chat_members<C: AsRef<AddChatMembers>>(
        &self,
        add_chat_members: C,
    ) -> RTDResult<Ok> {
        let extra = add_chat_members.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_chat_members.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a chat to a chat list. A chat can't be simultaneously in Main and Archive chat lists, so it is automatically removed from another one if needed
    pub async fn add_chat_to_list<C: AsRef<AddChatToList>>(
        &self,
        add_chat_to_list: C,
    ) -> RTDResult<Ok> {
        let extra = add_chat_to_list.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_chat_to_list.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a user to the contact list or edits an existing contact by their user identifier
    pub async fn add_contact<C: AsRef<AddContact>>(&self, add_contact: C) -> RTDResult<Ok> {
        let extra = add_contact.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_contact.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization
    pub async fn add_custom_server_language_pack<C: AsRef<AddCustomServerLanguagePack>>(
        &self,
        add_custom_server_language_pack: C,
    ) -> RTDResult<Ok> {
        let extra = add_custom_server_language_pack
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            add_custom_server_language_pack.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
    pub async fn add_favorite_sticker<C: AsRef<AddFavoriteSticker>>(
        &self,
        add_favorite_sticker: C,
    ) -> RTDResult<Ok> {
        let extra = add_favorite_sticker.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_favorite_sticker.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message
    pub async fn add_local_message<C: AsRef<AddLocalMessage>>(
        &self,
        add_local_message: C,
    ) -> RTDResult<Message> {
        let extra = add_local_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_local_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a message to TDLib internal log. Can be called synchronously
    pub async fn add_log_message<C: AsRef<AddLogMessage>>(
        &self,
        add_log_message: C,
    ) -> RTDResult<Ok> {
        let extra = add_log_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_log_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds the specified data to data usage statistics. Can be called before authorization
    pub async fn add_network_statistics<C: AsRef<AddNetworkStatistics>>(
        &self,
        add_network_statistics: C,
    ) -> RTDResult<Ok> {
        let extra = add_network_statistics.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_network_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a proxy server for network requests. Can be called before authorization
    pub async fn add_proxy<C: AsRef<AddProxy>>(&self, add_proxy: C) -> RTDResult<Proxy> {
        let extra = add_proxy.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Proxy>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
    pub async fn add_recent_sticker<C: AsRef<AddRecentSticker>>(
        &self,
        add_recent_sticker: C,
    ) -> RTDResult<Stickers> {
        let extra = add_recent_sticker.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_recent_sticker.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Stickers>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first
    pub async fn add_recently_found_chat<C: AsRef<AddRecentlyFoundChat>>(
        &self,
        add_recently_found_chat: C,
    ) -> RTDResult<Ok> {
        let extra = add_recently_found_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_recently_found_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type "video/mp4" can be added to the list
    pub async fn add_saved_animation<C: AsRef<AddSavedAnimation>>(
        &self,
        add_saved_animation: C,
    ) -> RTDResult<Ok> {
        let extra = add_saved_animation.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_saved_animation.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds a new sticker to a set; for bots only. Returns the sticker set
    pub async fn add_sticker_to_set<C: AsRef<AddStickerToSet>>(
        &self,
        add_sticker_to_set: C,
    ) -> RTDResult<StickerSet> {
        let extra = add_sticker_to_set.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, add_sticker_to_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSet>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the result of a callback query; for bots only
    pub async fn answer_callback_query<C: AsRef<AnswerCallbackQuery>>(
        &self,
        answer_callback_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_callback_query.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, answer_callback_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Answers a custom query; for bots only
    pub async fn answer_custom_query<C: AsRef<AnswerCustomQuery>>(
        &self,
        answer_custom_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_custom_query.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, answer_custom_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the result of an inline query; for bots only
    pub async fn answer_inline_query<C: AsRef<AnswerInlineQuery>>(
        &self,
        answer_inline_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_inline_query.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, answer_inline_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the result of a pre-checkout query; for bots only
    pub async fn answer_pre_checkout_query<C: AsRef<AnswerPreCheckoutQuery>>(
        &self,
        answer_pre_checkout_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_pre_checkout_query.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, answer_pre_checkout_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the result of a shipping query; for bots only
    pub async fn answer_shipping_query<C: AsRef<AnswerShippingQuery>>(
        &self,
        answer_shipping_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_shipping_query.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, answer_shipping_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Bans a member in a chat. Members can't be banned in private or secret chats. In supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first
    pub async fn ban_chat_member<C: AsRef<BanChatMember>>(
        &self,
        ban_chat_member: C,
    ) -> RTDResult<Ok> {
        let extra = ban_chat_member.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, ban_chat_member.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Blocks an original sender of a message in the Replies chat
    pub async fn block_message_sender_from_replies<C: AsRef<BlockMessageSenderFromReplies>>(
        &self,
        block_message_sender_from_replies: C,
    ) -> RTDResult<Ok> {
        let extra = block_message_sender_from_replies
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            block_message_sender_from_replies.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks whether the current session can be used to transfer a chat ownership to another user
    pub async fn can_transfer_ownership<C: AsRef<CanTransferOwnership>>(
        &self,
        can_transfer_ownership: C,
    ) -> RTDResult<CanTransferOwnershipResult> {
        let extra = can_transfer_ownership.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, can_transfer_ownership.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<CanTransferOwnershipResult>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Stops the downloading of a file. If a file has already been downloaded, does nothing
    pub async fn cancel_download_file<C: AsRef<CancelDownloadFile>>(
        &self,
        cancel_download_file: C,
    ) -> RTDResult<Ok> {
        let extra = cancel_download_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, cancel_download_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Cancels reset of 2-step verification password. The method can be called if passwordState.pending_reset_date > 0
    pub async fn cancel_password_reset<C: AsRef<CancelPasswordReset>>(
        &self,
        cancel_password_reset: C,
    ) -> RTDResult<Ok> {
        let extra = cancel_password_reset.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, cancel_password_reset.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Stops the uploading of a file. Supported only for files uploaded by using uploadFile. For other files the behavior is undefined
    pub async fn cancel_upload_file<C: AsRef<CancelUploadFile>>(
        &self,
        cancel_upload_file: C,
    ) -> RTDResult<Ok> {
        let extra = cancel_upload_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, cancel_upload_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes imported contacts using the list of contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time
    pub async fn change_imported_contacts<C: AsRef<ChangeImportedContacts>>(
        &self,
        change_imported_contacts: C,
    ) -> RTDResult<ImportedContacts> {
        let extra = change_imported_contacts.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, change_imported_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ImportedContacts>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code
    pub async fn change_phone_number<C: AsRef<ChangePhoneNumber>>(
        &self,
        change_phone_number: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = change_phone_number.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, change_phone_number.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Installs/uninstalls or activates/archives a sticker set
    pub async fn change_sticker_set<C: AsRef<ChangeStickerSet>>(
        &self,
        change_sticker_set: C,
    ) -> RTDResult<Ok> {
        let extra = change_sticker_set.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, change_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in
    pub async fn check_authentication_bot_token<C: AsRef<CheckAuthenticationBotToken>>(
        &self,
        check_authentication_bot_token: C,
    ) -> RTDResult<Ok> {
        let extra = check_authentication_bot_token
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_authentication_bot_token.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode
    pub async fn check_authentication_code<C: AsRef<CheckAuthenticationCode>>(
        &self,
        check_authentication_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_authentication_code.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, check_authentication_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the authentication password for correctness. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn check_authentication_password<C: AsRef<CheckAuthenticationPassword>>(
        &self,
        check_authentication_password: C,
    ) -> RTDResult<Ok> {
        let extra = check_authentication_password
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_authentication_password.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks whether a password recovery code sent to an email address is valid. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn check_authentication_password_recovery_code<
        C: AsRef<CheckAuthenticationPasswordRecoveryCode>,
    >(
        &self,
        check_authentication_password_recovery_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_authentication_password_recovery_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_authentication_password_recovery_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the authentication code sent to confirm a new phone number of the user
    pub async fn check_change_phone_number_code<C: AsRef<CheckChangePhoneNumberCode>>(
        &self,
        check_change_phone_number_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_change_phone_number_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_change_phone_number_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the validity of an invite link for a chat and returns information about the corresponding chat
    pub async fn check_chat_invite_link<C: AsRef<CheckChatInviteLink>>(
        &self,
        check_chat_invite_link: C,
    ) -> RTDResult<ChatInviteLinkInfo> {
        let extra = check_chat_invite_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, check_chat_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLinkInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks whether a username can be set for a chat
    pub async fn check_chat_username<C: AsRef<CheckChatUsername>>(
        &self,
        check_chat_username: C,
    ) -> RTDResult<CheckChatUsernameResult> {
        let extra = check_chat_username.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, check_chat_username.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<CheckChatUsernameResult>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached
    pub async fn check_created_public_chats_limit<C: AsRef<CheckCreatedPublicChatsLimit>>(
        &self,
        check_created_public_chats_limit: C,
    ) -> RTDResult<Ok> {
        let extra = check_created_public_chats_limit
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_created_public_chats_limit.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the database encryption key for correctness. Works only when the current authorization state is authorizationStateWaitEncryptionKey
    pub async fn check_database_encryption_key<C: AsRef<CheckDatabaseEncryptionKey>>(
        &self,
        check_database_encryption_key: C,
    ) -> RTDResult<Ok> {
        let extra = check_database_encryption_key
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_database_encryption_key.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the email address verification code for Telegram Passport
    pub async fn check_email_address_verification_code<
        C: AsRef<CheckEmailAddressVerificationCode>,
    >(
        &self,
        check_email_address_verification_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_email_address_verification_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_email_address_verification_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks whether a 2-step verification password recovery code sent to an email address is valid
    pub async fn check_password_recovery_code<C: AsRef<CheckPasswordRecoveryCode>>(
        &self,
        check_password_recovery_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_password_recovery_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, check_password_recovery_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks phone number confirmation code
    pub async fn check_phone_number_confirmation_code<
        C: AsRef<CheckPhoneNumberConfirmationCode>,
    >(
        &self,
        check_phone_number_confirmation_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_phone_number_confirmation_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_phone_number_confirmation_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the phone number verification code for Telegram Passport
    pub async fn check_phone_number_verification_code<
        C: AsRef<CheckPhoneNumberVerificationCode>,
    >(
        &self,
        check_phone_number_verification_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_phone_number_verification_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_phone_number_verification_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks the 2-step verification recovery email address verification code
    pub async fn check_recovery_email_address_code<C: AsRef<CheckRecoveryEmailAddressCode>>(
        &self,
        check_recovery_email_address_code: C,
    ) -> RTDResult<PasswordState> {
        let extra = check_recovery_email_address_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            check_recovery_email_address_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PasswordState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Checks whether a name can be used for a new sticker set
    pub async fn check_sticker_set_name<C: AsRef<CheckStickerSetName>>(
        &self,
        check_sticker_set_name: C,
    ) -> RTDResult<CheckStickerSetNameResult> {
        let extra = check_sticker_set_name.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, check_sticker_set_name.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<CheckStickerSetNameResult>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. Can be called synchronously
    pub async fn clean_file_name<C: AsRef<CleanFileName>>(
        &self,
        clean_file_name: C,
    ) -> RTDResult<Text> {
        let extra = clean_file_name.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, clean_file_name.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Clears draft messages in all chats
    pub async fn clear_all_draft_messages<C: AsRef<ClearAllDraftMessages>>(
        &self,
        clear_all_draft_messages: C,
    ) -> RTDResult<Ok> {
        let extra = clear_all_draft_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, clear_all_draft_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Clears all imported contacts, contact list remains unchanged
    pub async fn clear_imported_contacts<C: AsRef<ClearImportedContacts>>(
        &self,
        clear_imported_contacts: C,
    ) -> RTDResult<Ok> {
        let extra = clear_imported_contacts.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, clear_imported_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Clears the list of recently used stickers
    pub async fn clear_recent_stickers<C: AsRef<ClearRecentStickers>>(
        &self,
        clear_recent_stickers: C,
    ) -> RTDResult<Ok> {
        let extra = clear_recent_stickers.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, clear_recent_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Clears the list of recently found chats
    pub async fn clear_recently_found_chats<C: AsRef<ClearRecentlyFoundChats>>(
        &self,
        clear_recently_found_chats: C,
    ) -> RTDResult<Ok> {
        let extra = clear_recently_found_chats
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, clear_recently_found_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs TDLib that a message with an animated emoji was clicked by the user. Returns a big animated sticker to be played or a 404 error if usual animation needs to be played
    pub async fn click_animated_emoji_message<C: AsRef<ClickAnimatedEmojiMessage>>(
        &self,
        click_animated_emoji_message: C,
    ) -> RTDResult<Sticker> {
        let extra = click_animated_emoji_message
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, click_animated_emoji_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Sticker>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, updateAuthorizationState with authorizationStateClosed will be sent. Can be called before initialization
    pub async fn close<C: AsRef<Close>>(&self, close: C) -> RTDResult<Ok> {
        let extra = close.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, close.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed
    pub async fn close_chat<C: AsRef<CloseChat>>(&self, close_chat: C) -> RTDResult<Ok> {
        let extra = close_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, close_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Closes a secret chat, effectively transferring its state to secretChatStateClosed
    pub async fn close_secret_chat<C: AsRef<CloseSecretChat>>(
        &self,
        close_secret_chat: C,
    ) -> RTDResult<Ok> {
        let extra = close_secret_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, close_secret_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Confirms QR code authentication on another device. Returns created session on success
    pub async fn confirm_qr_code_authentication<C: AsRef<ConfirmQrCodeAuthentication>>(
        &self,
        confirm_qr_code_authentication: C,
    ) -> RTDResult<Session> {
        let extra = confirm_qr_code_authentication
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            confirm_qr_code_authentication.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Session>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an existing chat corresponding to a known basic group
    pub async fn create_basic_group_chat<C: AsRef<CreateBasicGroupChat>>(
        &self,
        create_basic_group_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_basic_group_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_basic_group_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a new call
    pub async fn create_call<C: AsRef<CreateCall>>(&self, create_call: C) -> RTDResult<CallId> {
        let extra = create_call.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<CallId>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates new chat filter. Returns information about the created chat filter
    pub async fn create_chat_filter<C: AsRef<CreateChatFilter>>(
        &self,
        create_chat_filter: C,
    ) -> RTDResult<ChatFilterInfo> {
        let extra = create_chat_filter.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_chat_filter.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatFilterInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a new invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat
    pub async fn create_chat_invite_link<C: AsRef<CreateChatInviteLink>>(
        &self,
        create_chat_invite_link: C,
    ) -> RTDResult<ChatInviteLink> {
        let extra = create_chat_invite_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_chat_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLink>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns the newly created chat
    pub async fn create_new_basic_group_chat<C: AsRef<CreateNewBasicGroupChat>>(
        &self,
        create_new_basic_group_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_new_basic_group_chat
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_new_basic_group_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a new secret chat. Returns the newly created chat
    pub async fn create_new_secret_chat<C: AsRef<CreateNewSecretChat>>(
        &self,
        create_new_secret_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_new_secret_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_new_secret_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a new sticker set. Returns the newly created sticker set
    pub async fn create_new_sticker_set<C: AsRef<CreateNewStickerSet>>(
        &self,
        create_new_sticker_set: C,
    ) -> RTDResult<StickerSet> {
        let extra = create_new_sticker_set.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_new_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSet>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
    pub async fn create_new_supergroup_chat<C: AsRef<CreateNewSupergroupChat>>(
        &self,
        create_new_supergroup_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_new_supergroup_chat
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_new_supergroup_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an existing chat corresponding to a given user
    pub async fn create_private_chat<C: AsRef<CreatePrivateChat>>(
        &self,
        create_private_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_private_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_private_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an existing chat corresponding to a known secret chat
    pub async fn create_secret_chat<C: AsRef<CreateSecretChat>>(
        &self,
        create_secret_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_secret_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_secret_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an existing chat corresponding to a known supergroup or channel
    pub async fn create_supergroup_chat<C: AsRef<CreateSupergroupChat>>(
        &self,
        create_supergroup_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_supergroup_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_supergroup_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a new temporary password for processing payments
    pub async fn create_temporary_password<C: AsRef<CreateTemporaryPassword>>(
        &self,
        create_temporary_password: C,
    ) -> RTDResult<TemporaryPasswordState> {
        let extra = create_temporary_password.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_temporary_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TemporaryPasswordState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a video chat (a group call bound to a chat). Available only for basic groups, supergroups and channels; requires can_manage_video_chats rights
    pub async fn create_video_chat<C: AsRef<CreateVideoChat>>(
        &self,
        create_video_chat: C,
    ) -> RTDResult<GroupCallId> {
        let extra = create_video_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, create_video_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<GroupCallId>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is authorizationStateWaitPassword
    pub async fn delete_account<C: AsRef<DeleteAccount>>(
        &self,
        delete_account: C,
    ) -> RTDResult<Ok> {
        let extra = delete_account.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_account.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes all call messages
    pub async fn delete_all_call_messages<C: AsRef<DeleteAllCallMessages>>(
        &self,
        delete_all_call_messages: C,
    ) -> RTDResult<Ok> {
        let extra = delete_all_call_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_all_call_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes all revoked chat invite links created by a given chat administrator. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
    pub async fn delete_all_revoked_chat_invite_links<C: AsRef<DeleteAllRevokedChatInviteLinks>>(
        &self,
        delete_all_revoked_chat_invite_links: C,
    ) -> RTDResult<Ok> {
        let extra = delete_all_revoked_chat_invite_links
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            delete_all_revoked_chat_invite_links.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes a chat along with all messages in the corresponding chat for all chat members; requires owner privileges. For group chats this will release the username and remove all members. Chats with more than 1000 members can't be deleted using this method
    pub async fn delete_chat<C: AsRef<DeleteChat>>(&self, delete_chat: C) -> RTDResult<Ok> {
        let extra = delete_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes existing chat filter
    pub async fn delete_chat_filter<C: AsRef<DeleteChatFilter>>(
        &self,
        delete_chat_filter: C,
    ) -> RTDResult<Ok> {
        let extra = delete_chat_filter.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_chat_filter.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes all messages in the chat. Use chat.can_be_deleted_only_for_self and chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat
    pub async fn delete_chat_history<C: AsRef<DeleteChatHistory>>(
        &self,
        delete_chat_history: C,
    ) -> RTDResult<Ok> {
        let extra = delete_chat_history.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_chat_history.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes all messages between the specified dates in a chat. Supported only for private chats and basic groups. Messages sent in the last 30 seconds will not be deleted
    pub async fn delete_chat_messages_by_date<C: AsRef<DeleteChatMessagesByDate>>(
        &self,
        delete_chat_messages_by_date: C,
    ) -> RTDResult<Ok> {
        let extra = delete_chat_messages_by_date
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_chat_messages_by_date.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes all messages sent by the specified message sender in a chat. Supported only for supergroups; requires can_delete_messages administrator privileges
    pub async fn delete_chat_messages_by_sender<C: AsRef<DeleteChatMessagesBySender>>(
        &self,
        delete_chat_messages_by_sender: C,
    ) -> RTDResult<Ok> {
        let extra = delete_chat_messages_by_sender
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            delete_chat_messages_by_sender.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup is changed
    pub async fn delete_chat_reply_markup<C: AsRef<DeleteChatReplyMarkup>>(
        &self,
        delete_chat_reply_markup: C,
    ) -> RTDResult<Ok> {
        let extra = delete_chat_reply_markup.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_chat_reply_markup.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes commands supported by the bot for the given user scope and language; for bots only
    pub async fn delete_commands<C: AsRef<DeleteCommands>>(
        &self,
        delete_commands: C,
    ) -> RTDResult<Ok> {
        let extra = delete_commands.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_commands.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes a file from the TDLib file cache
    pub async fn delete_file<C: AsRef<DeleteFile>>(&self, delete_file: C) -> RTDResult<Ok> {
        let extra = delete_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization
    pub async fn delete_language_pack<C: AsRef<DeleteLanguagePack>>(
        &self,
        delete_language_pack: C,
    ) -> RTDResult<Ok> {
        let extra = delete_language_pack.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_language_pack.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes messages
    pub async fn delete_messages<C: AsRef<DeleteMessages>>(
        &self,
        delete_messages: C,
    ) -> RTDResult<Ok> {
        let extra = delete_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes a Telegram Passport element
    pub async fn delete_passport_element<C: AsRef<DeletePassportElement>>(
        &self,
        delete_passport_element: C,
    ) -> RTDResult<Ok> {
        let extra = delete_passport_element.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_passport_element.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes a profile photo
    pub async fn delete_profile_photo<C: AsRef<DeleteProfilePhoto>>(
        &self,
        delete_profile_photo: C,
    ) -> RTDResult<Ok> {
        let extra = delete_profile_photo.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_profile_photo.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes revoked chat invite links. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
    pub async fn delete_revoked_chat_invite_link<C: AsRef<DeleteRevokedChatInviteLink>>(
        &self,
        delete_revoked_chat_invite_link: C,
    ) -> RTDResult<Ok> {
        let extra = delete_revoked_chat_invite_link
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            delete_revoked_chat_invite_link.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes saved credentials for all payment provider bots
    pub async fn delete_saved_credentials<C: AsRef<DeleteSavedCredentials>>(
        &self,
        delete_saved_credentials: C,
    ) -> RTDResult<Ok> {
        let extra = delete_saved_credentials.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_saved_credentials.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Deletes saved order info
    pub async fn delete_saved_order_info<C: AsRef<DeleteSavedOrderInfo>>(
        &self,
        delete_saved_order_info: C,
    ) -> RTDResult<Ok> {
        let extra = delete_saved_order_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, delete_saved_order_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent. Can be called before authorization
    pub async fn destroy<C: AsRef<Destroy>>(&self, destroy: C) -> RTDResult<Ok> {
        let extra = destroy.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, destroy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Disables the currently enabled proxy. Can be called before authorization
    pub async fn disable_proxy<C: AsRef<DisableProxy>>(&self, disable_proxy: C) -> RTDResult<Ok> {
        let extra = disable_proxy.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, disable_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Discards a call
    pub async fn discard_call<C: AsRef<DiscardCall>>(&self, discard_call: C) -> RTDResult<Ok> {
        let extra = discard_call.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, discard_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Disconnects all websites from the current user's Telegram account
    pub async fn disconnect_all_websites<C: AsRef<DisconnectAllWebsites>>(
        &self,
        disconnect_all_websites: C,
    ) -> RTDResult<Ok> {
        let extra = disconnect_all_websites.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, disconnect_all_websites.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Disconnects website from the current user's Telegram account
    pub async fn disconnect_website<C: AsRef<DisconnectWebsite>>(
        &self,
        disconnect_website: C,
    ) -> RTDResult<Ok> {
        let extra = disconnect_website.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, disconnect_website.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates
    pub async fn download_file<C: AsRef<DownloadFile>>(&self, download_file: C) -> RTDResult<File> {
        let extra = download_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, download_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<File>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits existing chat filter. Returns information about the edited chat filter
    pub async fn edit_chat_filter<C: AsRef<EditChatFilter>>(
        &self,
        edit_chat_filter: C,
    ) -> RTDResult<ChatFilterInfo> {
        let extra = edit_chat_filter.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_chat_filter.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatFilterInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits a non-primary invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
    pub async fn edit_chat_invite_link<C: AsRef<EditChatInviteLink>>(
        &self,
        edit_chat_invite_link: C,
    ) -> RTDResult<ChatInviteLink> {
        let extra = edit_chat_invite_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_chat_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLink>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits information about a custom local language pack in the current localization target. Can be called before authorization
    pub async fn edit_custom_language_pack_info<C: AsRef<EditCustomLanguagePackInfo>>(
        &self,
        edit_custom_language_pack_info: C,
    ) -> RTDResult<Ok> {
        let extra = edit_custom_language_pack_info
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            edit_custom_language_pack_info.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the caption of an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_caption<C: AsRef<EditInlineMessageCaption>>(
        &self,
        edit_inline_message_caption: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_caption
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_inline_message_caption.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the content of a live location in an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_live_location<C: AsRef<EditInlineMessageLiveLocation>>(
        &self,
        edit_inline_message_live_location: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_live_location
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            edit_inline_message_live_location.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_media<C: AsRef<EditInlineMessageMedia>>(
        &self,
        edit_inline_message_media: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_media.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_inline_message_media.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the reply markup of an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_reply_markup<C: AsRef<EditInlineMessageReplyMarkup>>(
        &self,
        edit_inline_message_reply_markup: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_reply_markup
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            edit_inline_message_reply_markup.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the text of an inline text or game message sent via a bot; for bots only
    pub async fn edit_inline_message_text<C: AsRef<EditInlineMessageText>>(
        &self,
        edit_inline_message_text: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_text.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_inline_message_text.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the message content caption. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_caption<C: AsRef<EditMessageCaption>>(
        &self,
        edit_message_caption: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_caption.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_message_caption.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_live_location<C: AsRef<EditMessageLiveLocation>>(
        &self,
        edit_message_live_location: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_live_location
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_message_live_location.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the content of a message with an animation, an audio, a document, a photo or a video, including message caption. If only the caption needs to be edited, use editMessageCaption instead. The media can't be edited if the message was set to self-destruct or to a self-destructing media. The type of message content in an album can't be changed with exception of replacing a photo with a video or vice versa. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_media<C: AsRef<EditMessageMedia>>(
        &self,
        edit_message_media: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_media.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_message_media.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_reply_markup<C: AsRef<EditMessageReplyMarkup>>(
        &self,
        edit_message_reply_markup: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_reply_markup.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_message_reply_markup.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed
    pub async fn edit_message_scheduling_state<C: AsRef<EditMessageSchedulingState>>(
        &self,
        edit_message_scheduling_state: C,
    ) -> RTDResult<Ok> {
        let extra = edit_message_scheduling_state
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            edit_message_scheduling_state.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_text<C: AsRef<EditMessageText>>(
        &self,
        edit_message_text: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_text.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_message_text.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Edits an existing proxy server for network requests. Can be called before authorization
    pub async fn edit_proxy<C: AsRef<EditProxy>>(&self, edit_proxy: C) -> RTDResult<Proxy> {
        let extra = edit_proxy.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, edit_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Proxy>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization
    pub async fn enable_proxy<C: AsRef<EnableProxy>>(&self, enable_proxy: C) -> RTDResult<Ok> {
        let extra = enable_proxy.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, enable_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Ends a group call. Requires groupCall.can_be_managed
    pub async fn end_group_call<C: AsRef<EndGroupCall>>(&self, end_group_call: C) -> RTDResult<Ok> {
        let extra = end_group_call.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, end_group_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Ends recording of an active group call. Requires groupCall.can_be_managed group call flag
    pub async fn end_group_call_recording<C: AsRef<EndGroupCallRecording>>(
        &self,
        end_group_call_recording: C,
    ) -> RTDResult<Ok> {
        let extra = end_group_call_recording.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, end_group_call_recording.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Ends screen sharing in a joined group call
    pub async fn end_group_call_screen_sharing<C: AsRef<EndGroupCallScreenSharing>>(
        &self,
        end_group_call_screen_sharing: C,
    ) -> RTDResult<Ok> {
        let extra = end_group_call_screen_sharing
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            end_group_call_screen_sharing.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Finishes the file generation
    pub async fn finish_file_generation<C: AsRef<FinishFileGeneration>>(
        &self,
        finish_file_generation: C,
    ) -> RTDResult<Ok> {
        let extra = finish_file_generation.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, finish_file_generation.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message
    pub async fn forward_messages<C: AsRef<ForwardMessages>>(
        &self,
        forward_messages: C,
    ) -> RTDResult<Messages> {
        let extra = forward_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, forward_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the period of inactivity after which the account of the current user will automatically be deleted
    pub async fn get_account_ttl<C: AsRef<GetAccountTtl>>(
        &self,
        get_account_ttl: C,
    ) -> RTDResult<AccountTtl> {
        let extra = get_account_ttl.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_account_ttl.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AccountTtl>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns all active live locations that need to be updated by the application. The list is persistent across application restarts only if the message database is used
    pub async fn get_active_live_location_messages<C: AsRef<GetActiveLiveLocationMessages>>(
        &self,
        get_active_live_location_messages: C,
    ) -> RTDResult<Messages> {
        let extra = get_active_live_location_messages
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_active_live_location_messages.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns all active sessions of the current user
    pub async fn get_active_sessions<C: AsRef<GetActiveSessions>>(
        &self,
        get_active_sessions: C,
    ) -> RTDResult<Sessions> {
        let extra = get_active_sessions.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_active_sessions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Sessions>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns all available Telegram Passport elements
    pub async fn get_all_passport_elements<C: AsRef<GetAllPassportElements>>(
        &self,
        get_all_passport_elements: C,
    ) -> RTDResult<PassportElements> {
        let extra = get_all_passport_elements.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_all_passport_elements.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PassportElements>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an animated emoji corresponding to a given emoji. Returns a 404 error if the emoji has no animated emoji
    pub async fn get_animated_emoji<C: AsRef<GetAnimatedEmoji>>(
        &self,
        get_animated_emoji: C,
    ) -> RTDResult<AnimatedEmoji> {
        let extra = get_animated_emoji.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_animated_emoji.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AnimatedEmoji>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns application config, provided by the server. Can be called before authorization
    pub async fn get_application_config<C: AsRef<GetApplicationConfig>>(
        &self,
        get_application_config: C,
    ) -> RTDResult<JsonValue> {
        let extra = get_application_config.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_application_config.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<JsonValue>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the link for downloading official Telegram application to be used when the current user invites friends to Telegram
    pub async fn get_application_download_link<C: AsRef<GetApplicationDownloadLink>>(
        &self,
        get_application_download_link: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_application_download_link
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_application_download_link.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<HttpUrl>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of archived sticker sets
    pub async fn get_archived_sticker_sets<C: AsRef<GetArchivedStickerSets>>(
        &self,
        get_archived_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = get_archived_sticker_sets.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_archived_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSets>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of sticker sets attached to a file. Currently, only photos and videos can have attached sticker sets
    pub async fn get_attached_sticker_sets<C: AsRef<GetAttachedStickerSets>>(
        &self,
        get_attached_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = get_attached_sticker_sets.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_attached_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSets>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state. Can be called before initialization
    pub async fn get_authorization_state<C: AsRef<GetAuthorizationState>>(
        &self,
        get_authorization_state: C,
    ) -> RTDResult<AuthorizationState> {
        let extra = get_authorization_state.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_authorization_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AuthorizationState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns auto-download settings presets for the current user
    pub async fn get_auto_download_settings_presets<C: AsRef<GetAutoDownloadSettingsPresets>>(
        &self,
        get_auto_download_settings_presets: C,
    ) -> RTDResult<AutoDownloadSettingsPresets> {
        let extra = get_auto_download_settings_presets
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_auto_download_settings_presets.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AutoDownloadSettingsPresets>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Constructs a persistent HTTP URL for a background
    pub async fn get_background_url<C: AsRef<GetBackgroundUrl>>(
        &self,
        get_background_url: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_background_url.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_background_url.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<HttpUrl>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns backgrounds installed by the user
    pub async fn get_backgrounds<C: AsRef<GetBackgrounds>>(
        &self,
        get_backgrounds: C,
    ) -> RTDResult<Backgrounds> {
        let extra = get_backgrounds.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_backgrounds.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Backgrounds>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a bank card
    pub async fn get_bank_card_info<C: AsRef<GetBankCardInfo>>(
        &self,
        get_bank_card_info: C,
    ) -> RTDResult<BankCardInfo> {
        let extra = get_bank_card_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_bank_card_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<BankCardInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot
    pub async fn get_basic_group<C: AsRef<GetBasicGroup>>(
        &self,
        get_basic_group: C,
    ) -> RTDResult<BasicGroup> {
        let extra = get_basic_group.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_basic_group.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<BasicGroup>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns full information about a basic group by its identifier
    pub async fn get_basic_group_full_info<C: AsRef<GetBasicGroupFullInfo>>(
        &self,
        get_basic_group_full_info: C,
    ) -> RTDResult<BasicGroupFullInfo> {
        let extra = get_basic_group_full_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_basic_group_full_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<BasicGroupFullInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns users and chats that were blocked by the current user
    pub async fn get_blocked_message_senders<C: AsRef<GetBlockedMessageSenders>>(
        &self,
        get_blocked_message_senders: C,
    ) -> RTDResult<MessageSenders> {
        let extra = get_blocked_message_senders
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_blocked_message_senders.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageSenders>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
    pub async fn get_callback_query_answer<C: AsRef<GetCallbackQueryAnswer>>(
        &self,
        get_callback_query_answer: C,
    ) -> RTDResult<CallbackQueryAnswer> {
        let extra = get_callback_query_answer.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_callback_query_answer.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<CallbackQueryAnswer>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a message with the callback button that originated a callback query; for bots only
    pub async fn get_callback_query_message<C: AsRef<GetCallbackQueryMessage>>(
        &self,
        get_callback_query_message: C,
    ) -> RTDResult<Message> {
        let extra = get_callback_query_message
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_callback_query_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a chat by its identifier, this is an offline request if the current user is not a bot
    pub async fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> RTDResult<Chat> {
        let extra = get_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of administrators of the chat with their custom titles
    pub async fn get_chat_administrators<C: AsRef<GetChatAdministrators>>(
        &self,
        get_chat_administrators: C,
    ) -> RTDResult<ChatAdministrators> {
        let extra = get_chat_administrators.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_administrators.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatAdministrators>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns list of message sender identifiers, which can be used to send messages in a chat
    pub async fn get_chat_available_message_senders<C: AsRef<GetChatAvailableMessageSenders>>(
        &self,
        get_chat_available_message_senders: C,
    ) -> RTDResult<MessageSenders> {
        let extra = get_chat_available_message_senders
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_chat_available_message_senders.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageSenders>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only for supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id)
    pub async fn get_chat_event_log<C: AsRef<GetChatEventLog>>(
        &self,
        get_chat_event_log: C,
    ) -> RTDResult<ChatEvents> {
        let extra = get_chat_event_log.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_event_log.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatEvents>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a chat filter by its identifier
    pub async fn get_chat_filter<C: AsRef<GetChatFilter>>(
        &self,
        get_chat_filter: C,
    ) -> RTDResult<ChatFilter> {
        let extra = get_chat_filter.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_filter.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatFilter>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns default icon name for a filter. Can be called synchronously
    pub async fn get_chat_filter_default_icon_name<C: AsRef<GetChatFilterDefaultIconName>>(
        &self,
        get_chat_filter_default_icon_name: C,
    ) -> RTDResult<Text> {
        let extra = get_chat_filter_default_icon_name
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_chat_filter_default_icon_name.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib. This is an offline request if only_local is true
    pub async fn get_chat_history<C: AsRef<GetChatHistory>>(
        &self,
        get_chat_history: C,
    ) -> RTDResult<Messages> {
        let extra = get_chat_history.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_history.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about an invite link. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
    pub async fn get_chat_invite_link<C: AsRef<GetChatInviteLink>>(
        &self,
        get_chat_invite_link: C,
    ) -> RTDResult<ChatInviteLink> {
        let extra = get_chat_invite_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLink>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns list of chat administrators with number of their invite links. Requires owner privileges in the chat
    pub async fn get_chat_invite_link_counts<C: AsRef<GetChatInviteLinkCounts>>(
        &self,
        get_chat_invite_link_counts: C,
    ) -> RTDResult<ChatInviteLinkCounts> {
        let extra = get_chat_invite_link_counts
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_invite_link_counts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLinkCounts>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns chat members joined a chat via an invite link. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
    pub async fn get_chat_invite_link_members<C: AsRef<GetChatInviteLinkMembers>>(
        &self,
        get_chat_invite_link_members: C,
    ) -> RTDResult<ChatInviteLinkMembers> {
        let extra = get_chat_invite_link_members
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_invite_link_members.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLinkMembers>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns invite links for a chat created by specified administrator. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
    pub async fn get_chat_invite_links<C: AsRef<GetChatInviteLinks>>(
        &self,
        get_chat_invite_links: C,
    ) -> RTDResult<ChatInviteLinks> {
        let extra = get_chat_invite_links.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_invite_links.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLinks>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns pending join requests in a chat
    pub async fn get_chat_join_requests<C: AsRef<GetChatJoinRequests>>(
        &self,
        get_chat_join_requests: C,
    ) -> RTDResult<ChatJoinRequests> {
        let extra = get_chat_join_requests.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_join_requests.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatJoinRequests>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns chat lists to which the chat can be added. This is an offline request
    pub async fn get_chat_lists_to_add_chat<C: AsRef<GetChatListsToAddChat>>(
        &self,
        get_chat_lists_to_add_chat: C,
    ) -> RTDResult<ChatLists> {
        let extra = get_chat_lists_to_add_chat
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_lists_to_add_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatLists>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a single member of a chat
    pub async fn get_chat_member<C: AsRef<GetChatMember>>(
        &self,
        get_chat_member: C,
    ) -> RTDResult<ChatMember> {
        let extra = get_chat_member.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_member.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatMember>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the last message sent in a chat no later than the specified date
    pub async fn get_chat_message_by_date<C: AsRef<GetChatMessageByDate>>(
        &self,
        get_chat_message_by_date: C,
    ) -> RTDResult<Message> {
        let extra = get_chat_message_by_date.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_message_by_date.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about the next messages of the specified type in the chat split by days. Returns the results in reverse chronological order. Can return partial result for the last returned day. Behavior of this method depends on the value of the option "utc_time_offset"
    pub async fn get_chat_message_calendar<C: AsRef<GetChatMessageCalendar>>(
        &self,
        get_chat_message_calendar: C,
    ) -> RTDResult<MessageCalendar> {
        let extra = get_chat_message_calendar.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_message_calendar.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageCalendar>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns approximate number of messages of the specified type in the chat
    pub async fn get_chat_message_count<C: AsRef<GetChatMessageCount>>(
        &self,
        get_chat_message_count: C,
    ) -> RTDResult<Count> {
        let extra = get_chat_message_count.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_message_count.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Count>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns list of chats with non-default notification settings
    pub async fn get_chat_notification_settings_exceptions<
        C: AsRef<GetChatNotificationSettingsExceptions>,
    >(
        &self,
        get_chat_notification_settings_exceptions: C,
    ) -> RTDResult<Chats> {
        let extra = get_chat_notification_settings_exceptions
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_chat_notification_settings_exceptions.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a newest pinned message in the chat
    pub async fn get_chat_pinned_message<C: AsRef<GetChatPinnedMessage>>(
        &self,
        get_chat_pinned_message: C,
    ) -> RTDResult<Message> {
        let extra = get_chat_pinned_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_pinned_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns all scheduled messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id)
    pub async fn get_chat_scheduled_messages<C: AsRef<GetChatScheduledMessages>>(
        &self,
        get_chat_scheduled_messages: C,
    ) -> RTDResult<Messages> {
        let extra = get_chat_scheduled_messages
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_scheduled_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns sparse positions of messages of the specified type in the chat to be used for shared media scroll implementation. Returns the results in reverse chronological order (i.e., in order of decreasing message_id). Cannot be used in secret chats or with searchMessagesFilterFailedToSend filter without an enabled message database
    pub async fn get_chat_sparse_message_positions<C: AsRef<GetChatSparseMessagePositions>>(
        &self,
        get_chat_sparse_message_positions: C,
    ) -> RTDResult<MessagePositions> {
        let extra = get_chat_sparse_message_positions
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_chat_sparse_message_positions.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessagePositions>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns sponsored message to be shown in a chat; for channel chats only. Returns a 404 error if there is no sponsored message in the chat
    pub async fn get_chat_sponsored_message<C: AsRef<GetChatSponsoredMessage>>(
        &self,
        get_chat_sponsored_message: C,
    ) -> RTDResult<SponsoredMessage> {
        let extra = get_chat_sponsored_message
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_sponsored_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<SponsoredMessage>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns detailed statistics about a chat. Currently, this method can be used only for supergroups and channels. Can be used only if supergroupFullInfo.can_get_statistics == true
    pub async fn get_chat_statistics<C: AsRef<GetChatStatistics>>(
        &self,
        get_chat_statistics: C,
    ) -> RTDResult<ChatStatistics> {
        let extra = get_chat_statistics.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chat_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatStatistics>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an ordered list of chats from the beginning of a chat list. For informational purposes only. Use loadChats and updates processing instead to maintain chat lists in a consistent state
    pub async fn get_chats<C: AsRef<GetChats>>(&self, get_chats: C) -> RTDResult<Chats> {
        let extra = get_chats.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the list of commands supported by the bot for the given user scope and language; for bots only
    pub async fn get_commands<C: AsRef<GetCommands>>(
        &self,
        get_commands: C,
    ) -> RTDResult<BotCommands> {
        let extra = get_commands.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_commands.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<BotCommands>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns all website where the current user used Telegram to log in
    pub async fn get_connected_websites<C: AsRef<GetConnectedWebsites>>(
        &self,
        get_connected_websites: C,
    ) -> RTDResult<ConnectedWebsites> {
        let extra = get_connected_websites.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_connected_websites.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ConnectedWebsites>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns all user contacts
    pub async fn get_contacts<C: AsRef<GetContacts>>(&self, get_contacts: C) -> RTDResult<Users> {
        let extra = get_contacts.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Users>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about existing countries. Can be called before authorization
    pub async fn get_countries<C: AsRef<GetCountries>>(
        &self,
        get_countries: C,
    ) -> RTDResult<Countries> {
        let extra = get_countries.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_countries.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Countries>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Uses the current IP address to find the current country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization
    pub async fn get_country_code<C: AsRef<GetCountryCode>>(
        &self,
        get_country_code: C,
    ) -> RTDResult<Text> {
        let extra = get_country_code.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_country_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of public chats of the specified type, owned by the user
    pub async fn get_created_public_chats<C: AsRef<GetCreatedPublicChats>>(
        &self,
        get_created_public_chats: C,
    ) -> RTDResult<Chats> {
        let extra = get_created_public_chats.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_created_public_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially useful if TDLib is run in a separate process. Can be called before initialization
    pub async fn get_current_state<C: AsRef<GetCurrentState>>(
        &self,
        get_current_state: C,
    ) -> RTDResult<Updates> {
        let extra = get_current_state.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_current_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Updates>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns database statistics
    pub async fn get_database_statistics<C: AsRef<GetDatabaseStatistics>>(
        &self,
        get_database_statistics: C,
    ) -> RTDResult<DatabaseStatistics> {
        let extra = get_database_statistics.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_database_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<DatabaseStatistics>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a tg:// deep link. Use "tg://need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization
    pub async fn get_deep_link_info<C: AsRef<GetDeepLinkInfo>>(
        &self,
        get_deep_link_info: C,
    ) -> RTDResult<DeepLinkInfo> {
        let extra = get_deep_link_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_deep_link_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<DeepLinkInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation
    pub async fn get_emoji_suggestions_url<C: AsRef<GetEmojiSuggestionsUrl>>(
        &self,
        get_emoji_suggestions_url: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_emoji_suggestions_url.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_emoji_suggestions_url.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<HttpUrl>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an HTTP URL which can be used to automatically authorize the current user on a website after clicking an HTTP link. Use the method getExternalLinkInfo to find whether a prior user confirmation is needed
    pub async fn get_external_link<C: AsRef<GetExternalLink>>(
        &self,
        get_external_link: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_external_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_external_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<HttpUrl>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about an action to be done when the current user clicks an external link. Don't use this method for links from secret chats if web page preview is disabled in secret chats
    pub async fn get_external_link_info<C: AsRef<GetExternalLinkInfo>>(
        &self,
        get_external_link_info: C,
    ) -> RTDResult<LoginUrlInfo> {
        let extra = get_external_link_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_external_link_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LoginUrlInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns favorite stickers
    pub async fn get_favorite_stickers<C: AsRef<GetFavoriteStickers>>(
        &self,
        get_favorite_stickers: C,
    ) -> RTDResult<Stickers> {
        let extra = get_favorite_stickers.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_favorite_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Stickers>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a file; this is an offline request
    pub async fn get_file<C: AsRef<GetFile>>(&self, get_file: C) -> RTDResult<File> {
        let extra = get_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<File>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns file downloaded prefix size from a given offset, in bytes
    pub async fn get_file_downloaded_prefix_size<C: AsRef<GetFileDownloadedPrefixSize>>(
        &self,
        get_file_downloaded_prefix_size: C,
    ) -> RTDResult<Count> {
        let extra = get_file_downloaded_prefix_size
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_file_downloaded_prefix_size.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Count>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. Can be called synchronously
    pub async fn get_file_extension<C: AsRef<GetFileExtension>>(
        &self,
        get_file_extension: C,
    ) -> RTDResult<Text> {
        let extra = get_file_extension.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_file_extension.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. Can be called synchronously
    pub async fn get_file_mime_type<C: AsRef<GetFileMimeType>>(
        &self,
        get_file_mime_type: C,
    ) -> RTDResult<Text> {
        let extra = get_file_mime_type.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_file_mime_type.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only
    pub async fn get_game_high_scores<C: AsRef<GetGameHighScores>>(
        &self,
        get_game_high_scores: C,
    ) -> RTDResult<GameHighScores> {
        let extra = get_game_high_scores.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_game_high_scores.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<GameHighScores>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a group call
    pub async fn get_group_call<C: AsRef<GetGroupCall>>(
        &self,
        get_group_call: C,
    ) -> RTDResult<GroupCall> {
        let extra = get_group_call.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_group_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<GroupCall>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns invite link to a video chat in a public chat
    pub async fn get_group_call_invite_link<C: AsRef<GetGroupCallInviteLink>>(
        &self,
        get_group_call_invite_link: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_group_call_invite_link
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_group_call_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<HttpUrl>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a file with a segment of a group call stream in a modified OGG format for audio or MPEG-4 format for video
    pub async fn get_group_call_stream_segment<C: AsRef<GetGroupCallStreamSegment>>(
        &self,
        get_group_call_stream_segment: C,
    ) -> RTDResult<FilePart> {
        let extra = get_group_call_stream_segment
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_group_call_stream_segment.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<FilePart>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of common group chats with a given user. Chats are sorted by their type and creation date
    pub async fn get_groups_in_common<C: AsRef<GetGroupsInCommon>>(
        &self,
        get_groups_in_common: C,
    ) -> RTDResult<Chats> {
        let extra = get_groups_in_common.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_groups_in_common.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the total number of imported contacts
    pub async fn get_imported_contact_count<C: AsRef<GetImportedContactCount>>(
        &self,
        get_imported_contact_count: C,
    ) -> RTDResult<Count> {
        let extra = get_imported_contact_count
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_imported_contact_count.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Count>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of recently inactive supergroups and channels. Can be used when user reaches limit on the number of joined supergroups and channels and receives CHANNELS_TOO_MUCH error
    pub async fn get_inactive_supergroup_chats<C: AsRef<GetInactiveSupergroupChats>>(
        &self,
        get_inactive_supergroup_chats: C,
    ) -> RTDResult<Chats> {
        let extra = get_inactive_supergroup_chats
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_inactive_supergroup_chats.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns game high scores and some part of the high score table in the range of the specified user; for bots only
    pub async fn get_inline_game_high_scores<C: AsRef<GetInlineGameHighScores>>(
        &self,
        get_inline_game_high_scores: C,
    ) -> RTDResult<GameHighScores> {
        let extra = get_inline_game_high_scores
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_inline_game_high_scores.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<GameHighScores>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
    pub async fn get_inline_query_results<C: AsRef<GetInlineQueryResults>>(
        &self,
        get_inline_query_results: C,
    ) -> RTDResult<InlineQueryResults> {
        let extra = get_inline_query_results.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_inline_query_results.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<InlineQueryResults>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of installed sticker sets
    pub async fn get_installed_sticker_sets<C: AsRef<GetInstalledStickerSets>>(
        &self,
        get_installed_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = get_installed_sticker_sets
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_installed_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSets>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about the type of an internal link. Returns a 404 error if the link is not internal. Can be called before authorization
    pub async fn get_internal_link_type<C: AsRef<GetInternalLinkType>>(
        &self,
        get_internal_link_type: C,
    ) -> RTDResult<InternalLinkType> {
        let extra = get_internal_link_type.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_internal_link_type.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<InternalLinkType>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Converts a JsonValue object to corresponding JSON-serialized string. Can be called synchronously
    pub async fn get_json_string<C: AsRef<GetJsonString>>(
        &self,
        get_json_string: C,
    ) -> RTDResult<Text> {
        let extra = get_json_string.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_json_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Converts a JSON-serialized string to corresponding JsonValue object. Can be called synchronously
    pub async fn get_json_value<C: AsRef<GetJsonValue>>(
        &self,
        get_json_value: C,
    ) -> RTDResult<JsonValue> {
        let extra = get_json_value.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_json_value.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<JsonValue>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization
    pub async fn get_language_pack_info<C: AsRef<GetLanguagePackInfo>>(
        &self,
        get_language_pack_info: C,
    ) -> RTDResult<LanguagePackInfo> {
        let extra = get_language_pack_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_language_pack_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LanguagePackInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. Can be called synchronously
    pub async fn get_language_pack_string<C: AsRef<GetLanguagePackString>>(
        &self,
        get_language_pack_string: C,
    ) -> RTDResult<LanguagePackStringValue> {
        let extra = get_language_pack_string.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_language_pack_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LanguagePackStringValue>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns strings from a language pack in the current localization target by their keys. Can be called before authorization
    pub async fn get_language_pack_strings<C: AsRef<GetLanguagePackStrings>>(
        &self,
        get_language_pack_strings: C,
    ) -> RTDResult<LanguagePackStrings> {
        let extra = get_language_pack_strings.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_language_pack_strings.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LanguagePackStrings>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization
    pub async fn get_localization_target_info<C: AsRef<GetLocalizationTargetInfo>>(
        &self,
        get_localization_target_info: C,
    ) -> RTDResult<LocalizationTargetInfo> {
        let extra = get_localization_target_info
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_localization_target_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LocalizationTargetInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about currently used log stream for internal logging of TDLib. Can be called synchronously
    pub async fn get_log_stream<C: AsRef<GetLogStream>>(
        &self,
        get_log_stream: C,
    ) -> RTDResult<LogStream> {
        let extra = get_log_stream.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_log_stream.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LogStream>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns current verbosity level for a specified TDLib internal log tag. Can be called synchronously
    pub async fn get_log_tag_verbosity_level<C: AsRef<GetLogTagVerbosityLevel>>(
        &self,
        get_log_tag_verbosity_level: C,
    ) -> RTDResult<LogVerbosityLevel> {
        let extra = get_log_tag_verbosity_level
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_log_tag_verbosity_level.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LogVerbosityLevel>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. Can be called synchronously
    pub async fn get_log_tags<C: AsRef<GetLogTags>>(&self, get_log_tags: C) -> RTDResult<LogTags> {
        let extra = get_log_tags.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_log_tags.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LogTags>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns current verbosity level of the internal logging of TDLib. Can be called synchronously
    pub async fn get_log_verbosity_level<C: AsRef<GetLogVerbosityLevel>>(
        &self,
        get_log_verbosity_level: C,
    ) -> RTDResult<LogVerbosityLevel> {
        let extra = get_log_verbosity_level.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_log_verbosity_level.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LogVerbosityLevel>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl. Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button
    pub async fn get_login_url<C: AsRef<GetLoginUrl>>(
        &self,
        get_login_url: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_login_url.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_login_url.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<HttpUrl>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
    pub async fn get_login_url_info<C: AsRef<GetLoginUrlInfo>>(
        &self,
        get_login_url_info: C,
    ) -> RTDResult<LoginUrlInfo> {
        let extra = get_login_url_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_login_url_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<LoginUrlInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded
    pub async fn get_map_thumbnail_file<C: AsRef<GetMapThumbnailFile>>(
        &self,
        get_map_thumbnail_file: C,
    ) -> RTDResult<File> {
        let extra = get_map_thumbnail_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_map_thumbnail_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<File>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Replaces text entities with Markdown formatting in a human-friendly format. Entities that can't be represented in Markdown unambiguously are kept as is. Can be called synchronously
    pub async fn get_markdown_text<C: AsRef<GetMarkdownText>>(
        &self,
        get_markdown_text: C,
    ) -> RTDResult<FormattedText> {
        let extra = get_markdown_text.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_markdown_text.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<FormattedText>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the current user
    pub async fn get_me<C: AsRef<GetMe>>(&self, get_me: C) -> RTDResult<User> {
        let extra = get_me.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_me.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<User>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a message
    pub async fn get_message<C: AsRef<GetMessage>>(&self, get_message: C) -> RTDResult<Message> {
        let extra = get_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an HTML code for embedding the message. Available only for messages in supergroups and channels with a username
    pub async fn get_message_embedding_code<C: AsRef<GetMessageEmbeddingCode>>(
        &self,
        get_message_embedding_code: C,
    ) -> RTDResult<Text> {
        let extra = get_message_embedding_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_embedding_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a file with messages exported from another app
    pub async fn get_message_file_type<C: AsRef<GetMessageFileType>>(
        &self,
        get_message_file_type: C,
    ) -> RTDResult<MessageFileType> {
        let extra = get_message_file_type.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_file_type.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageFileType>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a confirmation text to be shown to the user before starting message import
    pub async fn get_message_import_confirmation_text<
        C: AsRef<GetMessageImportConfirmationText>,
    >(
        &self,
        get_message_import_confirmation_text: C,
    ) -> RTDResult<Text> {
        let extra = get_message_import_confirmation_text
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_message_import_confirmation_text.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels, or if message.can_get_media_timestamp_links and a media timestamp link is generated. This is an offline request
    pub async fn get_message_link<C: AsRef<GetMessageLink>>(
        &self,
        get_message_link: C,
    ) -> RTDResult<MessageLink> {
        let extra = get_message_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageLink>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a public or private message link. Can be called for any internal link of the type internalLinkTypeMessage
    pub async fn get_message_link_info<C: AsRef<GetMessageLinkInfo>>(
        &self,
        get_message_link_info: C,
    ) -> RTDResult<MessageLinkInfo> {
        let extra = get_message_link_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_link_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageLinkInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a message, if it is available locally without sending network request. This is an offline request
    pub async fn get_message_locally<C: AsRef<GetMessageLocally>>(
        &self,
        get_message_locally: C,
    ) -> RTDResult<Message> {
        let extra = get_message_locally.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_locally.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns forwarded copies of a channel message to different public channels. For optimal performance, the number of returned messages is chosen by TDLib
    pub async fn get_message_public_forwards<C: AsRef<GetMessagePublicForwards>>(
        &self,
        get_message_public_forwards: C,
    ) -> RTDResult<FoundMessages> {
        let extra = get_message_public_forwards
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_public_forwards.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<FoundMessages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns detailed statistics about a message. Can be used only if message.can_get_statistics == true
    pub async fn get_message_statistics<C: AsRef<GetMessageStatistics>>(
        &self,
        get_message_statistics: C,
    ) -> RTDResult<MessageStatistics> {
        let extra = get_message_statistics.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageStatistics>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a message thread. Can be used only if message.can_get_message_thread == true
    pub async fn get_message_thread<C: AsRef<GetMessageThread>>(
        &self,
        get_message_thread: C,
    ) -> RTDResult<MessageThreadInfo> {
        let extra = get_message_thread.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_thread.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageThreadInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns messages in a message thread of a message. Can be used only if message.can_get_message_thread == true. Message thread of a channel message is in the channel's linked supergroup. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
    pub async fn get_message_thread_history<C: AsRef<GetMessageThreadHistory>>(
        &self,
        get_message_thread_history: C,
    ) -> RTDResult<Messages> {
        let extra = get_message_thread_history
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_thread_history.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns viewers of a recent outgoing message in a basic group or a supergroup chat. For video notes and voice notes only users, opened content of the message, are returned. The method can be called if message.can_get_viewers == true
    pub async fn get_message_viewers<C: AsRef<GetMessageViewers>>(
        &self,
        get_message_viewers: C,
    ) -> RTDResult<Users> {
        let extra = get_message_viewers.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_message_viewers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Users>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about messages. If a message is not found, returns null on the corresponding position of the result
    pub async fn get_messages<C: AsRef<GetMessages>>(
        &self,
        get_messages: C,
    ) -> RTDResult<Messages> {
        let extra = get_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns network data usage statistics. Can be called before authorization
    pub async fn get_network_statistics<C: AsRef<GetNetworkStatistics>>(
        &self,
        get_network_statistics: C,
    ) -> RTDResult<NetworkStatistics> {
        let extra = get_network_statistics.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_network_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<NetworkStatistics>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization
    pub async fn get_option<C: AsRef<GetOption>>(&self, get_option: C) -> RTDResult<OptionValue> {
        let extra = get_option.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_option.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<OptionValue>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a Telegram Passport authorization form for sharing data with a service
    pub async fn get_passport_authorization_form<C: AsRef<GetPassportAuthorizationForm>>(
        &self,
        get_passport_authorization_form: C,
    ) -> RTDResult<PassportAuthorizationForm> {
        let extra = get_passport_authorization_form
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_passport_authorization_form.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PassportAuthorizationForm>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form
    pub async fn get_passport_authorization_form_available_elements<
        C: AsRef<GetPassportAuthorizationFormAvailableElements>,
    >(
        &self,
        get_passport_authorization_form_available_elements: C,
    ) -> RTDResult<PassportElementsWithErrors> {
        let extra = get_passport_authorization_form_available_elements
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_passport_authorization_form_available_elements.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PassportElementsWithErrors>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns one of the available Telegram Passport elements
    pub async fn get_passport_element<C: AsRef<GetPassportElement>>(
        &self,
        get_passport_element: C,
    ) -> RTDResult<PassportElement> {
        let extra = get_passport_element.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_passport_element.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PassportElement>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the current state of 2-step verification
    pub async fn get_password_state<C: AsRef<GetPasswordState>>(
        &self,
        get_password_state: C,
    ) -> RTDResult<PasswordState> {
        let extra = get_password_state.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_password_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PasswordState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an invoice payment form. This method must be called when the user presses inlineKeyboardButtonBuy
    pub async fn get_payment_form<C: AsRef<GetPaymentForm>>(
        &self,
        get_payment_form: C,
    ) -> RTDResult<PaymentForm> {
        let extra = get_payment_form.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_payment_form.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PaymentForm>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a successful payment
    pub async fn get_payment_receipt<C: AsRef<GetPaymentReceipt>>(
        &self,
        get_payment_receipt: C,
    ) -> RTDResult<PaymentReceipt> {
        let extra = get_payment_receipt.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_payment_receipt.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PaymentReceipt>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a phone number by its prefix. Can be called before authorization
    pub async fn get_phone_number_info<C: AsRef<GetPhoneNumberInfo>>(
        &self,
        get_phone_number_info: C,
    ) -> RTDResult<PhoneNumberInfo> {
        let extra = get_phone_number_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_phone_number_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PhoneNumberInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a phone number by its prefix synchronously. getCountries must be called at least once after changing localization to the specified language if properly localized country information is expected. Can be called synchronously
    pub async fn get_phone_number_info_sync<C: AsRef<GetPhoneNumberInfoSync>>(
        &self,
        get_phone_number_info_sync: C,
    ) -> RTDResult<PhoneNumberInfo> {
        let extra = get_phone_number_info_sync
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_phone_number_info_sync.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PhoneNumberInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns users voted for the specified option in a non-anonymous polls. For optimal performance, the number of returned users is chosen by TDLib
    pub async fn get_poll_voters<C: AsRef<GetPollVoters>>(
        &self,
        get_poll_voters: C,
    ) -> RTDResult<Users> {
        let extra = get_poll_voters.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_poll_voters.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Users>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an IETF language tag of the language preferred in the country, which must be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown
    pub async fn get_preferred_country_language<C: AsRef<GetPreferredCountryLanguage>>(
        &self,
        get_preferred_country_language: C,
    ) -> RTDResult<Text> {
        let extra = get_preferred_country_language
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_preferred_country_language.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns list of proxies that are currently set up. Can be called before authorization
    pub async fn get_proxies<C: AsRef<GetProxies>>(&self, get_proxies: C) -> RTDResult<Proxies> {
        let extra = get_proxies.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_proxies.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Proxies>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization
    pub async fn get_proxy_link<C: AsRef<GetProxyLink>>(
        &self,
        get_proxy_link: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_proxy_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_proxy_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<HttpUrl>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. Can be called synchronously
    pub async fn get_push_receiver_id<C: AsRef<GetPushReceiverId>>(
        &self,
        get_push_receiver_id: C,
    ) -> RTDResult<PushReceiverId> {
        let extra = get_push_receiver_id.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_push_receiver_id.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PushReceiverId>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns up to 20 recently used inline bots in the order of their last usage
    pub async fn get_recent_inline_bots<C: AsRef<GetRecentInlineBots>>(
        &self,
        get_recent_inline_bots: C,
    ) -> RTDResult<Users> {
        let extra = get_recent_inline_bots.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_recent_inline_bots.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Users>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of recently used stickers
    pub async fn get_recent_stickers<C: AsRef<GetRecentStickers>>(
        &self,
        get_recent_stickers: C,
    ) -> RTDResult<Stickers> {
        let extra = get_recent_stickers.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_recent_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Stickers>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns recently opened chats, this is an offline request. Returns chats in the order of last opening
    pub async fn get_recently_opened_chats<C: AsRef<GetRecentlyOpenedChats>>(
        &self,
        get_recently_opened_chats: C,
    ) -> RTDResult<Chats> {
        let extra = get_recently_opened_chats.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_recently_opened_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns t.me URLs recently visited by a newly registered user
    pub async fn get_recently_visited_t_me_urls<C: AsRef<GetRecentlyVisitedTMeUrls>>(
        &self,
        get_recently_visited_t_me_urls: C,
    ) -> RTDResult<TMeUrls> {
        let extra = get_recently_visited_t_me_urls
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_recently_visited_t_me_urls.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TMeUrls>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns recommended chat filters for the current user
    pub async fn get_recommended_chat_filters<C: AsRef<GetRecommendedChatFilters>>(
        &self,
        get_recommended_chat_filters: C,
    ) -> RTDResult<RecommendedChatFilters> {
        let extra = get_recommended_chat_filters
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_recommended_chat_filters.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<RecommendedChatFilters>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user
    pub async fn get_recovery_email_address<C: AsRef<GetRecoveryEmailAddress>>(
        &self,
        get_recovery_email_address: C,
    ) -> RTDResult<RecoveryEmailAddress> {
        let extra = get_recovery_email_address
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_recovery_email_address.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<RecoveryEmailAddress>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. Even the request succeeds, the file can be used only if it is still accessible to the user. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
    pub async fn get_remote_file<C: AsRef<GetRemoteFile>>(
        &self,
        get_remote_file: C,
    ) -> RTDResult<File> {
        let extra = get_remote_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_remote_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<File>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a message that is replied by a given message. Also returns the pinned message, the game message, and the invoice message for messages of the types messagePinMessage, messageGameScore, and messagePaymentSuccessful respectively
    pub async fn get_replied_message<C: AsRef<GetRepliedMessage>>(
        &self,
        get_replied_message: C,
    ) -> RTDResult<Message> {
        let extra = get_replied_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_replied_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns saved animations
    pub async fn get_saved_animations<C: AsRef<GetSavedAnimations>>(
        &self,
        get_saved_animations: C,
    ) -> RTDResult<Animations> {
        let extra = get_saved_animations.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_saved_animations.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Animations>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns saved order info, if any
    pub async fn get_saved_order_info<C: AsRef<GetSavedOrderInfo>>(
        &self,
        get_saved_order_info: C,
    ) -> RTDResult<OrderInfo> {
        let extra = get_saved_order_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_saved_order_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<OrderInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the notification settings for chats of a given type
    pub async fn get_scope_notification_settings<C: AsRef<GetScopeNotificationSettings>>(
        &self,
        get_scope_notification_settings: C,
    ) -> RTDResult<ScopeNotificationSettings> {
        let extra = get_scope_notification_settings
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_scope_notification_settings.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ScopeNotificationSettings>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a secret chat by its identifier. This is an offline request
    pub async fn get_secret_chat<C: AsRef<GetSecretChat>>(
        &self,
        get_secret_chat: C,
    ) -> RTDResult<SecretChat> {
        let extra = get_secret_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_secret_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<SecretChat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Loads an asynchronous or a zoomed in statistical graph
    pub async fn get_statistical_graph<C: AsRef<GetStatisticalGraph>>(
        &self,
        get_statistical_graph: C,
    ) -> RTDResult<StatisticalGraph> {
        let extra = get_statistical_graph.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_statistical_graph.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StatisticalGraph>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
    pub async fn get_sticker_emojis<C: AsRef<GetStickerEmojis>>(
        &self,
        get_sticker_emojis: C,
    ) -> RTDResult<Emojis> {
        let extra = get_sticker_emojis.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_sticker_emojis.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Emojis>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a sticker set by its identifier
    pub async fn get_sticker_set<C: AsRef<GetStickerSet>>(
        &self,
        get_sticker_set: C,
    ) -> RTDResult<StickerSet> {
        let extra = get_sticker_set.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSet>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is non-empty, favorite and recently used stickers may also be returned
    pub async fn get_stickers<C: AsRef<GetStickers>>(
        &self,
        get_stickers: C,
    ) -> RTDResult<Stickers> {
        let extra = get_stickers.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Stickers>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns storage usage statistics. Can be called before authorization
    pub async fn get_storage_statistics<C: AsRef<GetStorageStatistics>>(
        &self,
        get_storage_statistics: C,
    ) -> RTDResult<StorageStatistics> {
        let extra = get_storage_statistics.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_storage_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StorageStatistics>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Quickly returns approximate storage usage statistics. Can be called before authorization
    pub async fn get_storage_statistics_fast<C: AsRef<GetStorageStatisticsFast>>(
        &self,
        get_storage_statistics_fast: C,
    ) -> RTDResult<StorageStatisticsFast> {
        let extra = get_storage_statistics_fast
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_storage_statistics_fast.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StorageStatisticsFast>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns suggested name for saving a file in a given directory
    pub async fn get_suggested_file_name<C: AsRef<GetSuggestedFileName>>(
        &self,
        get_suggested_file_name: C,
    ) -> RTDResult<Text> {
        let extra = get_suggested_file_name.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_suggested_file_name.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a suggested name for a new sticker set with a given title
    pub async fn get_suggested_sticker_set_name<C: AsRef<GetSuggestedStickerSetName>>(
        &self,
        get_suggested_sticker_set_name: C,
    ) -> RTDResult<Text> {
        let extra = get_suggested_sticker_set_name
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_suggested_sticker_set_name.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of basic group and supergroup chats, which can be used as a discussion group for a channel. Returned basic group chats must be first upgraded to supergroups before they can be set as a discussion group. To set a returned supergroup as a discussion group, access to its old messages must be enabled using toggleSupergroupIsAllHistoryAvailable first
    pub async fn get_suitable_discussion_chats<C: AsRef<GetSuitableDiscussionChats>>(
        &self,
        get_suitable_discussion_chats: C,
    ) -> RTDResult<Chats> {
        let extra = get_suitable_discussion_chats
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_suitable_discussion_chats.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a supergroup or a channel by its identifier. This is an offline request if the current user is not a bot
    pub async fn get_supergroup<C: AsRef<GetSupergroup>>(
        &self,
        get_supergroup: C,
    ) -> RTDResult<Supergroup> {
        let extra = get_supergroup.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_supergroup.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Supergroup>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute
    pub async fn get_supergroup_full_info<C: AsRef<GetSupergroupFullInfo>>(
        &self,
        get_supergroup_full_info: C,
    ) -> RTDResult<SupergroupFullInfo> {
        let extra = get_supergroup_full_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_supergroup_full_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<SupergroupFullInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about members or banned users in a supergroup or channel. Can be used only if supergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters
    pub async fn get_supergroup_members<C: AsRef<GetSupergroupMembers>>(
        &self,
        get_supergroup_members: C,
    ) -> RTDResult<ChatMembers> {
        let extra = get_supergroup_members.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_supergroup_members.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatMembers>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a user that can be contacted to get support
    pub async fn get_support_user<C: AsRef<GetSupportUser>>(
        &self,
        get_support_user: C,
    ) -> RTDResult<User> {
        let extra = get_support_user.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_support_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<User>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about the current temporary password
    pub async fn get_temporary_password_state<C: AsRef<GetTemporaryPasswordState>>(
        &self,
        get_temporary_password_state: C,
    ) -> RTDResult<TemporaryPasswordState> {
        let extra = get_temporary_password_state
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_temporary_password_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TemporaryPasswordState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns all entities (mentions, hashtags, cashtags, bot commands, bank card numbers, URLs, and email addresses) contained in the text. Can be called synchronously
    pub async fn get_text_entities<C: AsRef<GetTextEntities>>(
        &self,
        get_text_entities: C,
    ) -> RTDResult<TextEntities> {
        let extra = get_text_entities.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_text_entities.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TextEntities>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of frequently used chats. Supported only if the chat info database is enabled
    pub async fn get_top_chats<C: AsRef<GetTopChats>>(&self, get_top_chats: C) -> RTDResult<Chats> {
        let extra = get_top_chats.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_top_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of trending sticker sets. For optimal performance, the number of returned sticker sets is chosen by TDLib
    pub async fn get_trending_sticker_sets<C: AsRef<GetTrendingStickerSets>>(
        &self,
        get_trending_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = get_trending_sticker_sets.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_trending_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSets>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about a user by their identifier. This is an offline request if the current user is not a bot
    pub async fn get_user<C: AsRef<GetUser>>(&self, get_user: C) -> RTDResult<User> {
        let extra = get_user.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<User>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns full information about a user by their identifier
    pub async fn get_user_full_info<C: AsRef<GetUserFullInfo>>(
        &self,
        get_user_full_info: C,
    ) -> RTDResult<UserFullInfo> {
        let extra = get_user_full_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_user_full_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<UserFullInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the current privacy settings
    pub async fn get_user_privacy_setting_rules<C: AsRef<GetUserPrivacySettingRules>>(
        &self,
        get_user_privacy_setting_rules: C,
    ) -> RTDResult<UserPrivacySettingRules> {
        let extra = get_user_privacy_setting_rules
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_user_privacy_setting_rules.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<UserPrivacySettingRules>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already
    pub async fn get_user_profile_photos<C: AsRef<GetUserProfilePhotos>>(
        &self,
        get_user_profile_photos: C,
    ) -> RTDResult<ChatPhotos> {
        let extra = get_user_profile_photos.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_user_profile_photos.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatPhotos>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns list of participant identifiers, on whose behalf a video chat in the chat can be joined
    pub async fn get_video_chat_available_participants<
        C: AsRef<GetVideoChatAvailableParticipants>,
    >(
        &self,
        get_video_chat_available_participants: C,
    ) -> RTDResult<MessageSenders> {
        let extra = get_video_chat_available_participants
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            get_video_chat_available_participants.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<MessageSenders>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page
    pub async fn get_web_page_instant_view<C: AsRef<GetWebPageInstantView>>(
        &self,
        get_web_page_instant_view: C,
    ) -> RTDResult<WebPageInstantView> {
        let extra = get_web_page_instant_view.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_web_page_instant_view.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<WebPageInstantView>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview
    pub async fn get_web_page_preview<C: AsRef<GetWebPagePreview>>(
        &self,
        get_web_page_preview: C,
    ) -> RTDResult<WebPage> {
        let extra = get_web_page_preview.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, get_web_page_preview.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<WebPage>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Hides a suggested action
    pub async fn hide_suggested_action<C: AsRef<HideSuggestedAction>>(
        &self,
        hide_suggested_action: C,
    ) -> RTDResult<Ok> {
        let extra = hide_suggested_action.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, hide_suggested_action.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored
    pub async fn import_contacts<C: AsRef<ImportContacts>>(
        &self,
        import_contacts: C,
    ) -> RTDResult<ImportedContacts> {
        let extra = import_contacts.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, import_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ImportedContacts>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Imports messages exported from another app
    pub async fn import_messages<C: AsRef<ImportMessages>>(
        &self,
        import_messages: C,
    ) -> RTDResult<Ok> {
        let extra = import_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, import_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Invites users to an active group call. Sends a service message of type messageInviteToGroupCall for video chats
    pub async fn invite_group_call_participants<C: AsRef<InviteGroupCallParticipants>>(
        &self,
        invite_group_call_participants: C,
    ) -> RTDResult<Ok> {
        let extra = invite_group_call_participants
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            invite_group_call_participants.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds the current user as a new member to a chat. Private and secret chats can't be joined using this method
    pub async fn join_chat<C: AsRef<JoinChat>>(&self, join_chat: C) -> RTDResult<Ok> {
        let extra = join_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, join_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Uses an invite link to add the current user to the chat if possible
    pub async fn join_chat_by_invite_link<C: AsRef<JoinChatByInviteLink>>(
        &self,
        join_chat_by_invite_link: C,
    ) -> RTDResult<Chat> {
        let extra = join_chat_by_invite_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, join_chat_by_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Joins an active group call. Returns join response payload for tgcalls
    pub async fn join_group_call<C: AsRef<JoinGroupCall>>(
        &self,
        join_group_call: C,
    ) -> RTDResult<Text> {
        let extra = join_group_call.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, join_group_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes the current user from chat members. Private and secret chats can't be left using this method
    pub async fn leave_chat<C: AsRef<LeaveChat>>(&self, leave_chat: C) -> RTDResult<Ok> {
        let extra = leave_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, leave_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Leaves a group call
    pub async fn leave_group_call<C: AsRef<LeaveGroupCall>>(
        &self,
        leave_group_call: C,
    ) -> RTDResult<Ok> {
        let extra = leave_group_call.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, leave_group_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Loads more chats from a chat list. The loaded chats and their positions in the chat list will be sent through updates. Chats are sorted by the pair (chat.position.order, chat.id) in descending order. Returns a 404 error if all chats have been loaded
    pub async fn load_chats<C: AsRef<LoadChats>>(&self, load_chats: C) -> RTDResult<Ok> {
        let extra = load_chats.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, load_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Loads more participants of a group call. The loaded participants will be received through updates. Use the field groupCall.loaded_all_participants to check whether all participants have already been loaded
    pub async fn load_group_call_participants<C: AsRef<LoadGroupCallParticipants>>(
        &self,
        load_group_call_participants: C,
    ) -> RTDResult<Ok> {
        let extra = load_group_call_participants
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, load_group_call_participants.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent
    pub async fn log_out<C: AsRef<LogOut>>(&self, log_out: C) -> RTDResult<Ok> {
        let extra = log_out.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, log_out.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats)
    pub async fn open_chat<C: AsRef<OpenChat>>(&self, open_chat: C) -> RTDResult<Ok> {
        let extra = open_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, open_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An updateMessageContentOpened update will be generated if something has changed
    pub async fn open_message_content<C: AsRef<OpenMessageContent>>(
        &self,
        open_message_content: C,
    ) -> RTDResult<Ok> {
        let extra = open_message_content.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, open_message_content.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted
    pub async fn optimize_storage<C: AsRef<OptimizeStorage>>(
        &self,
        optimize_storage: C,
    ) -> RTDResult<StorageStatistics> {
        let extra = optimize_storage.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, optimize_storage.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StorageStatistics>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Parses Markdown entities in a human-friendly format, ignoring markup errors. Can be called synchronously
    pub async fn parse_markdown<C: AsRef<ParseMarkdown>>(
        &self,
        parse_markdown: C,
    ) -> RTDResult<FormattedText> {
        let extra = parse_markdown.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, parse_markdown.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<FormattedText>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Parses Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities contained in the text. Can be called synchronously
    pub async fn parse_text_entities<C: AsRef<ParseTextEntities>>(
        &self,
        parse_text_entities: C,
    ) -> RTDResult<FormattedText> {
        let extra = parse_text_entities.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, parse_text_entities.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<FormattedText>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Pins a message in a chat; requires can_pin_messages rights or can_edit_messages rights in the channel
    pub async fn pin_chat_message<C: AsRef<PinChatMessage>>(
        &self,
        pin_chat_message: C,
    ) -> RTDResult<Ok> {
        let extra = pin_chat_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, pin_chat_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization
    pub async fn ping_proxy<C: AsRef<PingProxy>>(&self, ping_proxy: C) -> RTDResult<Seconds> {
        let extra = ping_proxy.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, ping_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Seconds>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Handles a pending join request in a chat
    pub async fn process_chat_join_request<C: AsRef<ProcessChatJoinRequest>>(
        &self,
        process_chat_join_request: C,
    ) -> RTDResult<Ok> {
        let extra = process_chat_join_request.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, process_chat_join_request.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Handles all pending join requests for a given link in a chat
    pub async fn process_chat_join_requests<C: AsRef<ProcessChatJoinRequests>>(
        &self,
        process_chat_join_requests: C,
    ) -> RTDResult<Ok> {
        let extra = process_chat_join_requests
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, process_chat_join_requests.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization
    pub async fn process_push_notification<C: AsRef<ProcessPushNotification>>(
        &self,
        process_push_notification: C,
    ) -> RTDResult<Ok> {
        let extra = process_push_notification.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, process_push_notification.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Marks all mentions in a chat as read
    pub async fn read_all_chat_mentions<C: AsRef<ReadAllChatMentions>>(
        &self,
        read_all_chat_mentions: C,
    ) -> RTDResult<Ok> {
        let extra = read_all_chat_mentions.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, read_all_chat_mentions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct read from the file
    pub async fn read_file_part<C: AsRef<ReadFilePart>>(
        &self,
        read_file_part: C,
    ) -> RTDResult<FilePart> {
        let extra = read_file_part.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, read_file_part.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<FilePart>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn recover_authentication_password<C: AsRef<RecoverAuthenticationPassword>>(
        &self,
        recover_authentication_password: C,
    ) -> RTDResult<Ok> {
        let extra = recover_authentication_password
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            recover_authentication_password.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Recovers the 2-step verification password using a recovery code sent to an email address that was previously set up
    pub async fn recover_password<C: AsRef<RecoverPassword>>(
        &self,
        recover_password: C,
    ) -> RTDResult<PasswordState> {
        let extra = recover_password.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, recover_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PasswordState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription
    pub async fn register_device<C: AsRef<RegisterDevice>>(
        &self,
        register_device: C,
    ) -> RTDResult<PushReceiverId> {
        let extra = register_device.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, register_device.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PushReceiverId>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration
    pub async fn register_user<C: AsRef<RegisterUser>>(&self, register_user: C) -> RTDResult<Ok> {
        let extra = register_user.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, register_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes background from the list of installed backgrounds
    pub async fn remove_background<C: AsRef<RemoveBackground>>(
        &self,
        remove_background: C,
    ) -> RTDResult<Ok> {
        let extra = remove_background.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_background.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a chat action bar without any other action
    pub async fn remove_chat_action_bar<C: AsRef<RemoveChatActionBar>>(
        &self,
        remove_chat_action_bar: C,
    ) -> RTDResult<Ok> {
        let extra = remove_chat_action_bar.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_chat_action_bar.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes users from the contact list
    pub async fn remove_contacts<C: AsRef<RemoveContacts>>(
        &self,
        remove_contacts: C,
    ) -> RTDResult<Ok> {
        let extra = remove_contacts.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a sticker from the list of favorite stickers
    pub async fn remove_favorite_sticker<C: AsRef<RemoveFavoriteSticker>>(
        &self,
        remove_favorite_sticker: C,
    ) -> RTDResult<Ok> {
        let extra = remove_favorite_sticker.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_favorite_sticker.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user
    pub async fn remove_notification<C: AsRef<RemoveNotification>>(
        &self,
        remove_notification: C,
    ) -> RTDResult<Ok> {
        let extra = remove_notification.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_notification.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user
    pub async fn remove_notification_group<C: AsRef<RemoveNotificationGroup>>(
        &self,
        remove_notification_group: C,
    ) -> RTDResult<Ok> {
        let extra = remove_notification_group.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_notification_group.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a proxy server. Can be called before authorization
    pub async fn remove_proxy<C: AsRef<RemoveProxy>>(&self, remove_proxy: C) -> RTDResult<Ok> {
        let extra = remove_proxy.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a hashtag from the list of recently used hashtags
    pub async fn remove_recent_hashtag<C: AsRef<RemoveRecentHashtag>>(
        &self,
        remove_recent_hashtag: C,
    ) -> RTDResult<Ok> {
        let extra = remove_recent_hashtag.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_recent_hashtag.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a sticker from the list of recently used stickers
    pub async fn remove_recent_sticker<C: AsRef<RemoveRecentSticker>>(
        &self,
        remove_recent_sticker: C,
    ) -> RTDResult<Ok> {
        let extra = remove_recent_sticker.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_recent_sticker.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a chat from the list of recently found chats
    pub async fn remove_recently_found_chat<C: AsRef<RemoveRecentlyFoundChat>>(
        &self,
        remove_recently_found_chat: C,
    ) -> RTDResult<Ok> {
        let extra = remove_recently_found_chat
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_recently_found_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes an animation from the list of saved animations
    pub async fn remove_saved_animation<C: AsRef<RemoveSavedAnimation>>(
        &self,
        remove_saved_animation: C,
    ) -> RTDResult<Ok> {
        let extra = remove_saved_animation.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_saved_animation.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot
    pub async fn remove_sticker_from_set<C: AsRef<RemoveStickerFromSet>>(
        &self,
        remove_sticker_from_set: C,
    ) -> RTDResult<Ok> {
        let extra = remove_sticker_from_set.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_sticker_from_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled
    pub async fn remove_top_chat<C: AsRef<RemoveTopChat>>(
        &self,
        remove_top_chat: C,
    ) -> RTDResult<Ok> {
        let extra = remove_top_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, remove_top_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the order of chat filters
    pub async fn reorder_chat_filters<C: AsRef<ReorderChatFilters>>(
        &self,
        reorder_chat_filters: C,
    ) -> RTDResult<Ok> {
        let extra = reorder_chat_filters.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, reorder_chat_filters.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the order of installed sticker sets
    pub async fn reorder_installed_sticker_sets<C: AsRef<ReorderInstalledStickerSets>>(
        &self,
        reorder_installed_sticker_sets: C,
    ) -> RTDResult<Ok> {
        let extra = reorder_installed_sticker_sets
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            reorder_installed_sticker_sets.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Replaces current primary invite link for a chat with a new primary invite link. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right
    pub async fn replace_primary_chat_invite_link<C: AsRef<ReplacePrimaryChatInviteLink>>(
        &self,
        replace_primary_chat_invite_link: C,
    ) -> RTDResult<ChatInviteLink> {
        let extra = replace_primary_chat_invite_link
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            replace_primary_chat_invite_link.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLink>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Reports a chat to the Telegram moderators. A chat can be reported only from the chat action bar, or if chat.can_be_reported
    pub async fn report_chat<C: AsRef<ReportChat>>(&self, report_chat: C) -> RTDResult<Ok> {
        let extra = report_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, report_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Reports a chat photo to the Telegram moderators. A chat photo can be reported only if chat.can_be_reported
    pub async fn report_chat_photo<C: AsRef<ReportChatPhoto>>(
        &self,
        report_chat_photo: C,
    ) -> RTDResult<Ok> {
        let extra = report_chat_photo.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, report_chat_photo.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Reports messages in a supergroup as spam; requires administrator rights in the supergroup
    pub async fn report_supergroup_spam<C: AsRef<ReportSupergroupSpam>>(
        &self,
        report_supergroup_spam: C,
    ) -> RTDResult<Ok> {
        let extra = report_supergroup_spam.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, report_supergroup_spam.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn request_authentication_password_recovery<
        C: AsRef<RequestAuthenticationPasswordRecovery>,
    >(
        &self,
        request_authentication_password_recovery: C,
    ) -> RTDResult<Ok> {
        let extra = request_authentication_password_recovery
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            request_authentication_password_recovery.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Requests to send a 2-step verification password recovery code to an email address that was previously set up
    pub async fn request_password_recovery<C: AsRef<RequestPasswordRecovery>>(
        &self,
        request_password_recovery: C,
    ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
        let extra = request_password_recovery.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, request_password_recovery.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<EmailAddressAuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Requests QR code authentication by scanning a QR code on another logged in device. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
    pub async fn request_qr_code_authentication<C: AsRef<RequestQrCodeAuthentication>>(
        &self,
        request_qr_code_authentication: C,
    ) -> RTDResult<Ok> {
        let extra = request_qr_code_authentication
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            request_qr_code_authentication.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Re-sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode, the next_code_type of the result is not null and the server-specified timeout has passed
    pub async fn resend_authentication_code<C: AsRef<ResendAuthenticationCode>>(
        &self,
        resend_authentication_code: C,
    ) -> RTDResult<Ok> {
        let extra = resend_authentication_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, resend_authentication_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Re-sends the authentication code sent to confirm a new phone number for the current user. Works only if the previously received authenticationCodeInfo next_code_type was not null and the server-specified timeout has passed
    pub async fn resend_change_phone_number_code<C: AsRef<ResendChangePhoneNumberCode>>(
        &self,
        resend_change_phone_number_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = resend_change_phone_number_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            resend_change_phone_number_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Re-sends the code to verify an email address to be added to a user's Telegram Passport
    pub async fn resend_email_address_verification_code<
        C: AsRef<ResendEmailAddressVerificationCode>,
    >(
        &self,
        resend_email_address_verification_code: C,
    ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
        let extra = resend_email_address_verification_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            resend_email_address_verification_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<EmailAddressAuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed. If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message
    pub async fn resend_messages<C: AsRef<ResendMessages>>(
        &self,
        resend_messages: C,
    ) -> RTDResult<Messages> {
        let extra = resend_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, resend_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Resends phone number confirmation code
    pub async fn resend_phone_number_confirmation_code<
        C: AsRef<ResendPhoneNumberConfirmationCode>,
    >(
        &self,
        resend_phone_number_confirmation_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = resend_phone_number_confirmation_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            resend_phone_number_confirmation_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Re-sends the code to verify a phone number to be added to a user's Telegram Passport
    pub async fn resend_phone_number_verification_code<
        C: AsRef<ResendPhoneNumberVerificationCode>,
    >(
        &self,
        resend_phone_number_verification_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = resend_phone_number_verification_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            resend_phone_number_verification_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Resends the 2-step verification recovery email address verification code
    pub async fn resend_recovery_email_address_code<C: AsRef<ResendRecoveryEmailAddressCode>>(
        &self,
        resend_recovery_email_address_code: C,
    ) -> RTDResult<PasswordState> {
        let extra = resend_recovery_email_address_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            resend_recovery_email_address_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PasswordState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to "default" and message previews are shown
    pub async fn reset_all_notification_settings<C: AsRef<ResetAllNotificationSettings>>(
        &self,
        reset_all_notification_settings: C,
    ) -> RTDResult<Ok> {
        let extra = reset_all_notification_settings
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            reset_all_notification_settings.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Resets list of installed backgrounds to its default value
    pub async fn reset_backgrounds<C: AsRef<ResetBackgrounds>>(
        &self,
        reset_backgrounds: C,
    ) -> RTDResult<Ok> {
        let extra = reset_backgrounds.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, reset_backgrounds.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Resets all network data usage statistics to zero. Can be called before authorization
    pub async fn reset_network_statistics<C: AsRef<ResetNetworkStatistics>>(
        &self,
        reset_network_statistics: C,
    ) -> RTDResult<Ok> {
        let extra = reset_network_statistics.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, reset_network_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes 2-step verification password without previous password and access to recovery email address. The password can't be reset immediately and the request needs to be repeated after the specified time
    pub async fn reset_password<C: AsRef<ResetPassword>>(
        &self,
        reset_password: C,
    ) -> RTDResult<ResetPasswordResult> {
        let extra = reset_password.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, reset_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ResetPasswordResult>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Revokes invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links. If a primary link is revoked, then additionally to the revoked link returns new primary link
    pub async fn revoke_chat_invite_link<C: AsRef<RevokeChatInviteLink>>(
        &self,
        revoke_chat_invite_link: C,
    ) -> RTDResult<ChatInviteLinks> {
        let extra = revoke_chat_invite_link.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, revoke_chat_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatInviteLinks>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Revokes invite link for a group call. Requires groupCall.can_be_managed group call flag
    pub async fn revoke_group_call_invite_link<C: AsRef<RevokeGroupCallInviteLink>>(
        &self,
        revoke_group_call_invite_link: C,
    ) -> RTDResult<Ok> {
        let extra = revoke_group_call_invite_link
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            revoke_group_call_invite_link.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Saves application log event on the server. Can be called before authorization
    pub async fn save_application_log_event<C: AsRef<SaveApplicationLogEvent>>(
        &self,
        save_application_log_event: C,
    ) -> RTDResult<Ok> {
        let extra = save_application_log_event
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, save_application_log_event.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for a background by its name
    pub async fn search_background<C: AsRef<SearchBackground>>(
        &self,
        search_background: C,
    ) -> RTDResult<Background> {
        let extra = search_background.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_background.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Background>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
    pub async fn search_call_messages<C: AsRef<SearchCallMessages>>(
        &self,
        search_call_messages: C,
    ) -> RTDResult<Messages> {
        let extra = search_call_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_call_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels
    pub async fn search_chat_members<C: AsRef<SearchChatMembers>>(
        &self,
        search_chat_members: C,
    ) -> RTDResult<ChatMembers> {
        let extra = search_chat_members.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_chat_members.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatMembers>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query (searchSecretMessages must be used instead), or without an enabled message database. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
    pub async fn search_chat_messages<C: AsRef<SearchChatMessages>>(
        &self,
        search_chat_messages: C,
    ) -> RTDResult<Messages> {
        let extra = search_chat_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_chat_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user
    pub async fn search_chat_recent_location_messages<
        C: AsRef<SearchChatRecentLocationMessages>,
    >(
        &self,
        search_chat_recent_location_messages: C,
    ) -> RTDResult<Messages> {
        let extra = search_chat_recent_location_messages
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            search_chat_recent_location_messages.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the main chat list
    pub async fn search_chats<C: AsRef<SearchChats>>(&self, search_chats: C) -> RTDResult<Chats> {
        let extra = search_chats.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns a list of users and location-based supergroups nearby. The list of users nearby will be updated for 60 seconds after the request by the updates updateUsersNearby. The request must be sent again every 25 seconds with adjusted location to not miss new chats
    pub async fn search_chats_nearby<C: AsRef<SearchChatsNearby>>(
        &self,
        search_chats_nearby: C,
    ) -> RTDResult<ChatsNearby> {
        let extra = search_chats_nearby.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_chats_nearby.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ChatsNearby>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the main chat list
    pub async fn search_chats_on_server<C: AsRef<SearchChatsOnServer>>(
        &self,
        search_chats_on_server: C,
    ) -> RTDResult<Chats> {
        let extra = search_chats_on_server.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_chats_on_server.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for the specified query in the first names, last names and usernames of the known user contacts
    pub async fn search_contacts<C: AsRef<SearchContacts>>(
        &self,
        search_contacts: C,
    ) -> RTDResult<Users> {
        let extra = search_contacts.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Users>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for emojis by keywords. Supported only if the file database is enabled
    pub async fn search_emojis<C: AsRef<SearchEmojis>>(
        &self,
        search_emojis: C,
    ) -> RTDResult<Emojis> {
        let extra = search_emojis.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_emojis.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Emojis>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for recently used hashtags by their prefix
    pub async fn search_hashtags<C: AsRef<SearchHashtags>>(
        &self,
        search_hashtags: C,
    ) -> RTDResult<Hashtags> {
        let extra = search_hashtags.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_hashtags.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Hashtags>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for installed sticker sets by looking for specified query in their title and name
    pub async fn search_installed_sticker_sets<C: AsRef<SearchInstalledStickerSets>>(
        &self,
        search_installed_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = search_installed_sticker_sets
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            search_installed_sticker_sets.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSets>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
    pub async fn search_messages<C: AsRef<SearchMessages>>(
        &self,
        search_messages: C,
    ) -> RTDResult<Messages> {
        let extra = search_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches a public chat by its username. Currently, only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned
    pub async fn search_public_chat<C: AsRef<SearchPublicChat>>(
        &self,
        search_public_chat: C,
    ) -> RTDResult<Chat> {
        let extra = search_public_chat.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_public_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches public chats by looking for specified query in their username and title. Currently, only private chats, supergroups and channels can be public. Returns a meaningful number of results. Excludes private chats with contacts and chats from the chat list from the results
    pub async fn search_public_chats<C: AsRef<SearchPublicChats>>(
        &self,
        search_public_chats: C,
    ) -> RTDResult<Chats> {
        let extra = search_public_chats.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_public_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chats>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance, the number of returned messages is chosen by TDLib
    pub async fn search_secret_messages<C: AsRef<SearchSecretMessages>>(
        &self,
        search_secret_messages: C,
    ) -> RTDResult<FoundMessages> {
        let extra = search_secret_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_secret_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<FoundMessages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for a sticker set by its name
    pub async fn search_sticker_set<C: AsRef<SearchStickerSet>>(
        &self,
        search_sticker_set: C,
    ) -> RTDResult<StickerSet> {
        let extra = search_sticker_set.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSet>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results
    pub async fn search_sticker_sets<C: AsRef<SearchStickerSets>>(
        &self,
        search_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = search_sticker_sets.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSets>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Searches for stickers from public sticker sets that correspond to a given emoji
    pub async fn search_stickers<C: AsRef<SearchStickers>>(
        &self,
        search_stickers: C,
    ) -> RTDResult<Stickers> {
        let extra = search_stickers.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, search_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Stickers>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message
    pub async fn send_bot_start_message<C: AsRef<SendBotStartMessage>>(
        &self,
        send_bot_start_message: C,
    ) -> RTDResult<Message> {
        let extra = send_bot_start_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_bot_start_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends debug information for a call
    pub async fn send_call_debug_information<C: AsRef<SendCallDebugInformation>>(
        &self,
        send_call_debug_information: C,
    ) -> RTDResult<Ok> {
        let extra = send_call_debug_information
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_call_debug_information.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a call rating
    pub async fn send_call_rating<C: AsRef<SendCallRating>>(
        &self,
        send_call_rating: C,
    ) -> RTDResult<Ok> {
        let extra = send_call_rating.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_call_rating.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends call signaling data
    pub async fn send_call_signaling_data<C: AsRef<SendCallSignalingData>>(
        &self,
        send_call_signaling_data: C,
    ) -> RTDResult<Ok> {
        let extra = send_call_signaling_data.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_call_signaling_data.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a notification about user activity in a chat
    pub async fn send_chat_action<C: AsRef<SendChatAction>>(
        &self,
        send_chat_action: C,
    ) -> RTDResult<Ok> {
        let extra = send_chat_action.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_chat_action.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats
    pub async fn send_chat_screenshot_taken_notification<
        C: AsRef<SendChatScreenshotTakenNotification>,
    >(
        &self,
        send_chat_screenshot_taken_notification: C,
    ) -> RTDResult<Ok> {
        let extra = send_chat_screenshot_taken_notification
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            send_chat_screenshot_taken_notification.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a custom request; for bots only
    pub async fn send_custom_request<C: AsRef<SendCustomRequest>>(
        &self,
        send_custom_request: C,
    ) -> RTDResult<CustomRequestResult> {
        let extra = send_custom_request.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_custom_request.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<CustomRequestResult>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a code to verify an email address to be added to a user's Telegram Passport
    pub async fn send_email_address_verification_code<
        C: AsRef<SendEmailAddressVerificationCode>,
    >(
        &self,
        send_email_address_verification_code: C,
    ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
        let extra = send_email_address_verification_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            send_email_address_verification_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<EmailAddressAuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message
    pub async fn send_inline_query_result_message<C: AsRef<SendInlineQueryResultMessage>>(
        &self,
        send_inline_query_result_message: C,
    ) -> RTDResult<Message> {
        let extra = send_inline_query_result_message
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            send_inline_query_result_message.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a message. Returns the sent message
    pub async fn send_message<C: AsRef<SendMessage>>(&self, send_message: C) -> RTDResult<Message> {
        let extra = send_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends 2-10 messages grouped together into an album. Currently, only audio, document, photo and video messages can be grouped into an album. Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
    pub async fn send_message_album<C: AsRef<SendMessageAlbum>>(
        &self,
        send_message_album: C,
    ) -> RTDResult<Messages> {
        let extra = send_message_album.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_message_album.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Messages>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements are going to be reused
    pub async fn send_passport_authorization_form<C: AsRef<SendPassportAuthorizationForm>>(
        &self,
        send_passport_authorization_form: C,
    ) -> RTDResult<Ok> {
        let extra = send_passport_authorization_form
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            send_passport_authorization_form.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a filled-out payment form to the bot for final verification
    pub async fn send_payment_form<C: AsRef<SendPaymentForm>>(
        &self,
        send_payment_form: C,
    ) -> RTDResult<PaymentResult> {
        let extra = send_payment_form.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, send_payment_form.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PaymentResult>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends phone number confirmation code to handle links of the type internalLinkTypePhoneNumberConfirmation
    pub async fn send_phone_number_confirmation_code<C: AsRef<SendPhoneNumberConfirmationCode>>(
        &self,
        send_phone_number_confirmation_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = send_phone_number_confirmation_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            send_phone_number_confirmation_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a code to verify a phone number to be added to a user's Telegram Passport
    pub async fn send_phone_number_verification_code<C: AsRef<SendPhoneNumberVerificationCode>>(
        &self,
        send_phone_number_verification_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = send_phone_number_verification_code
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            send_phone_number_verification_code.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<AuthenticationCodeInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the period of inactivity after which the account of the current user will automatically be deleted
    pub async fn set_account_ttl<C: AsRef<SetAccountTtl>>(
        &self,
        set_account_ttl: C,
    ) -> RTDResult<Ok> {
        let extra = set_account_ttl.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_account_ttl.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Succeeds after a specified amount of time has passed. Can be called before initialization
    pub async fn set_alarm<C: AsRef<SetAlarm>>(&self, set_alarm: C) -> RTDResult<Ok> {
        let extra = set_alarm.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_alarm.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
    pub async fn set_authentication_phone_number<C: AsRef<SetAuthenticationPhoneNumber>>(
        &self,
        set_authentication_phone_number: C,
    ) -> RTDResult<Ok> {
        let extra = set_authentication_phone_number
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            set_authentication_phone_number.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets auto-download settings
    pub async fn set_auto_download_settings<C: AsRef<SetAutoDownloadSettings>>(
        &self,
        set_auto_download_settings: C,
    ) -> RTDResult<Ok> {
        let extra = set_auto_download_settings
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_auto_download_settings.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the background selected by the user; adds background to the list of installed backgrounds
    pub async fn set_background<C: AsRef<SetBackground>>(
        &self,
        set_background: C,
    ) -> RTDResult<Background> {
        let extra = set_background.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_background.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Background>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the bio of the current user
    pub async fn set_bio<C: AsRef<SetBio>>(&self, set_bio: C) -> RTDResult<Ok> {
        let extra = set_bio.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_bio.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only
    pub async fn set_bot_updates_status<C: AsRef<SetBotUpdatesStatus>>(
        &self,
        set_bot_updates_status: C,
    ) -> RTDResult<Ok> {
        let extra = set_bot_updates_status.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_bot_updates_status.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes application-specific data associated with a chat
    pub async fn set_chat_client_data<C: AsRef<SetChatClientData>>(
        &self,
        set_chat_client_data: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_client_data.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_client_data.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info administrator right
    pub async fn set_chat_description<C: AsRef<SetChatDescription>>(
        &self,
        set_chat_description: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_description.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_description.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the discussion group of a channel chat; requires can_change_info administrator right in the channel if it is specified
    pub async fn set_chat_discussion_group<C: AsRef<SetChatDiscussionGroup>>(
        &self,
        set_chat_discussion_group: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_discussion_group.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_discussion_group.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the draft message in a chat
    pub async fn set_chat_draft_message<C: AsRef<SetChatDraftMessage>>(
        &self,
        set_chat_draft_message: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_draft_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_draft_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use
    pub async fn set_chat_location<C: AsRef<SetChatLocation>>(
        &self,
        set_chat_location: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_location.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_location.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for transferring chat ownership; use transferChatOwnership instead. Use addChatMember or banChatMember if some additional parameters needs to be passed
    pub async fn set_chat_member_status<C: AsRef<SetChatMemberStatus>>(
        &self,
        set_chat_member_status: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_member_status.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_member_status.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Selects a message sender to send messages in a chat
    pub async fn set_chat_message_sender<C: AsRef<SetChatMessageSender>>(
        &self,
        set_chat_message_sender: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_message_sender.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_message_sender.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the message TTL in a chat. Requires can_delete_messages administrator right in basic groups, supergroups and channels Message TTL can't be changed in a chat with the current user (Saved Messages) and the chat 777000 (Telegram)
    pub async fn set_chat_message_ttl<C: AsRef<SetChatMessageTtl>>(
        &self,
        set_chat_message_ttl: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_message_ttl.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_message_ttl.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed
    pub async fn set_chat_notification_settings<C: AsRef<SetChatNotificationSettings>>(
        &self,
        set_chat_notification_settings: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_notification_settings
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            set_chat_notification_settings.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right
    pub async fn set_chat_permissions<C: AsRef<SetChatPermissions>>(
        &self,
        set_chat_permissions: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_permissions.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_permissions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info administrator right
    pub async fn set_chat_photo<C: AsRef<SetChatPhoto>>(&self, set_chat_photo: C) -> RTDResult<Ok> {
        let extra = set_chat_photo.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_photo.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members rights
    pub async fn set_chat_slow_mode_delay<C: AsRef<SetChatSlowModeDelay>>(
        &self,
        set_chat_slow_mode_delay: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_slow_mode_delay.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_slow_mode_delay.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the chat theme. Supported only in private and secret chats
    pub async fn set_chat_theme<C: AsRef<SetChatTheme>>(&self, set_chat_theme: C) -> RTDResult<Ok> {
        let extra = set_chat_theme.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_theme.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info administrator right
    pub async fn set_chat_title<C: AsRef<SetChatTitle>>(&self, set_chat_title: C) -> RTDResult<Ok> {
        let extra = set_chat_title.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_chat_title.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the list of commands supported by the bot for the given user scope and language; for bots only
    pub async fn set_commands<C: AsRef<SetCommands>>(&self, set_commands: C) -> RTDResult<Ok> {
        let extra = set_commands.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_commands.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds or changes a custom local language pack to the current localization target
    pub async fn set_custom_language_pack<C: AsRef<SetCustomLanguagePack>>(
        &self,
        set_custom_language_pack: C,
    ) -> RTDResult<Ok> {
        let extra = set_custom_language_pack.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_custom_language_pack.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds, edits or deletes a string in a custom local language pack. Can be called before authorization
    pub async fn set_custom_language_pack_string<C: AsRef<SetCustomLanguagePackString>>(
        &self,
        set_custom_language_pack_string: C,
    ) -> RTDResult<Ok> {
        let extra = set_custom_language_pack_string
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            set_custom_language_pack_string.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain
    pub async fn set_database_encryption_key<C: AsRef<SetDatabaseEncryptionKey>>(
        &self,
        set_database_encryption_key: C,
    ) -> RTDResult<Ok> {
        let extra = set_database_encryption_key
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_database_encryption_key.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs TDLib on a file generation progress
    pub async fn set_file_generation_progress<C: AsRef<SetFileGenerationProgress>>(
        &self,
        set_file_generation_progress: C,
    ) -> RTDResult<Ok> {
        let extra = set_file_generation_progress
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_file_generation_progress.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Updates the game score of the specified user in the game; for bots only
    pub async fn set_game_score<C: AsRef<SetGameScore>>(
        &self,
        set_game_score: C,
    ) -> RTDResult<Message> {
        let extra = set_game_score.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_game_score.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Message>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs TDLib that speaking state of a participant of an active group has changed
    pub async fn set_group_call_participant_is_speaking<
        C: AsRef<SetGroupCallParticipantIsSpeaking>,
    >(
        &self,
        set_group_call_participant_is_speaking: C,
    ) -> RTDResult<Ok> {
        let extra = set_group_call_participant_is_speaking
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            set_group_call_participant_is_speaking.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes volume level of a participant of an active group call. If the current user can manage the group call, then the participant's volume level will be changed for all users with the default volume level
    pub async fn set_group_call_participant_volume_level<
        C: AsRef<SetGroupCallParticipantVolumeLevel>,
    >(
        &self,
        set_group_call_participant_volume_level: C,
    ) -> RTDResult<Ok> {
        let extra = set_group_call_participant_volume_level
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            set_group_call_participant_volume_level.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets group call title. Requires groupCall.can_be_managed group call flag
    pub async fn set_group_call_title<C: AsRef<SetGroupCallTitle>>(
        &self,
        set_group_call_title: C,
    ) -> RTDResult<Ok> {
        let extra = set_group_call_title.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_group_call_title.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the period of inactivity after which sessions will automatically be terminated
    pub async fn set_inactive_session_ttl<C: AsRef<SetInactiveSessionTtl>>(
        &self,
        set_inactive_session_ttl: C,
    ) -> RTDResult<Ok> {
        let extra = set_inactive_session_ttl.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_inactive_session_ttl.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Updates the game score of the specified user in a game; for bots only
    pub async fn set_inline_game_score<C: AsRef<SetInlineGameScore>>(
        &self,
        set_inline_game_score: C,
    ) -> RTDResult<Ok> {
        let extra = set_inline_game_score.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_inline_game_score.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the location of the current user. Needs to be called if GetOption("is_location_visible") is true and location changes for more than 1 kilometer
    pub async fn set_location<C: AsRef<SetLocation>>(&self, set_location: C) -> RTDResult<Ok> {
        let extra = set_location.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_location.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets new log stream for internal logging of TDLib. Can be called synchronously
    pub async fn set_log_stream<C: AsRef<SetLogStream>>(&self, set_log_stream: C) -> RTDResult<Ok> {
        let extra = set_log_stream.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_log_stream.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the verbosity level for a specified TDLib internal log tag. Can be called synchronously
    pub async fn set_log_tag_verbosity_level<C: AsRef<SetLogTagVerbosityLevel>>(
        &self,
        set_log_tag_verbosity_level: C,
    ) -> RTDResult<Ok> {
        let extra = set_log_tag_verbosity_level
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_log_tag_verbosity_level.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the verbosity level of the internal logging of TDLib. Can be called synchronously
    pub async fn set_log_verbosity_level<C: AsRef<SetLogVerbosityLevel>>(
        &self,
        set_log_verbosity_level: C,
    ) -> RTDResult<Ok> {
        let extra = set_log_verbosity_level.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_log_verbosity_level.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the first and last name of the current user
    pub async fn set_name<C: AsRef<SetName>>(&self, set_name: C) -> RTDResult<Ok> {
        let extra = set_name.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_name.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it must be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics
    pub async fn set_network_type<C: AsRef<SetNetworkType>>(
        &self,
        set_network_type: C,
    ) -> RTDResult<Ok> {
        let extra = set_network_type.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_network_type.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the value of an option. (Check the list of available options on https://core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization
    pub async fn set_option<C: AsRef<SetOption>>(&self, set_option: C) -> RTDResult<Ok> {
        let extra = set_option.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_option.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
    pub async fn set_passport_element<C: AsRef<SetPassportElement>>(
        &self,
        set_passport_element: C,
    ) -> RTDResult<PassportElement> {
        let extra = set_passport_element.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_passport_element.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PassportElement>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed
    pub async fn set_passport_element_errors<C: AsRef<SetPassportElementErrors>>(
        &self,
        set_passport_element_errors: C,
    ) -> RTDResult<Ok> {
        let extra = set_passport_element_errors
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_passport_element_errors.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the password for the current user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed
    pub async fn set_password<C: AsRef<SetPassword>>(
        &self,
        set_password: C,
    ) -> RTDResult<PasswordState> {
        let extra = set_password.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PasswordState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the order of pinned chats
    pub async fn set_pinned_chats<C: AsRef<SetPinnedChats>>(
        &self,
        set_pinned_chats: C,
    ) -> RTDResult<Ok> {
        let extra = set_pinned_chats.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_pinned_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the user answer to a poll. A poll in quiz mode can be answered only once
    pub async fn set_poll_answer<C: AsRef<SetPollAnswer>>(
        &self,
        set_poll_answer: C,
    ) -> RTDResult<Ok> {
        let extra = set_poll_answer.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_poll_answer.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes a profile photo for the current user
    pub async fn set_profile_photo<C: AsRef<SetProfilePhoto>>(
        &self,
        set_profile_photo: C,
    ) -> RTDResult<Ok> {
        let extra = set_profile_photo.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_profile_photo.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed. If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation
    pub async fn set_recovery_email_address<C: AsRef<SetRecoveryEmailAddress>>(
        &self,
        set_recovery_email_address: C,
    ) -> RTDResult<PasswordState> {
        let extra = set_recovery_email_address
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_recovery_email_address.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<PasswordState>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes notification settings for chats of a given type
    pub async fn set_scope_notification_settings<C: AsRef<SetScopeNotificationSettings>>(
        &self,
        set_scope_notification_settings: C,
    ) -> RTDResult<Ok> {
        let extra = set_scope_notification_settings
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            set_scope_notification_settings.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot
    pub async fn set_sticker_position_in_set<C: AsRef<SetStickerPositionInSet>>(
        &self,
        set_sticker_position_in_set: C,
    ) -> RTDResult<Ok> {
        let extra = set_sticker_position_in_set
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_sticker_position_in_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets a sticker set thumbnail; for bots only. Returns the sticker set
    pub async fn set_sticker_set_thumbnail<C: AsRef<SetStickerSetThumbnail>>(
        &self,
        set_sticker_set_thumbnail: C,
    ) -> RTDResult<StickerSet> {
        let extra = set_sticker_set_thumbnail.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_sticker_set_thumbnail.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<StickerSet>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the sticker set of a supergroup; requires can_change_info administrator right
    pub async fn set_supergroup_sticker_set<C: AsRef<SetSupergroupStickerSet>>(
        &self,
        set_supergroup_sticker_set: C,
    ) -> RTDResult<Ok> {
        let extra = set_supergroup_sticker_set
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_supergroup_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the username of a supergroup or channel, requires owner privileges in the supergroup or channel
    pub async fn set_supergroup_username<C: AsRef<SetSupergroupUsername>>(
        &self,
        set_supergroup_username: C,
    ) -> RTDResult<Ok> {
        let extra = set_supergroup_username.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_supergroup_username.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
    pub async fn set_tdlib_parameters<C: AsRef<SetTdlibParameters>>(
        &self,
        set_tdlib_parameters: C,
    ) -> RTDResult<Ok> {
        let extra = set_tdlib_parameters.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_tdlib_parameters.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes user privacy settings
    pub async fn set_user_privacy_setting_rules<C: AsRef<SetUserPrivacySettingRules>>(
        &self,
        set_user_privacy_setting_rules: C,
    ) -> RTDResult<Ok> {
        let extra = set_user_privacy_setting_rules
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            set_user_privacy_setting_rules.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the username of the current user
    pub async fn set_username<C: AsRef<SetUsername>>(&self, set_username: C) -> RTDResult<Ok> {
        let extra = set_username.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, set_username.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes default participant identifier, on whose behalf a video chat in the chat will be joined
    pub async fn set_video_chat_default_participant<C: AsRef<SetVideoChatDefaultParticipant>>(
        &self,
        set_video_chat_default_participant: C,
    ) -> RTDResult<Ok> {
        let extra = set_video_chat_default_participant
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            set_video_chat_default_participant.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber
    pub async fn share_phone_number<C: AsRef<SharePhoneNumber>>(
        &self,
        share_phone_number: C,
    ) -> RTDResult<Ok> {
        let extra = share_phone_number.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, share_phone_number.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Starts recording of an active group call. Requires groupCall.can_be_managed group call flag
    pub async fn start_group_call_recording<C: AsRef<StartGroupCallRecording>>(
        &self,
        start_group_call_recording: C,
    ) -> RTDResult<Ok> {
        let extra = start_group_call_recording
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, start_group_call_recording.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Starts screen sharing in a joined group call. Returns join response payload for tgcalls
    pub async fn start_group_call_screen_sharing<C: AsRef<StartGroupCallScreenSharing>>(
        &self,
        start_group_call_screen_sharing: C,
    ) -> RTDResult<Text> {
        let extra = start_group_call_screen_sharing
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            start_group_call_screen_sharing.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Text>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Starts a scheduled group call
    pub async fn start_scheduled_group_call<C: AsRef<StartScheduledGroupCall>>(
        &self,
        start_scheduled_group_call: C,
    ) -> RTDResult<Ok> {
        let extra = start_scheduled_group_call
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, start_scheduled_group_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set
    pub async fn stop_poll<C: AsRef<StopPoll>>(&self, stop_poll: C) -> RTDResult<Ok> {
        let extra = stop_poll.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, stop_poll.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization
    pub async fn synchronize_language_pack<C: AsRef<SynchronizeLanguagePack>>(
        &self,
        synchronize_language_pack: C,
    ) -> RTDResult<Ok> {
        let extra = synchronize_language_pack.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, synchronize_language_pack.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Terminates all other sessions of the current user
    pub async fn terminate_all_other_sessions<C: AsRef<TerminateAllOtherSessions>>(
        &self,
        terminate_all_other_sessions: C,
    ) -> RTDResult<Ok> {
        let extra = terminate_all_other_sessions
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, terminate_all_other_sessions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Terminates a session of the current user
    pub async fn terminate_session<C: AsRef<TerminateSession>>(
        &self,
        terminate_session: C,
    ) -> RTDResult<Ok> {
        let extra = terminate_session.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, terminate_session.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the received bytes; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_bytes<C: AsRef<TestCallBytes>>(
        &self,
        test_call_bytes: C,
    ) -> RTDResult<TestBytes> {
        let extra = test_call_bytes.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_call_bytes.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TestBytes>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Does nothing; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_empty<C: AsRef<TestCallEmpty>>(
        &self,
        test_call_empty: C,
    ) -> RTDResult<Ok> {
        let extra = test_call_empty.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_call_empty.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the received string; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_string<C: AsRef<TestCallString>>(
        &self,
        test_call_string: C,
    ) -> RTDResult<TestString> {
        let extra = test_call_string.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_call_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TestString>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_int<C: AsRef<TestCallVectorInt>>(
        &self,
        test_call_vector_int: C,
    ) -> RTDResult<TestVectorInt> {
        let extra = test_call_vector_int.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_call_vector_int.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TestVectorInt>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_int_object<C: AsRef<TestCallVectorIntObject>>(
        &self,
        test_call_vector_int_object: C,
    ) -> RTDResult<TestVectorIntObject> {
        let extra = test_call_vector_int_object
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_call_vector_int_object.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TestVectorIntObject>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_string<C: AsRef<TestCallVectorString>>(
        &self,
        test_call_vector_string: C,
    ) -> RTDResult<TestVectorString> {
        let extra = test_call_vector_string.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_call_vector_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TestVectorString>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_string_object<C: AsRef<TestCallVectorStringObject>>(
        &self,
        test_call_vector_string_object: C,
    ) -> RTDResult<TestVectorStringObject> {
        let extra = test_call_vector_string_object
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            test_call_vector_string_object.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TestVectorStringObject>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Forces an updates.getDifference call to the Telegram servers; for testing only
    pub async fn test_get_difference<C: AsRef<TestGetDifference>>(
        &self,
        test_get_difference: C,
    ) -> RTDResult<Ok> {
        let extra = test_get_difference.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_get_difference.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization
    pub async fn test_network<C: AsRef<TestNetwork>>(&self, test_network: C) -> RTDResult<Ok> {
        let extra = test_network.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_network.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization
    pub async fn test_proxy<C: AsRef<TestProxy>>(&self, test_proxy: C) -> RTDResult<Ok> {
        let extra = test_proxy.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the specified error and ensures that the Error object is used; for testing only. Can be called synchronously
    pub async fn test_return_error<C: AsRef<TestReturnError>>(
        &self,
        test_return_error: C,
    ) -> RTDResult<Error> {
        let extra = test_return_error.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_return_error.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Error>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Returns the squared received number; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_square_int<C: AsRef<TestSquareInt>>(
        &self,
        test_square_int: C,
    ) -> RTDResult<TestInt> {
        let extra = test_square_int.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_square_int.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<TestInt>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_use_update<C: AsRef<TestUseUpdate>>(
        &self,
        test_use_update: C,
    ) -> RTDResult<Update> {
        let extra = test_use_update.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, test_use_update.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Update>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the value of the default disable_notification parameter, used when a message is sent to a chat
    pub async fn toggle_chat_default_disable_notification<
        C: AsRef<ToggleChatDefaultDisableNotification>,
    >(
        &self,
        toggle_chat_default_disable_notification: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_chat_default_disable_notification
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_chat_default_disable_notification.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the ability of users to save, forward, or copy chat content. Supported only for basic groups, supergroups and channels. Requires owner privileges
    pub async fn toggle_chat_has_protected_content<C: AsRef<ToggleChatHasProtectedContent>>(
        &self,
        toggle_chat_has_protected_content: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_chat_has_protected_content
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_chat_has_protected_content.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the marked as unread state of a chat
    pub async fn toggle_chat_is_marked_as_unread<C: AsRef<ToggleChatIsMarkedAsUnread>>(
        &self,
        toggle_chat_is_marked_as_unread: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_chat_is_marked_as_unread
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_chat_is_marked_as_unread.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the pinned state of a chat. There can be up to GetOption("pinned_chat_count_max")/GetOption("pinned_archived_chat_count_max") pinned non-secret chats and the same number of secret chats in the main/arhive chat list
    pub async fn toggle_chat_is_pinned<C: AsRef<ToggleChatIsPinned>>(
        &self,
        toggle_chat_is_pinned: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_chat_is_pinned.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, toggle_chat_is_pinned.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether the current user will receive a notification when the group call will start; scheduled group calls only
    pub async fn toggle_group_call_enabled_start_notification<
        C: AsRef<ToggleGroupCallEnabledStartNotification>,
    >(
        &self,
        toggle_group_call_enabled_start_notification: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_group_call_enabled_start_notification
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_group_call_enabled_start_notification.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether current user's video is enabled
    pub async fn toggle_group_call_is_my_video_enabled<
        C: AsRef<ToggleGroupCallIsMyVideoEnabled>,
    >(
        &self,
        toggle_group_call_is_my_video_enabled: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_group_call_is_my_video_enabled
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_group_call_is_my_video_enabled.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether current user's video is paused
    pub async fn toggle_group_call_is_my_video_paused<C: AsRef<ToggleGroupCallIsMyVideoPaused>>(
        &self,
        toggle_group_call_is_my_video_paused: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_group_call_is_my_video_paused
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_group_call_is_my_video_paused.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether new participants of a group call can be unmuted only by administrators of the group call. Requires groupCall.can_toggle_mute_new_participants group call flag
    pub async fn toggle_group_call_mute_new_participants<
        C: AsRef<ToggleGroupCallMuteNewParticipants>,
    >(
        &self,
        toggle_group_call_mute_new_participants: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_group_call_mute_new_participants
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_group_call_mute_new_participants.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether a group call participant hand is rased
    pub async fn toggle_group_call_participant_is_hand_raised<
        C: AsRef<ToggleGroupCallParticipantIsHandRaised>,
    >(
        &self,
        toggle_group_call_participant_is_hand_raised: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_group_call_participant_is_hand_raised
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_group_call_participant_is_hand_raised.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether a participant of an active group call is muted, unmuted, or allowed to unmute themselves
    pub async fn toggle_group_call_participant_is_muted<
        C: AsRef<ToggleGroupCallParticipantIsMuted>,
    >(
        &self,
        toggle_group_call_participant_is_muted: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_group_call_participant_is_muted
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_group_call_participant_is_muted.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Pauses or unpauses screen sharing in a joined group call
    pub async fn toggle_group_call_screen_sharing_is_paused<
        C: AsRef<ToggleGroupCallScreenSharingIsPaused>,
    >(
        &self,
        toggle_group_call_screen_sharing_is_paused: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_group_call_screen_sharing_is_paused
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_group_call_screen_sharing_is_paused.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the block state of a message sender. Currently, only users and supergroup chats can be blocked
    pub async fn toggle_message_sender_is_blocked<C: AsRef<ToggleMessageSenderIsBlocked>>(
        &self,
        toggle_message_sender_is_blocked: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_message_sender_is_blocked
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_message_sender_is_blocked.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether a session can accept incoming calls
    pub async fn toggle_session_can_accept_calls<C: AsRef<ToggleSessionCanAcceptCalls>>(
        &self,
        toggle_session_can_accept_calls: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_session_can_accept_calls
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_session_can_accept_calls.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether a session can accept incoming secret chats
    pub async fn toggle_session_can_accept_secret_chats<
        C: AsRef<ToggleSessionCanAcceptSecretChats>,
    >(
        &self,
        toggle_session_can_accept_secret_chats: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_session_can_accept_secret_chats
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_session_can_accept_secret_chats.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether the message history of a supergroup is available to new members; requires can_change_info administrator right
    pub async fn toggle_supergroup_is_all_history_available<
        C: AsRef<ToggleSupergroupIsAllHistoryAvailable>,
    >(
        &self,
        toggle_supergroup_is_all_history_available: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_supergroup_is_all_history_available
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_supergroup_is_all_history_available.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Upgrades supergroup to a broadcast group; requires owner privileges in the supergroup
    pub async fn toggle_supergroup_is_broadcast_group<
        C: AsRef<ToggleSupergroupIsBroadcastGroup>,
    >(
        &self,
        toggle_supergroup_is_broadcast_group: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_supergroup_is_broadcast_group
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_supergroup_is_broadcast_group.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Toggles whether sender signature is added to sent messages in a channel; requires can_change_info administrator right
    pub async fn toggle_supergroup_sign_messages<C: AsRef<ToggleSupergroupSignMessages>>(
        &self,
        toggle_supergroup_sign_messages: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_supergroup_sign_messages
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            toggle_supergroup_sign_messages.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Changes the owner of a chat. The current user must be a current owner of the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session. Available only for supergroups and channel chats
    pub async fn transfer_chat_ownership<C: AsRef<TransferChatOwnership>>(
        &self,
        transfer_chat_ownership: C,
    ) -> RTDResult<Ok> {
        let extra = transfer_chat_ownership.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, transfer_chat_ownership.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes all pinned messages from a chat; requires can_pin_messages rights in the group or can_edit_messages rights in the channel
    pub async fn unpin_all_chat_messages<C: AsRef<UnpinAllChatMessages>>(
        &self,
        unpin_all_chat_messages: C,
    ) -> RTDResult<Ok> {
        let extra = unpin_all_chat_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, unpin_all_chat_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Removes a pinned message from a chat; requires can_pin_messages rights in the group or can_edit_messages rights in the channel
    pub async fn unpin_chat_message<C: AsRef<UnpinChatMessage>>(
        &self,
        unpin_chat_message: C,
    ) -> RTDResult<Ok> {
        let extra = unpin_chat_message.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, unpin_chat_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires creator privileges. Deactivates the original basic group
    pub async fn upgrade_basic_group_chat_to_supergroup_chat<
        C: AsRef<UpgradeBasicGroupChatToSupergroupChat>,
    >(
        &self,
        upgrade_basic_group_chat_to_supergroup_chat: C,
    ) -> RTDResult<Chat> {
        let extra = upgrade_basic_group_chat_to_supergroup_chat
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client.send(
            self.get_client_id()?,
            upgrade_basic_group_chat_to_supergroup_chat.as_ref(),
        )?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Chat>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Asynchronously uploads a file to the cloud without sending it in a message. updateFile will be used to notify about upload progress and successful completion of the upload. The file will not have a persistent remote identifier until it will be sent in a message
    pub async fn upload_file<C: AsRef<UploadFile>>(&self, upload_file: C) -> RTDResult<File> {
        let extra = upload_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, upload_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<File>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Uploads a file with a sticker; returns the uploaded file
    pub async fn upload_sticker_file<C: AsRef<UploadStickerFile>>(
        &self,
        upload_sticker_file: C,
    ) -> RTDResult<File> {
        let extra = upload_sticker_file.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, upload_sticker_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<File>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Validates the order information provided by a user and returns the available shipping options for a flexible invoice
    pub async fn validate_order_info<C: AsRef<ValidateOrderInfo>>(
        &self,
        validate_order_info: C,
    ) -> RTDResult<ValidatedOrderInfo> {
        let extra = validate_order_info.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, validate_order_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<ValidatedOrderInfo>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs TDLib that messages are being viewed by the user. Sponsored messages must be marked as viewed only when the entire text of the message is shown on the screen (excluding the button). Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)
    pub async fn view_messages<C: AsRef<ViewMessages>>(&self, view_messages: C) -> RTDResult<Ok> {
        let extra = view_messages.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, view_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Informs the server that some trending sticker sets have been viewed by the user
    pub async fn view_trending_sticker_sets<C: AsRef<ViewTrendingStickerSets>>(
        &self,
        view_trending_sticker_sets: C,
    ) -> RTDResult<Ok> {
        let extra = view_trending_sticker_sets
            .as_ref()
            .extra()
            .ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, view_trending_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }

    // Writes a part of a generated file. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file
    pub async fn write_generated_file_part<C: AsRef<WriteGeneratedFilePart>>(
        &self,
        write_generated_file_part: C,
    ) -> RTDResult<Ok> {
        let extra = write_generated_file_part.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(&extra);
        self.tdlib_client
            .send(self.get_client_id()?, write_generated_file_part.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => match serde_json::from_value::<Ok>(v) {
                Ok(v) => Ok(v),
                Err(e) => {
                    log::error!("response serialization error: {:?}", e);
                    Err(INVALID_RESPONSE_ERROR)
                }
            },
        }
    }
}
