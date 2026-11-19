use leptos::prelude::*;
use crate::components::chat_window::ChatWindow;
use crate::components::embed_layout::EmbedLayout;

#[component]
pub fn App() -> impl IntoView {
    let is_embed = move || {
        if let Some(win) = web_sys::window() {
            if let Ok(search) = win.location().search() {
                return search.contains("embed=true");
            }
        }
        false
    };

    view! {
        <main class="w-full h-screen bg-[#09090b] text-zinc-100 font-sans">
            {move || {
                if is_embed() {
                    view! { <EmbedLayout /> }.into_any()
                } else {
                    view! {
                        <div class="max-w-3xl mx-auto h-screen md:py-6 md:px-4">
                            <div class="h-full border-0 md:border md:border-zinc-800 md:rounded-2xl overflow-hidden shadow-2xl bg-zinc-950">
                                <ChatWindow />
                            </div>
                        </div>
                    }.into_any()
                }
            }}
        </main>
    }
}
