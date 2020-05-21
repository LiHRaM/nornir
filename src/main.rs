use async_std::stream::StreamExt;
use swayipc::{
    reply::Output,
    reply::{Event, WorkspaceChange},
    Connection, EventStream, EventType, Fallible,
};

fn main() -> Fallible<()> {
    smol::run(async {
        let mut events: EventStream = workspace_events().await?;
        while let Some(Ok(Event::Workspace(event))) = events.next().await {
            let outputs = get_outputs().await?;
            let active = outputs
                .iter()
                .filter(move |output| output.active)
                .collect::<Vec<_>>();

            // Nothing to do with only one monitor
            if active.len() <= 1 {
                println!("Nope!");
            } else {
                match event.change {
                    WorkspaceChange::Init => {}
                    WorkspaceChange::Empty => {}
                    WorkspaceChange::Focus => {}
                    WorkspaceChange::Move => {}
                    WorkspaceChange::Rename => {}
                    WorkspaceChange::Urgent => {}
                    WorkspaceChange::Reload => {}
                }
            }
        }
        Ok(())
    })
}

async fn workspace_events() -> Fallible<EventStream> {
    Connection::new()
        .await?
        .subscribe(&[EventType::Workspace])
        .await
}

async fn get_outputs() -> Fallible<Vec<Output>> {
    let mut connection: Connection = Connection::new().await?;
    connection.get_outputs().await
}

type WorkspaceName = String;

enum UpdateMessage {
    Move(WorkspaceName),
}

async fn message(message: UpdateMessage) -> Fallible<()> {
    let mut connection: Connection = Connection::new().await?;
    match message {
        UpdateMessage::Move(name) => {
            connection
                .run_command(format!("move workspace to {}", name))
                .await?;
        }
    }

    Ok(())
}
