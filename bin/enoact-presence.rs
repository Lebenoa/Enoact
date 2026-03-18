use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::Activity};

fn main() {
    let mut buffer = String::with_capacity(1024);
    let stdin = std::io::stdin();

    stdin.read_line(&mut buffer).unwrap();
    let mut client = DiscordIpcClient::new(buffer.trim());
    client.connect().unwrap();
    buffer.clear();

    while stdin.read_line(&mut buffer).is_ok() {
        let act: Activity = serde_json::from_str(buffer.trim()).unwrap();
        client.set_activity(act).unwrap();
        buffer.clear();
    }
}
