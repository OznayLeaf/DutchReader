//! The 30-second page-reading rule.
//!
//! Pure Rust with an injected millisecond clock so the boundary cases are
//! testable on the desktop. The UI owns the clock and calls [`PageTimer::poll`]
//! regularly; the timer fires exactly once per page visit.

/// A page counts as read after this many milliseconds.
pub const READ_MILLIS: u64 = 30_000;

/// Tracks how long the current page has been open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageTimer {
    started_at_ms: u64,
    fired: bool,
}

impl PageTimer {
    /// Start a fresh page visit at `now_ms`.
    pub fn new(now_ms: u64) -> Self {
        PageTimer {
            started_at_ms: now_ms,
            fired: false,
        }
    }

    /// Restart the timer, e.g. after a page turn, leaving the reader, or
    /// returning from the background.
    pub fn reset(&mut self, now_ms: u64) {
        self.started_at_ms = now_ms;
        self.fired = false;
    }

    pub fn elapsed_ms(&self, now_ms: u64) -> u64 {
        now_ms.saturating_sub(self.started_at_ms)
    }

    pub fn is_armed(&self) -> bool {
        !self.fired
    }

    /// Return `true` exactly once, when the page has been open for at least
    /// 30 seconds. A clock that jumps backwards never fires early.
    pub fn poll(&mut self, now_ms: u64) -> bool {
        if !self.fired && self.elapsed_ms(now_ms) >= READ_MILLIS {
            self.fired = true;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twenty_nine_seconds_does_not_fire() {
        let mut timer = PageTimer::new(1_000);
        assert!(!timer.poll(1_000 + 29_000));
        assert!(timer.is_armed());
        assert_eq!(timer.elapsed_ms(30_000), 29_000);
    }

    #[test]
    fn thirty_seconds_fires_exactly_once() {
        let mut timer = PageTimer::new(0);
        assert!(timer.poll(READ_MILLIS));
        assert!(!timer.poll(READ_MILLIS));
        assert!(!timer.poll(READ_MILLIS + 60_000));
        assert!(!timer.is_armed());
    }

    #[test]
    fn reset_restarts_the_countdown() {
        let mut timer = PageTimer::new(0);
        assert!(!timer.poll(29_000));
        timer.reset(29_000);
        assert!(!timer.poll(29_000 + 29_999));
        assert!(timer.poll(29_000 + 30_000));
    }

    #[test]
    fn clock_going_backwards_never_fires_early() {
        let mut timer = PageTimer::new(100_000);
        assert!(!timer.poll(0));
        assert!(!timer.poll(30_000));
        assert!(!timer.poll(99_999));
        assert!(timer.poll(130_000));
    }

    #[test]
    fn a_background_pause_can_be_modelled_as_reset() {
        let mut timer = PageTimer::new(0);
        assert!(!timer.poll(20_000));
        // App backgrounded for two minutes; on resume the UI resets the timer.
        timer.reset(140_000);
        assert!(!timer.poll(169_999));
        assert!(timer.poll(170_000));
    }
}
