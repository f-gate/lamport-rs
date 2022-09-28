fn main() {
    println!("Hello, world!");
}

type Index = u64;

#[derive(Clone)]
enum Event {
    DoThing,
    DoSomethingElse,
    DropTable,
}

trait LogicalClock {
    fn set_clock(&mut self, time: Index, event: Event) -> Index;

    // only to handle modifications of time on a system.
    // Returns 
    fn recieve_event(&mut self, event: Event, time: Index) -> Index;

    /// Return the Event u wanna send and the time associated.
    fn send_event(&mut self, event: Event) -> (Event, Index);

}

struct Lamport {
    time: Index,
    last_events: Vec<Event>, 
}

impl Default for Lamport {
    fn default() -> Self {
        Self { time: 0u64, last_events: vec![]}
    }
}

/// Lamport clock where a recieve event is counted.
/// Returns new current time.
impl LogicalClock for Lamport {
    fn set_clock(&mut self, time: Index, event: Event) -> Index {
        if time > self.time      {
            self.time = time + 1u64;
        }
        self.last_events.push(event);
        self.time
    }
    fn recieve_event(&mut self, event: Event, time: Index) -> Index {
        // since all events are counted and this function only handles time _ is used 

        // Handle Event if let Ok(n) then 
        match event {
            _ => return Self::set_clock(self, time + 1u64, event) 
        }
    }

    /// Dummy function for sending an event.
    fn send_event(&mut self, event: Event) -> (Event, Index) {
        //increment own clock by one and return restul
        self.set_clock(self.time + 1u64, event.clone());
        return (event, self.time)
    }
}


#[test]
fn test_lamport_recieve() {

    // Both indexes are zero;
    let mut l1 = Lamport::default();
    let mut l2 = Lamport::default();
    
    assert!(l1.time == 0);
    assert!(l2.time == 0);

    // Send thus increment time.
    let l2_send = l2.send_event(Event::DoThing);

    assert!(l2.time == 1);
    assert!(l1.time == 0);

    // Recieved thus increment time.
    l1.recieve_event(l2_send.0, l2_send.1);

    assert!(l2.time == 1);
    assert!(l1.time == 2);

}
