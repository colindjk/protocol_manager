use serde_json::{ Map, Value };

/// -- Global Constants --
pub static DELIMITER : u8 = b'\n' as u8;
type JsonMap = Map<String, Value>;

// Envelope types
#[macro_use]
pub mod macros;

pub mod message;
pub mod notification;
pub mod command;
pub mod session;

// Types heald by envelopes
pub mod reason;
pub mod resources;

mod ser;
mod codec;
mod helper;

pub use self::codec::{LimeCodec, EnvelopeStream};

pub use self::message::{Message, Content};
pub use self::notification::{Notification, NotificationEvent};
pub use self::command::Command;
pub use self::session::*;

pub use self::reason::Reason as ErrReason;

pub type Node = String;
pub type UserID = String;
pub type Resources = Value;
pub type MsgID = u64;
pub type TimeStamp = u64;

/// Known / supported types of envelopes.
pub enum EnvelopeType {
    Message, Notification, Command, Session
}

/// Outlines the kinds of envelopes one can receive.
/// TODO: Resource field as separate struct, uri?
#[derive(Debug)]
pub enum Envelope {
    Message(Message),
    Notification(Notification),
    Command(Command),
    Session(Session),
    Unknown(JsonMap),
}

impl Envelope {
    pub fn id(&self) -> Option<MsgID> {
        use self::Envelope::*;
        match *self {
            Message(ref val) => val.id,
            Notification(ref val) => Some(val.id),
            Command(ref val) => val.id,
            Session(ref val) => Some(val.id),
            Unknown(ref map) => {
                if let Some(val) = map.get("id") {
                    val.as_u64()
                } else { None }
            },
        }
    }
}

impl Into<Envelope> for Message {
    fn into(self) -> Envelope {
        Envelope::Message(self)
    }
}

impl Into<Envelope> for Notification {
    fn into(self) -> Envelope {
        Envelope::Notification(self)
    }
}

impl Into<Envelope> for Command {
    fn into(self) -> Envelope {
        Envelope::Command(self)
    }
}

impl Into<Envelope> for Session {
    fn into(self) -> Envelope {
        Envelope::Session(self)
    }
}

/// Trait for all envelope related types.
/// TODO: Include 'pp' and 'metadata'
/// TODO: Convert to MIME
pub trait EnvelopeTrait {
    //type Ty;

    //fn unique<'a>() -> &'a str;

    fn id(&self) -> Option<&str>;
    fn to(&self) -> Option<&str>;
    fn from(&self) -> Option<&str>;
    //fn pp(&self) -> Option<Node>;
    //fn metadata(&self) -> Option<Node>;

    fn set_id(&mut self, Option<String>);
    fn set_to(&mut self, Option<String>);
    fn set_from(&mut self, Option<String>);
    //fn set_pp(&self) -> Option<Node>;
    //fn set_metadata(&self) -> Option<Node>;
}
