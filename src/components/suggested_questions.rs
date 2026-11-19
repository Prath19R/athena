use leptos::prelude::*;

#[component]
pub fn SuggestedQuestions<F>(on_select: F) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let questions = vec![
        "Tell me about Prathmesh's distributed systems experience",
        "What did Prathmesh achieve on the WFM Forecaster platform?",
        "What did Prathmesh build with Envoy & Rust WASM?",
        "Tell me about Prathmesh's projects at YASH Technologies",
        "What is Prathmesh's core tech stack & background?",
    ];

    view! {
        <div class="px-3 pb-2 pt-2 border-t border-zinc-800/80 bg-zinc-950/80">
            <p class="text-xs font-mono uppercase tracking-wider text-zinc-400 mb-2 flex items-center gap-1.5 font-semibold">
                <span class="inline-block w-2 h-2 rounded-full bg-emerald-400"></span>
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
                            class="shrink-0 text-xs sm:text-sm text-zinc-200 hover:text-white bg-zinc-900 hover:bg-zinc-800 border border-zinc-700 hover:border-emerald-500/60 px-3.5 py-1.5 rounded-xl font-medium transition-colors duration-150 shadow-sm"
                        >
                            {text}
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
