use athena::app::App;
use leptos::prelude::*;

fn main() {
    _ = console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
