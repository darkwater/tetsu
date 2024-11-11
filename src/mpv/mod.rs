use anyhow::{Context, Result};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{UnixListener, UnixStream},
    process::{Child, Command},
    time::{sleep, Duration},
};

mod request;
mod response;

pub use request::*;

pub struct Mpv {
    process: Child,
    connection: Option<UnixStream>,
}

impl Mpv {
    pub async fn new() -> Result<Self> {
        let _ = UnixListener::bind("/tmp/mpv-socket");

        let process = Command::new("mpv")
            .arg("--idle")
            .arg("--input-ipc-server=/tmp/mpv-socket")
            .spawn()
            .context("Failed to spawn mpv")?;

        // sleep(Duration::from_millis(100)).await;

        Ok(Self { process, connection: None })
    }

    pub async fn connect(&mut self) -> Result<&mut UnixStream> {
        match self.connection {
            Some(ref mut conn) => Ok(conn),
            None => {
                for _ in 0..20 {
                    match UnixStream::connect("/tmp/mpv-socket").await {
                        Ok(conn) => {
                            self.connection = Some(conn);
                            return Ok(self.connection.as_mut().unwrap());
                        }
                        Err(e) if e.kind() == io::ErrorKind::ConnectionRefused => {
                            sleep(Duration::from_millis(100)).await;
                        }
                        Err(e) => return Err(e.into()),
                    }
                }

                Err(anyhow::anyhow!("Connection to mpv timed out"))
            }
        }
    }

    pub async fn request<C: request::Command>(&mut self, command: C) -> Result<C::Output> {
        let conn = self.connect().await.context("failed to connect to mpv")?;

        let req = Request {
            command,
            request_id: None,
            r#async: false,
        };

        let req = serde_json::to_string(&req)?;
        conn.write_all(req.as_bytes()).await?;
        conn.write_all(b"\n").await?;

        let mut buf = String::new();
        BufReader::new(conn).read_line(&mut buf).await?;

        let resp = serde_json::from_str::<response::Response>(&buf)
            .with_context(|| format!("Received invalid response: {buf}"))?;

        resp.into_result_of()
    }

    pub async fn wait(&mut self) -> Result<()> {
        self.process.wait().await?;

        Ok(())
    }
}

impl Drop for Mpv {
    fn drop(&mut self) {
        let _ = self.process.start_kill();
    }
}
