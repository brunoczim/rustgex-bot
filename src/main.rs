use futures::StreamExt;
use regex::{Regex, RegexBuilder};
use std::{collections::VecDeque, env, error::Error as StdError, fmt, process};
use telegram_bot::CanReplySendMessage;

const MESSAGE_LIMIT: usize = 4000;

#[derive(Debug)]
enum FatalError {
    MissingToken(env::VarError),
    Telegram(telegram_bot::Error),
}

impl fmt::Display for FatalError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FatalError::MissingToken(cause) => {
                write!(fmtr, "Problem with TELEGRAM_BOT_TOKEN: {}", cause)?
            },
            FatalError::Telegram(cause) => write!(fmtr, "{}", cause)?,
        }
        Ok(())
    }
}

impl StdError for FatalError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            FatalError::MissingToken(cause) => Some(cause),
            FatalError::Telegram(cause) => Some(cause),
        }
    }
}

impl From<telegram_bot::Error> for FatalError {
    fn from(cause: telegram_bot::Error) -> Self {
        FatalError::Telegram(cause)
    }
}

#[derive(Debug)]
enum RuleError {
    MissingMessage,
    MissingReplacement,
    UnrecognizedFlag(char),
    DuplicatedFlag(char),
    InvalidRegex(regex::Error),
}

impl fmt::Display for RuleError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleError::MissingMessage => {
                write!(fmtr, "No message could be reached")?
            },
            RuleError::MissingReplacement => {
                write!(fmtr, "Replacement is missing")?
            },
            RuleError::UnrecognizedFlag(character) => {
                write!(fmtr, "Unrecognized flag {:?}", character)?
            },
            RuleError::DuplicatedFlag(character) => {
                write!(fmtr, "Flag {:?} given more than once", character)?
            },
            RuleError::InvalidRegex(cause) => {
                write!(fmtr, "Invalid regex:\n{}", cause)?
            },
        }
        Ok(())
    }
}

impl StdError for RuleError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            RuleError::InvalidRegex(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<regex::Error> for RuleError {
    fn from(cause: regex::Error) -> Self {
        RuleError::InvalidRegex(cause)
    }
}

#[derive(Debug)]
enum CommandError {
    Rule(RuleError),
}

impl fmt::Display for CommandError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::Rule(cause) => write!(fmtr, "{}", cause)?,
        }
        Ok(())
    }
}

impl StdError for CommandError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CommandError::Rule(cause) => Some(cause),
        }
    }
}

impl From<RuleError> for CommandError {
    fn from(cause: RuleError) -> Self {
        CommandError::Rule(cause)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Flags {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    octal: bool,
    global: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            ignore_whitespace: false,
            unicode: false,
            octal: false,
            global: false,
        }
    }
}

impl Flags {
    fn parse(flag_str: &str) -> Result<Self, RuleError> {
        let mut this = Self::default();

        for character in flag_str.chars() {
            let mut duplicated = false;
            match character {
                'i' => Self::set(&mut this.case_insensitive, character)?,
                'm' => Self::set(&mut this.multi_line, character)?,
                's' => Self::set(&mut this.dot_matches_new_line, character)?,
                'U' => Self::set(&mut this.swap_greed, character)?,
                'x' => Self::set(&mut this.ignore_whitespace, character)?,
                'o' => Self::set(&mut this.octal, character)?,
                'g' => Self::set(&mut this.global, character)?,
                _ => Err(RuleError::UnrecognizedFlag(character))?,
            }
        }

        Ok(this)
    }

    fn set(field: &mut bool, flag_char: char) -> Result<(), RuleError> {
        if *field {
            Err(RuleError::DuplicatedFlag(flag_char))
        } else {
            *field = true;
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
struct Rule<'msg> {
    regex: Regex,
    replacment: &'msg str,
    flags: Flags,
}

impl<'msg> Rule<'msg> {
    fn from_command(command: RuleCommand<'msg>) -> Result<Self, RuleError> {
        let flags = Flags::parse(command.flags)?;
        let regex = RegexBuilder::new(command.search)
            .case_insensitive(flags.case_insensitive)
            .multi_line(flags.multi_line)
            .dot_matches_new_line(flags.dot_matches_new_line)
            .swap_greed(flags.swap_greed)
            .ignore_whitespace(flags.ignore_whitespace)
            .unicode(flags.unicode)
            .octal(flags.octal)
            .build()?;
        Ok(Self { regex, replacment: command.replacement, flags })
    }

    fn replace(&self, subject: &str) -> String {
        if self.flags.global {
            self.regex.replace_all(subject, self.replacment).into_owned()
        } else {
            self.regex.replace(subject, self.replacment).into_owned()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RuleCommand<'msg> {
    search: &'msg str,
    replacement: &'msg str,
    flags: &'msg str,
}

impl<'msg> RuleCommand<'msg> {
    fn parse(message: &'msg str) -> Result<Option<Self>, RuleError> {
        let mut data = message;
        if let Some((_, tail)) = data.split_once("s/") {
            data = tail;
            let (search, tail) = Self::split_next_param(data)
                .ok_or(RuleError::MissingReplacement)?;
            data = tail;
            let (replacement, flags) =
                Self::split_next_param(data).unwrap_or((data, ""));
            Ok(Some(Self { search, replacement, flags }))
        } else {
            Ok(None)
        }
    }

    fn split_next_param(input: &str) -> Option<(&str, &str)> {
        let mut escape = false;
        input.split_once(|character| {
            let should_split = character == '/' && !escape;
            escape = character == '\\';
            should_split
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HelpCommand;

impl HelpCommand {
    fn parse(message: &str, handle: Option<&str>) -> Option<Self> {
        if message.trim() == "/help" {
            Some(HelpCommand)
        } else {
            match message.split_once("@") {
                Some(("/help", maybe_handle)) if maybe_handle == handle? => {
                    Some(HelpCommand)
                },
                _ => None,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Command<'msg> {
    Help(HelpCommand),
    Rule(RuleCommand<'msg>),
}

impl<'msg> Command<'msg> {
    fn parse(
        message: &str,
        handle: Option<&str>,
    ) -> Result<Option<Self>, CommandError> {
        if let Some(command) = HelpCommand::parse(message, handle) {
            return Ok(Some(Command::Help(command)));
        }
        if let Some(command) = RuleCommand::parse(message)? {
            return Ok(Some(Command::Rule(command)));
        }
        Ok(None)
    }
}

struct App {
    previous_msg_or_post: Option<telegram_bot::MessageOrChannelPost>,
    handle: Option<String>,
}

impl App {
    async fn handle_update(
        &mut self,
        update: telegram_bot::Update,
        api: &telegram_bot::Api,
    ) -> Result<(), FatalError> {
        match update.kind {
            telegram_bot::UpdateKind::Message(message) => {
                self.handle_msg_or_post(
                    &telegram_bot::MessageOrChannelPost::Message(message),
                    api,
                )
                .await?;
            },
            telegram_bot::UpdateKind::ChannelPost(post) => {
                self.handle_msg_or_post(
                    &telegram_bot::MessageOrChannelPost::ChannelPost(post),
                    api,
                )
                .await?;
            },
            _ => (),
        }
        Ok(())
    }

    async fn handle_msg_or_post(
        &mut self,
        msg_or_post: &telegram_bot::MessageOrChannelPost,
        api: &telegram_bot::Api,
    ) -> Result<(), FatalError> {
        match msg_or_post {
            telegram_bot::MessageOrChannelPost::Message(message) => {
                if let telegram_bot::MessageKind::Text {
                    data: msg_data, ..
                } = &message.kind
                {
                    let result = self.run_command(msg_or_post, msg_data);
                    self.execute_command_result(msg_or_post, result, api)
                        .await?;
                }
            },
            telegram_bot::MessageOrChannelPost::ChannelPost(post) => {
                if let telegram_bot::MessageKind::Text {
                    data: msg_data, ..
                } = &post.kind
                {
                    let result = self.run_command(msg_or_post, msg_data);
                    self.execute_command_result(msg_or_post, result, api)
                        .await?;
                }
            },
        }
        Ok(())
    }

    fn run_command(
        &mut self,
        msg_or_post: &telegram_bot::MessageOrChannelPost,
        message: &str,
    ) -> Result<Option<String>, CommandError> {
        match Command::parse(message, self.handle.as_ref().map(String::as_ref))?
        {
            Some(Command::Help(_)) => Ok(Some(format!(
                "{}{}{}",
                "Commands:\n",
                "/help - Sends this help message\n",
                "s/search/replace/flags  - Replaces <search> with <replace> \
                 using (optional) <flags>\n",
            ))),

            Some(Command::Rule(rule)) => {
                Ok(Some(Rule::from_command(rule)?.replace()))
            },

            None => Ok(None),
        }
    }

    fn target_message(
        &self,
        msg_or_post: &telegram_bot::MessageOrChannelPost,
    ) -> Option<&telegram_bot::MessageOrChannelPost> {
        let reply_to_message = match msg_or_post {
            telegram_bot::MessageOrChannelPost::Message(message) => {
                &message.reply_to_message
            },

            telegram_bot::MessageOrChannelPost::ChannelPost(post) => {
                &post.reply_to_message
            },
        };

        reply_to_message
            .as_ref()
            .map(Box::as_ref)
            .or(self.previous_msg_or_post.as_ref())
    }

    async fn execute_command_result(
        &mut self,
        msg_or_post: &telegram_bot::MessageOrChannelPost,
        result: Result<Option<String>, CommandError>,
        api: &telegram_bot::Api,
    ) -> Result<(), FatalError> {
        match result {
            Ok(Some(reply)) => match self.target_message(msg_or_post) {
                Some(to) => {
                    api.send(to.text_reply(reply)).await?;
                },
                None => {
                    api.send(
                        msg_or_post
                            .text_reply("Could not found a message to reply"),
                    )
                    .await?;
                },
            },

            Ok(None) => (),

            Err(error) => {
                api.send(msg_or_post.text_reply(error.to_string())).await?;
            },
        }
        Ok(())
    }
}

async fn try_main() -> Result<(), FatalError> {
    let token =
        env::var("TELEGRAM_BOT_TOKEN").map_err(FatalError::MissingToken)?;
    let telegram_api = telegram_bot::Api::new(token);
    let mut telegram_stream = telegram_api.stream();

    let mut messages = VecDeque::new();

    while let Some(update) = telegram_stream.next().await.transpose()? {
        if let telegram_bot::UpdateKind::Message(message) = update.kind {
            if let telegram_bot::MessageKind::Text { data: raw_data, .. } =
                &message.kind
            {
                let mut data = raw_data.trim();
                if let Some((_, tail)) = data.split_once("s/") {
                    data = tail;
                    let mut last_char = 'a';
                    match data.split_once(|current_char| {
                        let should_split =
                            current_char == '/' && last_char != '\\';
                        last_char = current_char;
                        should_split
                    }) {
                        Some((search, tail)) => {
                            data = tail;
                            last_char = 'a';
                            let (replacement, flags) = data
                                .split_once(|current_char| {
                                    let should_split = current_char == '/'
                                        && last_char != '\\';
                                    last_char = current_char;
                                    should_split
                                })
                                .unwrap_or((data, ""));
                            match &message.reply_to_message {
                                Some(boxed_target) => {
                                    if let telegram_bot::MessageOrChannelPost::Message(replied_message) = boxed_target.as_ref() {
                                        if let telegram_bot::MessageKind::Text { data: replied_data, entities } =
                                            &replied_message.kind
                                        {

                                            match regex::Regex::new(search) {
                                                Ok(regex) => {
                                            telegram_api.send(replied_message.text_reply(
                                                    &regex.replace(replied_data, replacement),
                                                    )).await?;
                                                },
                                                Err(_) => todo!("send error"),
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        None => todo!("send error"),
                    }
                }
            }
            messages.push_front(message);
            messages.truncate(MESSAGE_LIMIT);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = try_main().await {
        eprintln!("{}", error);
        process::exit(1);
    }
}
