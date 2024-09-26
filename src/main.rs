#![allow(dead_code)]

// Our data. We want to model a subscription that's subscribed to a particular plan, and has some
// status.
#[derive(Debug)]
struct Subscription {
    plan: String,
    status: String,
}

// In rust, the data definition and its methods are separated. The `impl` block lets us add methods
// to the struct.
impl Subscription {
    // Convenience function for creating a new Subscription. We could also build one directly:
    // let sub = Subscription { plan: "my-plan".to_string(), status: "active".to_string }
    //
    // One benefit of the `new` method is that we can a default value for `status`
    fn new(plan: impl Into<String>) -> Self {
        Self {
            plan: plan.into(),
            status: "active".to_string(),
        }
    }
}

fn main() {
    let sub = Subscription::new("my-plan");
    // dbg! is a macro that lets you inspect various objects
    dbg!(sub);
}
