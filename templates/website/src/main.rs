use passionfruitdev::website::Website;
use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <Website> <Body/> </Website> })
}

#[component]
fn Body() -> impl IntoView {
    view! {
        "Hello from passion fruit!"
    }
}   