
type Index = u64;

#[derive(Clone, Debug, PartialEq)]
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
/// The lamport clock can describe a happens before relationship but not the other way round.
/// Thus is it consistant with "potential" causality
/// 1: Each process has a counter init = 0
/// 2: Each event increments the counter
/// 3: On send we must include the counter
/// 4: On recieve we set the counter to max(local, recieved)
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
            self.time = time;
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
        println!("time is {}", &self.time);
        self.set_clock(self.time + 1u64, event.clone());
        println!("time is {}", &self.time);
        return (event, self.time)
    }
}


#[test]
fn test_lamport_recieve_send() {

    // Both indexes are zero;
    let mut l1 = Lamport::default();
    let mut l2 = Lamport::default();
    
    assert!(l1.time == 0);
    assert!(l2.time == 0);

    // Send thus increment l2 time.
    let l2_send = l2.send_event(Event::DoThing);

    assert!(l2.time == 1);
    assert!(l1.time == 0);

    // Recieved thus increment l1 time.
    l1.recieve_event(l2_send.0, l2_send.1);

    // l1 takes the l2s time and increments thus 2.
    assert!(l1.time == 2);
    assert!(l2.time == 1);
}

#[test]
fn test_lamport_event_history() {
    let mut l1 = Lamport::default();

    l1.recieve_event(Event::DoThing, 1u64);
    l1.recieve_event(Event::DropTable, 2u64);
    l1.recieve_event(Event::DoSomethingElse, 3u64);

    assert_eq!(l1.last_events.len(), 3usize);
    assert_eq!(l1.last_events[0], Event::DoThing);
    assert_eq!(l1.last_events[1], Event::DropTable);
    assert_eq!(l1.last_events[2], Event::DoSomethingElse);
}
