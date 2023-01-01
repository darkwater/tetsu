use std::collections::HashMap;

use serde::{de::DeserializeOwned, ser::SerializeSeq, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Request<C: Command> {
    #[serde(serialize_with = "C::serialize")]
    pub command: C,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<u64>,
    #[serde(skip_serializing_if = "is_false")]
    pub r#async: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

pub trait Command {
    type Output: DeserializeOwned;

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

macro_rules! request {
    ($name:ident $ser:literal ( $($arg:ident: $argty:ty ),* ) -> $retty:ty) => {
        pub struct $name {
            $(pub $arg: $argty,)*
        }

        impl Command for $name {
            type Output = $retty;

            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(None)?;
                seq.serialize_element($ser)?;
                $( seq.serialize_element(&self.$arg)?;)*
                seq.end()
            }
        }

        impl $name {
            pub fn new($($arg: $argty,)*) -> Self {
                $name { $($arg,)* }
            }
        }
    };
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoadfileMode {
    #[default]
    Replace,
    Append,
    AppendPlay,
}

impl From<LoadfileMode> for Value {
    fn from(mode: LoadfileMode) -> Self {
        match mode {
            LoadfileMode::Replace => "replace".into(),
            LoadfileMode::Append => "append".into(),
            LoadfileMode::AppendPlay => "append-play".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadfileResponse {
    /// Seems 1-indexed
    pub playlist_entry_id: u64,
}

request!(GetProperty "get_property" (name: String) -> Value);
request!(SetProperty "set_property" (name: String, value: Value) -> ());
request!(Loadfile "loadfile" (path: String, mode: LoadfileMode) -> LoadfileResponse);
request!(Stop "stop" () -> ());
request!(PlaylistPlayIndex "playlist-play-index" (index: u64) -> ());
