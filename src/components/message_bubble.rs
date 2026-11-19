use leptos::prelude::*;
use crate::services::api::ChatMessage;
use pulldown_cmark::{Parser, Options, html};

pub fn render_markdown(text: &str) -> String {
    // Clean up annoying nested delimiter artifacts like ":** - **" or ":** - " or "-  **"
    let normalized = text
        .replace(":** - **", ": **")
        .replace(":** - ", ": ")
        .replace("** - **", " — ")
        .replace("-  **", "- **");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&normalized, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[component]
pub fn MessageBubble(message: ChatMessage) -> impl IntoView {
    let is_user = message.role == "user";
    let formatted_html = render_markdown(&message.content);

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
                <div class="flex items-center gap-1.5 mb-1.5 text-[11px] font-mono tracking-wider text-zinc-500 uppercase">
                    {if is_user { "You" } else { "Athena" }}
                </div>
                {if is_user {
                    view! {
                        <div class="whitespace-pre-wrap font-sans text-zinc-100">
                            {message.content}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div
                            class="font-sans text-zinc-300 space-y-2 [&_p]:leading-relaxed [&_p]:my-1.5 [&_strong]:font-semibold [&_strong]:text-zinc-100 [&_ul]:list-disc [&_ul]:pl-5 [&_ul]:my-2 [&_ul]:space-y-1 [&_ol]:list-decimal [&_ol]:pl-5 [&_ol]:my-2 [&_ol]:space-y-1 [&_li]:leading-relaxed [&_code]:bg-zinc-800 [&_code]:text-emerald-400 [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:rounded [&_code]:font-mono [&_code]:text-[11px] [&_a]:text-emerald-400 [&_a]:underline [&_h1]:text-base [&_h1]:font-semibold [&_h1]:text-zinc-100 [&_h2]:text-sm [&_h2]:font-semibold [&_h2]:text-zinc-100 [&_h3]:text-xs [&_h3]:font-semibold [&_h3]:text-zinc-200"
                            inner_html=formatted_html
                        />
                    }.into_any()
                }}
            </div>
        </div>
    }
}
