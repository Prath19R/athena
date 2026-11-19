use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::components::message_bubble::MessageBubble;
use crate::components::input_bar::InputBar;
use crate::components::suggested_questions::SuggestedQuestions;
use crate::services::api::{ChatMessage, send_chat_message};

#[component]
pub fn ChatWindow() -> impl IntoView {
    let initial_messages = vec![
        ChatMessage {
            role: "assistant".into(),
            content: "Hello! I am Athena, Prathmesh Ranadive's AI guide. I can answer any questions regarding his distributed systems experience at NICE CXone (ACDHub & WFM Forecaster), his projects at YASH Technologies, or his overall resume. How can I help you?".into(),
        }
    ];

    let (messages, set_messages) = signal(initial_messages);
    let (is_loading, set_is_loading) = signal(false);

    let handle_send = move |user_input: String| {
        if is_loading.get() {
            return;
        }

        let user_msg = ChatMessage {
            role: "user".into(),
            content: user_input,
        };

        set_messages.update(|list| list.push(user_msg));
        set_is_loading.set(true);

        let current_history = messages.get();

        spawn_local(async move {
            match send_chat_message(current_history).await {
                Ok(reply) => {
                    set_messages.update(|list| {
                        list.push(ChatMessage {
                            role: "assistant".into(),
                            content: reply,
                        });
                    });
                }
                Err(_) => {
                    set_messages.update(|list| {
                        list.push(ChatMessage {
                            role: "assistant".into(),
                            content: "Athena is momentarily synchronizing with the server. Please try asking again in a moment, or reach out to Prathmesh at prathmeshranadive@gmail.com.".into(),
                        });
                    });
                }
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="flex flex-col h-full bg-[#09090b] text-zinc-100 overflow-hidden">
            // Header
            <header class="h-14 border-b border-zinc-800/80 px-4 flex items-center justify-between bg-zinc-950/80 backdrop-blur-sm shrink-0">
                <div class="flex items-center gap-2.5">
                    <div class="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 flex items-center justify-center text-sm shadow-inner">
                        "🏛️"
                    </div>
                    <div>
                        <div class="flex items-center gap-2">
                            <h1 class="text-sm font-semibold tracking-tight text-zinc-100">"Athena"</h1>
                            <span class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-full text-[10px] font-mono bg-emerald-950/80 text-emerald-400 border border-emerald-800/50">
                                <span class="w-1 h-1 rounded-full bg-emerald-400 animate-pulse"></span>
                                "online"
                            </span>
                        </div>
                        <p class="text-[11px] text-zinc-500 font-mono">"AI Portfolio Guide · Prathmesh Ranadive"</p>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <a
                        href="https://prath19r.github.io/akashic-records/assets/resume.pdf"
                        target="_blank"
                        rel="noreferrer"
                        class="text-xs font-mono text-zinc-400 hover:text-zinc-200 border border-zinc-800 hover:border-zinc-700 bg-zinc-900 px-2.5 py-1 rounded-lg transition-colors flex items-center gap-1.5"
                    >
                        <span>"Resume"</span>
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                            <polyline points="15 3 21 3 21 9"></polyline>
                            <line x1="10" y1="14" x2="21" y2="3"></line>
                        </svg>
                    </a>
                </div>
            </header>

            // Message list container
            <div class="flex-1 overflow-y-auto p-4 space-y-1">
                {move || messages.get().into_iter().map(|msg| {
                    view! { <MessageBubble message=msg /> }
                }).collect_view()}

                // Loading indicator
                {move || is_loading.get().then(|| {
                    view! {
                        <div class="flex justify-start mb-3">
                            <div class="bg-zinc-900 border border-zinc-800 text-zinc-400 text-xs px-3.5 py-2.5 rounded-xl rounded-bl-none flex items-center gap-2">
                                <span class="w-1.5 h-1.5 rounded-full bg-zinc-400 animate-bounce"></span>
                                <span class="w-1.5 h-1.5 rounded-full bg-zinc-400 animate-bounce [animation-delay:0.2s]"></span>
                                <span class="w-1.5 h-1.5 rounded-full bg-zinc-400 animate-bounce [animation-delay:0.4s]"></span>
                                <span class="text-[11px] font-mono text-zinc-500 ml-1">"Athena is thinking..."</span>
                            </div>
                        </div>
                    }
                })}
            </div>

            // Suggested quick questions
            <SuggestedQuestions on_select=handle_send />

            // Input Bar
            <InputBar on_send=handle_send is_loading=is_loading />
        </div>
    }
}
