// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender, TrySendError};
use std::thread::spawn;

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sendto_channel: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<Command>> {
        let (tx, rx) = channel();
        let command = Command::Insert {
            draft,
            response_channel: tx,
        };
        self.sendto_channel.try_send(command)?;
        Ok(rx.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<Command>> {
        let (tx, rx) = channel();
        let command = Command::Get {
            id,
            response_channel: tx,
        };
        self.sendto_channel.try_send(command)?;
        Ok(rx.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (tx, rx) = sync_channel(capacity);
    spawn(move || server(rx));
    TicketStoreClient { sendto_channel: tx }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
