use leptos::prelude::*;

#[component]
pub fn TuffButton() -> impl IntoView {
    view! {
        <div class="hero">
            <h1>"tufflauncher"</h1>
            <button
                class="btn"
                on:click=move |_| {
                    let _ = window().location().set_href("/assets/builds/wasm/");
                }
            >
                {"TuffClient"}
            </button>
        </div>
    }
}