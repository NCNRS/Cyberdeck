use async_nats::Message;
use fixer::FixerMsg;

/// Process incoming messages from the fixer. Most of these are
/// updates about tools/tasks we have running and we store these results 
/// in the db.
pub async fn process_msg(msg: Message) {
    // Deserialize msg
    let body: FixerMsg = rmp_serde::from_slice(&msg.payload).unwrap();

    match body.kind {
        fixer::MsgType::QuickHack => (),
    }
}

