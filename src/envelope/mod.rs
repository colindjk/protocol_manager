use serde_json::{ Map, Value };

/// -- Global Constants --
pub static DELIMITER : u8 = b'\n' as u8;
type JsonMap = Map<String, Value>;

#[macro_use]
pub mod envelope;
pub mod message;
pub mod notification;
pub mod command;
pub mod session;
pub mod codec;

mod helper;
use self::helper::*;

pub use self::codec::LimeCodec;
pub use self::envelope::*;
pub use self::message::{Message, Content};
pub use self::notification::{Notification, NotificationEvent};
pub use self::command::Command;
pub use self::session::*;

/// Outlines the kinds of envelopes one can receive.
/// TODO: HOW SHOULD I HANDLE DIFFERENT ENVELOPE TYPES WAAA
#[derive(Debug)]
pub enum SealedEnvelope {
    Message(Message),
    Notification(Notification),
    Command(Command),
    SessionReq(SessionRequest),
    SessionRes(SessionResponse),
    Unknown(JsonMap),
}

// SerDe section

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, MapVisitor};

/// When an Error occurs, this will exist.
/// TODO: Use this for other structs aside from just Notification.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrReason {
    pub code: u8,
    pub description: Option<String>
}

/// Deserialization implementation distinguishes the specific type of 'frame'
/// being received.
impl Deserialize for SealedEnvelope {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        struct EnvelopeVisitor;

        impl Visitor for EnvelopeVisitor {
            type Value = SealedEnvelope;

            fn visit_map<V>(&mut self, mut visitor: V)
                            -> Result<SealedEnvelope, V::Error>
                where V: MapVisitor,
            {
                let mut to          = None;
                let mut from        = None;
                let mut pp          = None;
                let mut id          = None;
                let mut metadata    = None;

                let mut content     = None;
                let mut event       = None;
                let mut method      = None;
                let mut state       = None;

                let mut status      = None;
                let mut encryption  = None;
                let mut compression = None;
                let mut scheme      = None;
                let mut e_options   = None;
                let mut c_options   = None;
                let mut s_options   = None;

                let mut mime_type   = None;
                let mut uri         = None;
                let mut reason      = None;
                let mut other       = Map::new();

                use self::helper::FieldHelper::*;
                while let Some(field) = visitor.visit_key()? {
                    match field {
                        To => to = Some(visitor.visit_value()?),
                        From => from = Some(visitor.visit_value()?),
                        Pp => pp = Some(visitor.visit_value()?),
                        Id => id = Some(visitor.visit_value()?),
                        Metadata => metadata = Some(visitor.visit_value()?),

                        Content => content = Some(visitor.visit_value()?),
                        Event => event = Some(visitor.visit_value()?),
                        Method => method = Some(visitor.visit_value()?),
                        State => state = Some(visitor.visit_value()?),

                        Encryption => encryption = Some(visitor.visit_value()?),
                        Compression => compression = Some(visitor.visit_value()?),
                        Scheme => scheme = Some(visitor.visit_value()?),
                        EncryptionOptions => e_options = Some(visitor.visit_value()?),
                        CompressionOptions => c_options = Some(visitor.visit_value()?),
                        SchemeOptions => s_options = Some(visitor.visit_value()?),

                        Type => mime_type = Some(visitor.visit_value()?),
                        Uri => uri = Some(visitor.visit_value()?),
                        Reason => reason = Some(visitor.visit_value()?),
                        Status => status = Some(visitor.visit_value()?),
                        Other(key) => {
                            other.insert(key, visitor.visit_value()?);
                        }
                    }
                }
                visitor.end()?;

                // TODO: Match all fields which are at some point required.
                Ok(match (content, event, method, state, id, mime_type) {
                    (Some(content), None, None, None, id, Some(mime_type)) => {
                        SealedEnvelope::Message(Message {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            mime_type: mime_type,
                            content: content,
                        })
                    }
                    (None, Some(event), None, None, Some(id), None) => {
                        let event = into_event(event, reason);
                        SealedEnvelope::Notification(Notification {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            event: event,
                        })
                    }
                    (None, None, Some(method), None, id, mime_type) => {
                        let status = into_status(status, reason);
                        SealedEnvelope::Command(Command {
                            to: to,
                            from: from,
                            pp: pp,
                            id: id,
                            metadata: metadata,
                            mime_type: mime_type,
                            method: method,
                            status: status,
                            uri: uri,
                        })
                    }
                    (None, None, None, Some(state), Some(id), None) => {
                        let state = into_state(state, reason);
                        match (encryption, compression, scheme) {
                            (None, None, None) => {
                                SealedEnvelope::SessionReq(SessionRequest {
                                    to: to,
                                    from: from,
                                    pp: pp,
                                    id: id,
                                    metadata: metadata,
                                    state: state,
                                    encryption_options: e_options,
                                    compression_options: c_options,
                                    scheme_options: s_options,
                                })
                            }
                            (encryption, compression, scheme) => {
                                SealedEnvelope::SessionRes(SessionResponse {
                                    to: to,
                                    from: from,
                                    pp: pp,
                                    id: id,
                                    metadata: metadata,
                                    state: state,
                                    encryption: encryption,
                                    compression: compression,
                                    scheme: scheme,
                                })
                            }
                        }
                    }
                    _ => unimplemented!(), // take care of this at some point
                })
            }
        }

        deserializer.deserialize_map(EnvelopeVisitor)
    }
}

impl Serialize for SealedEnvelope {
    fn serialize<S>(&self, serializer: &mut S)
            -> Result<(), S::Error> where S: Serializer
    {
        use self::SealedEnvelope::*;
        match *self {
            Message(ref val)      => val.serialize(serializer),
            Notification(ref val) => val.serialize(serializer),
            //Command(ref val)        => val.serialize(serializer),
            //SessionReq(ref val)     => val.serialize(serializer),
            //SessionRes(ref val)     => val.serialize(serializer),
            //Unknown(ref val)        => val.serialize(serializer),
            _ => panic!()
        }
    }
}