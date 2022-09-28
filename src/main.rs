fn main() {
    println!("Hello, world!");
}

type Index = u64;

enum Event {
    Send,
    Recieve,
    Inherent,
}

enum Error {
    SendError,
    RecieveError,
    InherentError,
}

trait LogicalClock {
    fn set_clock(&mut self, time: Index, event: Event) -> Index;

    // only to handle modifications of time on a system.
    // Returns 
    fn recieve_event(&mut self, event: Event, time: Index) -> Index;

    fn send_event(other_clock: Self);

}

struct Lamport {
    time: Index,
    last_events: Vec<Event>, 
}

/// Lamport clock where a recieve event is counted.
/// Returns new current time.
impl LogicalClock for Lamport {
    fn set_clock(&mut self, time: Index, event: Event) -> Index {
        if time > self.time {
            self.time = time + 1.into();
        }
        self.last_events.push(event);
        self.time
    }
    fn recieve_event(&mut self, event: Event, time: Index) -> Index {
        // since all events are counted and this function only handles time _ is used 
        match event {
            _ => return Self::set_clock(self, time, event) 
        }
    }
    fn send_event<T: LogicalClock>(&self, other_clock: T, event: Event) {
        other_clock::recieve_event(event, self.time);
    }

}



