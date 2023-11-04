use leptos::*;

pub struct Website;

impl Website {
    pub fn new() -> impl IntoView {
        leptos::mount_to_body(|| view! { <App/> })
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
            {count}
        </button>
        <progress
    max="50"
    // signals are functions, so this <=> `move || count.get()`
    value=double_count
/>
    }
}   
