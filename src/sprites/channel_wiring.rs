use std::any::TypeId;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Default)]
pub struct TheChannelWiring<M> {
    /// The initial sender, created by channel()
    channel_senders: HashMap<TypeId, Sender<M>>,
    /// The initial receiver, created by channel()
    channel_receivers: HashMap<TypeId, Receiver<M>>,
    /// The senders that a type will be wired to
    senders: HashMap<TypeId, Vec<Sender<M>>>,
}

impl<M> TheChannelWiring<M> {
    /// Wire a `Sender` type to a `Receiver` type. This may create a channel for the `Receiver`.
    pub fn wire<S, R>(&mut self)
    where
        S: 'static,
        R: 'static,
    {
        // get the sender for type R, this can create a (tx,rx) pair
        // add it to the senders for type S
        let tx = self.get_or_create_sender::<R>();
        Self::wire_channel::<S, _>(tx, &mut self.senders);
    }

    /// Get the `Sender`s and `Receiver` for a type
    pub fn channels<T>(&mut self) -> (Option<Vec<Sender<M>>>, Option<Receiver<M>>)
    where
        T: 'static,
    {
        (self.extract_senders::<T>(), self.extract_receiver::<T>())
    }

    /// Internal function for wiring a
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

    /// Get the sender for a type. This may create a channel for the type.
    fn get_or_create_sender<R>(&mut self) -> Sender<M>
    where
        R: 'static,
    {
        match self.channel_senders.get(&TypeId::of::<R>()) {
            Some(tx) => tx.clone(),
            None => self.create_channel::<R>(),
        }
    }

    /// Create a channel and return a clone of the sender
    fn create_channel<R>(&mut self) -> Sender<M>
    where
        R: 'static,
    {
        let (tx, rx) = channel();
        self.channel_senders.insert(TypeId::of::<R>(), tx.clone());
        self.channel_receivers.insert(TypeId::of::<R>(), rx);
        tx
    }

    fn extract_senders<S>(&mut self) -> Option<Vec<Sender<M>>>
    where
        S: 'static,
    {
        Self::extract::<S, _>(&mut self.senders)
    }

    fn extract_receiver<R>(&mut self) -> Option<Receiver<M>>
    where
        R: 'static,
    {
        self.channel_receivers.remove(&TypeId::of::<R>())
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
        receiver: Option<Receiver<Message>>,
    }

    impl Exchange {
        fn new(senders: Option<Vec<Sender<Message>>>, receiver: Option<Receiver<Message>>) -> Self {
            Self {
                messages: Vec::new(),
                senders: senders.unwrap(),
                receiver: receiver,
            }
        }
    }

    impl MessageExchange for Exchange {
        fn pull(&mut self) {
            if let Some(receiver) = &self.receiver {
                for message in receiver.try_iter() {
                    self.messages.push(message)
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
            let (senders, receiver) = wiring.channels::<Self>();
            let exchange = Exchange {
                messages: Vec::new(),
                senders: senders.unwrap(),
                receiver: receiver,
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
            let (senders, receiver) = wiring.channels::<Obj2>();
            let exchange = Exchange {
                messages: Vec::new(),
                senders: senders.unwrap(),
                receiver: receiver,
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
            let (senders, receiver) = wiring.channels::<Obj3>();
            let exchange = Exchange {
                messages: Vec::new(),
                senders: senders.unwrap(),
                receiver: receiver,
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

        let (senders, receiver) = wire_channel.channels::<Obj1>();
        assert!(senders.is_some());
        assert!(receiver.is_none());
        assert_eq!(senders.unwrap().len(), 1);

        let (senders, receiver) = wire_channel.channels::<Obj2>();
        assert!(senders.is_none());
        assert!(receiver.is_some());
    }

    #[test]
    fn test_complex_wiring() {
        struct TestWiring; // need for test's wiring

        let mut wire_channel = TheChannelWiring::<Message>::default();
        wire_channel.wire::<Obj1, Obj2>();
        wire_channel.wire::<Obj2, Obj3>();
        wire_channel.wire::<TestWiring, Obj1>();
        wire_channel.wire::<Obj3, TestWiring>();

        use std::boxed::Box;
        let obj1 = Box::new(Obj1::new(&mut wire_channel));
        let obj2 = Box::new(Obj2::new(&mut wire_channel));
        let obj3 = Box::new(Obj3::new(&mut wire_channel));
        let mut objs: Vec<Box<dyn MessageExchange>> = Vec::new();
        objs.push(obj1 as Box<dyn MessageExchange>);
        objs.push(obj2 as Box<dyn MessageExchange>);
        objs.push(obj3 as Box<dyn MessageExchange>);

        let (senders, receiver) = wire_channel.channels::<TestWiring>();
        let receiver = receiver.unwrap();
        let senders = senders.unwrap();
        let sender = senders.first().unwrap();

        // send the message
        sender
            .send(Message::StateChanged(TestState::Initialized))
            .unwrap();
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
