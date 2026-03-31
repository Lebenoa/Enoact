use std::io::{BufWriter, ErrorKind, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

use anyhow::Result;
use discord_rich_presence::activity::Activity;

use crate::client_id::ClientId;

pub const PRESENCE_BIN: &str = "enoact-presence";

struct PresenceInstance {
    child: Child,
    stdin: BufWriter<ChildStdin>,
}

impl PresenceInstance {
    #[inline]
    fn new(mut child: Child) -> Result<Self> {
        let Some(stdin) = child.stdin.take() else {
            anyhow::bail!("Presence Instance STDIN not found");
        };
        Ok(PresenceInstance {
            child,
            stdin: BufWriter::new(stdin),
        })
    }

    // Prevent zombie process
    fn cleanup(mut self) -> Result<()> {
        let status = self.child.try_wait()?;
        if status.is_none() {
            self.child.kill()?;
        }
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.stdin.write_all(buf)?;
        self.stdin.write_all(b"\n")?;
        self.stdin.flush()?;
        Ok(())
    }
}

pub struct ActivityManager {
    client_id: ClientId,
    instance: Option<PresenceInstance>,
}

impl ActivityManager {
    fn init(&mut self) -> Result<()> {
        let child = Command::new(PRESENCE_BIN)
            .stdin(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let mut ins = PresenceInstance::new(child)?;

        ins.write(self.client_id.as_bytes())?;
        self.instance = Some(ins);

        Ok(())
    }

    pub fn new() -> Result<Self> {
        let mut ins = ActivityManager {
            client_id: ClientId::new(),
            instance: None,
        };
        ins.init()?;
        Ok(ins)
    }

    pub fn set(&mut self, activity: Activity) -> Result<()> {
        loop {
            if self.instance.is_none() {
                self.init()?;
            }

            let write_error = {
                let mut_ins = self.instance.as_mut().unwrap();
                mut_ins.write(&serde_json::to_vec(&activity)?)
            };

            if let Err(e) = write_error {
                self.instance.take().unwrap().cleanup()?;
                match e.kind() {
                    ErrorKind::BrokenPipe => continue,
                    _ => {
                        return Err(e.into());
                    }
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        if let Some(ins) = self.instance.take() {
            ins.cleanup()?;
        }
        Ok(())
    }
}

impl Drop for ActivityManager {
    fn drop(&mut self) {
        if let Some(ins) = self.instance.take() {
            ins.cleanup().unwrap();
        }
    }
}
