use std::any::TypeId;
use std::collections::HashMap;

trait Event {
    type Wrapper;
}

struct Transition<S, E> {
    target: S,
    actions: Vec<fn(&E)>,
}

struct Machine<S, E> {
    state: S,
    on: HashMap<TypeId, Transition<S, E>>,
}
impl<S, E> Machine<S, E> {
    fn send<Ev>(&self, event: Ev) -> &S
    where
        Ev: Event<Wrapper = E> + 'static + Into<E>,
    {
        let transition = self.on.get(&TypeId::of::<Ev>()).unwrap();

        let event_enum: E = event.into();

        for action in &transition.actions {
            action(&event_enum);
        }

        &transition.target
    }
}

///////////////////////////////////////////////////////
///////////////////////////////////////////////////////
////////////////// ACTUAL MACHINE HERE ////////////////
///////////////////////////////////////////////////////
///////////////////////////////////////////////////////

enum EventWrapper {
    DataEvent(DataEvent),
    DatalessEvent(DatalessEvent),
}

struct DataEvent {
    number: u8,
    string: String,
}
impl Event for DataEvent {
    type Wrapper = EventWrapper;
}
impl From<DataEvent> for EventWrapper {
    fn from(event: DataEvent) -> Self {
        EventWrapper::DataEvent(event)
    }
}

struct DatalessEvent;
impl Event for DatalessEvent {
    type Wrapper = EventWrapper;
}
impl From<DatalessEvent> for EventWrapper {
    fn from(event: DatalessEvent) -> Self {
        EventWrapper::DatalessEvent(event)
    }
}

#[derive(Debug)]
enum State {
    StateA,
    StateB,
}

fn process_data_event(event: &EventWrapper) {
    if let EventWrapper::DataEvent(event) = event {
        println!(
            "We received an event! data: {}, {}",
            event.number, event.string
        );
    } else {
        println!("*** BEEP ***");
    }
}

fn process_dataless_event(event: &EventWrapper) {
    if let EventWrapper::DatalessEvent(_) = event {
        println!("No data in this event, but we got it!");
    } else {
        println!("--- BOOP ---")
    }
}

pub fn main() {
    let mut transitions = HashMap::new();
    transitions.insert(
        TypeId::of::<DataEvent>(),
        Transition {
            target: State::StateA,
            actions: vec![process_data_event, process_dataless_event],
        },
    );
    transitions.insert(
        TypeId::of::<DatalessEvent>(),
        Transition {
            target: State::StateB,
            actions: vec![process_data_event, process_dataless_event],
        },
    );

    let machine = Machine {
        state: State::StateA,
        on: transitions,
    };

    println!("\n==========\n");
    let state = machine.send(DataEvent {
        number: 1,
        string: "Testing!".to_string(),
    });
    println!("\n{:?}\n\n==========\n", state);
    let state = machine.send(DatalessEvent {});
    println!("\n{:?}\n\n==========\n", state);
    let state = machine.send(DataEvent {
        number: 200,
        string: "A secret message".to_string(),
    });
    println!("\n{:?}\n\n==========\n", state);
}
