#![allow(dead_code)]
use chrono::{Datelike, NaiveDateTime};

#[derive(Debug)]
struct Subscription {
    plan: String,
    status: Status,
    // New field! when did a user cancel?
    // `Option` is a built-in enum to reflect that a type may not exist.
    // It's definition is basically:
    // enum Option<T> {
    //   Some(T),
    //   None
    // }
    cancelled_at: Option<NaiveDateTime>,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Active,
    Cancelled,
    Paused,
}

impl Subscription {
    fn new(plan: impl Into<String>) -> Self {
        Self {
            plan: plan.into(),
            status: Status::Active,
            cancelled_at: None,
        }
    }

    fn is_active(&self) -> bool {
        self.status == Status::Active
    }

    fn is_inactive(&self) -> bool {
        match self.status {
            Status::Cancelled | Status::Paused => true,
            Status::Active => false,
        }
    }

    fn cancelled_at_month(&self) -> Option<u32> {
        // We can not call self.cancelled_at.month() like we do in ruby, because cancelled_at might
        // be None. Rust forces us to check for nullity, which makes it impossible to get errors
        // like `undefined method `id' for nil:NilClass`
        //
        // The way we've written it below is the "dumbest" way to write it. Can also do something
        // like
        // self.cancelled_at.map(|d| d.month())
        // or
        // let d = self.cancelled_at?;
        // Some(d.month())
        match self.cancelled_at {
            Some(d) => Some(d.month()),
            None => None,
        }
    }
}

fn main() {
    let mut sub = Subscription::new("my-plan");
    dbg!(&sub.status);
    dbg!(&sub.cancelled_at_month());
    sub.status = Status::Cancelled;
    // Whoops, this is an inconsistent state… We want to make it impossible to have a `Cancelled`
    // status with a `None` cancelled_at.
    dbg!(&sub.status);
    dbg!(&sub.cancelled_at_month());
}
