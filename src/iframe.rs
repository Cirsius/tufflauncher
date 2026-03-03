use leptos::prelude::*;

#[component]
pub fn Iframe() -> impl IntoView {
    let (iframe, set_iframe) = signal(false);

    view! {
        <button on:click=move |_| set_iframe.set(!iframe.get())>
            "TuffClient"
        </button>
        <iframe src="builds/wasm/" hidden=move || iframe.get()></iframe>
    }
}
