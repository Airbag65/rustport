use tokio::{runtime::Handle, task::block_in_place};

use crate::{cmd::Command, net::NetworkManager};

pub struct RemoveCommand {
    #[allow(unused)]
    pub value: String,
}

impl Command for RemoveCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let nm: NetworkManager = NetworkManager::new();

        block_in_place(move || {
            Handle::current().block_on(async move {
                let _ = nm.remove(self.value.clone()).await;
            })
        });
        Ok(())
    }
}
