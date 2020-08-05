use std::mem::swap;
use std::sync::mpsc::{Receiver, Sender};

use crate::sprites::GameState;

pub trait MessageExchange {
    fn pull(&mut self) -> u32 {
        0
    }
    fn push(&mut self) {}
}

/// Channel messages is the data that flows through chanels.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChannelMessage {
    Dummy,                       //
    GameStateChanged(GameState), //< Game state change
    FlagStateChanged(bool),      //< Flag state change, either exhausted or not
    Flagged(bool),               //< Tile has been flagged (true) or unflagged(false)
    Revealed(bool, bool),        //< Tile has been revealed and is_mine, has_adjacent_mines
    Clear, //< Trying to clear an area of mines, neigbors use this to determine if they can reveal
}
impl Default for ChannelMessage {
    fn default() -> Self {
        ChannelMessage::Dummy
    }
}

use crate::sprites::channel_wiring;
pub type ChannelWiring = channel_wiring::TheChannelWiring<ChannelMessage>;

#[derive(Debug, Default)]
pub struct Exchange {
    messages: Vec<ChannelMessage>,
    senders: Vec<Sender<ChannelMessage>>,
    receivers: Vec<Receiver<ChannelMessage>>,
}

impl Exchange {
    pub fn new_from_wiring<T>(wiring: &mut ChannelWiring) -> Self
    where
        T: 'static,
    {
        let (senders, receivers) = wiring.channels::<T>();
        Self {
            messages: Vec::new(),
            senders: senders.unwrap_or_default(),
            receivers: receivers.unwrap_or_default(),
        }
    }
    pub fn new(
        senders: Vec<Sender<ChannelMessage>>,
        receivers: Vec<Receiver<ChannelMessage>>,
    ) -> Self {
        Self {
            messages: Vec::new(),
            senders: senders,
            receivers: receivers,
        }
    }
    /*
    fn new(senders: Option<Vec<Sender<ChannelMessage>>>, receivers: Option<Vec<Receiver<ChannelMessage>>>) -> Self {
        Self {
            messages: Vec::new(),
            senders: senders.unwrap(),
            receivers: receivers.unwrap(),
        }
    }
    */
    pub fn push(&self, message: ChannelMessage) {
        for tx in self.senders.iter() {
            tx.send(message);
        }
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
        for rx in self.receivers.iter() {
            if let Ok(message) = rx.try_recv() {
                count += 1;
                self.messages.push(message);
            }
        }
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
