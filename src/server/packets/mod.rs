use chat::message::Message;
use lobby::leave::LobbyLeave;
use other::ping::Ping;

#[derive(Serialize, Deserialize, Debug)]
pub enum Packets {
    Ping(Ping),
    Message(Message),
    Leave(),
}
