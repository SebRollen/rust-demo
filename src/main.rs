#![allow(dead_code)]
use chrono::{Datelike, NaiveDateTime};

#[derive(Debug)]
struct Subscription {
    plan: String,
    status: Status,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Active,
    Cancelled {
        // Surprise! enums can also contain state.
        // In this case, we don't use an Option<NaiveDateTime>, because we want to guarantee that a
        // subscription in the Cancelled state has a cancelled_at time set.
        cancelled_at: NaiveDateTime,
    },
    Paused,
}

impl Subscription {
    fn new(plan: impl Into<String>) -> Self {
        Self {
            plan: plan.into(),
            status: Status::Active,
        }
    }

    fn is_active(&self) -> bool {
        self.status == Status::Active
    }

    fn is_inactive(&self) -> bool {
        match self.status {
            Status::Cancelled { .. } | Status::Paused => true,
            Status::Active => false,
        }
    }

    fn cancelled_at_month(&self) -> Option<u32> {
        // We now need to match on status rather than cancelled_at, since the field lives inside
        // the status enum
        match self.status {
            Status::Cancelled { cancelled_at } => Some(cancelled_at.month()),
            // Any other status us uncancelled, so all we can do is return None
            _ => None,
        }
    }

    // This method takes &mut self, and returns nothing, meaning that we just modify our instance
    // in place.
    // We'll return the cancellation date for easy inspection
    fn cancel(&mut self) -> NaiveDateTime {
        let cancelled_at = chrono::Local::now().naive_local();
        self.status = Status::Cancelled { cancelled_at };
        cancelled_at
    }
}

fn main() {
    let mut sub = Subscription::new("my-plan");
    // This is no longer valid code. We've made it impossible to get into an inconsistent state!
    // sub.status = Status::Cancelled;
    dbg!(sub.cancel());
    dbg!(&sub.cancelled_at_month());
    // Question: what happens if we cancel twice?
    dbg!(sub.cancel());
    // Uh-oh…
    dbg!(&sub.cancelled_at_month());
}
