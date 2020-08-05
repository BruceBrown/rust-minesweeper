use std::any::TypeId;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Default)]
pub struct TheChannelWiring<M> {
    senders: HashMap<TypeId, Vec<Sender<M>>>,
    receivers: HashMap<TypeId, Vec<Receiver<M>>>,
}

impl<M> TheChannelWiring<M> {
    /// Wire a channel from a Sender to a Reciever.
    pub fn wire<S, R>(&mut self)
    where
        S: 'static,
        R: 'static,
    {
        let (tx, rx) = channel();
        Self::wire_channel::<S, _>(tx, &mut self.senders);
        Self::wire_channel::<R, _>(rx, &mut self.receivers);
    }

    /// wire only the sender
    #[allow(dead_code)]
    pub fn wire_tx<T>(&mut self, tx: Sender<M>)
    where
        T: 'static,
    {
        Self::wire_channel::<T, _>(tx, &mut self.senders);
    }

    /// wire only the receiver
    #[allow(dead_code)]
    pub fn wire_rx<T>(&mut self, rx: Receiver<M>)
    where
        T: 'static,
    {
        Self::wire_channel::<T, _>(rx, &mut self.receivers);
    }

    /// get the `Sender`s and `Receiver`s for a type
    pub fn channels<T>(&mut self) -> (Option<Vec<Sender<M>>>, Option<Vec<Receiver<M>>>)
    where
        T: 'static,
    {
        (self.extract_senders::<T>(), self.extract_receivers::<T>())
    }

    fn wire_channel<S, T>(channel: T, map: &mut HashMap<TypeId, Vec<T>>)
    where
        S: 'static,
    {
        match map.get_mut(&TypeId::of::<S>()) {
            Some(vec) => vec.push(channel),
            None => {
                map.insert(TypeId::of::<S>(), vec![channel]);
            }
        }
    }

    fn extract_senders<S>(&mut self) -> Option<Vec<Sender<M>>>
    where
        S: 'static,
    {
        Self::extract::<S, _>(&mut self.senders)
    }

    fn extract_receivers<S>(&mut self) -> Option<Vec<Receiver<M>>>
    where
        S: 'static,
    {
        Self::extract::<S, _>(&mut self.receivers)
    }

    fn extract<R, T>(map: &mut HashMap<TypeId, Vec<T>>) -> Option<Vec<T>>
    where
        R: 'static,
    {
        map.remove(&TypeId::of::<R>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait MessageExchange {
        fn pull(&mut self);
        fn push(&mut self);
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum TestState {
        Initialized,
        Playing,
        Win,
        Lose,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum TestMessage {
        Dummy,
        BoolChanged(bool),
        StateChanged(TestState),
    }

    impl Default for TestMessage {
        fn default() -> TestMessage {
            TestMessage::Dummy
        }
    }

    type Message = TestMessage;

    #[derive(Debug, Default)]
    struct Exchange {
        messages: Vec<Message>,
        senders: Vec<Sender<Message>>,
        receivers: Vec<Receiver<Message>>,
    }

    impl Exchange {
        fn new(
            senders: Option<Vec<Sender<Message>>>,
            receivers: Option<Vec<Receiver<Message>>>,
        ) -> Self {
            Self {
                messages: Vec::new(),
                senders: senders.unwrap(),
                receivers: receivers.unwrap(),
            }
        }
    }

    impl MessageExchange for Exchange {
        fn pull(&mut self) {
            for rx in self.receivers.iter() {
                match rx.try_recv() {
                    Ok(message) => self.messages.push(message),
                    Err(_e) => {}
                }
            }
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

    #[derive(Debug, Default)]
    struct Obj1 {
        exchange: Exchange,
    }
    impl Obj1 {
        fn new(wiring: &mut TheChannelWiring<Message>) -> Self {
            let (senders, receivers) = wiring.channels::<Self>();
            let exchange = Exchange {
                messages: Vec::new(),
                senders: senders.unwrap(),
                receivers: receivers.unwrap(),
            };
            Self { exchange }
        }
    }

    impl MessageExchange for Obj1 {
        fn pull(&mut self) {
            self.exchange.pull();
        }
        fn push(&mut self) {
            self.exchange.push();
        }
    }

    #[derive(Debug, Default)]
    struct Obj2 {
        exchange: Exchange,
    }
    impl Obj2 {
        fn new(wiring: &mut TheChannelWiring<Message>) -> Self {
            let (senders, receivers) = wiring.channels::<Obj2>();
            let exchange = Exchange {
                messages: Vec::new(),
                senders: senders.unwrap(),
                receivers: receivers.unwrap(),
            };
            Self { exchange }
        }
    }

    impl MessageExchange for Obj2 {
        fn pull(&mut self) {
            self.exchange.pull();
        }
        fn push(&mut self) {
            self.exchange.push();
        }
    }

    #[derive(Debug, Default)]
    struct Obj3 {
        exchange: Exchange,
    }
    impl Obj3 {
        fn new(wiring: &mut TheChannelWiring<Message>) -> Self {
            let (senders, receivers) = wiring.channels::<Obj3>();
            let exchange = Exchange {
                messages: Vec::new(),
                senders: senders.unwrap(),
                receivers: receivers.unwrap(),
            };
            Self { exchange }
        }
    }

    impl MessageExchange for Obj3 {
        fn pull(&mut self) {
            self.exchange.pull();
        }
        fn push(&mut self) {
            self.exchange.push();
        }
    }

    #[test]
    fn test_simple_wiring() {
        let mut wire_channel = TheChannelWiring::<Message>::default();
        wire_channel.wire::<Obj1, Obj2>();
        assert_eq!(wire_channel.senders.len(), 1);
        assert_eq!(wire_channel.receivers.len(), 1);

        let (senders, receivers) = wire_channel.channels::<Obj1>();
        assert!(senders.is_some());
        assert!(receivers.is_none());
        assert_eq!(senders.unwrap().len(), 1);

        let (senders, receivers) = wire_channel.channels::<Obj2>();
        assert!(senders.is_none());
        assert!(receivers.is_some());
        assert_eq!(receivers.unwrap().len(), 1);
    }

    #[test]
    fn test_complex_wiring() {
        let mut wire_channel = TheChannelWiring::<Message>::default();
        wire_channel.wire::<Obj1, Obj2>();
        wire_channel.wire::<Obj2, Obj3>();

        let (sender, rx) = channel::<Message>();
        wire_channel.wire_rx::<Obj1>(rx);

        let (tx, receiver) = channel::<Message>();
        wire_channel.wire_tx::<Obj3>(tx);

        use std::boxed::Box;
        let mut obj1 = Box::new(Obj1::new(&mut wire_channel));
        let mut obj2 = Box::new(Obj2::new(&mut wire_channel));
        let mut obj3 = Box::new(Obj3::new(&mut wire_channel));
        let mut objs: Vec<Box<dyn MessageExchange>> = Vec::new();
        objs.push(obj1 as Box<dyn MessageExchange>);
        objs.push(obj2 as Box<dyn MessageExchange>);
        objs.push(obj3 as Box<dyn MessageExchange>);

        // send the message
        sender.send(Message::StateChanged(TestState::Initialized));
        assert!(receiver.try_recv().is_err());

        // move the message through the chain
        for obj in objs.iter_mut() {
            obj.pull();
            obj.push();
        }

        // receive it
        let m = receiver.try_recv();
        assert!(m.is_ok());
        assert_eq!(m.unwrap(), Message::StateChanged(TestState::Initialized));
    }
}
