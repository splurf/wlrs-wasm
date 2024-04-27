use yew::{html, Html};

enum Status {
    Success,
    Warning,
    Failure,
    Initial,
}

impl Status {
    const fn as_str(&self) -> &'static str {
        match self {
            Status::Success => "#4dff4d",
            Status::Warning => "#ffe400",
            Status::Failure => "#ff5050",
            Status::Initial => "#ffffff",
        }
    }
}

#[derive(PartialEq)]
pub enum StatusKind {
    Initial,
    Connection,
    ServerDown,
    PlayerNotFound,
    Whitelisted,
    Success,
    InvalidInput,
    Connecting,
    Unexpected,
}

impl StatusKind {
    pub const fn from_u8(byte: &u8) -> Self {
        match byte {
            0 => Self::ServerDown,
            1 => Self::PlayerNotFound,
            2 => Self::Whitelisted,
            3 => Self::Success,
            _ => Self::Unexpected,
        }
    }

    const fn status(&self) -> Status {
        match self {
            Self::Success | Self::Whitelisted => Status::Success,
            Self::PlayerNotFound | Self::InvalidInput => Status::Warning,
            Self::Connection | Self::ServerDown | Self::Unexpected => Status::Failure,
            Self::Connecting => Status::Initial,
            _ => unreachable!(),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Connection => "Failed to connect to server",
            Self::ServerDown => "Minecraft server is down",
            Self::PlayerNotFound => "Player doesn't exist",
            Self::Whitelisted => "Already whitelisted",
            Self::Success => "Success",
            Self::InvalidInput => "Invalid input",
            Self::Connecting => "Connecting...",
            Self::Unexpected => "Unexpected server response",
            _ => unreachable!(),
        }
    }

    pub const fn is_new(&self) -> bool {
        !matches!(self, Self::Initial)
    }

    pub fn as_html(&self) -> Html {
        html! {
            <p style={"color: ".to_owned() + self.status().as_str() + "; font-size: large;"}> {self.as_str()} </p>
        }
    }
}
