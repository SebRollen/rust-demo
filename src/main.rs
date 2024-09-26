#![allow(dead_code)]

#[derive(Debug)]
struct Subscription {
    plan: String,
    status: String,
}

impl Subscription {
    fn new(plan: impl Into<String>) -> Self {
        Self {
            plan: plan.into(),
            status: "active".to_string(),
        }
    }

    // New method: lets users easily check if the subscription is active.
    // The `&self` here means it's a method, so it can be called directly on an instance of this
    // struct.
    //
    // We could also have written a pure function:
    // `fn is_active(sub: &Subscription) -> bool`
    // and called it using `is_active(sub)` rather than `sub.is_active()`
    fn is_active(&self) -> bool {
        self.status == "actvie" // oops!
    }
}

fn main() {
    let sub = Subscription::new("my-plan");
    // assert! will panic (error) if the argument is not true
    assert!(sub.is_active());
}
