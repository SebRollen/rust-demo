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
    Cancelled { cancelled_at: NaiveDateTime },
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
        match self.status {
            Status::Cancelled { cancelled_at } => Some(cancelled_at.month()),
            _ => None,
        }
    }

    // Cancellation is _fallible_. If a subscription is already cancelled, we want to fail, not
    // cancel it again.
    //
    // We can encode fallible functions as types using the `Result` enum. This enum has two
    // variants, and is defined like this:
    // enum Result<T, E> {
    //   Ok(T),
    //   Err(E)
    // }
    //
    // If an operation succeeds, we return Ok. If it fails, we return Err
    fn cancel(&mut self) -> Result<NaiveDateTime, &'static str> {
        if let Status::Cancelled { .. } = self.status {
            Err("Already cancelled!")
        } else {
            let cancelled_at = chrono::Local::now().naive_local();
            self.status = Status::Cancelled { cancelled_at };
            Ok(cancelled_at)
        }
    }
}

fn main() {
    let mut sub = Subscription::new("my-plan");
    // Unwrap is a convenience method on Result. If the Result is of the Ok variant, we extract the
    // inner value. If it's of the Err variant, we panic
    dbg!(sub.cancel().unwrap());
    dbg!(&sub.cancelled_at_month());
    dbg!(sub.cancel().unwrap());
    dbg!(&sub.cancelled_at_month());
}
