//! Foreground/background tracking shared with the Android event listener.
//!
//! The Slint Android backend reports lifecycle events on its own thread. The
//! UI thread polls [`Lifecycle::epoch`] from the same timer that drives the
//! 30-second page rule, so a background pause (or a resume) resets the timer
//! without any locking around the reader state.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Debug)]
struct Shared {
    foreground: AtomicBool,
    /// Incremented on every foreground/background transition.
    epoch: AtomicU64,
}

/// Cheap, cloneable handle to the app's foreground state.
#[derive(Debug, Clone)]
pub struct Lifecycle {
    shared: Arc<Shared>,
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl Lifecycle {
    /// Start in the foreground, which is correct for the desktop preview and
    /// for the first frame on Android (the activity is resumed right after).
    pub fn new() -> Self {
        Lifecycle {
            shared: Arc::new(Shared {
                foreground: AtomicBool::new(true),
                epoch: AtomicU64::new(0),
            }),
        }
    }

    /// Update the foreground flag. Every change bumps the epoch so the reader
    /// can reset its page timer even if it missed the transition itself.
    pub fn set_foreground(&self, foreground: bool) {
        if self.shared.foreground.swap(foreground, Ordering::SeqCst) != foreground {
            self.shared.epoch.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn is_foreground(&self) -> bool {
        self.shared.foreground.load(Ordering::SeqCst)
    }

    pub fn epoch(&self) -> u64 {
        self.shared.epoch.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_in_the_foreground() {
        let lifecycle = Lifecycle::new();
        assert!(lifecycle.is_foreground());
        assert_eq!(lifecycle.epoch(), 0);
    }

    #[test]
    fn transitions_bump_the_epoch_once() {
        let lifecycle = Lifecycle::new();
        lifecycle.set_foreground(false);
        assert!(!lifecycle.is_foreground());
        assert_eq!(lifecycle.epoch(), 1);

        // Repeating the same state is not a transition.
        lifecycle.set_foreground(false);
        assert_eq!(lifecycle.epoch(), 1);

        lifecycle.set_foreground(true);
        assert!(lifecycle.is_foreground());
        assert_eq!(lifecycle.epoch(), 2);
    }

    #[test]
    fn clones_share_state() {
        let lifecycle = Lifecycle::new();
        let clone = lifecycle.clone();
        clone.set_foreground(false);
        assert!(!lifecycle.is_foreground());
        assert_eq!(lifecycle.epoch(), 1);
    }
}
