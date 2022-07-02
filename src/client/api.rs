use super::tdlib_client::TdLibClient;
use super::Client;
use crate::{errors::Result, types::*};

impl<R> Client<R>
where
    R: TdLibClient + Clone,
{
    // Accepts an incoming call
    pub async fn accept_call<C: AsRef<AcceptCall>>(&self, accept_call: C) -> Result<Ok> {
        self.make_request(accept_call).await
    }

    // Accepts Telegram terms of services
    pub async fn accept_terms_of_service<C: AsRef<AcceptTermsOfService>>(
        &self,
        accept_terms_of_service: C,
    ) -> Result<Ok> {
        self.make_request(accept_terms_of_service).await
    }

    // Adds a new member to a chat. Members can't be added to private or secret chats
    pub async fn add_chat_member<C: AsRef<AddChatMember>>(&self, add_chat_member: C) -> Result<Ok> {
        self.make_request(add_chat_member).await
    }

    // Adds multiple new members to a chat. Currently, this method is only available for supergroups and channels. This method can't be used to join a chat. Members can't be added to a channel if it has more than 200 members
    pub async fn add_chat_members<C: AsRef<AddChatMembers>>(
        &self,
        add_chat_members: C,
    ) -> Result<Ok> {
        self.make_request(add_chat_members).await
    }

    // Adds a chat to a chat list. A chat can't be simultaneously in Main and Archive chat lists, so it is automatically removed from another one if needed
    pub async fn add_chat_to_list<C: AsRef<AddChatToList>>(
        &self,
        add_chat_to_list: C,
    ) -> Result<Ok> {
        self.make_request(add_chat_to_list).await
    }

    // Adds a user to the contact list or edits an existing contact by their user identifier
    pub async fn add_contact<C: AsRef<AddContact>>(&self, add_contact: C) -> Result<Ok> {
        self.make_request(add_contact).await
    }

    // Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization
    pub async fn add_custom_server_language_pack<C: AsRef<AddCustomServerLanguagePack>>(
        &self,
        add_custom_server_language_pack: C,
    ) -> Result<Ok> {
        self.make_request(add_custom_server_language_pack).await
    }

    // Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
    pub async fn add_favorite_sticker<C: AsRef<AddFavoriteSticker>>(
        &self,
        add_favorite_sticker: C,
    ) -> Result<Ok> {
        self.make_request(add_favorite_sticker).await
    }

    // Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message
    pub async fn add_local_message<C: AsRef<AddLocalMessage>>(
        &self,
        add_local_message: C,
    ) -> Result<Message> {
        self.make_request(add_local_message).await
    }

    // Adds a message to TDLib internal log. Can be called synchronously
    pub async fn add_log_message<C: AsRef<AddLogMessage>>(&self, add_log_message: C) -> Result<Ok> {
        self.make_request(add_log_message).await
    }

    // Adds the specified data to data usage statistics. Can be called before authorization
    pub async fn add_network_statistics<C: AsRef<AddNetworkStatistics>>(
        &self,
        add_network_statistics: C,
    ) -> Result<Ok> {
        self.make_request(add_network_statistics).await
    }

    // Adds a proxy server for network requests. Can be called before authorization
    pub async fn add_proxy<C: AsRef<AddProxy>>(&self, add_proxy: C) -> Result<Proxy> {
        self.make_request(add_proxy).await
    }

    // Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
    pub async fn add_recent_sticker<C: AsRef<AddRecentSticker>>(
        &self,
        add_recent_sticker: C,
    ) -> Result<Stickers> {
        self.make_request(add_recent_sticker).await
    }

    // Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first
    pub async fn add_recently_found_chat<C: AsRef<AddRecentlyFoundChat>>(
        &self,
        add_recently_found_chat: C,
    ) -> Result<Ok> {
        self.make_request(add_recently_found_chat).await
    }

    // Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type "video/mp4" can be added to the list
    pub async fn add_saved_animation<C: AsRef<AddSavedAnimation>>(
        &self,
        add_saved_animation: C,
    ) -> Result<Ok> {
        self.make_request(add_saved_animation).await
    }

    // Adds a new sticker to a set; for bots only. Returns the sticker set
    pub async fn add_sticker_to_set<C: AsRef<AddStickerToSet>>(
        &self,
        add_sticker_to_set: C,
    ) -> Result<StickerSet> {
        self.make_request(add_sticker_to_set).await
    }

    // Sets the result of a callback query; for bots only
    pub async fn answer_callback_query<C: AsRef<AnswerCallbackQuery>>(
        &self,
        answer_callback_query: C,
    ) -> Result<Ok> {
        self.make_request(answer_callback_query).await
    }

    // Answers a custom query; for bots only
    pub async fn answer_custom_query<C: AsRef<AnswerCustomQuery>>(
        &self,
        answer_custom_query: C,
    ) -> Result<Ok> {
        self.make_request(answer_custom_query).await
    }

    // Sets the result of an inline query; for bots only
    pub async fn answer_inline_query<C: AsRef<AnswerInlineQuery>>(
        &self,
        answer_inline_query: C,
    ) -> Result<Ok> {
        self.make_request(answer_inline_query).await
    }

    // Sets the result of a pre-checkout query; for bots only
    pub async fn answer_pre_checkout_query<C: AsRef<AnswerPreCheckoutQuery>>(
        &self,
        answer_pre_checkout_query: C,
    ) -> Result<Ok> {
        self.make_request(answer_pre_checkout_query).await
    }

    // Sets the result of a shipping query; for bots only
    pub async fn answer_shipping_query<C: AsRef<AnswerShippingQuery>>(
        &self,
        answer_shipping_query: C,
    ) -> Result<Ok> {
        self.make_request(answer_shipping_query).await
    }

    // Bans a member in a chat. Members can't be banned in private or secret chats. In supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first
    pub async fn ban_chat_member<C: AsRef<BanChatMember>>(&self, ban_chat_member: C) -> Result<Ok> {
        self.make_request(ban_chat_member).await
    }

    // Blocks an original sender of a message in the Replies chat
    pub async fn block_message_sender_from_replies<C: AsRef<BlockMessageSenderFromReplies>>(
        &self,
        block_message_sender_from_replies: C,
    ) -> Result<Ok> {
        self.make_request(block_message_sender_from_replies).await
    }

    // Checks whether the current session can be used to transfer a chat ownership to another user
    pub async fn can_transfer_ownership<C: AsRef<CanTransferOwnership>>(
        &self,
        can_transfer_ownership: C,
    ) -> Result<CanTransferOwnershipResult> {
        self.make_request(can_transfer_ownership).await
    }

    // Stops the downloading of a file. If a file has already been downloaded, does nothing
    pub async fn cancel_download_file<C: AsRef<CancelDownloadFile>>(
        &self,
        cancel_download_file: C,
    ) -> Result<Ok> {
        self.make_request(cancel_download_file).await
    }

    // Cancels reset of 2-step verification password. The method can be called if passwordState.pending_reset_date > 0
    pub async fn cancel_password_reset<C: AsRef<CancelPasswordReset>>(
        &self,
        cancel_password_reset: C,
    ) -> Result<Ok> {
        self.make_request(cancel_password_reset).await
    }

    // Stops the uploading of a file. Supported only for files uploaded by using uploadFile. For other files the behavior is undefined
    pub async fn cancel_upload_file<C: AsRef<CancelUploadFile>>(
        &self,
        cancel_upload_file: C,
    ) -> Result<Ok> {
        self.make_request(cancel_upload_file).await
    }

    // Changes imported contacts using the list of contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time
    pub async fn change_imported_contacts<C: AsRef<ChangeImportedContacts>>(
        &self,
        change_imported_contacts: C,
    ) -> Result<ImportedContacts> {
        self.make_request(change_imported_contacts).await
    }

    // Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code
    pub async fn change_phone_number<C: AsRef<ChangePhoneNumber>>(
        &self,
        change_phone_number: C,
    ) -> Result<AuthenticationCodeInfo> {
        self.make_request(change_phone_number).await
    }

    // Installs/uninstalls or activates/archives a sticker set
    pub async fn change_sticker_set<C: AsRef<ChangeStickerSet>>(
        &self,
        change_sticker_set: C,
    ) -> Result<Ok> {
        self.make_request(change_sticker_set).await
    }

    // Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in
    pub async fn check_authentication_bot_token<C: AsRef<CheckAuthenticationBotToken>>(
        &self,
        check_authentication_bot_token: C,
    ) -> Result<Ok> {
        self.make_request(check_authentication_bot_token).await
    }

    // Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode
    pub async fn check_authentication_code<C: AsRef<CheckAuthenticationCode>>(
        &self,
        check_authentication_code: C,
    ) -> Result<Ok> {
        self.make_request(check_authentication_code).await
    }

    // Checks the authentication password for correctness. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn check_authentication_password<C: AsRef<CheckAuthenticationPassword>>(
        &self,
        check_authentication_password: C,
    ) -> Result<Ok> {
        self.make_request(check_authentication_password).await
    }

    // Checks whether a password recovery code sent to an email address is valid. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn check_authentication_password_recovery_code<
        C: AsRef<CheckAuthenticationPasswordRecoveryCode>,
    >(
        &self,
        check_authentication_password_recovery_code: C,
    ) -> Result<Ok> {
        self.make_request(check_authentication_password_recovery_code)
            .await
    }

    // Checks the authentication code sent to confirm a new phone number of the user
    pub async fn check_change_phone_number_code<C: AsRef<CheckChangePhoneNumberCode>>(
        &self,
        check_change_phone_number_code: C,
    ) -> Result<Ok> {
        self.make_request(check_change_phone_number_code).await
    }

    // Checks the validity of an invite link for a chat and returns information about the corresponding chat
    pub async fn check_chat_invite_link<C: AsRef<CheckChatInviteLink>>(
        &self,
        check_chat_invite_link: C,
    ) -> Result<ChatInviteLinkInfo> {
        self.make_request(check_chat_invite_link).await
    }

    // Checks whether a username can be set for a chat
    pub async fn check_chat_username<C: AsRef<CheckChatUsername>>(
        &self,
        check_chat_username: C,
    ) -> Result<CheckChatUsernameResult> {
        self.make_request(check_chat_username).await
    }

    // Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached
    pub async fn check_created_public_chats_limit<C: AsRef<CheckCreatedPublicChatsLimit>>(
        &self,
        check_created_public_chats_limit: C,
    ) -> Result<Ok> {
        self.make_request(check_created_public_chats_limit).await
    }

    // Checks the database encryption key for correctness. Works only when the current authorization state is authorizationStateWaitEncryptionKey
    pub async fn check_database_encryption_key<C: AsRef<CheckDatabaseEncryptionKey>>(
        &self,
        check_database_encryption_key: C,
    ) -> Result<Ok> {
        self.make_request(check_database_encryption_key).await
    }

    // Checks the email address verification code for Telegram Passport
    pub async fn check_email_address_verification_code<
        C: AsRef<CheckEmailAddressVerificationCode>,
    >(
        &self,
        check_email_address_verification_code: C,
    ) -> Result<Ok> {
        self.make_request(check_email_address_verification_code)
            .await
    }

    // Checks whether a 2-step verification password recovery code sent to an email address is valid
    pub async fn check_password_recovery_code<C: AsRef<CheckPasswordRecoveryCode>>(
        &self,
        check_password_recovery_code: C,
    ) -> Result<Ok> {
        self.make_request(check_password_recovery_code).await
    }

    // Checks phone number confirmation code
    pub async fn check_phone_number_confirmation_code<
        C: AsRef<CheckPhoneNumberConfirmationCode>,
    >(
        &self,
        check_phone_number_confirmation_code: C,
    ) -> Result<Ok> {
        self.make_request(check_phone_number_confirmation_code)
            .await
    }

    // Checks the phone number verification code for Telegram Passport
    pub async fn check_phone_number_verification_code<
        C: AsRef<CheckPhoneNumberVerificationCode>,
    >(
        &self,
        check_phone_number_verification_code: C,
    ) -> Result<Ok> {
        self.make_request(check_phone_number_verification_code)
            .await
    }

    // Checks the 2-step verification recovery email address verification code
    pub async fn check_recovery_email_address_code<C: AsRef<CheckRecoveryEmailAddressCode>>(
        &self,
        check_recovery_email_address_code: C,
    ) -> Result<PasswordState> {
        self.make_request(check_recovery_email_address_code).await
    }

    // Checks whether a name can be used for a new sticker set
    pub async fn check_sticker_set_name<C: AsRef<CheckStickerSetName>>(
        &self,
        check_sticker_set_name: C,
    ) -> Result<CheckStickerSetNameResult> {
        self.make_request(check_sticker_set_name).await
    }

    // Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. Can be called synchronously
    pub async fn clean_file_name<C: AsRef<CleanFileName>>(
        &self,
        clean_file_name: C,
    ) -> Result<Text> {
        self.make_request(clean_file_name).await
    }

    // Clears draft messages in all chats
    pub async fn clear_all_draft_messages<C: AsRef<ClearAllDraftMessages>>(
        &self,
        clear_all_draft_messages: C,
    ) -> Result<Ok> {
        self.make_request(clear_all_draft_messages).await
    }

    // Clears all imported contacts, contact list remains unchanged
    pub async fn clear_imported_contacts<C: AsRef<ClearImportedContacts>>(
        &self,
        clear_imported_contacts: C,
    ) -> Result<Ok> {
        self.make_request(clear_imported_contacts).await
    }

    // Clears the list of recently used stickers
    pub async fn clear_recent_stickers<C: AsRef<ClearRecentStickers>>(
        &self,
        clear_recent_stickers: C,
    ) -> Result<Ok> {
        self.make_request(clear_recent_stickers).await
    }

    // Clears the list of recently found chats
    pub async fn clear_recently_found_chats<C: AsRef<ClearRecentlyFoundChats>>(
        &self,
        clear_recently_found_chats: C,
    ) -> Result<Ok> {
        self.make_request(clear_recently_found_chats).await
    }

    // Informs TDLib that a message with an animated emoji was clicked by the user. Returns a big animated sticker to be played or a 404 error if usual animation needs to be played
    pub async fn click_animated_emoji_message<C: AsRef<ClickAnimatedEmojiMessage>>(
        &self,
        click_animated_emoji_message: C,
    ) -> Result<Sticker> {
        self.make_request(click_animated_emoji_message).await
    }

    // Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, updateAuthorizationState with authorizationStateClosed will be sent. Can be called before initialization
    pub async fn close<C: AsRef<Close>>(&self, close: C) -> Result<Ok> {
        self.make_request(close).await
    }

    // Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed
    pub async fn close_chat<C: AsRef<CloseChat>>(&self, close_chat: C) -> Result<Ok> {
        self.make_request(close_chat).await
    }

    // Closes a secret chat, effectively transferring its state to secretChatStateClosed
    pub async fn close_secret_chat<C: AsRef<CloseSecretChat>>(
        &self,
        close_secret_chat: C,
    ) -> Result<Ok> {
        self.make_request(close_secret_chat).await
    }

    // Confirms QR code authentication on another device. Returns created session on success
    pub async fn confirm_qr_code_authentication<C: AsRef<ConfirmQrCodeAuthentication>>(
        &self,
        confirm_qr_code_authentication: C,
    ) -> Result<Session> {
        self.make_request(confirm_qr_code_authentication).await
    }

    // Returns an existing chat corresponding to a known basic group
    pub async fn create_basic_group_chat<C: AsRef<CreateBasicGroupChat>>(
        &self,
        create_basic_group_chat: C,
    ) -> Result<Chat> {
        self.make_request(create_basic_group_chat).await
    }

    // Creates a new call
    pub async fn create_call<C: AsRef<CreateCall>>(&self, create_call: C) -> Result<CallId> {
        self.make_request(create_call).await
    }

    // Creates new chat filter. Returns information about the created chat filter
    pub async fn create_chat_filter<C: AsRef<CreateChatFilter>>(
        &self,
        create_chat_filter: C,
    ) -> Result<ChatFilterInfo> {
        self.make_request(create_chat_filter).await
    }

    // Creates a new invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat
    pub async fn create_chat_invite_link<C: AsRef<CreateChatInviteLink>>(
        &self,
        create_chat_invite_link: C,
    ) -> Result<ChatInviteLink> {
        self.make_request(create_chat_invite_link).await
    }

    // Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns the newly created chat
    pub async fn create_new_basic_group_chat<C: AsRef<CreateNewBasicGroupChat>>(
        &self,
        create_new_basic_group_chat: C,
    ) -> Result<Chat> {
        self.make_request(create_new_basic_group_chat).await
    }

    // Creates a new secret chat. Returns the newly created chat
    pub async fn create_new_secret_chat<C: AsRef<CreateNewSecretChat>>(
        &self,
        create_new_secret_chat: C,
    ) -> Result<Chat> {
        self.make_request(create_new_secret_chat).await
    }

    // Creates a new sticker set. Returns the newly created sticker set
    pub async fn create_new_sticker_set<C: AsRef<CreateNewStickerSet>>(
        &self,
        create_new_sticker_set: C,
    ) -> Result<StickerSet> {
        self.make_request(create_new_sticker_set).await
    }

    // Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
    pub async fn create_new_supergroup_chat<C: AsRef<CreateNewSupergroupChat>>(
        &self,
        create_new_supergroup_chat: C,
    ) -> Result<Chat> {
        self.make_request(create_new_supergroup_chat).await
    }

    // Returns an existing chat corresponding to a given user
    pub async fn create_private_chat<C: AsRef<CreatePrivateChat>>(
        &self,
        create_private_chat: C,
    ) -> Result<Chat> {
        self.make_request(create_private_chat).await
    }

    // Returns an existing chat corresponding to a known secret chat
    pub async fn create_secret_chat<C: AsRef<CreateSecretChat>>(
        &self,
        create_secret_chat: C,
    ) -> Result<Chat> {
        self.make_request(create_secret_chat).await
    }

    // Returns an existing chat corresponding to a known supergroup or channel
    pub async fn create_supergroup_chat<C: AsRef<CreateSupergroupChat>>(
        &self,
        create_supergroup_chat: C,
    ) -> Result<Chat> {
        self.make_request(create_supergroup_chat).await
    }

    // Creates a new temporary password for processing payments
    pub async fn create_temporary_password<C: AsRef<CreateTemporaryPassword>>(
        &self,
        create_temporary_password: C,
    ) -> Result<TemporaryPasswordState> {
        self.make_request(create_temporary_password).await
    }

    // Creates a video chat (a group call bound to a chat). Available only for basic groups, supergroups and channels; requires can_manage_video_chats rights
    pub async fn create_video_chat<C: AsRef<CreateVideoChat>>(
        &self,
        create_video_chat: C,
    ) -> Result<GroupCallId> {
        self.make_request(create_video_chat).await
    }

    // Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is authorizationStateWaitPassword
    pub async fn delete_account<C: AsRef<DeleteAccount>>(&self, delete_account: C) -> Result<Ok> {
        self.make_request(delete_account).await
    }

    // Deletes all call messages
    pub async fn delete_all_call_messages<C: AsRef<DeleteAllCallMessages>>(
        &self,
        delete_all_call_messages: C,
    ) -> Result<Ok> {
        self.make_request(delete_all_call_messages).await
    }

    // Deletes all revoked chat invite links created by a given chat administrator. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
    pub async fn delete_all_revoked_chat_invite_links<C: AsRef<DeleteAllRevokedChatInviteLinks>>(
        &self,
        delete_all_revoked_chat_invite_links: C,
    ) -> Result<Ok> {
        self.make_request(delete_all_revoked_chat_invite_links)
            .await
    }

    // Deletes a chat along with all messages in the corresponding chat for all chat members; requires owner privileges. For group chats this will release the username and remove all members. Chats with more than 1000 members can't be deleted using this method
    pub async fn delete_chat<C: AsRef<DeleteChat>>(&self, delete_chat: C) -> Result<Ok> {
        self.make_request(delete_chat).await
    }

    // Deletes existing chat filter
    pub async fn delete_chat_filter<C: AsRef<DeleteChatFilter>>(
        &self,
        delete_chat_filter: C,
    ) -> Result<Ok> {
        self.make_request(delete_chat_filter).await
    }

    // Deletes all messages in the chat. Use chat.can_be_deleted_only_for_self and chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat
    pub async fn delete_chat_history<C: AsRef<DeleteChatHistory>>(
        &self,
        delete_chat_history: C,
    ) -> Result<Ok> {
        self.make_request(delete_chat_history).await
    }

    // Deletes all messages between the specified dates in a chat. Supported only for private chats and basic groups. Messages sent in the last 30 seconds will not be deleted
    pub async fn delete_chat_messages_by_date<C: AsRef<DeleteChatMessagesByDate>>(
        &self,
        delete_chat_messages_by_date: C,
    ) -> Result<Ok> {
        self.make_request(delete_chat_messages_by_date).await
    }

    // Deletes all messages sent by the specified message sender in a chat. Supported only for supergroups; requires can_delete_messages administrator privileges
    pub async fn delete_chat_messages_by_sender<C: AsRef<DeleteChatMessagesBySender>>(
        &self,
        delete_chat_messages_by_sender: C,
    ) -> Result<Ok> {
        self.make_request(delete_chat_messages_by_sender).await
    }

    // Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup is changed
    pub async fn delete_chat_reply_markup<C: AsRef<DeleteChatReplyMarkup>>(
        &self,
        delete_chat_reply_markup: C,
    ) -> Result<Ok> {
        self.make_request(delete_chat_reply_markup).await
    }

    // Deletes commands supported by the bot for the given user scope and language; for bots only
    pub async fn delete_commands<C: AsRef<DeleteCommands>>(
        &self,
        delete_commands: C,
    ) -> Result<Ok> {
        self.make_request(delete_commands).await
    }

    // Deletes a file from the TDLib file cache
    pub async fn delete_file<C: AsRef<DeleteFile>>(&self, delete_file: C) -> Result<Ok> {
        self.make_request(delete_file).await
    }

    // Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization
    pub async fn delete_language_pack<C: AsRef<DeleteLanguagePack>>(
        &self,
        delete_language_pack: C,
    ) -> Result<Ok> {
        self.make_request(delete_language_pack).await
    }

    // Deletes messages
    pub async fn delete_messages<C: AsRef<DeleteMessages>>(
        &self,
        delete_messages: C,
    ) -> Result<Ok> {
        self.make_request(delete_messages).await
    }

    // Deletes a Telegram Passport element
    pub async fn delete_passport_element<C: AsRef<DeletePassportElement>>(
        &self,
        delete_passport_element: C,
    ) -> Result<Ok> {
        self.make_request(delete_passport_element).await
    }

    // Deletes a profile photo
    pub async fn delete_profile_photo<C: AsRef<DeleteProfilePhoto>>(
        &self,
        delete_profile_photo: C,
    ) -> Result<Ok> {
        self.make_request(delete_profile_photo).await
    }

    // Deletes revoked chat invite links. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
    pub async fn delete_revoked_chat_invite_link<C: AsRef<DeleteRevokedChatInviteLink>>(
        &self,
        delete_revoked_chat_invite_link: C,
    ) -> Result<Ok> {
        self.make_request(delete_revoked_chat_invite_link).await
    }

    // Deletes saved credentials for all payment provider bots
    pub async fn delete_saved_credentials<C: AsRef<DeleteSavedCredentials>>(
        &self,
        delete_saved_credentials: C,
    ) -> Result<Ok> {
        self.make_request(delete_saved_credentials).await
    }

    // Deletes saved order info
    pub async fn delete_saved_order_info<C: AsRef<DeleteSavedOrderInfo>>(
        &self,
        delete_saved_order_info: C,
    ) -> Result<Ok> {
        self.make_request(delete_saved_order_info).await
    }

    // Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent. Can be called before authorization
    pub async fn destroy<C: AsRef<Destroy>>(&self, destroy: C) -> Result<Ok> {
        self.make_request(destroy).await
    }

    // Disables the currently enabled proxy. Can be called before authorization
    pub async fn disable_proxy<C: AsRef<DisableProxy>>(&self, disable_proxy: C) -> Result<Ok> {
        self.make_request(disable_proxy).await
    }

    // Discards a call
    pub async fn discard_call<C: AsRef<DiscardCall>>(&self, discard_call: C) -> Result<Ok> {
        self.make_request(discard_call).await
    }

    // Disconnects all websites from the current user's Telegram account
    pub async fn disconnect_all_websites<C: AsRef<DisconnectAllWebsites>>(
        &self,
        disconnect_all_websites: C,
    ) -> Result<Ok> {
        self.make_request(disconnect_all_websites).await
    }

    // Disconnects website from the current user's Telegram account
    pub async fn disconnect_website<C: AsRef<DisconnectWebsite>>(
        &self,
        disconnect_website: C,
    ) -> Result<Ok> {
        self.make_request(disconnect_website).await
    }

    // Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates
    pub async fn download_file<C: AsRef<DownloadFile>>(&self, download_file: C) -> Result<File> {
        self.make_request(download_file).await
    }

    // Edits existing chat filter. Returns information about the edited chat filter
    pub async fn edit_chat_filter<C: AsRef<EditChatFilter>>(
        &self,
        edit_chat_filter: C,
    ) -> Result<ChatFilterInfo> {
        self.make_request(edit_chat_filter).await
    }

    // Edits a non-primary invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
    pub async fn edit_chat_invite_link<C: AsRef<EditChatInviteLink>>(
        &self,
        edit_chat_invite_link: C,
    ) -> Result<ChatInviteLink> {
        self.make_request(edit_chat_invite_link).await
    }

    // Edits information about a custom local language pack in the current localization target. Can be called before authorization
    pub async fn edit_custom_language_pack_info<C: AsRef<EditCustomLanguagePackInfo>>(
        &self,
        edit_custom_language_pack_info: C,
    ) -> Result<Ok> {
        self.make_request(edit_custom_language_pack_info).await
    }

    // Edits the caption of an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_caption<C: AsRef<EditInlineMessageCaption>>(
        &self,
        edit_inline_message_caption: C,
    ) -> Result<Ok> {
        self.make_request(edit_inline_message_caption).await
    }

    // Edits the content of a live location in an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_live_location<C: AsRef<EditInlineMessageLiveLocation>>(
        &self,
        edit_inline_message_live_location: C,
    ) -> Result<Ok> {
        self.make_request(edit_inline_message_live_location).await
    }

    // Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_media<C: AsRef<EditInlineMessageMedia>>(
        &self,
        edit_inline_message_media: C,
    ) -> Result<Ok> {
        self.make_request(edit_inline_message_media).await
    }

    // Edits the reply markup of an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_reply_markup<C: AsRef<EditInlineMessageReplyMarkup>>(
        &self,
        edit_inline_message_reply_markup: C,
    ) -> Result<Ok> {
        self.make_request(edit_inline_message_reply_markup).await
    }

    // Edits the text of an inline text or game message sent via a bot; for bots only
    pub async fn edit_inline_message_text<C: AsRef<EditInlineMessageText>>(
        &self,
        edit_inline_message_text: C,
    ) -> Result<Ok> {
        self.make_request(edit_inline_message_text).await
    }

    // Edits the message content caption. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_caption<C: AsRef<EditMessageCaption>>(
        &self,
        edit_message_caption: C,
    ) -> Result<Message> {
        self.make_request(edit_message_caption).await
    }

    // Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_live_location<C: AsRef<EditMessageLiveLocation>>(
        &self,
        edit_message_live_location: C,
    ) -> Result<Message> {
        self.make_request(edit_message_live_location).await
    }

    // Edits the content of a message with an animation, an audio, a document, a photo or a video, including message caption. If only the caption needs to be edited, use editMessageCaption instead. The media can't be edited if the message was set to self-destruct or to a self-destructing media. The type of message content in an album can't be changed with exception of replacing a photo with a video or vice versa. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_media<C: AsRef<EditMessageMedia>>(
        &self,
        edit_message_media: C,
    ) -> Result<Message> {
        self.make_request(edit_message_media).await
    }

    // Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_reply_markup<C: AsRef<EditMessageReplyMarkup>>(
        &self,
        edit_message_reply_markup: C,
    ) -> Result<Message> {
        self.make_request(edit_message_reply_markup).await
    }

    // Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed
    pub async fn edit_message_scheduling_state<C: AsRef<EditMessageSchedulingState>>(
        &self,
        edit_message_scheduling_state: C,
    ) -> Result<Ok> {
        self.make_request(edit_message_scheduling_state).await
    }

    // Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_text<C: AsRef<EditMessageText>>(
        &self,
        edit_message_text: C,
    ) -> Result<Message> {
        self.make_request(edit_message_text).await
    }

    // Edits an existing proxy server for network requests. Can be called before authorization
    pub async fn edit_proxy<C: AsRef<EditProxy>>(&self, edit_proxy: C) -> Result<Proxy> {
        self.make_request(edit_proxy).await
    }

    // Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization
    pub async fn enable_proxy<C: AsRef<EnableProxy>>(&self, enable_proxy: C) -> Result<Ok> {
        self.make_request(enable_proxy).await
    }

    // Ends a group call. Requires groupCall.can_be_managed
    pub async fn end_group_call<C: AsRef<EndGroupCall>>(&self, end_group_call: C) -> Result<Ok> {
        self.make_request(end_group_call).await
    }

    // Ends recording of an active group call. Requires groupCall.can_be_managed group call flag
    pub async fn end_group_call_recording<C: AsRef<EndGroupCallRecording>>(
        &self,
        end_group_call_recording: C,
    ) -> Result<Ok> {
        self.make_request(end_group_call_recording).await
    }

    // Ends screen sharing in a joined group call
    pub async fn end_group_call_screen_sharing<C: AsRef<EndGroupCallScreenSharing>>(
        &self,
        end_group_call_screen_sharing: C,
    ) -> Result<Ok> {
        self.make_request(end_group_call_screen_sharing).await
    }

    // Finishes the file generation
    pub async fn finish_file_generation<C: AsRef<FinishFileGeneration>>(
        &self,
        finish_file_generation: C,
    ) -> Result<Ok> {
        self.make_request(finish_file_generation).await
    }

    // Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message
    pub async fn forward_messages<C: AsRef<ForwardMessages>>(
        &self,
        forward_messages: C,
    ) -> Result<Messages> {
        self.make_request(forward_messages).await
    }

    // Returns the period of inactivity after which the account of the current user will automatically be deleted
    pub async fn get_account_ttl<C: AsRef<GetAccountTtl>>(
        &self,
        get_account_ttl: C,
    ) -> Result<AccountTtl> {
        self.make_request(get_account_ttl).await
    }

    // Returns all active live locations that need to be updated by the application. The list is persistent across application restarts only if the message database is used
    pub async fn get_active_live_location_messages<C: AsRef<GetActiveLiveLocationMessages>>(
        &self,
        get_active_live_location_messages: C,
    ) -> Result<Messages> {
        self.make_request(get_active_live_location_messages).await
    }

    // Returns all active sessions of the current user
    pub async fn get_active_sessions<C: AsRef<GetActiveSessions>>(
        &self,
        get_active_sessions: C,
    ) -> Result<Sessions> {
        self.make_request(get_active_sessions).await
    }

    // Returns all available Telegram Passport elements
    pub async fn get_all_passport_elements<C: AsRef<GetAllPassportElements>>(
        &self,
        get_all_passport_elements: C,
    ) -> Result<PassportElements> {
        self.make_request(get_all_passport_elements).await
    }

    // Returns an animated emoji corresponding to a given emoji. Returns a 404 error if the emoji has no animated emoji
    pub async fn get_animated_emoji<C: AsRef<GetAnimatedEmoji>>(
        &self,
        get_animated_emoji: C,
    ) -> Result<AnimatedEmoji> {
        self.make_request(get_animated_emoji).await
    }

    // Returns application config, provided by the server. Can be called before authorization
    pub async fn get_application_config<C: AsRef<GetApplicationConfig>>(
        &self,
        get_application_config: C,
    ) -> Result<JsonValue> {
        self.make_request(get_application_config).await
    }

    // Returns the link for downloading official Telegram application to be used when the current user invites friends to Telegram
    pub async fn get_application_download_link<C: AsRef<GetApplicationDownloadLink>>(
        &self,
        get_application_download_link: C,
    ) -> Result<HttpUrl> {
        self.make_request(get_application_download_link).await
    }

    // Returns a list of archived sticker sets
    pub async fn get_archived_sticker_sets<C: AsRef<GetArchivedStickerSets>>(
        &self,
        get_archived_sticker_sets: C,
    ) -> Result<StickerSets> {
        self.make_request(get_archived_sticker_sets).await
    }

    // Returns a list of sticker sets attached to a file. Currently, only photos and videos can have attached sticker sets
    pub async fn get_attached_sticker_sets<C: AsRef<GetAttachedStickerSets>>(
        &self,
        get_attached_sticker_sets: C,
    ) -> Result<StickerSets> {
        self.make_request(get_attached_sticker_sets).await
    }

    // Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state. Can be called before initialization
    pub async fn get_authorization_state<C: AsRef<GetAuthorizationState>>(
        &self,
        get_authorization_state: C,
    ) -> Result<AuthorizationState> {
        self.make_request(get_authorization_state).await
    }

    // Returns auto-download settings presets for the current user
    pub async fn get_auto_download_settings_presets<C: AsRef<GetAutoDownloadSettingsPresets>>(
        &self,
        get_auto_download_settings_presets: C,
    ) -> Result<AutoDownloadSettingsPresets> {
        self.make_request(get_auto_download_settings_presets).await
    }

    // Constructs a persistent HTTP URL for a background
    pub async fn get_background_url<C: AsRef<GetBackgroundUrl>>(
        &self,
        get_background_url: C,
    ) -> Result<HttpUrl> {
        self.make_request(get_background_url).await
    }

    // Returns backgrounds installed by the user
    pub async fn get_backgrounds<C: AsRef<GetBackgrounds>>(
        &self,
        get_backgrounds: C,
    ) -> Result<Backgrounds> {
        self.make_request(get_backgrounds).await
    }

    // Returns information about a bank card
    pub async fn get_bank_card_info<C: AsRef<GetBankCardInfo>>(
        &self,
        get_bank_card_info: C,
    ) -> Result<BankCardInfo> {
        self.make_request(get_bank_card_info).await
    }

    // Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot
    pub async fn get_basic_group<C: AsRef<GetBasicGroup>>(
        &self,
        get_basic_group: C,
    ) -> Result<BasicGroup> {
        self.make_request(get_basic_group).await
    }

    // Returns full information about a basic group by its identifier
    pub async fn get_basic_group_full_info<C: AsRef<GetBasicGroupFullInfo>>(
        &self,
        get_basic_group_full_info: C,
    ) -> Result<BasicGroupFullInfo> {
        self.make_request(get_basic_group_full_info).await
    }

    // Returns users and chats that were blocked by the current user
    pub async fn get_blocked_message_senders<C: AsRef<GetBlockedMessageSenders>>(
        &self,
        get_blocked_message_senders: C,
    ) -> Result<MessageSenders> {
        self.make_request(get_blocked_message_senders).await
    }

    // Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
    pub async fn get_callback_query_answer<C: AsRef<GetCallbackQueryAnswer>>(
        &self,
        get_callback_query_answer: C,
    ) -> Result<CallbackQueryAnswer> {
        self.make_request(get_callback_query_answer).await
    }

    // Returns information about a message with the callback button that originated a callback query; for bots only
    pub async fn get_callback_query_message<C: AsRef<GetCallbackQueryMessage>>(
        &self,
        get_callback_query_message: C,
    ) -> Result<Message> {
        self.make_request(get_callback_query_message).await
    }

    // Returns information about a chat by its identifier, this is an offline request if the current user is not a bot
    pub async fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> Result<Chat> {
        self.make_request(get_chat).await
    }

    // Returns a list of administrators of the chat with their custom titles
    pub async fn get_chat_administrators<C: AsRef<GetChatAdministrators>>(
        &self,
        get_chat_administrators: C,
    ) -> Result<ChatAdministrators> {
        self.make_request(get_chat_administrators).await
    }

    // Returns list of message sender identifiers, which can be used to send messages in a chat
    pub async fn get_chat_available_message_senders<C: AsRef<GetChatAvailableMessageSenders>>(
        &self,
        get_chat_available_message_senders: C,
    ) -> Result<MessageSenders> {
        self.make_request(get_chat_available_message_senders).await
    }

    // Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only for supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id)
    pub async fn get_chat_event_log<C: AsRef<GetChatEventLog>>(
        &self,
        get_chat_event_log: C,
    ) -> Result<ChatEvents> {
        self.make_request(get_chat_event_log).await
    }

    // Returns information about a chat filter by its identifier
    pub async fn get_chat_filter<C: AsRef<GetChatFilter>>(
        &self,
        get_chat_filter: C,
    ) -> Result<ChatFilter> {
        self.make_request(get_chat_filter).await
    }

    // Returns default icon name for a filter. Can be called synchronously
    pub async fn get_chat_filter_default_icon_name<C: AsRef<GetChatFilterDefaultIconName>>(
        &self,
        get_chat_filter_default_icon_name: C,
    ) -> Result<Text> {
        self.make_request(get_chat_filter_default_icon_name).await
    }

    // Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib. This is an offline request if only_local is true
    pub async fn get_chat_history<C: AsRef<GetChatHistory>>(
        &self,
        get_chat_history: C,
    ) -> Result<Messages> {
        self.make_request(get_chat_history).await
    }

    // Returns information about an invite link. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
    pub async fn get_chat_invite_link<C: AsRef<GetChatInviteLink>>(
        &self,
        get_chat_invite_link: C,
    ) -> Result<ChatInviteLink> {
        self.make_request(get_chat_invite_link).await
    }

    // Returns list of chat administrators with number of their invite links. Requires owner privileges in the chat
    pub async fn get_chat_invite_link_counts<C: AsRef<GetChatInviteLinkCounts>>(
        &self,
        get_chat_invite_link_counts: C,
    ) -> Result<ChatInviteLinkCounts> {
        self.make_request(get_chat_invite_link_counts).await
    }

    // Returns chat members joined a chat via an invite link. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
    pub async fn get_chat_invite_link_members<C: AsRef<GetChatInviteLinkMembers>>(
        &self,
        get_chat_invite_link_members: C,
    ) -> Result<ChatInviteLinkMembers> {
        self.make_request(get_chat_invite_link_members).await
    }

    // Returns invite links for a chat created by specified administrator. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
    pub async fn get_chat_invite_links<C: AsRef<GetChatInviteLinks>>(
        &self,
        get_chat_invite_links: C,
    ) -> Result<ChatInviteLinks> {
        self.make_request(get_chat_invite_links).await
    }

    // Returns pending join requests in a chat
    pub async fn get_chat_join_requests<C: AsRef<GetChatJoinRequests>>(
        &self,
        get_chat_join_requests: C,
    ) -> Result<ChatJoinRequests> {
        self.make_request(get_chat_join_requests).await
    }

    // Returns chat lists to which the chat can be added. This is an offline request
    pub async fn get_chat_lists_to_add_chat<C: AsRef<GetChatListsToAddChat>>(
        &self,
        get_chat_lists_to_add_chat: C,
    ) -> Result<ChatLists> {
        self.make_request(get_chat_lists_to_add_chat).await
    }

    // Returns information about a single member of a chat
    pub async fn get_chat_member<C: AsRef<GetChatMember>>(
        &self,
        get_chat_member: C,
    ) -> Result<ChatMember> {
        self.make_request(get_chat_member).await
    }

    // Returns the last message sent in a chat no later than the specified date
    pub async fn get_chat_message_by_date<C: AsRef<GetChatMessageByDate>>(
        &self,
        get_chat_message_by_date: C,
    ) -> Result<Message> {
        self.make_request(get_chat_message_by_date).await
    }

    // Returns information about the next messages of the specified type in the chat split by days. Returns the results in reverse chronological order. Can return partial result for the last returned day. Behavior of this method depends on the value of the option "utc_time_offset"
    pub async fn get_chat_message_calendar<C: AsRef<GetChatMessageCalendar>>(
        &self,
        get_chat_message_calendar: C,
    ) -> Result<MessageCalendar> {
        self.make_request(get_chat_message_calendar).await
    }

    // Returns approximate number of messages of the specified type in the chat
    pub async fn get_chat_message_count<C: AsRef<GetChatMessageCount>>(
        &self,
        get_chat_message_count: C,
    ) -> Result<Count> {
        self.make_request(get_chat_message_count).await
    }

    // Returns list of chats with non-default notification settings
    pub async fn get_chat_notification_settings_exceptions<
        C: AsRef<GetChatNotificationSettingsExceptions>,
    >(
        &self,
        get_chat_notification_settings_exceptions: C,
    ) -> Result<Chats> {
        self.make_request(get_chat_notification_settings_exceptions)
            .await
    }

    // Returns information about a newest pinned message in the chat
    pub async fn get_chat_pinned_message<C: AsRef<GetChatPinnedMessage>>(
        &self,
        get_chat_pinned_message: C,
    ) -> Result<Message> {
        self.make_request(get_chat_pinned_message).await
    }

    // Returns all scheduled messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id)
    pub async fn get_chat_scheduled_messages<C: AsRef<GetChatScheduledMessages>>(
        &self,
        get_chat_scheduled_messages: C,
    ) -> Result<Messages> {
        self.make_request(get_chat_scheduled_messages).await
    }

    // Returns sparse positions of messages of the specified type in the chat to be used for shared media scroll implementation. Returns the results in reverse chronological order (i.e., in order of decreasing message_id). Cannot be used in secret chats or with searchMessagesFilterFailedToSend filter without an enabled message database
    pub async fn get_chat_sparse_message_positions<C: AsRef<GetChatSparseMessagePositions>>(
        &self,
        get_chat_sparse_message_positions: C,
    ) -> Result<MessagePositions> {
        self.make_request(get_chat_sparse_message_positions).await
    }

    // Returns sponsored message to be shown in a chat; for channel chats only. Returns a 404 error if there is no sponsored message in the chat
    pub async fn get_chat_sponsored_message<C: AsRef<GetChatSponsoredMessage>>(
        &self,
        get_chat_sponsored_message: C,
    ) -> Result<SponsoredMessage> {
        self.make_request(get_chat_sponsored_message).await
    }

    // Returns detailed statistics about a chat. Currently, this method can be used only for supergroups and channels. Can be used only if supergroupFullInfo.can_get_statistics == true
    pub async fn get_chat_statistics<C: AsRef<GetChatStatistics>>(
        &self,
        get_chat_statistics: C,
    ) -> Result<ChatStatistics> {
        self.make_request(get_chat_statistics).await
    }

    // Returns an ordered list of chats from the beginning of a chat list. For informational purposes only. Use loadChats and updates processing instead to maintain chat lists in a consistent state
    pub async fn get_chats<C: AsRef<GetChats>>(&self, get_chats: C) -> Result<Chats> {
        self.make_request(get_chats).await
    }

    // Returns the list of commands supported by the bot for the given user scope and language; for bots only
    pub async fn get_commands<C: AsRef<GetCommands>>(
        &self,
        get_commands: C,
    ) -> Result<BotCommands> {
        self.make_request(get_commands).await
    }

    // Returns all website where the current user used Telegram to log in
    pub async fn get_connected_websites<C: AsRef<GetConnectedWebsites>>(
        &self,
        get_connected_websites: C,
    ) -> Result<ConnectedWebsites> {
        self.make_request(get_connected_websites).await
    }

    // Returns all user contacts
    pub async fn get_contacts<C: AsRef<GetContacts>>(&self, get_contacts: C) -> Result<Users> {
        self.make_request(get_contacts).await
    }

    // Returns information about existing countries. Can be called before authorization
    pub async fn get_countries<C: AsRef<GetCountries>>(
        &self,
        get_countries: C,
    ) -> Result<Countries> {
        self.make_request(get_countries).await
    }

    // Uses the current IP address to find the current country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization
    pub async fn get_country_code<C: AsRef<GetCountryCode>>(
        &self,
        get_country_code: C,
    ) -> Result<Text> {
        self.make_request(get_country_code).await
    }

    // Returns a list of public chats of the specified type, owned by the user
    pub async fn get_created_public_chats<C: AsRef<GetCreatedPublicChats>>(
        &self,
        get_created_public_chats: C,
    ) -> Result<Chats> {
        self.make_request(get_created_public_chats).await
    }

    // Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially useful if TDLib is run in a separate process. Can be called before initialization
    pub async fn get_current_state<C: AsRef<GetCurrentState>>(
        &self,
        get_current_state: C,
    ) -> Result<Updates> {
        self.make_request(get_current_state).await
    }

    // Returns database statistics
    pub async fn get_database_statistics<C: AsRef<GetDatabaseStatistics>>(
        &self,
        get_database_statistics: C,
    ) -> Result<DatabaseStatistics> {
        self.make_request(get_database_statistics).await
    }

    // Returns information about a tg:// deep link. Use "tg://need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization
    pub async fn get_deep_link_info<C: AsRef<GetDeepLinkInfo>>(
        &self,
        get_deep_link_info: C,
    ) -> Result<DeepLinkInfo> {
        self.make_request(get_deep_link_info).await
    }

    // Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation
    pub async fn get_emoji_suggestions_url<C: AsRef<GetEmojiSuggestionsUrl>>(
        &self,
        get_emoji_suggestions_url: C,
    ) -> Result<HttpUrl> {
        self.make_request(get_emoji_suggestions_url).await
    }

    // Returns an HTTP URL which can be used to automatically authorize the current user on a website after clicking an HTTP link. Use the method getExternalLinkInfo to find whether a prior user confirmation is needed
    pub async fn get_external_link<C: AsRef<GetExternalLink>>(
        &self,
        get_external_link: C,
    ) -> Result<HttpUrl> {
        self.make_request(get_external_link).await
    }

    // Returns information about an action to be done when the current user clicks an external link. Don't use this method for links from secret chats if web page preview is disabled in secret chats
    pub async fn get_external_link_info<C: AsRef<GetExternalLinkInfo>>(
        &self,
        get_external_link_info: C,
    ) -> Result<LoginUrlInfo> {
        self.make_request(get_external_link_info).await
    }

    // Returns favorite stickers
    pub async fn get_favorite_stickers<C: AsRef<GetFavoriteStickers>>(
        &self,
        get_favorite_stickers: C,
    ) -> Result<Stickers> {
        self.make_request(get_favorite_stickers).await
    }

    // Returns information about a file; this is an offline request
    pub async fn get_file<C: AsRef<GetFile>>(&self, get_file: C) -> Result<File> {
        self.make_request(get_file).await
    }

    // Returns file downloaded prefix size from a given offset, in bytes
    pub async fn get_file_downloaded_prefix_size<C: AsRef<GetFileDownloadedPrefixSize>>(
        &self,
        get_file_downloaded_prefix_size: C,
    ) -> Result<Count> {
        self.make_request(get_file_downloaded_prefix_size).await
    }

    // Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. Can be called synchronously
    pub async fn get_file_extension<C: AsRef<GetFileExtension>>(
        &self,
        get_file_extension: C,
    ) -> Result<Text> {
        self.make_request(get_file_extension).await
    }

    // Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. Can be called synchronously
    pub async fn get_file_mime_type<C: AsRef<GetFileMimeType>>(
        &self,
        get_file_mime_type: C,
    ) -> Result<Text> {
        self.make_request(get_file_mime_type).await
    }

    // Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only
    pub async fn get_game_high_scores<C: AsRef<GetGameHighScores>>(
        &self,
        get_game_high_scores: C,
    ) -> Result<GameHighScores> {
        self.make_request(get_game_high_scores).await
    }

    // Returns information about a group call
    pub async fn get_group_call<C: AsRef<GetGroupCall>>(
        &self,
        get_group_call: C,
    ) -> Result<GroupCall> {
        self.make_request(get_group_call).await
    }

    // Returns invite link to a video chat in a public chat
    pub async fn get_group_call_invite_link<C: AsRef<GetGroupCallInviteLink>>(
        &self,
        get_group_call_invite_link: C,
    ) -> Result<HttpUrl> {
        self.make_request(get_group_call_invite_link).await
    }

    // Returns a file with a segment of a group call stream in a modified OGG format for audio or MPEG-4 format for video
    pub async fn get_group_call_stream_segment<C: AsRef<GetGroupCallStreamSegment>>(
        &self,
        get_group_call_stream_segment: C,
    ) -> Result<FilePart> {
        self.make_request(get_group_call_stream_segment).await
    }

    // Returns a list of common group chats with a given user. Chats are sorted by their type and creation date
    pub async fn get_groups_in_common<C: AsRef<GetGroupsInCommon>>(
        &self,
        get_groups_in_common: C,
    ) -> Result<Chats> {
        self.make_request(get_groups_in_common).await
    }

    // Returns the total number of imported contacts
    pub async fn get_imported_contact_count<C: AsRef<GetImportedContactCount>>(
        &self,
        get_imported_contact_count: C,
    ) -> Result<Count> {
        self.make_request(get_imported_contact_count).await
    }

    // Returns a list of recently inactive supergroups and channels. Can be used when user reaches limit on the number of joined supergroups and channels and receives CHANNELS_TOO_MUCH error
    pub async fn get_inactive_supergroup_chats<C: AsRef<GetInactiveSupergroupChats>>(
        &self,
        get_inactive_supergroup_chats: C,
    ) -> Result<Chats> {
        self.make_request(get_inactive_supergroup_chats).await
    }

    // Returns game high scores and some part of the high score table in the range of the specified user; for bots only
    pub async fn get_inline_game_high_scores<C: AsRef<GetInlineGameHighScores>>(
        &self,
        get_inline_game_high_scores: C,
    ) -> Result<GameHighScores> {
        self.make_request(get_inline_game_high_scores).await
    }

    // Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
    pub async fn get_inline_query_results<C: AsRef<GetInlineQueryResults>>(
        &self,
        get_inline_query_results: C,
    ) -> Result<InlineQueryResults> {
        self.make_request(get_inline_query_results).await
    }

    // Returns a list of installed sticker sets
    pub async fn get_installed_sticker_sets<C: AsRef<GetInstalledStickerSets>>(
        &self,
        get_installed_sticker_sets: C,
    ) -> Result<StickerSets> {
        self.make_request(get_installed_sticker_sets).await
    }

    // Returns information about the type of an internal link. Returns a 404 error if the link is not internal. Can be called before authorization
    pub async fn get_internal_link_type<C: AsRef<GetInternalLinkType>>(
        &self,
        get_internal_link_type: C,
    ) -> Result<InternalLinkType> {
        self.make_request(get_internal_link_type).await
    }

    // Converts a JsonValue object to corresponding JSON-serialized string. Can be called synchronously
    pub async fn get_json_string<C: AsRef<GetJsonString>>(
        &self,
        get_json_string: C,
    ) -> Result<Text> {
        self.make_request(get_json_string).await
    }

    // Converts a JSON-serialized string to corresponding JsonValue object. Can be called synchronously
    pub async fn get_json_value<C: AsRef<GetJsonValue>>(
        &self,
        get_json_value: C,
    ) -> Result<JsonValue> {
        self.make_request(get_json_value).await
    }

    // Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization
    pub async fn get_language_pack_info<C: AsRef<GetLanguagePackInfo>>(
        &self,
        get_language_pack_info: C,
    ) -> Result<LanguagePackInfo> {
        self.make_request(get_language_pack_info).await
    }

    // Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. Can be called synchronously
    pub async fn get_language_pack_string<C: AsRef<GetLanguagePackString>>(
        &self,
        get_language_pack_string: C,
    ) -> Result<LanguagePackStringValue> {
        self.make_request(get_language_pack_string).await
    }

    // Returns strings from a language pack in the current localization target by their keys. Can be called before authorization
    pub async fn get_language_pack_strings<C: AsRef<GetLanguagePackStrings>>(
        &self,
        get_language_pack_strings: C,
    ) -> Result<LanguagePackStrings> {
        self.make_request(get_language_pack_strings).await
    }

    // Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization
    pub async fn get_localization_target_info<C: AsRef<GetLocalizationTargetInfo>>(
        &self,
        get_localization_target_info: C,
    ) -> Result<LocalizationTargetInfo> {
        self.make_request(get_localization_target_info).await
    }

    // Returns information about currently used log stream for internal logging of TDLib. Can be called synchronously
    pub async fn get_log_stream<C: AsRef<GetLogStream>>(
        &self,
        get_log_stream: C,
    ) -> Result<LogStream> {
        self.make_request(get_log_stream).await
    }

    // Returns current verbosity level for a specified TDLib internal log tag. Can be called synchronously
    pub async fn get_log_tag_verbosity_level<C: AsRef<GetLogTagVerbosityLevel>>(
        &self,
        get_log_tag_verbosity_level: C,
    ) -> Result<LogVerbosityLevel> {
        self.make_request(get_log_tag_verbosity_level).await
    }

    // Returns list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. Can be called synchronously
    pub async fn get_log_tags<C: AsRef<GetLogTags>>(&self, get_log_tags: C) -> Result<LogTags> {
        self.make_request(get_log_tags).await
    }

    // Returns current verbosity level of the internal logging of TDLib. Can be called synchronously
    pub async fn get_log_verbosity_level<C: AsRef<GetLogVerbosityLevel>>(
        &self,
        get_log_verbosity_level: C,
    ) -> Result<LogVerbosityLevel> {
        self.make_request(get_log_verbosity_level).await
    }

    // Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl. Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button
    pub async fn get_login_url<C: AsRef<GetLoginUrl>>(&self, get_login_url: C) -> Result<HttpUrl> {
        self.make_request(get_login_url).await
    }

    // Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
    pub async fn get_login_url_info<C: AsRef<GetLoginUrlInfo>>(
        &self,
        get_login_url_info: C,
    ) -> Result<LoginUrlInfo> {
        self.make_request(get_login_url_info).await
    }

    // Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded
    pub async fn get_map_thumbnail_file<C: AsRef<GetMapThumbnailFile>>(
        &self,
        get_map_thumbnail_file: C,
    ) -> Result<File> {
        self.make_request(get_map_thumbnail_file).await
    }

    // Replaces text entities with Markdown formatting in a human-friendly format. Entities that can't be represented in Markdown unambiguously are kept as is. Can be called synchronously
    pub async fn get_markdown_text<C: AsRef<GetMarkdownText>>(
        &self,
        get_markdown_text: C,
    ) -> Result<FormattedText> {
        self.make_request(get_markdown_text).await
    }

    // Returns the current user
    pub async fn get_me<C: AsRef<GetMe>>(&self, get_me: C) -> Result<User> {
        self.make_request(get_me).await
    }

    // Returns information about a message
    pub async fn get_message<C: AsRef<GetMessage>>(&self, get_message: C) -> Result<Message> {
        self.make_request(get_message).await
    }

    // Returns an HTML code for embedding the message. Available only for messages in supergroups and channels with a username
    pub async fn get_message_embedding_code<C: AsRef<GetMessageEmbeddingCode>>(
        &self,
        get_message_embedding_code: C,
    ) -> Result<Text> {
        self.make_request(get_message_embedding_code).await
    }

    // Returns information about a file with messages exported from another app
    pub async fn get_message_file_type<C: AsRef<GetMessageFileType>>(
        &self,
        get_message_file_type: C,
    ) -> Result<MessageFileType> {
        self.make_request(get_message_file_type).await
    }

    // Returns a confirmation text to be shown to the user before starting message import
    pub async fn get_message_import_confirmation_text<
        C: AsRef<GetMessageImportConfirmationText>,
    >(
        &self,
        get_message_import_confirmation_text: C,
    ) -> Result<Text> {
        self.make_request(get_message_import_confirmation_text)
            .await
    }

    // Returns an HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels, or if message.can_get_media_timestamp_links and a media timestamp link is generated. This is an offline request
    pub async fn get_message_link<C: AsRef<GetMessageLink>>(
        &self,
        get_message_link: C,
    ) -> Result<MessageLink> {
        self.make_request(get_message_link).await
    }

    // Returns information about a public or private message link. Can be called for any internal link of the type internalLinkTypeMessage
    pub async fn get_message_link_info<C: AsRef<GetMessageLinkInfo>>(
        &self,
        get_message_link_info: C,
    ) -> Result<MessageLinkInfo> {
        self.make_request(get_message_link_info).await
    }

    // Returns information about a message, if it is available locally without sending network request. This is an offline request
    pub async fn get_message_locally<C: AsRef<GetMessageLocally>>(
        &self,
        get_message_locally: C,
    ) -> Result<Message> {
        self.make_request(get_message_locally).await
    }

    // Returns forwarded copies of a channel message to different public channels. For optimal performance, the number of returned messages is chosen by TDLib
    pub async fn get_message_public_forwards<C: AsRef<GetMessagePublicForwards>>(
        &self,
        get_message_public_forwards: C,
    ) -> Result<FoundMessages> {
        self.make_request(get_message_public_forwards).await
    }

    // Returns detailed statistics about a message. Can be used only if message.can_get_statistics == true
    pub async fn get_message_statistics<C: AsRef<GetMessageStatistics>>(
        &self,
        get_message_statistics: C,
    ) -> Result<MessageStatistics> {
        self.make_request(get_message_statistics).await
    }

    // Returns information about a message thread. Can be used only if message.can_get_message_thread == true
    pub async fn get_message_thread<C: AsRef<GetMessageThread>>(
        &self,
        get_message_thread: C,
    ) -> Result<MessageThreadInfo> {
        self.make_request(get_message_thread).await
    }

    // Returns messages in a message thread of a message. Can be used only if message.can_get_message_thread == true. Message thread of a channel message is in the channel's linked supergroup. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
    pub async fn get_message_thread_history<C: AsRef<GetMessageThreadHistory>>(
        &self,
        get_message_thread_history: C,
    ) -> Result<Messages> {
        self.make_request(get_message_thread_history).await
    }

    // Returns viewers of a recent outgoing message in a basic group or a supergroup chat. For video notes and voice notes only users, opened content of the message, are returned. The method can be called if message.can_get_viewers == true
    pub async fn get_message_viewers<C: AsRef<GetMessageViewers>>(
        &self,
        get_message_viewers: C,
    ) -> Result<Users> {
        self.make_request(get_message_viewers).await
    }

    // Returns information about messages. If a message is not found, returns null on the corresponding position of the result
    pub async fn get_messages<C: AsRef<GetMessages>>(&self, get_messages: C) -> Result<Messages> {
        self.make_request(get_messages).await
    }

    // Returns network data usage statistics. Can be called before authorization
    pub async fn get_network_statistics<C: AsRef<GetNetworkStatistics>>(
        &self,
        get_network_statistics: C,
    ) -> Result<NetworkStatistics> {
        self.make_request(get_network_statistics).await
    }

    // Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization
    pub async fn get_option<C: AsRef<GetOption>>(&self, get_option: C) -> Result<OptionValue> {
        self.make_request(get_option).await
    }

    // Returns a Telegram Passport authorization form for sharing data with a service
    pub async fn get_passport_authorization_form<C: AsRef<GetPassportAuthorizationForm>>(
        &self,
        get_passport_authorization_form: C,
    ) -> Result<PassportAuthorizationForm> {
        self.make_request(get_passport_authorization_form).await
    }

    // Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form
    pub async fn get_passport_authorization_form_available_elements<
        C: AsRef<GetPassportAuthorizationFormAvailableElements>,
    >(
        &self,
        get_passport_authorization_form_available_elements: C,
    ) -> Result<PassportElementsWithErrors> {
        self.make_request(get_passport_authorization_form_available_elements)
            .await
    }

    // Returns one of the available Telegram Passport elements
    pub async fn get_passport_element<C: AsRef<GetPassportElement>>(
        &self,
        get_passport_element: C,
    ) -> Result<PassportElement> {
        self.make_request(get_passport_element).await
    }

    // Returns the current state of 2-step verification
    pub async fn get_password_state<C: AsRef<GetPasswordState>>(
        &self,
        get_password_state: C,
    ) -> Result<PasswordState> {
        self.make_request(get_password_state).await
    }

    // Returns an invoice payment form. This method must be called when the user presses inlineKeyboardButtonBuy
    pub async fn get_payment_form<C: AsRef<GetPaymentForm>>(
        &self,
        get_payment_form: C,
    ) -> Result<PaymentForm> {
        self.make_request(get_payment_form).await
    }

    // Returns information about a successful payment
    pub async fn get_payment_receipt<C: AsRef<GetPaymentReceipt>>(
        &self,
        get_payment_receipt: C,
    ) -> Result<PaymentReceipt> {
        self.make_request(get_payment_receipt).await
    }

    // Returns information about a phone number by its prefix. Can be called before authorization
    pub async fn get_phone_number_info<C: AsRef<GetPhoneNumberInfo>>(
        &self,
        get_phone_number_info: C,
    ) -> Result<PhoneNumberInfo> {
        self.make_request(get_phone_number_info).await
    }

    // Returns information about a phone number by its prefix synchronously. getCountries must be called at least once after changing localization to the specified language if properly localized country information is expected. Can be called synchronously
    pub async fn get_phone_number_info_sync<C: AsRef<GetPhoneNumberInfoSync>>(
        &self,
        get_phone_number_info_sync: C,
    ) -> Result<PhoneNumberInfo> {
        self.make_request(get_phone_number_info_sync).await
    }

    // Returns users voted for the specified option in a non-anonymous polls. For optimal performance, the number of returned users is chosen by TDLib
    pub async fn get_poll_voters<C: AsRef<GetPollVoters>>(
        &self,
        get_poll_voters: C,
    ) -> Result<Users> {
        self.make_request(get_poll_voters).await
    }

    // Returns an IETF language tag of the language preferred in the country, which must be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown
    pub async fn get_preferred_country_language<C: AsRef<GetPreferredCountryLanguage>>(
        &self,
        get_preferred_country_language: C,
    ) -> Result<Text> {
        self.make_request(get_preferred_country_language).await
    }

    // Returns list of proxies that are currently set up. Can be called before authorization
    pub async fn get_proxies<C: AsRef<GetProxies>>(&self, get_proxies: C) -> Result<Proxies> {
        self.make_request(get_proxies).await
    }

    // Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization
    pub async fn get_proxy_link<C: AsRef<GetProxyLink>>(
        &self,
        get_proxy_link: C,
    ) -> Result<HttpUrl> {
        self.make_request(get_proxy_link).await
    }

    // Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. Can be called synchronously
    pub async fn get_push_receiver_id<C: AsRef<GetPushReceiverId>>(
        &self,
        get_push_receiver_id: C,
    ) -> Result<PushReceiverId> {
        self.make_request(get_push_receiver_id).await
    }

    // Returns up to 20 recently used inline bots in the order of their last usage
    pub async fn get_recent_inline_bots<C: AsRef<GetRecentInlineBots>>(
        &self,
        get_recent_inline_bots: C,
    ) -> Result<Users> {
        self.make_request(get_recent_inline_bots).await
    }

    // Returns a list of recently used stickers
    pub async fn get_recent_stickers<C: AsRef<GetRecentStickers>>(
        &self,
        get_recent_stickers: C,
    ) -> Result<Stickers> {
        self.make_request(get_recent_stickers).await
    }

    // Returns recently opened chats, this is an offline request. Returns chats in the order of last opening
    pub async fn get_recently_opened_chats<C: AsRef<GetRecentlyOpenedChats>>(
        &self,
        get_recently_opened_chats: C,
    ) -> Result<Chats> {
        self.make_request(get_recently_opened_chats).await
    }

    // Returns t.me URLs recently visited by a newly registered user
    pub async fn get_recently_visited_t_me_urls<C: AsRef<GetRecentlyVisitedTMeUrls>>(
        &self,
        get_recently_visited_t_me_urls: C,
    ) -> Result<TMeUrls> {
        self.make_request(get_recently_visited_t_me_urls).await
    }

    // Returns recommended chat filters for the current user
    pub async fn get_recommended_chat_filters<C: AsRef<GetRecommendedChatFilters>>(
        &self,
        get_recommended_chat_filters: C,
    ) -> Result<RecommendedChatFilters> {
        self.make_request(get_recommended_chat_filters).await
    }

    // Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user
    pub async fn get_recovery_email_address<C: AsRef<GetRecoveryEmailAddress>>(
        &self,
        get_recovery_email_address: C,
    ) -> Result<RecoveryEmailAddress> {
        self.make_request(get_recovery_email_address).await
    }

    // Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. Even the request succeeds, the file can be used only if it is still accessible to the user. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
    pub async fn get_remote_file<C: AsRef<GetRemoteFile>>(
        &self,
        get_remote_file: C,
    ) -> Result<File> {
        self.make_request(get_remote_file).await
    }

    // Returns information about a message that is replied by a given message. Also returns the pinned message, the game message, and the invoice message for messages of the types messagePinMessage, messageGameScore, and messagePaymentSuccessful respectively
    pub async fn get_replied_message<C: AsRef<GetRepliedMessage>>(
        &self,
        get_replied_message: C,
    ) -> Result<Message> {
        self.make_request(get_replied_message).await
    }

    // Returns saved animations
    pub async fn get_saved_animations<C: AsRef<GetSavedAnimations>>(
        &self,
        get_saved_animations: C,
    ) -> Result<Animations> {
        self.make_request(get_saved_animations).await
    }

    // Returns saved order info, if any
    pub async fn get_saved_order_info<C: AsRef<GetSavedOrderInfo>>(
        &self,
        get_saved_order_info: C,
    ) -> Result<OrderInfo> {
        self.make_request(get_saved_order_info).await
    }

    // Returns the notification settings for chats of a given type
    pub async fn get_scope_notification_settings<C: AsRef<GetScopeNotificationSettings>>(
        &self,
        get_scope_notification_settings: C,
    ) -> Result<ScopeNotificationSettings> {
        self.make_request(get_scope_notification_settings).await
    }

    // Returns information about a secret chat by its identifier. This is an offline request
    pub async fn get_secret_chat<C: AsRef<GetSecretChat>>(
        &self,
        get_secret_chat: C,
    ) -> Result<SecretChat> {
        self.make_request(get_secret_chat).await
    }

    // Loads an asynchronous or a zoomed in statistical graph
    pub async fn get_statistical_graph<C: AsRef<GetStatisticalGraph>>(
        &self,
        get_statistical_graph: C,
    ) -> Result<StatisticalGraph> {
        self.make_request(get_statistical_graph).await
    }

    // Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
    pub async fn get_sticker_emojis<C: AsRef<GetStickerEmojis>>(
        &self,
        get_sticker_emojis: C,
    ) -> Result<Emojis> {
        self.make_request(get_sticker_emojis).await
    }

    // Returns information about a sticker set by its identifier
    pub async fn get_sticker_set<C: AsRef<GetStickerSet>>(
        &self,
        get_sticker_set: C,
    ) -> Result<StickerSet> {
        self.make_request(get_sticker_set).await
    }

    // Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is non-empty, favorite and recently used stickers may also be returned
    pub async fn get_stickers<C: AsRef<GetStickers>>(&self, get_stickers: C) -> Result<Stickers> {
        self.make_request(get_stickers).await
    }

    // Returns storage usage statistics. Can be called before authorization
    pub async fn get_storage_statistics<C: AsRef<GetStorageStatistics>>(
        &self,
        get_storage_statistics: C,
    ) -> Result<StorageStatistics> {
        self.make_request(get_storage_statistics).await
    }

    // Quickly returns approximate storage usage statistics. Can be called before authorization
    pub async fn get_storage_statistics_fast<C: AsRef<GetStorageStatisticsFast>>(
        &self,
        get_storage_statistics_fast: C,
    ) -> Result<StorageStatisticsFast> {
        self.make_request(get_storage_statistics_fast).await
    }

    // Returns suggested name for saving a file in a given directory
    pub async fn get_suggested_file_name<C: AsRef<GetSuggestedFileName>>(
        &self,
        get_suggested_file_name: C,
    ) -> Result<Text> {
        self.make_request(get_suggested_file_name).await
    }

    // Returns a suggested name for a new sticker set with a given title
    pub async fn get_suggested_sticker_set_name<C: AsRef<GetSuggestedStickerSetName>>(
        &self,
        get_suggested_sticker_set_name: C,
    ) -> Result<Text> {
        self.make_request(get_suggested_sticker_set_name).await
    }

    // Returns a list of basic group and supergroup chats, which can be used as a discussion group for a channel. Returned basic group chats must be first upgraded to supergroups before they can be set as a discussion group. To set a returned supergroup as a discussion group, access to its old messages must be enabled using toggleSupergroupIsAllHistoryAvailable first
    pub async fn get_suitable_discussion_chats<C: AsRef<GetSuitableDiscussionChats>>(
        &self,
        get_suitable_discussion_chats: C,
    ) -> Result<Chats> {
        self.make_request(get_suitable_discussion_chats).await
    }

    // Returns information about a supergroup or a channel by its identifier. This is an offline request if the current user is not a bot
    pub async fn get_supergroup<C: AsRef<GetSupergroup>>(
        &self,
        get_supergroup: C,
    ) -> Result<Supergroup> {
        self.make_request(get_supergroup).await
    }

    // Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute
    pub async fn get_supergroup_full_info<C: AsRef<GetSupergroupFullInfo>>(
        &self,
        get_supergroup_full_info: C,
    ) -> Result<SupergroupFullInfo> {
        self.make_request(get_supergroup_full_info).await
    }

    // Returns information about members or banned users in a supergroup or channel. Can be used only if supergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters
    pub async fn get_supergroup_members<C: AsRef<GetSupergroupMembers>>(
        &self,
        get_supergroup_members: C,
    ) -> Result<ChatMembers> {
        self.make_request(get_supergroup_members).await
    }

    // Returns a user that can be contacted to get support
    pub async fn get_support_user<C: AsRef<GetSupportUser>>(
        &self,
        get_support_user: C,
    ) -> Result<User> {
        self.make_request(get_support_user).await
    }

    // Returns information about the current temporary password
    pub async fn get_temporary_password_state<C: AsRef<GetTemporaryPasswordState>>(
        &self,
        get_temporary_password_state: C,
    ) -> Result<TemporaryPasswordState> {
        self.make_request(get_temporary_password_state).await
    }

    // Returns all entities (mentions, hashtags, cashtags, bot commands, bank card numbers, URLs, and email addresses) contained in the text. Can be called synchronously
    pub async fn get_text_entities<C: AsRef<GetTextEntities>>(
        &self,
        get_text_entities: C,
    ) -> Result<TextEntities> {
        self.make_request(get_text_entities).await
    }

    // Returns a list of frequently used chats. Supported only if the chat info database is enabled
    pub async fn get_top_chats<C: AsRef<GetTopChats>>(&self, get_top_chats: C) -> Result<Chats> {
        self.make_request(get_top_chats).await
    }

    // Returns a list of trending sticker sets. For optimal performance, the number of returned sticker sets is chosen by TDLib
    pub async fn get_trending_sticker_sets<C: AsRef<GetTrendingStickerSets>>(
        &self,
        get_trending_sticker_sets: C,
    ) -> Result<StickerSets> {
        self.make_request(get_trending_sticker_sets).await
    }

    // Returns information about a user by their identifier. This is an offline request if the current user is not a bot
    pub async fn get_user<C: AsRef<GetUser>>(&self, get_user: C) -> Result<User> {
        self.make_request(get_user).await
    }

    // Returns full information about a user by their identifier
    pub async fn get_user_full_info<C: AsRef<GetUserFullInfo>>(
        &self,
        get_user_full_info: C,
    ) -> Result<UserFullInfo> {
        self.make_request(get_user_full_info).await
    }

    // Returns the current privacy settings
    pub async fn get_user_privacy_setting_rules<C: AsRef<GetUserPrivacySettingRules>>(
        &self,
        get_user_privacy_setting_rules: C,
    ) -> Result<UserPrivacySettingRules> {
        self.make_request(get_user_privacy_setting_rules).await
    }

    // Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already
    pub async fn get_user_profile_photos<C: AsRef<GetUserProfilePhotos>>(
        &self,
        get_user_profile_photos: C,
    ) -> Result<ChatPhotos> {
        self.make_request(get_user_profile_photos).await
    }

    // Returns list of participant identifiers, on whose behalf a video chat in the chat can be joined
    pub async fn get_video_chat_available_participants<
        C: AsRef<GetVideoChatAvailableParticipants>,
    >(
        &self,
        get_video_chat_available_participants: C,
    ) -> Result<MessageSenders> {
        self.make_request(get_video_chat_available_participants)
            .await
    }

    // Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page
    pub async fn get_web_page_instant_view<C: AsRef<GetWebPageInstantView>>(
        &self,
        get_web_page_instant_view: C,
    ) -> Result<WebPageInstantView> {
        self.make_request(get_web_page_instant_view).await
    }

    // Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview
    pub async fn get_web_page_preview<C: AsRef<GetWebPagePreview>>(
        &self,
        get_web_page_preview: C,
    ) -> Result<WebPage> {
        self.make_request(get_web_page_preview).await
    }

    // Hides a suggested action
    pub async fn hide_suggested_action<C: AsRef<HideSuggestedAction>>(
        &self,
        hide_suggested_action: C,
    ) -> Result<Ok> {
        self.make_request(hide_suggested_action).await
    }

    // Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored
    pub async fn import_contacts<C: AsRef<ImportContacts>>(
        &self,
        import_contacts: C,
    ) -> Result<ImportedContacts> {
        self.make_request(import_contacts).await
    }

    // Imports messages exported from another app
    pub async fn import_messages<C: AsRef<ImportMessages>>(
        &self,
        import_messages: C,
    ) -> Result<Ok> {
        self.make_request(import_messages).await
    }

    // Invites users to an active group call. Sends a service message of type messageInviteToGroupCall for video chats
    pub async fn invite_group_call_participants<C: AsRef<InviteGroupCallParticipants>>(
        &self,
        invite_group_call_participants: C,
    ) -> Result<Ok> {
        self.make_request(invite_group_call_participants).await
    }

    // Adds the current user as a new member to a chat. Private and secret chats can't be joined using this method
    pub async fn join_chat<C: AsRef<JoinChat>>(&self, join_chat: C) -> Result<Ok> {
        self.make_request(join_chat).await
    }

    // Uses an invite link to add the current user to the chat if possible
    pub async fn join_chat_by_invite_link<C: AsRef<JoinChatByInviteLink>>(
        &self,
        join_chat_by_invite_link: C,
    ) -> Result<Chat> {
        self.make_request(join_chat_by_invite_link).await
    }

    // Joins an active group call. Returns join response payload for tgcalls
    pub async fn join_group_call<C: AsRef<JoinGroupCall>>(
        &self,
        join_group_call: C,
    ) -> Result<Text> {
        self.make_request(join_group_call).await
    }

    // Removes the current user from chat members. Private and secret chats can't be left using this method
    pub async fn leave_chat<C: AsRef<LeaveChat>>(&self, leave_chat: C) -> Result<Ok> {
        self.make_request(leave_chat).await
    }

    // Leaves a group call
    pub async fn leave_group_call<C: AsRef<LeaveGroupCall>>(
        &self,
        leave_group_call: C,
    ) -> Result<Ok> {
        self.make_request(leave_group_call).await
    }

    // Loads more chats from a chat list. The loaded chats and their positions in the chat list will be sent through updates. Chats are sorted by the pair (chat.position.order, chat.id) in descending order. Returns a 404 error if all chats have been loaded
    pub async fn load_chats<C: AsRef<LoadChats>>(&self, load_chats: C) -> Result<Ok> {
        self.make_request(load_chats).await
    }

    // Loads more participants of a group call. The loaded participants will be received through updates. Use the field groupCall.loaded_all_participants to check whether all participants have already been loaded
    pub async fn load_group_call_participants<C: AsRef<LoadGroupCallParticipants>>(
        &self,
        load_group_call_participants: C,
    ) -> Result<Ok> {
        self.make_request(load_group_call_participants).await
    }

    // Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent
    pub async fn log_out<C: AsRef<LogOut>>(&self, log_out: C) -> Result<Ok> {
        self.make_request(log_out).await
    }

    // Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats)
    pub async fn open_chat<C: AsRef<OpenChat>>(&self, open_chat: C) -> Result<Ok> {
        self.make_request(open_chat).await
    }

    // Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An updateMessageContentOpened update will be generated if something has changed
    pub async fn open_message_content<C: AsRef<OpenMessageContent>>(
        &self,
        open_message_content: C,
    ) -> Result<Ok> {
        self.make_request(open_message_content).await
    }

    // Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted
    pub async fn optimize_storage<C: AsRef<OptimizeStorage>>(
        &self,
        optimize_storage: C,
    ) -> Result<StorageStatistics> {
        self.make_request(optimize_storage).await
    }

    // Parses Markdown entities in a human-friendly format, ignoring markup errors. Can be called synchronously
    pub async fn parse_markdown<C: AsRef<ParseMarkdown>>(
        &self,
        parse_markdown: C,
    ) -> Result<FormattedText> {
        self.make_request(parse_markdown).await
    }

    // Parses Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities contained in the text. Can be called synchronously
    pub async fn parse_text_entities<C: AsRef<ParseTextEntities>>(
        &self,
        parse_text_entities: C,
    ) -> Result<FormattedText> {
        self.make_request(parse_text_entities).await
    }

    // Pins a message in a chat; requires can_pin_messages rights or can_edit_messages rights in the channel
    pub async fn pin_chat_message<C: AsRef<PinChatMessage>>(
        &self,
        pin_chat_message: C,
    ) -> Result<Ok> {
        self.make_request(pin_chat_message).await
    }

    // Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization
    pub async fn ping_proxy<C: AsRef<PingProxy>>(&self, ping_proxy: C) -> Result<Seconds> {
        self.make_request(ping_proxy).await
    }

    // Handles a pending join request in a chat
    pub async fn process_chat_join_request<C: AsRef<ProcessChatJoinRequest>>(
        &self,
        process_chat_join_request: C,
    ) -> Result<Ok> {
        self.make_request(process_chat_join_request).await
    }

    // Handles all pending join requests for a given link in a chat
    pub async fn process_chat_join_requests<C: AsRef<ProcessChatJoinRequests>>(
        &self,
        process_chat_join_requests: C,
    ) -> Result<Ok> {
        self.make_request(process_chat_join_requests).await
    }

    // Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization
    pub async fn process_push_notification<C: AsRef<ProcessPushNotification>>(
        &self,
        process_push_notification: C,
    ) -> Result<Ok> {
        self.make_request(process_push_notification).await
    }

    // Marks all mentions in a chat as read
    pub async fn read_all_chat_mentions<C: AsRef<ReadAllChatMentions>>(
        &self,
        read_all_chat_mentions: C,
    ) -> Result<Ok> {
        self.make_request(read_all_chat_mentions).await
    }

    // Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct read from the file
    pub async fn read_file_part<C: AsRef<ReadFilePart>>(
        &self,
        read_file_part: C,
    ) -> Result<FilePart> {
        self.make_request(read_file_part).await
    }

    // Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn recover_authentication_password<C: AsRef<RecoverAuthenticationPassword>>(
        &self,
        recover_authentication_password: C,
    ) -> Result<Ok> {
        self.make_request(recover_authentication_password).await
    }

    // Recovers the 2-step verification password using a recovery code sent to an email address that was previously set up
    pub async fn recover_password<C: AsRef<RecoverPassword>>(
        &self,
        recover_password: C,
    ) -> Result<PasswordState> {
        self.make_request(recover_password).await
    }

    // Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription
    pub async fn register_device<C: AsRef<RegisterDevice>>(
        &self,
        register_device: C,
    ) -> Result<PushReceiverId> {
        self.make_request(register_device).await
    }

    // Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration
    pub async fn register_user<C: AsRef<RegisterUser>>(&self, register_user: C) -> Result<Ok> {
        self.make_request(register_user).await
    }

    // Removes background from the list of installed backgrounds
    pub async fn remove_background<C: AsRef<RemoveBackground>>(
        &self,
        remove_background: C,
    ) -> Result<Ok> {
        self.make_request(remove_background).await
    }

    // Removes a chat action bar without any other action
    pub async fn remove_chat_action_bar<C: AsRef<RemoveChatActionBar>>(
        &self,
        remove_chat_action_bar: C,
    ) -> Result<Ok> {
        self.make_request(remove_chat_action_bar).await
    }

    // Removes users from the contact list
    pub async fn remove_contacts<C: AsRef<RemoveContacts>>(
        &self,
        remove_contacts: C,
    ) -> Result<Ok> {
        self.make_request(remove_contacts).await
    }

    // Removes a sticker from the list of favorite stickers
    pub async fn remove_favorite_sticker<C: AsRef<RemoveFavoriteSticker>>(
        &self,
        remove_favorite_sticker: C,
    ) -> Result<Ok> {
        self.make_request(remove_favorite_sticker).await
    }

    // Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user
    pub async fn remove_notification<C: AsRef<RemoveNotification>>(
        &self,
        remove_notification: C,
    ) -> Result<Ok> {
        self.make_request(remove_notification).await
    }

    // Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user
    pub async fn remove_notification_group<C: AsRef<RemoveNotificationGroup>>(
        &self,
        remove_notification_group: C,
    ) -> Result<Ok> {
        self.make_request(remove_notification_group).await
    }

    // Removes a proxy server. Can be called before authorization
    pub async fn remove_proxy<C: AsRef<RemoveProxy>>(&self, remove_proxy: C) -> Result<Ok> {
        self.make_request(remove_proxy).await
    }

    // Removes a hashtag from the list of recently used hashtags
    pub async fn remove_recent_hashtag<C: AsRef<RemoveRecentHashtag>>(
        &self,
        remove_recent_hashtag: C,
    ) -> Result<Ok> {
        self.make_request(remove_recent_hashtag).await
    }

    // Removes a sticker from the list of recently used stickers
    pub async fn remove_recent_sticker<C: AsRef<RemoveRecentSticker>>(
        &self,
        remove_recent_sticker: C,
    ) -> Result<Ok> {
        self.make_request(remove_recent_sticker).await
    }

    // Removes a chat from the list of recently found chats
    pub async fn remove_recently_found_chat<C: AsRef<RemoveRecentlyFoundChat>>(
        &self,
        remove_recently_found_chat: C,
    ) -> Result<Ok> {
        self.make_request(remove_recently_found_chat).await
    }

    // Removes an animation from the list of saved animations
    pub async fn remove_saved_animation<C: AsRef<RemoveSavedAnimation>>(
        &self,
        remove_saved_animation: C,
    ) -> Result<Ok> {
        self.make_request(remove_saved_animation).await
    }

    // Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot
    pub async fn remove_sticker_from_set<C: AsRef<RemoveStickerFromSet>>(
        &self,
        remove_sticker_from_set: C,
    ) -> Result<Ok> {
        self.make_request(remove_sticker_from_set).await
    }

    // Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled
    pub async fn remove_top_chat<C: AsRef<RemoveTopChat>>(&self, remove_top_chat: C) -> Result<Ok> {
        self.make_request(remove_top_chat).await
    }

    // Changes the order of chat filters
    pub async fn reorder_chat_filters<C: AsRef<ReorderChatFilters>>(
        &self,
        reorder_chat_filters: C,
    ) -> Result<Ok> {
        self.make_request(reorder_chat_filters).await
    }

    // Changes the order of installed sticker sets
    pub async fn reorder_installed_sticker_sets<C: AsRef<ReorderInstalledStickerSets>>(
        &self,
        reorder_installed_sticker_sets: C,
    ) -> Result<Ok> {
        self.make_request(reorder_installed_sticker_sets).await
    }

    // Replaces current primary invite link for a chat with a new primary invite link. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right
    pub async fn replace_primary_chat_invite_link<C: AsRef<ReplacePrimaryChatInviteLink>>(
        &self,
        replace_primary_chat_invite_link: C,
    ) -> Result<ChatInviteLink> {
        self.make_request(replace_primary_chat_invite_link).await
    }

    // Reports a chat to the Telegram moderators. A chat can be reported only from the chat action bar, or if chat.can_be_reported
    pub async fn report_chat<C: AsRef<ReportChat>>(&self, report_chat: C) -> Result<Ok> {
        self.make_request(report_chat).await
    }

    // Reports a chat photo to the Telegram moderators. A chat photo can be reported only if chat.can_be_reported
    pub async fn report_chat_photo<C: AsRef<ReportChatPhoto>>(
        &self,
        report_chat_photo: C,
    ) -> Result<Ok> {
        self.make_request(report_chat_photo).await
    }

    // Reports messages in a supergroup as spam; requires administrator rights in the supergroup
    pub async fn report_supergroup_spam<C: AsRef<ReportSupergroupSpam>>(
        &self,
        report_supergroup_spam: C,
    ) -> Result<Ok> {
        self.make_request(report_supergroup_spam).await
    }

    // Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn request_authentication_password_recovery<
        C: AsRef<RequestAuthenticationPasswordRecovery>,
    >(
        &self,
        request_authentication_password_recovery: C,
    ) -> Result<Ok> {
        self.make_request(request_authentication_password_recovery)
            .await
    }

    // Requests to send a 2-step verification password recovery code to an email address that was previously set up
    pub async fn request_password_recovery<C: AsRef<RequestPasswordRecovery>>(
        &self,
        request_password_recovery: C,
    ) -> Result<EmailAddressAuthenticationCodeInfo> {
        self.make_request(request_password_recovery).await
    }

    // Requests QR code authentication by scanning a QR code on another logged in device. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
    pub async fn request_qr_code_authentication<C: AsRef<RequestQrCodeAuthentication>>(
        &self,
        request_qr_code_authentication: C,
    ) -> Result<Ok> {
        self.make_request(request_qr_code_authentication).await
    }

    // Re-sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode, the next_code_type of the result is not null and the server-specified timeout has passed
    pub async fn resend_authentication_code<C: AsRef<ResendAuthenticationCode>>(
        &self,
        resend_authentication_code: C,
    ) -> Result<Ok> {
        self.make_request(resend_authentication_code).await
    }

    // Re-sends the authentication code sent to confirm a new phone number for the current user. Works only if the previously received authenticationCodeInfo next_code_type was not null and the server-specified timeout has passed
    pub async fn resend_change_phone_number_code<C: AsRef<ResendChangePhoneNumberCode>>(
        &self,
        resend_change_phone_number_code: C,
    ) -> Result<AuthenticationCodeInfo> {
        self.make_request(resend_change_phone_number_code).await
    }

    // Re-sends the code to verify an email address to be added to a user's Telegram Passport
    pub async fn resend_email_address_verification_code<
        C: AsRef<ResendEmailAddressVerificationCode>,
    >(
        &self,
        resend_email_address_verification_code: C,
    ) -> Result<EmailAddressAuthenticationCodeInfo> {
        self.make_request(resend_email_address_verification_code)
            .await
    }

    // Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed. If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message
    pub async fn resend_messages<C: AsRef<ResendMessages>>(
        &self,
        resend_messages: C,
    ) -> Result<Messages> {
        self.make_request(resend_messages).await
    }

    // Resends phone number confirmation code
    pub async fn resend_phone_number_confirmation_code<
        C: AsRef<ResendPhoneNumberConfirmationCode>,
    >(
        &self,
        resend_phone_number_confirmation_code: C,
    ) -> Result<AuthenticationCodeInfo> {
        self.make_request(resend_phone_number_confirmation_code)
            .await
    }

    // Re-sends the code to verify a phone number to be added to a user's Telegram Passport
    pub async fn resend_phone_number_verification_code<
        C: AsRef<ResendPhoneNumberVerificationCode>,
    >(
        &self,
        resend_phone_number_verification_code: C,
    ) -> Result<AuthenticationCodeInfo> {
        self.make_request(resend_phone_number_verification_code)
            .await
    }

    // Resends the 2-step verification recovery email address verification code
    pub async fn resend_recovery_email_address_code<C: AsRef<ResendRecoveryEmailAddressCode>>(
        &self,
        resend_recovery_email_address_code: C,
    ) -> Result<PasswordState> {
        self.make_request(resend_recovery_email_address_code).await
    }

    // Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to "default" and message previews are shown
    pub async fn reset_all_notification_settings<C: AsRef<ResetAllNotificationSettings>>(
        &self,
        reset_all_notification_settings: C,
    ) -> Result<Ok> {
        self.make_request(reset_all_notification_settings).await
    }

    // Resets list of installed backgrounds to its default value
    pub async fn reset_backgrounds<C: AsRef<ResetBackgrounds>>(
        &self,
        reset_backgrounds: C,
    ) -> Result<Ok> {
        self.make_request(reset_backgrounds).await
    }

    // Resets all network data usage statistics to zero. Can be called before authorization
    pub async fn reset_network_statistics<C: AsRef<ResetNetworkStatistics>>(
        &self,
        reset_network_statistics: C,
    ) -> Result<Ok> {
        self.make_request(reset_network_statistics).await
    }

    // Removes 2-step verification password without previous password and access to recovery email address. The password can't be reset immediately and the request needs to be repeated after the specified time
    pub async fn reset_password<C: AsRef<ResetPassword>>(
        &self,
        reset_password: C,
    ) -> Result<ResetPasswordResult> {
        self.make_request(reset_password).await
    }

    // Revokes invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links. If a primary link is revoked, then additionally to the revoked link returns new primary link
    pub async fn revoke_chat_invite_link<C: AsRef<RevokeChatInviteLink>>(
        &self,
        revoke_chat_invite_link: C,
    ) -> Result<ChatInviteLinks> {
        self.make_request(revoke_chat_invite_link).await
    }

    // Revokes invite link for a group call. Requires groupCall.can_be_managed group call flag
    pub async fn revoke_group_call_invite_link<C: AsRef<RevokeGroupCallInviteLink>>(
        &self,
        revoke_group_call_invite_link: C,
    ) -> Result<Ok> {
        self.make_request(revoke_group_call_invite_link).await
    }

    // Saves application log event on the server. Can be called before authorization
    pub async fn save_application_log_event<C: AsRef<SaveApplicationLogEvent>>(
        &self,
        save_application_log_event: C,
    ) -> Result<Ok> {
        self.make_request(save_application_log_event).await
    }

    // Searches for a background by its name
    pub async fn search_background<C: AsRef<SearchBackground>>(
        &self,
        search_background: C,
    ) -> Result<Background> {
        self.make_request(search_background).await
    }

    // Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
    pub async fn search_call_messages<C: AsRef<SearchCallMessages>>(
        &self,
        search_call_messages: C,
    ) -> Result<Messages> {
        self.make_request(search_call_messages).await
    }

    // Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels
    pub async fn search_chat_members<C: AsRef<SearchChatMembers>>(
        &self,
        search_chat_members: C,
    ) -> Result<ChatMembers> {
        self.make_request(search_chat_members).await
    }

    // Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query (searchSecretMessages must be used instead), or without an enabled message database. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
    pub async fn search_chat_messages<C: AsRef<SearchChatMessages>>(
        &self,
        search_chat_messages: C,
    ) -> Result<Messages> {
        self.make_request(search_chat_messages).await
    }

    // Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user
    pub async fn search_chat_recent_location_messages<
        C: AsRef<SearchChatRecentLocationMessages>,
    >(
        &self,
        search_chat_recent_location_messages: C,
    ) -> Result<Messages> {
        self.make_request(search_chat_recent_location_messages)
            .await
    }

    // Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the main chat list
    pub async fn search_chats<C: AsRef<SearchChats>>(&self, search_chats: C) -> Result<Chats> {
        self.make_request(search_chats).await
    }

    // Returns a list of users and location-based supergroups nearby. The list of users nearby will be updated for 60 seconds after the request by the updates updateUsersNearby. The request must be sent again every 25 seconds with adjusted location to not miss new chats
    pub async fn search_chats_nearby<C: AsRef<SearchChatsNearby>>(
        &self,
        search_chats_nearby: C,
    ) -> Result<ChatsNearby> {
        self.make_request(search_chats_nearby).await
    }

    // Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the main chat list
    pub async fn search_chats_on_server<C: AsRef<SearchChatsOnServer>>(
        &self,
        search_chats_on_server: C,
    ) -> Result<Chats> {
        self.make_request(search_chats_on_server).await
    }

    // Searches for the specified query in the first names, last names and usernames of the known user contacts
    pub async fn search_contacts<C: AsRef<SearchContacts>>(
        &self,
        search_contacts: C,
    ) -> Result<Users> {
        self.make_request(search_contacts).await
    }

    // Searches for emojis by keywords. Supported only if the file database is enabled
    pub async fn search_emojis<C: AsRef<SearchEmojis>>(&self, search_emojis: C) -> Result<Emojis> {
        self.make_request(search_emojis).await
    }

    // Searches for recently used hashtags by their prefix
    pub async fn search_hashtags<C: AsRef<SearchHashtags>>(
        &self,
        search_hashtags: C,
    ) -> Result<Hashtags> {
        self.make_request(search_hashtags).await
    }

    // Searches for installed sticker sets by looking for specified query in their title and name
    pub async fn search_installed_sticker_sets<C: AsRef<SearchInstalledStickerSets>>(
        &self,
        search_installed_sticker_sets: C,
    ) -> Result<StickerSets> {
        self.make_request(search_installed_sticker_sets).await
    }

    // Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
    pub async fn search_messages<C: AsRef<SearchMessages>>(
        &self,
        search_messages: C,
    ) -> Result<Messages> {
        self.make_request(search_messages).await
    }

    // Searches a public chat by its username. Currently, only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned
    pub async fn search_public_chat<C: AsRef<SearchPublicChat>>(
        &self,
        search_public_chat: C,
    ) -> Result<Chat> {
        self.make_request(search_public_chat).await
    }

    // Searches public chats by looking for specified query in their username and title. Currently, only private chats, supergroups and channels can be public. Returns a meaningful number of results. Excludes private chats with contacts and chats from the chat list from the results
    pub async fn search_public_chats<C: AsRef<SearchPublicChats>>(
        &self,
        search_public_chats: C,
    ) -> Result<Chats> {
        self.make_request(search_public_chats).await
    }

    // Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance, the number of returned messages is chosen by TDLib
    pub async fn search_secret_messages<C: AsRef<SearchSecretMessages>>(
        &self,
        search_secret_messages: C,
    ) -> Result<FoundMessages> {
        self.make_request(search_secret_messages).await
    }

    // Searches for a sticker set by its name
    pub async fn search_sticker_set<C: AsRef<SearchStickerSet>>(
        &self,
        search_sticker_set: C,
    ) -> Result<StickerSet> {
        self.make_request(search_sticker_set).await
    }

    // Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results
    pub async fn search_sticker_sets<C: AsRef<SearchStickerSets>>(
        &self,
        search_sticker_sets: C,
    ) -> Result<StickerSets> {
        self.make_request(search_sticker_sets).await
    }

    // Searches for stickers from public sticker sets that correspond to a given emoji
    pub async fn search_stickers<C: AsRef<SearchStickers>>(
        &self,
        search_stickers: C,
    ) -> Result<Stickers> {
        self.make_request(search_stickers).await
    }

    // Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message
    pub async fn send_bot_start_message<C: AsRef<SendBotStartMessage>>(
        &self,
        send_bot_start_message: C,
    ) -> Result<Message> {
        self.make_request(send_bot_start_message).await
    }

    // Sends debug information for a call
    pub async fn send_call_debug_information<C: AsRef<SendCallDebugInformation>>(
        &self,
        send_call_debug_information: C,
    ) -> Result<Ok> {
        self.make_request(send_call_debug_information).await
    }

    // Sends a call rating
    pub async fn send_call_rating<C: AsRef<SendCallRating>>(
        &self,
        send_call_rating: C,
    ) -> Result<Ok> {
        self.make_request(send_call_rating).await
    }

    // Sends call signaling data
    pub async fn send_call_signaling_data<C: AsRef<SendCallSignalingData>>(
        &self,
        send_call_signaling_data: C,
    ) -> Result<Ok> {
        self.make_request(send_call_signaling_data).await
    }

    // Sends a notification about user activity in a chat
    pub async fn send_chat_action<C: AsRef<SendChatAction>>(
        &self,
        send_chat_action: C,
    ) -> Result<Ok> {
        self.make_request(send_chat_action).await
    }

    // Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats
    pub async fn send_chat_screenshot_taken_notification<
        C: AsRef<SendChatScreenshotTakenNotification>,
    >(
        &self,
        send_chat_screenshot_taken_notification: C,
    ) -> Result<Ok> {
        self.make_request(send_chat_screenshot_taken_notification)
            .await
    }

    // Sends a custom request; for bots only
    pub async fn send_custom_request<C: AsRef<SendCustomRequest>>(
        &self,
        send_custom_request: C,
    ) -> Result<CustomRequestResult> {
        self.make_request(send_custom_request).await
    }

    // Sends a code to verify an email address to be added to a user's Telegram Passport
    pub async fn send_email_address_verification_code<
        C: AsRef<SendEmailAddressVerificationCode>,
    >(
        &self,
        send_email_address_verification_code: C,
    ) -> Result<EmailAddressAuthenticationCodeInfo> {
        self.make_request(send_email_address_verification_code)
            .await
    }

    // Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message
    pub async fn send_inline_query_result_message<C: AsRef<SendInlineQueryResultMessage>>(
        &self,
        send_inline_query_result_message: C,
    ) -> Result<Message> {
        self.make_request(send_inline_query_result_message).await
    }

    // Sends a message. Returns the sent message
    pub async fn send_message<C: AsRef<SendMessage>>(&self, send_message: C) -> Result<Message> {
        self.make_request(send_message).await
    }

    // Sends 2-10 messages grouped together into an album. Currently, only audio, document, photo and video messages can be grouped into an album. Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
    pub async fn send_message_album<C: AsRef<SendMessageAlbum>>(
        &self,
        send_message_album: C,
    ) -> Result<Messages> {
        self.make_request(send_message_album).await
    }

    // Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements are going to be reused
    pub async fn send_passport_authorization_form<C: AsRef<SendPassportAuthorizationForm>>(
        &self,
        send_passport_authorization_form: C,
    ) -> Result<Ok> {
        self.make_request(send_passport_authorization_form).await
    }

    // Sends a filled-out payment form to the bot for final verification
    pub async fn send_payment_form<C: AsRef<SendPaymentForm>>(
        &self,
        send_payment_form: C,
    ) -> Result<PaymentResult> {
        self.make_request(send_payment_form).await
    }

    // Sends phone number confirmation code to handle links of the type internalLinkTypePhoneNumberConfirmation
    pub async fn send_phone_number_confirmation_code<C: AsRef<SendPhoneNumberConfirmationCode>>(
        &self,
        send_phone_number_confirmation_code: C,
    ) -> Result<AuthenticationCodeInfo> {
        self.make_request(send_phone_number_confirmation_code).await
    }

    // Sends a code to verify a phone number to be added to a user's Telegram Passport
    pub async fn send_phone_number_verification_code<C: AsRef<SendPhoneNumberVerificationCode>>(
        &self,
        send_phone_number_verification_code: C,
    ) -> Result<AuthenticationCodeInfo> {
        self.make_request(send_phone_number_verification_code).await
    }

    // Changes the period of inactivity after which the account of the current user will automatically be deleted
    pub async fn set_account_ttl<C: AsRef<SetAccountTtl>>(&self, set_account_ttl: C) -> Result<Ok> {
        self.make_request(set_account_ttl).await
    }

    // Succeeds after a specified amount of time has passed. Can be called before initialization
    pub async fn set_alarm<C: AsRef<SetAlarm>>(&self, set_alarm: C) -> Result<Ok> {
        self.make_request(set_alarm).await
    }

    // Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
    pub async fn set_authentication_phone_number<C: AsRef<SetAuthenticationPhoneNumber>>(
        &self,
        set_authentication_phone_number: C,
    ) -> Result<Ok> {
        self.make_request(set_authentication_phone_number).await
    }

    // Sets auto-download settings
    pub async fn set_auto_download_settings<C: AsRef<SetAutoDownloadSettings>>(
        &self,
        set_auto_download_settings: C,
    ) -> Result<Ok> {
        self.make_request(set_auto_download_settings).await
    }

    // Changes the background selected by the user; adds background to the list of installed backgrounds
    pub async fn set_background<C: AsRef<SetBackground>>(
        &self,
        set_background: C,
    ) -> Result<Background> {
        self.make_request(set_background).await
    }

    // Changes the bio of the current user
    pub async fn set_bio<C: AsRef<SetBio>>(&self, set_bio: C) -> Result<Ok> {
        self.make_request(set_bio).await
    }

    // Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only
    pub async fn set_bot_updates_status<C: AsRef<SetBotUpdatesStatus>>(
        &self,
        set_bot_updates_status: C,
    ) -> Result<Ok> {
        self.make_request(set_bot_updates_status).await
    }

    // Changes application-specific data associated with a chat
    pub async fn set_chat_client_data<C: AsRef<SetChatClientData>>(
        &self,
        set_chat_client_data: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_client_data).await
    }

    // Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info administrator right
    pub async fn set_chat_description<C: AsRef<SetChatDescription>>(
        &self,
        set_chat_description: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_description).await
    }

    // Changes the discussion group of a channel chat; requires can_change_info administrator right in the channel if it is specified
    pub async fn set_chat_discussion_group<C: AsRef<SetChatDiscussionGroup>>(
        &self,
        set_chat_discussion_group: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_discussion_group).await
    }

    // Changes the draft message in a chat
    pub async fn set_chat_draft_message<C: AsRef<SetChatDraftMessage>>(
        &self,
        set_chat_draft_message: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_draft_message).await
    }

    // Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use
    pub async fn set_chat_location<C: AsRef<SetChatLocation>>(
        &self,
        set_chat_location: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_location).await
    }

    // Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for transferring chat ownership; use transferChatOwnership instead. Use addChatMember or banChatMember if some additional parameters needs to be passed
    pub async fn set_chat_member_status<C: AsRef<SetChatMemberStatus>>(
        &self,
        set_chat_member_status: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_member_status).await
    }

    // Selects a message sender to send messages in a chat
    pub async fn set_chat_message_sender<C: AsRef<SetChatMessageSender>>(
        &self,
        set_chat_message_sender: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_message_sender).await
    }

    // Changes the message TTL in a chat. Requires can_delete_messages administrator right in basic groups, supergroups and channels Message TTL can't be changed in a chat with the current user (Saved Messages) and the chat 777000 (Telegram)
    pub async fn set_chat_message_ttl<C: AsRef<SetChatMessageTtl>>(
        &self,
        set_chat_message_ttl: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_message_ttl).await
    }

    // Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed
    pub async fn set_chat_notification_settings<C: AsRef<SetChatNotificationSettings>>(
        &self,
        set_chat_notification_settings: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_notification_settings).await
    }

    // Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right
    pub async fn set_chat_permissions<C: AsRef<SetChatPermissions>>(
        &self,
        set_chat_permissions: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_permissions).await
    }

    // Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info administrator right
    pub async fn set_chat_photo<C: AsRef<SetChatPhoto>>(&self, set_chat_photo: C) -> Result<Ok> {
        self.make_request(set_chat_photo).await
    }

    // Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members rights
    pub async fn set_chat_slow_mode_delay<C: AsRef<SetChatSlowModeDelay>>(
        &self,
        set_chat_slow_mode_delay: C,
    ) -> Result<Ok> {
        self.make_request(set_chat_slow_mode_delay).await
    }

    // Changes the chat theme. Supported only in private and secret chats
    pub async fn set_chat_theme<C: AsRef<SetChatTheme>>(&self, set_chat_theme: C) -> Result<Ok> {
        self.make_request(set_chat_theme).await
    }

    // Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info administrator right
    pub async fn set_chat_title<C: AsRef<SetChatTitle>>(&self, set_chat_title: C) -> Result<Ok> {
        self.make_request(set_chat_title).await
    }

    // Sets the list of commands supported by the bot for the given user scope and language; for bots only
    pub async fn set_commands<C: AsRef<SetCommands>>(&self, set_commands: C) -> Result<Ok> {
        self.make_request(set_commands).await
    }

    // Adds or changes a custom local language pack to the current localization target
    pub async fn set_custom_language_pack<C: AsRef<SetCustomLanguagePack>>(
        &self,
        set_custom_language_pack: C,
    ) -> Result<Ok> {
        self.make_request(set_custom_language_pack).await
    }

    // Adds, edits or deletes a string in a custom local language pack. Can be called before authorization
    pub async fn set_custom_language_pack_string<C: AsRef<SetCustomLanguagePackString>>(
        &self,
        set_custom_language_pack_string: C,
    ) -> Result<Ok> {
        self.make_request(set_custom_language_pack_string).await
    }

    // Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain
    pub async fn set_database_encryption_key<C: AsRef<SetDatabaseEncryptionKey>>(
        &self,
        set_database_encryption_key: C,
    ) -> Result<Ok> {
        self.make_request(set_database_encryption_key).await
    }

    // Informs TDLib on a file generation progress
    pub async fn set_file_generation_progress<C: AsRef<SetFileGenerationProgress>>(
        &self,
        set_file_generation_progress: C,
    ) -> Result<Ok> {
        self.make_request(set_file_generation_progress).await
    }

    // Updates the game score of the specified user in the game; for bots only
    pub async fn set_game_score<C: AsRef<SetGameScore>>(
        &self,
        set_game_score: C,
    ) -> Result<Message> {
        self.make_request(set_game_score).await
    }

    // Informs TDLib that speaking state of a participant of an active group has changed
    pub async fn set_group_call_participant_is_speaking<
        C: AsRef<SetGroupCallParticipantIsSpeaking>,
    >(
        &self,
        set_group_call_participant_is_speaking: C,
    ) -> Result<Ok> {
        self.make_request(set_group_call_participant_is_speaking)
            .await
    }

    // Changes volume level of a participant of an active group call. If the current user can manage the group call, then the participant's volume level will be changed for all users with the default volume level
    pub async fn set_group_call_participant_volume_level<
        C: AsRef<SetGroupCallParticipantVolumeLevel>,
    >(
        &self,
        set_group_call_participant_volume_level: C,
    ) -> Result<Ok> {
        self.make_request(set_group_call_participant_volume_level)
            .await
    }

    // Sets group call title. Requires groupCall.can_be_managed group call flag
    pub async fn set_group_call_title<C: AsRef<SetGroupCallTitle>>(
        &self,
        set_group_call_title: C,
    ) -> Result<Ok> {
        self.make_request(set_group_call_title).await
    }

    // Changes the period of inactivity after which sessions will automatically be terminated
    pub async fn set_inactive_session_ttl<C: AsRef<SetInactiveSessionTtl>>(
        &self,
        set_inactive_session_ttl: C,
    ) -> Result<Ok> {
        self.make_request(set_inactive_session_ttl).await
    }

    // Updates the game score of the specified user in a game; for bots only
    pub async fn set_inline_game_score<C: AsRef<SetInlineGameScore>>(
        &self,
        set_inline_game_score: C,
    ) -> Result<Ok> {
        self.make_request(set_inline_game_score).await
    }

    // Changes the location of the current user. Needs to be called if GetOption("is_location_visible") is true and location changes for more than 1 kilometer
    pub async fn set_location<C: AsRef<SetLocation>>(&self, set_location: C) -> Result<Ok> {
        self.make_request(set_location).await
    }

    // Sets new log stream for internal logging of TDLib. Can be called synchronously
    pub async fn set_log_stream<C: AsRef<SetLogStream>>(&self, set_log_stream: C) -> Result<Ok> {
        self.make_request(set_log_stream).await
    }

    // Sets the verbosity level for a specified TDLib internal log tag. Can be called synchronously
    pub async fn set_log_tag_verbosity_level<C: AsRef<SetLogTagVerbosityLevel>>(
        &self,
        set_log_tag_verbosity_level: C,
    ) -> Result<Ok> {
        self.make_request(set_log_tag_verbosity_level).await
    }

    // Sets the verbosity level of the internal logging of TDLib. Can be called synchronously
    pub async fn set_log_verbosity_level<C: AsRef<SetLogVerbosityLevel>>(
        &self,
        set_log_verbosity_level: C,
    ) -> Result<Ok> {
        self.make_request(set_log_verbosity_level).await
    }

    // Changes the first and last name of the current user
    pub async fn set_name<C: AsRef<SetName>>(&self, set_name: C) -> Result<Ok> {
        self.make_request(set_name).await
    }

    // Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it must be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics
    pub async fn set_network_type<C: AsRef<SetNetworkType>>(
        &self,
        set_network_type: C,
    ) -> Result<Ok> {
        self.make_request(set_network_type).await
    }

    // Sets the value of an option. (Check the list of available options on https://core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization
    pub async fn set_option<C: AsRef<SetOption>>(&self, set_option: C) -> Result<Ok> {
        self.make_request(set_option).await
    }

    // Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
    pub async fn set_passport_element<C: AsRef<SetPassportElement>>(
        &self,
        set_passport_element: C,
    ) -> Result<PassportElement> {
        self.make_request(set_passport_element).await
    }

    // Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed
    pub async fn set_passport_element_errors<C: AsRef<SetPassportElementErrors>>(
        &self,
        set_passport_element_errors: C,
    ) -> Result<Ok> {
        self.make_request(set_passport_element_errors).await
    }

    // Changes the password for the current user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed
    pub async fn set_password<C: AsRef<SetPassword>>(
        &self,
        set_password: C,
    ) -> Result<PasswordState> {
        self.make_request(set_password).await
    }

    // Changes the order of pinned chats
    pub async fn set_pinned_chats<C: AsRef<SetPinnedChats>>(
        &self,
        set_pinned_chats: C,
    ) -> Result<Ok> {
        self.make_request(set_pinned_chats).await
    }

    // Changes the user answer to a poll. A poll in quiz mode can be answered only once
    pub async fn set_poll_answer<C: AsRef<SetPollAnswer>>(&self, set_poll_answer: C) -> Result<Ok> {
        self.make_request(set_poll_answer).await
    }

    // Changes a profile photo for the current user
    pub async fn set_profile_photo<C: AsRef<SetProfilePhoto>>(
        &self,
        set_profile_photo: C,
    ) -> Result<Ok> {
        self.make_request(set_profile_photo).await
    }

    // Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed. If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation
    pub async fn set_recovery_email_address<C: AsRef<SetRecoveryEmailAddress>>(
        &self,
        set_recovery_email_address: C,
    ) -> Result<PasswordState> {
        self.make_request(set_recovery_email_address).await
    }

    // Changes notification settings for chats of a given type
    pub async fn set_scope_notification_settings<C: AsRef<SetScopeNotificationSettings>>(
        &self,
        set_scope_notification_settings: C,
    ) -> Result<Ok> {
        self.make_request(set_scope_notification_settings).await
    }

    // Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot
    pub async fn set_sticker_position_in_set<C: AsRef<SetStickerPositionInSet>>(
        &self,
        set_sticker_position_in_set: C,
    ) -> Result<Ok> {
        self.make_request(set_sticker_position_in_set).await
    }

    // Sets a sticker set thumbnail; for bots only. Returns the sticker set
    pub async fn set_sticker_set_thumbnail<C: AsRef<SetStickerSetThumbnail>>(
        &self,
        set_sticker_set_thumbnail: C,
    ) -> Result<StickerSet> {
        self.make_request(set_sticker_set_thumbnail).await
    }

    // Changes the sticker set of a supergroup; requires can_change_info administrator right
    pub async fn set_supergroup_sticker_set<C: AsRef<SetSupergroupStickerSet>>(
        &self,
        set_supergroup_sticker_set: C,
    ) -> Result<Ok> {
        self.make_request(set_supergroup_sticker_set).await
    }

    // Changes the username of a supergroup or channel, requires owner privileges in the supergroup or channel
    pub async fn set_supergroup_username<C: AsRef<SetSupergroupUsername>>(
        &self,
        set_supergroup_username: C,
    ) -> Result<Ok> {
        self.make_request(set_supergroup_username).await
    }

    // Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
    pub async fn set_tdlib_parameters<C: AsRef<SetTdlibParameters>>(
        &self,
        set_tdlib_parameters: C,
    ) -> Result<Ok> {
        self.make_request(set_tdlib_parameters).await
    }

    // Changes user privacy settings
    pub async fn set_user_privacy_setting_rules<C: AsRef<SetUserPrivacySettingRules>>(
        &self,
        set_user_privacy_setting_rules: C,
    ) -> Result<Ok> {
        self.make_request(set_user_privacy_setting_rules).await
    }

    // Changes the username of the current user
    pub async fn set_username<C: AsRef<SetUsername>>(&self, set_username: C) -> Result<Ok> {
        self.make_request(set_username).await
    }

    // Changes default participant identifier, on whose behalf a video chat in the chat will be joined
    pub async fn set_video_chat_default_participant<C: AsRef<SetVideoChatDefaultParticipant>>(
        &self,
        set_video_chat_default_participant: C,
    ) -> Result<Ok> {
        self.make_request(set_video_chat_default_participant).await
    }

    // Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber
    pub async fn share_phone_number<C: AsRef<SharePhoneNumber>>(
        &self,
        share_phone_number: C,
    ) -> Result<Ok> {
        self.make_request(share_phone_number).await
    }

    // Starts recording of an active group call. Requires groupCall.can_be_managed group call flag
    pub async fn start_group_call_recording<C: AsRef<StartGroupCallRecording>>(
        &self,
        start_group_call_recording: C,
    ) -> Result<Ok> {
        self.make_request(start_group_call_recording).await
    }

    // Starts screen sharing in a joined group call. Returns join response payload for tgcalls
    pub async fn start_group_call_screen_sharing<C: AsRef<StartGroupCallScreenSharing>>(
        &self,
        start_group_call_screen_sharing: C,
    ) -> Result<Text> {
        self.make_request(start_group_call_screen_sharing).await
    }

    // Starts a scheduled group call
    pub async fn start_scheduled_group_call<C: AsRef<StartScheduledGroupCall>>(
        &self,
        start_scheduled_group_call: C,
    ) -> Result<Ok> {
        self.make_request(start_scheduled_group_call).await
    }

    // Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set
    pub async fn stop_poll<C: AsRef<StopPoll>>(&self, stop_poll: C) -> Result<Ok> {
        self.make_request(stop_poll).await
    }

    // Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization
    pub async fn synchronize_language_pack<C: AsRef<SynchronizeLanguagePack>>(
        &self,
        synchronize_language_pack: C,
    ) -> Result<Ok> {
        self.make_request(synchronize_language_pack).await
    }

    // Terminates all other sessions of the current user
    pub async fn terminate_all_other_sessions<C: AsRef<TerminateAllOtherSessions>>(
        &self,
        terminate_all_other_sessions: C,
    ) -> Result<Ok> {
        self.make_request(terminate_all_other_sessions).await
    }

    // Terminates a session of the current user
    pub async fn terminate_session<C: AsRef<TerminateSession>>(
        &self,
        terminate_session: C,
    ) -> Result<Ok> {
        self.make_request(terminate_session).await
    }

    // Returns the received bytes; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_bytes<C: AsRef<TestCallBytes>>(
        &self,
        test_call_bytes: C,
    ) -> Result<TestBytes> {
        self.make_request(test_call_bytes).await
    }

    // Does nothing; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_empty<C: AsRef<TestCallEmpty>>(&self, test_call_empty: C) -> Result<Ok> {
        self.make_request(test_call_empty).await
    }

    // Returns the received string; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_string<C: AsRef<TestCallString>>(
        &self,
        test_call_string: C,
    ) -> Result<TestString> {
        self.make_request(test_call_string).await
    }

    // Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_int<C: AsRef<TestCallVectorInt>>(
        &self,
        test_call_vector_int: C,
    ) -> Result<TestVectorInt> {
        self.make_request(test_call_vector_int).await
    }

    // Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_int_object<C: AsRef<TestCallVectorIntObject>>(
        &self,
        test_call_vector_int_object: C,
    ) -> Result<TestVectorIntObject> {
        self.make_request(test_call_vector_int_object).await
    }

    // Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_string<C: AsRef<TestCallVectorString>>(
        &self,
        test_call_vector_string: C,
    ) -> Result<TestVectorString> {
        self.make_request(test_call_vector_string).await
    }

    // Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_string_object<C: AsRef<TestCallVectorStringObject>>(
        &self,
        test_call_vector_string_object: C,
    ) -> Result<TestVectorStringObject> {
        self.make_request(test_call_vector_string_object).await
    }

    // Forces an updates.getDifference call to the Telegram servers; for testing only
    pub async fn test_get_difference<C: AsRef<TestGetDifference>>(
        &self,
        test_get_difference: C,
    ) -> Result<Ok> {
        self.make_request(test_get_difference).await
    }

    // Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization
    pub async fn test_network<C: AsRef<TestNetwork>>(&self, test_network: C) -> Result<Ok> {
        self.make_request(test_network).await
    }

    // Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization
    pub async fn test_proxy<C: AsRef<TestProxy>>(&self, test_proxy: C) -> Result<Ok> {
        self.make_request(test_proxy).await
    }

    // Returns the specified error and ensures that the Error object is used; for testing only. Can be called synchronously
    pub async fn test_return_error<C: AsRef<TestReturnError>>(
        &self,
        test_return_error: C,
    ) -> Result<Error> {
        self.make_request(test_return_error).await
    }

    // Returns the squared received number; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_square_int<C: AsRef<TestSquareInt>>(
        &self,
        test_square_int: C,
    ) -> Result<TestInt> {
        self.make_request(test_square_int).await
    }

    // Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_use_update<C: AsRef<TestUseUpdate>>(
        &self,
        test_use_update: C,
    ) -> Result<Update> {
        self.make_request(test_use_update).await
    }

    // Changes the value of the default disable_notification parameter, used when a message is sent to a chat
    pub async fn toggle_chat_default_disable_notification<
        C: AsRef<ToggleChatDefaultDisableNotification>,
    >(
        &self,
        toggle_chat_default_disable_notification: C,
    ) -> Result<Ok> {
        self.make_request(toggle_chat_default_disable_notification)
            .await
    }

    // Changes the ability of users to save, forward, or copy chat content. Supported only for basic groups, supergroups and channels. Requires owner privileges
    pub async fn toggle_chat_has_protected_content<C: AsRef<ToggleChatHasProtectedContent>>(
        &self,
        toggle_chat_has_protected_content: C,
    ) -> Result<Ok> {
        self.make_request(toggle_chat_has_protected_content).await
    }

    // Changes the marked as unread state of a chat
    pub async fn toggle_chat_is_marked_as_unread<C: AsRef<ToggleChatIsMarkedAsUnread>>(
        &self,
        toggle_chat_is_marked_as_unread: C,
    ) -> Result<Ok> {
        self.make_request(toggle_chat_is_marked_as_unread).await
    }

    // Changes the pinned state of a chat. There can be up to GetOption("pinned_chat_count_max")/GetOption("pinned_archived_chat_count_max") pinned non-secret chats and the same number of secret chats in the main/arhive chat list
    pub async fn toggle_chat_is_pinned<C: AsRef<ToggleChatIsPinned>>(
        &self,
        toggle_chat_is_pinned: C,
    ) -> Result<Ok> {
        self.make_request(toggle_chat_is_pinned).await
    }

    // Toggles whether the current user will receive a notification when the group call will start; scheduled group calls only
    pub async fn toggle_group_call_enabled_start_notification<
        C: AsRef<ToggleGroupCallEnabledStartNotification>,
    >(
        &self,
        toggle_group_call_enabled_start_notification: C,
    ) -> Result<Ok> {
        self.make_request(toggle_group_call_enabled_start_notification)
            .await
    }

    // Toggles whether current user's video is enabled
    pub async fn toggle_group_call_is_my_video_enabled<
        C: AsRef<ToggleGroupCallIsMyVideoEnabled>,
    >(
        &self,
        toggle_group_call_is_my_video_enabled: C,
    ) -> Result<Ok> {
        self.make_request(toggle_group_call_is_my_video_enabled)
            .await
    }

    // Toggles whether current user's video is paused
    pub async fn toggle_group_call_is_my_video_paused<C: AsRef<ToggleGroupCallIsMyVideoPaused>>(
        &self,
        toggle_group_call_is_my_video_paused: C,
    ) -> Result<Ok> {
        self.make_request(toggle_group_call_is_my_video_paused)
            .await
    }

    // Toggles whether new participants of a group call can be unmuted only by administrators of the group call. Requires groupCall.can_toggle_mute_new_participants group call flag
    pub async fn toggle_group_call_mute_new_participants<
        C: AsRef<ToggleGroupCallMuteNewParticipants>,
    >(
        &self,
        toggle_group_call_mute_new_participants: C,
    ) -> Result<Ok> {
        self.make_request(toggle_group_call_mute_new_participants)
            .await
    }

    // Toggles whether a group call participant hand is rased
    pub async fn toggle_group_call_participant_is_hand_raised<
        C: AsRef<ToggleGroupCallParticipantIsHandRaised>,
    >(
        &self,
        toggle_group_call_participant_is_hand_raised: C,
    ) -> Result<Ok> {
        self.make_request(toggle_group_call_participant_is_hand_raised)
            .await
    }

    // Toggles whether a participant of an active group call is muted, unmuted, or allowed to unmute themselves
    pub async fn toggle_group_call_participant_is_muted<
        C: AsRef<ToggleGroupCallParticipantIsMuted>,
    >(
        &self,
        toggle_group_call_participant_is_muted: C,
    ) -> Result<Ok> {
        self.make_request(toggle_group_call_participant_is_muted)
            .await
    }

    // Pauses or unpauses screen sharing in a joined group call
    pub async fn toggle_group_call_screen_sharing_is_paused<
        C: AsRef<ToggleGroupCallScreenSharingIsPaused>,
    >(
        &self,
        toggle_group_call_screen_sharing_is_paused: C,
    ) -> Result<Ok> {
        self.make_request(toggle_group_call_screen_sharing_is_paused)
            .await
    }

    // Changes the block state of a message sender. Currently, only users and supergroup chats can be blocked
    pub async fn toggle_message_sender_is_blocked<C: AsRef<ToggleMessageSenderIsBlocked>>(
        &self,
        toggle_message_sender_is_blocked: C,
    ) -> Result<Ok> {
        self.make_request(toggle_message_sender_is_blocked).await
    }

    // Toggles whether a session can accept incoming calls
    pub async fn toggle_session_can_accept_calls<C: AsRef<ToggleSessionCanAcceptCalls>>(
        &self,
        toggle_session_can_accept_calls: C,
    ) -> Result<Ok> {
        self.make_request(toggle_session_can_accept_calls).await
    }

    // Toggles whether a session can accept incoming secret chats
    pub async fn toggle_session_can_accept_secret_chats<
        C: AsRef<ToggleSessionCanAcceptSecretChats>,
    >(
        &self,
        toggle_session_can_accept_secret_chats: C,
    ) -> Result<Ok> {
        self.make_request(toggle_session_can_accept_secret_chats)
            .await
    }

    // Toggles whether the message history of a supergroup is available to new members; requires can_change_info administrator right
    pub async fn toggle_supergroup_is_all_history_available<
        C: AsRef<ToggleSupergroupIsAllHistoryAvailable>,
    >(
        &self,
        toggle_supergroup_is_all_history_available: C,
    ) -> Result<Ok> {
        self.make_request(toggle_supergroup_is_all_history_available)
            .await
    }

    // Upgrades supergroup to a broadcast group; requires owner privileges in the supergroup
    pub async fn toggle_supergroup_is_broadcast_group<
        C: AsRef<ToggleSupergroupIsBroadcastGroup>,
    >(
        &self,
        toggle_supergroup_is_broadcast_group: C,
    ) -> Result<Ok> {
        self.make_request(toggle_supergroup_is_broadcast_group)
            .await
    }

    // Toggles whether sender signature is added to sent messages in a channel; requires can_change_info administrator right
    pub async fn toggle_supergroup_sign_messages<C: AsRef<ToggleSupergroupSignMessages>>(
        &self,
        toggle_supergroup_sign_messages: C,
    ) -> Result<Ok> {
        self.make_request(toggle_supergroup_sign_messages).await
    }

    // Changes the owner of a chat. The current user must be a current owner of the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session. Available only for supergroups and channel chats
    pub async fn transfer_chat_ownership<C: AsRef<TransferChatOwnership>>(
        &self,
        transfer_chat_ownership: C,
    ) -> Result<Ok> {
        self.make_request(transfer_chat_ownership).await
    }

    // Removes all pinned messages from a chat; requires can_pin_messages rights in the group or can_edit_messages rights in the channel
    pub async fn unpin_all_chat_messages<C: AsRef<UnpinAllChatMessages>>(
        &self,
        unpin_all_chat_messages: C,
    ) -> Result<Ok> {
        self.make_request(unpin_all_chat_messages).await
    }

    // Removes a pinned message from a chat; requires can_pin_messages rights in the group or can_edit_messages rights in the channel
    pub async fn unpin_chat_message<C: AsRef<UnpinChatMessage>>(
        &self,
        unpin_chat_message: C,
    ) -> Result<Ok> {
        self.make_request(unpin_chat_message).await
    }

    // Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires creator privileges. Deactivates the original basic group
    pub async fn upgrade_basic_group_chat_to_supergroup_chat<
        C: AsRef<UpgradeBasicGroupChatToSupergroupChat>,
    >(
        &self,
        upgrade_basic_group_chat_to_supergroup_chat: C,
    ) -> Result<Chat> {
        self.make_request(upgrade_basic_group_chat_to_supergroup_chat)
            .await
    }

    // Asynchronously uploads a file to the cloud without sending it in a message. updateFile will be used to notify about upload progress and successful completion of the upload. The file will not have a persistent remote identifier until it will be sent in a message
    pub async fn upload_file<C: AsRef<UploadFile>>(&self, upload_file: C) -> Result<File> {
        self.make_request(upload_file).await
    }

    // Uploads a file with a sticker; returns the uploaded file
    pub async fn upload_sticker_file<C: AsRef<UploadStickerFile>>(
        &self,
        upload_sticker_file: C,
    ) -> Result<File> {
        self.make_request(upload_sticker_file).await
    }

    // Validates the order information provided by a user and returns the available shipping options for a flexible invoice
    pub async fn validate_order_info<C: AsRef<ValidateOrderInfo>>(
        &self,
        validate_order_info: C,
    ) -> Result<ValidatedOrderInfo> {
        self.make_request(validate_order_info).await
    }

    // Informs TDLib that messages are being viewed by the user. Sponsored messages must be marked as viewed only when the entire text of the message is shown on the screen (excluding the button). Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)
    pub async fn view_messages<C: AsRef<ViewMessages>>(&self, view_messages: C) -> Result<Ok> {
        self.make_request(view_messages).await
    }

    // Informs the server that some trending sticker sets have been viewed by the user
    pub async fn view_trending_sticker_sets<C: AsRef<ViewTrendingStickerSets>>(
        &self,
        view_trending_sticker_sets: C,
    ) -> Result<Ok> {
        self.make_request(view_trending_sticker_sets).await
    }

    // Writes a part of a generated file. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file
    pub async fn write_generated_file_part<C: AsRef<WriteGeneratedFilePart>>(
        &self,
        write_generated_file_part: C,
    ) -> Result<Ok> {
        self.make_request(write_generated_file_part).await
    }
}
