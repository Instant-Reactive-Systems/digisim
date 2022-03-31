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
    pub fn advance(&mut self) -> (u32, Drain<Event>) {
        let mut count = 0;
        while count != self.max_delay && self.wheel[self.current_time as usize].is_empty() {
            self.current_time += 1;
            self.current_time %= self.max_delay;
            count += 1;
        }

        (count, self.wheel[self.current_time as usize].drain(..))
    }

    /// Schedules an event at time `current_time + delay`.
    pub fn schedule(&mut self, delay: u32, event: Event) {
        let scheduled_time = (self.current_time + delay) % self.max_delay;
        self.wheel[scheduled_time as usize].push(event);
    }
}

