use passionfruitdev::features::website::Website;
use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <Website> <Body/> </Website> })
}

#[component]
fn Body() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">Thank you for trying Passion Fruit</h2>
            <p class="px-10 pb-10 text-left">Change this area to personalize your website!</p>
        </div>
    }
}   