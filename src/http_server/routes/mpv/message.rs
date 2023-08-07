use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum Message {
    Control(ControlMessage),
    Mpv(serde_json::Value),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ControlMessage {
    Start,
    Started,
    Stop,
    Stopped,
}

impl From<ControlMessage> for Message {
    fn from(msg: ControlMessage) -> Self {
        Self::Control(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let msg = Message::Control(ControlMessage::Start);
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"kind":"Control","data":{"kind":"Start"}}"#);
    }
}
