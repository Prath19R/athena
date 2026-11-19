use leptos::prelude::*;
use crate::services::api::ChatMessage;

#[component]
pub fn MessageBubble(message: ChatMessage) -> impl IntoView {
    let is_user = message.role == "user";

    view! {
        <div class=format!(
            "flex w-full mb-3 {}",
            if is_user { "justify-end" } else { "justify-start" }
        )>
            <div class=format!(
                "max-w-[85%] text-xs md:text-sm px-3.5 py-2.5 rounded-xl border leading-relaxed {}",
                if is_user {
                    "bg-zinc-800 text-zinc-100 border-zinc-700/60 rounded-br-none shadow-sm"
                } else {
                    "bg-zinc-900/90 text-zinc-300 border-zinc-800 rounded-bl-none shadow-sm backdrop-blur-sm"
                }
            )>
                <div class="flex items-center gap-1.5 mb-1 text-[11px] font-mono tracking-wider text-zinc-500 uppercase">
                    {if is_user { "You" } else { "Athena" }}
                </div>
                <div class="whitespace-pre-wrap font-sans">
                    {message.content}
                </div>
            </div>
        </div>
    }
}
