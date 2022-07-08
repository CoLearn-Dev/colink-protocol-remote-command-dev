use colink_sdk_a::{decode_jwt_without_validation, CoLink, Participant, SubscriptionMessage};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let addr = &args[0];
    let jwt_a = &args[1];
    let jwt_b = &args[2];
    let msg = if args.len() > 3 { &args[3] } else { "hello" };
    let user_id_a = decode_jwt_without_validation(jwt_a).unwrap().user_id;
    let user_id_b = decode_jwt_without_validation(jwt_b).unwrap().user_id;

    let participants = vec![
        Participant {
            user_id: user_id_a.to_string(),
            ptype: "initiator".to_string(),
        },
        Participant {
            user_id: user_id_b.to_string(),
            ptype: "receiver".to_string(),
        },
    ];
    let cl = CoLink::new(addr, jwt_a);
    let task_id = cl
        .run_task("remote_command", msg.as_bytes(), &participants, true)
        .await?;

    let clt = CoLink::new(addr, jwt_b);
    // TODO use read_or_wait instead
    let key = format!("tasks:{}:output", task_id);
    let queue_name = clt.subscribe(&key, None).await?;
    let mut subscriber = clt.new_subscriber(&queue_name).await?;
    let data = subscriber.get_next().await?;
    let message: SubscriptionMessage = prost::Message::decode(&*data).unwrap();
    if message.change_type != "delete" {
        println!("{}", String::from_utf8_lossy(&*message.payload));
    }
    Ok(())
}
