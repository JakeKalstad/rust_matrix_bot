 
use std::env;

use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{events::room::message::{SyncRoomMessageEvent, RoomMessageEventContent, MessageType}, UserId}, room::Room,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let user_id_env = env::var("UID_MATRIX").expect("PW supplied");
    let user_id = <&UserId>::try_from(user_id_env.as_str()).expect("parse user id");
    let pw = env::var("PW_MATRIX").expect("PW supplied");
    let client = Client::builder().user_id(user_id).build().await?;
    client.login_username(user_id, &pw).send().await?;

    client.add_event_handler(|ev: SyncRoomMessageEvent, room: Room| async move {
        if let Room::Joined(room) = room {
            let MessageType::Text(ref text_content) = ev.as_original().expect("Get evt").content.msgtype else {
                return;
            };
            if text_content.body.contains("!hello") {
                let content = RoomMessageEventContent::text_plain("Well howdy doo-dee");
                room.send(content, None).await.expect("Send room");
            }
        }
    });
    client.sync(SyncSettings::default()).await?;
    Ok(())
}

