use colink_sdk_a::{CoLink, Participant};
use colink_sdk_p::ProtocolEntry;
use std::process::Command;

struct Initiator;
#[colink_sdk_p::async_trait]
impl ProtocolEntry for Initiator {
    async fn start(
        &self,
        _cl: CoLink,
        _param: Vec<u8>,
        _participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        println!("initiator");
        Ok(())
    }
}

struct Receiver;
#[colink_sdk_p::async_trait]
impl ProtocolEntry for Receiver {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        _participants: Vec<Participant>,
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
        Ok(())
    }
}

colink_sdk_p::protocol_start!(
    ("remote_command:initiator", Initiator),
    ("remote_command:receiver", Receiver)
);
