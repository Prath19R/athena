use leptos::prelude::*;
use crate::components::chat_window::ChatWindow;

#[component]
pub fn EmbedLayout() -> impl IntoView {
    view! {
        <div class="w-full h-screen bg-[#09090b] flex flex-col overflow-hidden">
            <ChatWindow />
        </div>
    }
}
