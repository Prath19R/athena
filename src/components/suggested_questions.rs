use leptos::prelude::*;

#[component]
pub fn SuggestedQuestions<F>(on_select: F) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let questions = vec![
        "Tell me about Pratham's distributed systems experience",
        "How does the Autonomous Trading Agent work?",
        "What did Pratham build with Envoy & Rust WASM?",
        "What is Pratham's core tech stack & background?",
    ];

    view! {
        <div class="px-3 pb-2 pt-1 border-t border-zinc-800/80 bg-zinc-950/60">
            <p class="text-[11px] font-mono uppercase tracking-wider text-zinc-500 mb-1.5 flex items-center gap-1.5">
                <span class="inline-block w-1.5 h-1.5 rounded-full bg-zinc-600"></span>
                "Suggested Questions"
            </p>
            <div class="flex gap-2 overflow-x-auto pb-1 scrollbar-none">
                {questions.into_iter().map(|q| {
                    let text = q.to_string();
                    let text_clone = text.clone();
                    view! {
                        <button
                            type="button"
                            on:click=move |_| on_select(text_clone.clone())
                            class="shrink-0 text-xs text-zinc-400 hover:text-zinc-100 bg-zinc-900/80 hover:bg-zinc-800 border border-zinc-800 hover:border-zinc-700 px-2.5 py-1 rounded-lg transition-colors duration-150"
                        >
                            {text}
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
