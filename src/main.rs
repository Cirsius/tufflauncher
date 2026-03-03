mod button;
mod modal;

use button::TuffButton;

fn main() {
    leptos::mount::mount_to_body(TuffButton)
}
