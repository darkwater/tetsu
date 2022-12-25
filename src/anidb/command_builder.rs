use std::fmt::{Display, Write};

pub struct CommandBuilder {
    name: String,
    args: Vec<(String, String)>,
}

impl CommandBuilder {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), args: vec![] }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arg<T: CmdArgument>(mut self, key: &str, value: T) -> Self {
        self.args.retain(|(k, _)| k != key);

        self.args
            .push((key.to_string(), value.escaped().to_string()));

        self
    }
}

impl Display for CommandBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut add_amp = false;

        for (key, value) in self.args.iter() {
            if add_amp {
                s += "&";
            } else {
                add_amp = true;
            }

            s += &format!("{key}={value}");
        }

        writeln!(f, "{} {s}", self.name)
    }
}

pub trait CmdArgument {
    type Output: Display;

    fn escaped(&self) -> Self::Output;
}

macro_rules! simple_cmdarg {
    ($($t:ty)*) => {
        $(
            impl CmdArgument for $t {
                type Output = $t;

                fn escaped(&self) -> Self::Output {
                    *self
                }
            }
        )*
    };
}

simple_cmdarg!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

impl CmdArgument for &str {
    type Output = String;

    fn escaped(&self) -> String {
        self.replace('&', "&amp;").replace('\n', "<br />")
    }
}

impl CmdArgument for String {
    type Output = String;

    fn escaped(&self) -> String {
        self.as_str().escaped()
    }
}

impl CmdArgument for &[u8] {
    type Output = String;

    fn escaped(&self) -> String {
        let mut out = String::new();

        for byte in self.iter() {
            write!(out, "{byte:02x}").unwrap();
        }

        out
    }
}

impl CmdArgument for bool {
    type Output = u8;

    fn escaped(&self) -> Self::Output {
        if *self {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            CommandBuilder::new("FOO")
                .arg("user", "name")
                .arg("pass", "w&rd")
                .arg("weeb", true)
                .arg("iq", 9000)
                .arg("bytes", &[0xd0, 0x0d][..])
                .to_string(),
            "FOO user=name&pass=w&amp;rd&weeb=1&iq=9000&bytes=d00d\n"
        );
    }
}
