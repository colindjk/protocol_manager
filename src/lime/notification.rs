use serde_json::{ Map, Value };

/// Signifies the event which pertains to a previously dealt with message.
/// Uses 'id' from sent message to determine which one should happen.
/// TODO: Unique set of 'id's per user or nah?
#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationEvent {
    #[serde(rename="accepted")]     Accepted,
    #[serde(rename="validated")]    Validated,
    #[serde(rename="authorized")]   Authorized,
    #[serde(rename="dispatched")]   Dispatched,
    #[serde(rename="received")]     Received,
    #[serde(rename="consumed")]     Consumed,
    #[serde(rename="failed")]       Failed(Reason),
}

/// When an Error occurs, this will exist.
#[derive(Debug, Serialize, Deserialize)]
pub struct Reason {
    code: u8,
    description: String
}

struct Notification {
    map: Map<String, Value>
}

