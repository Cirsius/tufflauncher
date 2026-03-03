use leptos::prelude::*;

#[component]
pub fn Modal(
    open: ReadSignal<bool>,
    on_close: impl Fn() + Copy + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div class="modal-backdrop" on:click=move |_| on_close()>
                <div class="modal" on:click=move |e| e.stop_propagation()>
                    <button class="modal-close" on:click=move |_| on_close()>"x"</button>
                    <h2>"uwu"</h2>
                    <p>"im so gay"</p>
                </div>
            </div>
        </Show>
    }
}
