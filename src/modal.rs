use leptos::prelude::*;

#[component]
pub fn Modal(
    open: ReadSignal<bool>,
    on_close: impl Fn() + Copy + Send + Sync + 'static,
) -> impl IntoView {
    let (closing, set_closing) = signal(false);

    let handle_close = move || {
        set_closing.set(true);
        set_timeout(
            move || {
                set_closing.set(false);
                on_close();
            },
            std::time::Duration::from_millis(250),
        );
    };

    view! {
        <Show when=move || open.get()>
            <div
                class=move || if closing.get() { "modal-backdrop closing" } else { "modal-backdrop" }
                on:click=move |_| handle_close()
            >
                <div
                    class=move || if closing.get() { "modal closing" } else { "modal" }
                    on:click=move |e| e.stop_propagation()
                >
                    <button class="modal-close" on:click=move |_| handle_close()>"×"</button>
                    <h2>"tuffclient"</h2>
                    <button
                        class="btn"
                        on:click=move |_| {
                            let _ = window().location().set_href("/assets/builds/wasm");
                        }
                    >
                        "play here"
                    </button>
                    <a class="btn" href="/assets/builds/wasm/offline-download.html" download>
                        "download"
                    </a>
                </div>
            </div>
        </Show>
    }
}
