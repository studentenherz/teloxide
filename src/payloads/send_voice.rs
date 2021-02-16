// This file is auto generated by `cg` <https://github.com/teloxide/cg> (8ee7ef2).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup};

impl_payload! {
    @[multipart]
    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as [`Audio`] or [`Document`]). On success, the sent [`Message`] is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    ///
    /// [`Document`]: crate::types::Document
    /// [`Audio`]: crate::types::Audio
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendVoice (SendVoiceSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub voice: InputFile,
        }
        optional {
            /// Voice message caption, 0-1024 characters after entities parsing
            pub caption: String [into],
            /// Mode for parsing entities in the voice message caption. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// Duration of the voice message in seconds
            pub duration: u32,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i32,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
