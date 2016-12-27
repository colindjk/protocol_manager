use serde_json::{ Map, Value };

use lime::envelope::*;
use lime::JsonMap;

pub type Content = Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub to: Option<Node>,
    pub from: Option<Node>,
    pub pp: Option<Node>,
    pub id: Option<MsgID>,
    pub metadata: Option<JsonMap>,

    #[serde(rename="type")]
    pub mime_type: String,
    pub content: Content,
}

impl Message {

}

impl_Envelope!(Message,
               MessageType,
               |_| Some(MessageType::Normal),
               Some(MessageType::Normal),
               "content");

/// TODO: Figure out other possible message types.
pub enum MessageType {
    Normal,
    Chat,
    Groupchat,
    Error
}

