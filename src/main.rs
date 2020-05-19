use async_std::stream::StreamExt;
use swayipc::{Connection, EventStream, EventType, Fallible};

fn main() -> Fallible<()> {
    smol::run(async {
        let subs = [EventType::Workspace];

        let mut events: EventStream = Connection::new().await?.subscribe(&subs).await?;
        while let Some(_event) = events.next().await {
            let mut connection: Connection = Connection::new().await?;
            let outputs = connection.get_outputs().await?;
            let active = outputs
                .iter()
                .filter(move |output| output.active)
                .collect::<Vec<_>>();

            // Nothing to do with only one monitor
            if active.len() <= 1 {
                continue;
            } else {
                // Put workspaces on screens based on pairing configurations
            }
        }
        Ok(())
    })
}
