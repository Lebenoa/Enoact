use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::sync::LazyLock;

use anyhow::Result;
use dashmap::DashMap;
use discord_rich_presence::activity::Activity;

use crate::activity_manager::PRESENCE_BIN;
use crate::client_id::ClientId;

pub struct HttpActivity<'a> {
    activity: Activity<'a>,
    child: Child,
}

pub static HTTP_PRESENCE: LazyLock<DashMap<ClientId, HttpActivity<'_>>> =
    LazyLock::new(DashMap::new);

pub fn new_session(activity: Activity<'static>) -> Result<String> {
    let mut child = Command::new(PRESENCE_BIN)
        .stdin(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let stdin = child.stdin.as_mut().expect("Missing Child's STDIN");

    let Some(client_id) = ClientId::new_from_free() else {
        return Err(anyhow::anyhow!("No free client IDs available"));
    };

    let to_be_return = client_id.0.to_string();
    stdin.write_all(client_id.as_bytes())?;
    stdin.write_all(b"\n")?;
    stdin.write_all(&serde_json::to_vec(&activity)?)?;
    stdin.write_all(b"\n")?;

    HTTP_PRESENCE.insert(client_id, HttpActivity { activity, child });

    Ok(to_be_return)
}

pub fn set_presence(client_id: &str, activity: Activity<'static>) -> Result<()> {
    if let Some(mut http_activity) = HTTP_PRESENCE.get_mut(client_id) {
        let stdin = http_activity
            .child
            .stdin
            .as_mut()
            .expect("Missing Child's STDIN");

        stdin.write_all(&serde_json::to_vec(&activity)?)?;
        stdin.write_all(b"\n")?;

        http_activity.activity = activity;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Client ID not found"))
    }
}
