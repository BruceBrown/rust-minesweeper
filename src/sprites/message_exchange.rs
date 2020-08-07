use std::mem::swap;
use std::sync::mpsc::{Receiver, Sender};

use crate::sprites::GameState;
use crate::sprites::{RendererContext, MouseEventData};

pub trait MessageExchange {
    fn pull(&mut self) -> u32 {
        0
    }
    fn push(&mut self) {}
}

use std::rc::Rc;
/// Channel messages is the data that flows through chanels.
#[derive(Clone)]
pub enum ChannelMessage {
    TestMessage,                 //
    GameStateChanged(GameState), //< Game state change
    FlagStateChanged(bool),      //< Flag state change, either exhausted or not
    Flagged(bool),               //< Tile has been flagged (true) or unflagged(false)
    Revealed(bool, bool),        //< Tile has been revealed and is_mine, has_adjacent_mines
    Clear, //< Trying to clear an area of mines, neigbors use this to determine if they can reveal
    Render(Rc<Box<dyn RendererContext + 'static>>),
    MouseEvent(MouseEventData),

}
impl Default for ChannelMessage {
    fn default() -> Self {
        ChannelMessage::TestMessage
    }
}

use crate::sprites::channel_wiring;
pub type ChannelWiring = channel_wiring::TheChannelWiring<ChannelMessage>;

pub struct Exchange {
    messages: Vec<ChannelMessage>,
    senders: Vec<Sender<ChannelMessage>>,
    receiver: Option<Receiver<ChannelMessage>>,
}

impl Exchange {
    pub fn new_from_wiring<T>(wiring: &mut ChannelWiring) -> Self
    where
        T: 'static,
    {
        let (senders, receiver) = wiring.channels::<T>();
        if receiver.is_none() {
            println!("No receiver found")
        }
        Self {
            messages: Vec::new(),
            senders: senders.unwrap_or_default(),
            receiver: receiver,
        }
    }
    pub fn new(
        senders: Vec<Sender<ChannelMessage>>,
        receivers: Option<Receiver<ChannelMessage>>,
    ) -> Self {
        Self {
            messages: Vec::new(),
            senders: senders,
            receiver: receivers,
        }
    }
   
    pub fn push_message(&self, message: ChannelMessage) {
        for tx in self.senders.iter() {
            tx.send(message.clone()).unwrap();
        }
    }

    pub fn push_message_to_index(&self, message: ChannelMessage, index: usize) {
        self.senders[index].send(message).unwrap();
    }

    pub fn clone_senders(&self) -> Vec<Sender<ChannelMessage>> {
        self.senders.clone()
    }

    pub fn get_messages(&mut self) -> Vec<ChannelMessage> {
        let mut messages: Vec<ChannelMessage> = Vec::new();
        swap(&mut self.messages, &mut messages);
        messages
    }

    pub fn replace_senders(&mut self, senders: &mut Vec<Sender<ChannelMessage>>) {
        swap(&mut self.senders, senders);
    }
}

impl MessageExchange for Exchange {
    fn pull(&mut self) -> u32 {
        let mut count: u32 = 0;
        if let Some(receiver) = &self.receiver {
            if let Ok(message) = receiver.try_recv() {
                self.messages.push(message);
                count = 1;
            }
        }
        /*
            for message in receiver.try_iter() {
                self.messages.push(message);
                count += 1
            }
        }
        */
        count
    }

    fn push(&mut self) {
        for message in self.messages.iter() {
            for tx in self.senders.iter() {
                tx.send(message.clone());
            }
        }
        self.messages.clear();
    }
}
