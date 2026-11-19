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
            "flex w-full mb-3.5 {}",
            if is_user { "justify-end" } else { "justify-start" }
        )>
            <div class=format!(
                "max-w-[88%] px-4 py-3 rounded-2xl border shadow-md {}",
                if is_user {
                    "bg-zinc-800 text-zinc-100 border-zinc-700/80 rounded-br-none"
                } else {
                    "bg-zinc-900/95 text-zinc-100 border-zinc-700/70 rounded-bl-none backdrop-blur-sm"
                }
            )>
                <div class=format!(
                    "flex items-center gap-1.5 mb-1.5 text-xs font-mono font-bold tracking-wider uppercase {}",
                    if is_user { "text-zinc-400" } else { "text-emerald-400" }
                )>
                    {if is_user { "You" } else { "Athena" }}
                </div>
                {if is_user {
                    view! {
                        <div class="whitespace-pre-wrap font-sans text-zinc-100 text-[15px] sm:text-[16px] leading-relaxed">
                            {message.content}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div
                            class="font-sans text-zinc-100 text-[15px] sm:text-[16px] leading-relaxed space-y-2.5 [&_p]:leading-[1.65] [&_p]:my-2 [&_p]:text-zinc-100 [&_strong]:font-bold [&_strong]:text-emerald-300 [&_ul]:list-disc [&_ul]:pl-5 [&_ul]:my-2.5 [&_ul]:space-y-1.5 [&_ol]:list-decimal [&_ol]:pl-5 [&_ol]:my-2.5 [&_ol]:space-y-1.5 [&_li]:leading-[1.65] [&_li]:text-zinc-200 [&_code]:bg-zinc-800/90 [&_code]:text-emerald-300 [&_code]:px-2 [&_code]:py-0.5 [&_code]:rounded [&_code]:font-mono [&_code]:text-sm [&_a]:text-emerald-400 [&_a]:underline [&_a]:font-medium [&_h1]:text-lg [&_h1]:font-bold [&_h1]:text-zinc-100 [&_h2]:text-base [&_h2]:font-bold [&_h2]:text-zinc-100 [&_h3]:text-sm [&_h3]:font-bold [&_h3]:text-zinc-200"
                            inner_html=formatted_html
                        />
                    }.into_any()
                }}
            </div>
        </div>
    }
}
