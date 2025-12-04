use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

//In the previous exercise we developed the server side response channel for sending the data back
//to the receiver . In the test we could see that the receiver making a channel and then
//construting various values for making the parameters for the server side functions.

//In this exercise we are to implement the client side of the ticket store.
//Here instead of making the channel and then writing sequential code for everytime we want to send
//something to the server for inserting, and getting the tickets back from the server.
//we abstract that code.
#[derive(Clone)]
// TODO: flesh out the client implementation.

//Here we store the Sender side of the channel to send commands to the server.
pub struct TicketStoreClient {
    sendto_channel: Sender<Command>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    // Here we make a 2 way mpsc channel .
    // 1. We send the command using the server's sender channel.
    // 2. In The command we include the response channel's sender side.
    // 3. We then recv the response from the response channel's receiver side.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (tx, rx) = std::sync::mpsc::channel();
        let command = Command::Insert {
            draft,
            response_channel: tx,
        };
        self.sendto_channel.send(command).unwrap();
        rx.recv().unwrap()
    }

    // Here we make a 2 way mpsc channel .
    // 1. We send the command using the server's sender channel.
    // 2. In The command we include the response channel's sender side.
    // 3. We then recv the response from the response channel's receiver side.
    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (tx, rx) = std::sync::mpsc::channel();
        let command = Command::Get {
            id,
            response_channel: tx,
        };
        self.sendto_channel.send(command).unwrap();
        rx.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        sendto_channel: sender,
    }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

//This is the server function which runs in a separate thread.
//it receives commands from the client and processes them.
//and sends back the response using the response channel included in the command.
fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
