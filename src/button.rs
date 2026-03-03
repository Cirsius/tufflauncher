use crate::modal::Modal;
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
    let (modal_open, set_modal_open) = signal(false);

    view! {
        <div class="hero">
            <h1>"tufflauncher"</h1>
            <button class="btn" on:click=move |_| set_modal_open.set(true)>
                "TuffClient"
            </button>
            <Button label="Discord" href="https://discord.tuff.ws" />
            <Modal open=modal_open on_close=move || set_modal_open.set(false) />
        </div>
    }
}
