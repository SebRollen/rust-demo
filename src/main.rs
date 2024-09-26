#![allow(dead_code)]

#[derive(Debug)]
struct Subscription {
    plan: String,
    status: Status,
}

// An enum is also a data type, but instead of taking multiple different types and grouping them
// into one object like a struct, and enum instead creates multiple different data that are all
// part of the same type. An instance of the enum can only hold one enum type at a time.
// In technical terms, it's a "sum" type, as opposed to a struct which is a "product" type.
#[derive(Debug, PartialEq, Eq)]
enum Status {
    Active,
}

impl Subscription {
    fn new(plan: impl Into<String>) -> Self {
        Self {
            plan: plan.into(),
            status: Status::Active,
        }
    }

    fn is_active(&self) -> bool {
        // This time we match against a specific enum variant. What happens if we create a typo
        // here?
        self.status == Status::Active
    }
}

fn main() {
    let sub = Subscription::new("my-plan");
    assert!(sub.is_active());
}
