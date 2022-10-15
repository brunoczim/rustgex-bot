use crate::{
    domain::{self, MessageData},
    future::DynFuture,
    port::{Disconnected, Receiver, Sender},
};
use core::fmt;
use futures::StreamExt;
use telegram_bot::{
    Api,
    ChatId,
    Error,
    MessageId,
    MessageKind,
    MessageOrChannelPost,
    SendMessage,
    UpdateKind,
};

fn tg_message_to_domain(
    msg_or_post: MessageOrChannelPost,
) -> Option<domain::Message<MessageId, ChatId>> {
    fn convert_with_custom_reply(
        msg_or_post: MessageOrChannelPost,
        convert_reply: bool,
    ) -> Option<domain::Message<MessageId, ChatId>> {
        let (id, chat_id, replying_to, kind) = match msg_or_post {
            MessageOrChannelPost::Message(message) => (
                message.id,
                message.chat.id(),
                message.reply_to_message,
                message.kind,
            ),
            MessageOrChannelPost::ChannelPost(post) => (
                post.id,
                ChatId::from(post.chat.id),
                post.reply_to_message,
                post.kind,
            ),
        };

        if let MessageKind::Text { data, .. } = kind {
            Some(domain::Message {
                id,
                data: MessageData {
                    chat_id,
                    content: data,
                    reply_target: if convert_reply {
                        match replying_to.and_then(|msg_or_post| {
                            convert_with_custom_reply(*msg_or_post, false)
                        }) {
                            Some(message) => {
                                domain::ReplyTarget::Message(Box::new(message))
                            },
                            None => domain::ReplyTarget::NotReplying,
                        }
                    } else {
                        domain::ReplyTarget::Prunned
                    },
                },
            })
        } else {
            None
        }
    }

    convert_with_custom_reply(msg_or_post, true)
}

#[derive(Clone)]
pub struct TgMessageChannel {
    api: Api,
}

impl TgMessageChannel {
    pub fn new(token: &str) -> Self {
        Self { api: Api::new(token) }
    }
}

impl fmt::Debug for TgMessageChannel {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("TgMessageChannel").finish_non_exhaustive()
    }
}

impl domain::Id for MessageId {}
impl domain::Id for ChatId {}

impl Sender for TgMessageChannel {
    type Error = Error;
    type MessageId = MessageId;
    type ChatId = ChatId;

    fn send<'fut>(
        &'fut self,
        message: &'fut domain::NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        Box::pin(async move {
            let mut request =
                SendMessage::new(message.data.chat_id, &message.data.content);
            match &message.data.reply_target {
                domain::ReplyTarget::Message(message) => {
                    request.reply_to(message.id);
                },
                domain::ReplyTarget::MessageId(message_id) => {
                    request.reply_to(message_id);
                },
                _ => (),
            }
            self.api.send(request).await?;
            Ok(())
        })
    }
}

impl Receiver for TgMessageChannel {
    type Error = Error;
    type MessageId = MessageId;
    type ChatId = ChatId;

    fn receive<'fut>(
        &'fut self,
    ) -> DynFuture<
        'fut,
        Result<
            Result<
                domain::Message<Self::MessageId, Self::ChatId>,
                Disconnected,
            >,
            Self::Error,
        >,
    > {
        Box::pin(async move {
            loop {
                let update = match self.api.stream().next().await {
                    Some(result) => result?,
                    None => break Ok(Err(Disconnected)),
                };
                match update.kind {
                    UpdateKind::Message(message) => {
                        if let Some(domain_msg) = tg_message_to_domain(
                            MessageOrChannelPost::Message(message),
                        ) {
                            break Ok(Ok(domain_msg));
                        }
                    },
                    UpdateKind::ChannelPost(post) => {
                        if let Some(domain_msg) = tg_message_to_domain(
                            MessageOrChannelPost::ChannelPost(post),
                        ) {
                            break Ok(Ok(domain_msg));
                        }
                    },
                    _ => (),
                }
            }
        })
    }
}
