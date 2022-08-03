use colink_sdk::{CoLink, Participant, ProtocolEntry};
use std::process::Command;

struct Initiator;
#[colink_sdk::async_trait]
impl ProtocolEntry for Initiator {
    async fn start(
        &self,
        cl: CoLink,
        _param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        println!("initiator");
        cl.get_variable("output", &participants[1]).await?;
        let key = format!(
            "_remote_storage:private:{}:_variable_transfer:{}:output",
            participants[1].user_id,
            cl.get_task_id()?
        );
        cl.create_entry(
            &format!("tasks:{}:output", cl.get_task_id()?),
            key.as_bytes(),
        )
        .await?;
        Ok(())
    }
}

struct Receiver;
#[colink_sdk::async_trait]
impl ProtocolEntry for Receiver {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let cmd = String::from_utf8_lossy(&param);
        println!("execute: {}", cmd);
        let res = Command::new("bash")
            .arg("-c")
            .arg(&*cmd)
            .current_dir("./")
            .output()
            .unwrap();
        cl.create_entry(&format!("tasks:{}:output", cl.get_task_id()?), &res.stdout)
            .await?;
        cl.set_variable("output", &res.stdout, &[participants[0].clone()])
            .await?;
        Ok(())
    }
}

colink_sdk::protocol_start!(
    ("remote_command:initiator", Initiator),
    ("remote_command:receiver", Receiver)
);
