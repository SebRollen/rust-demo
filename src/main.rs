#![allow(dead_code)]

#[derive(Debug)]
struct Subscription {
    plan: String,
    status: Status,
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Active,
    // new!
    Cancelled,
    // new!
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
        // This is fine, but not great to read, especially if we add more variants…
        //
        // self.status == Status::Cancelled || self.status == Status::Paused
        //
        // Lets instead use `match`
        match self.status {
            Status::Cancelled | Status::Paused => true,
            Status::Active => false,
        }
        // What happens if we add another status type?
    }
}

fn main() {
    let sub = Subscription::new("my-plan");
    assert!(sub.is_active());
}
