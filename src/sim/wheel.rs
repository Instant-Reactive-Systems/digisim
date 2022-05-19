use super::Event;
use std::vec::Drain;

/// An event storing structure.
#[derive(Debug)]
pub struct TimingWheel {
    max_delay: u32,
    current_time: u32,
    wheel: Vec<Vec<Event>>,
}

impl TimingWheel {
    /// Creates a new timing wheel with max_delay wheel size.
    pub fn new(max_delay: u32) -> Self {
        Self {
            max_delay,
            current_time: 0,
            wheel: vec![Default::default(); max_delay as usize],
        }
    }

    /// Advances through the wheel until a time point in which events are scheduled is hit.
    ///
    /// # Returns
    /// The time elapsed since the advance and an iterator over the time point's events.
    pub fn advance(&mut self) -> Drain<Event> {
        let drained = self.wheel[self.current_time as usize].drain(..);
        self.current_time += 1;
        self.current_time %= self.max_delay;
        
        drained
    }

    /// Schedules an event at time `current_time + delay`.
    pub fn schedule(&mut self, delay: u32, event: Event) {
        let scheduled_time = (self.current_time + delay) % self.max_delay;
        self.wheel[scheduled_time as usize].push(event);
    }

    /// Sets the max delay of the timing wheel.
    pub fn set_max_delay(&mut self, max_delay: u32) {
        self.max_delay = max_delay;
        self.wheel.resize(max_delay as usize, Default::default());
    }
}

impl Default for TimingWheel {
    fn default() -> Self {
        Self {
            max_delay: 1024,
            current_time: 0,
            wheel: vec![Default::default(); 1024],
        }
    }
}

