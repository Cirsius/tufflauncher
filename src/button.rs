use leptos::prelude::*;

#[component]
pub fn Button(label: &'static str, href: &'static str) -> impl IntoView {
    view! {
            <button
                class="btn"
                on:click=move |_| {
                    let _ = window().location().set_href(href);
                }
            >
        {label}
            </button>
    }
}
#[component]
pub fn TuffButton() -> impl IntoView {
    view! {
        <div class="hero">
            <h1>"tufflauncher"</h1>
            <Button label="TuffClient" href="/assets/builds/wasm/" />
            <Button label="Discord" href="https://discord.tuff.ws" />
        </div>
    }
}