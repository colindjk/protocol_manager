use serde_json::{ Value };

use envelope::{JsonMap, Node, MsgID};

mod ser;

pub type Content = Value;

#[derive(Debug, Serialize)]
pub struct Message {
    pub to: Option<Node>,
    pub from: Option<Node>,
    pub pp: Option<Node>,
    pub id: Option<MsgID>,
    pub metadata: Option<JsonMap>,

    pub mime_type: String,
    pub content: Content,
}

impl Message {

}

// TODO : Import this
//impl_Envelope!(Message,
               //MessageType,
               //|_| Some(MessageType::Normal),
               //Some(MessageType::Normal),
               //"content");

/// TODO: Figure out other possible message types.
pub enum MessageType {
    Normal,
    Chat,
    Groupchat,
    Error
}

