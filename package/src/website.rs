use leptos::*;

#[component]
pub fn Website(children: Children) -> impl IntoView {
    view! {
        {children()}
    }
}   
