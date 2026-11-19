use leptos::prelude::*;
use web_sys::KeyboardEvent;

#[component]
pub fn InputBar<F>(
    on_send: F,
    is_loading: ReadSignal<bool>,
) -> impl IntoView
where
    F: Fn(String) + 'static + Copy,
{
    let (text, set_text) = signal(String::new());

    let do_send = move || {
        let val = text.get().trim().to_string();
        if !val.is_empty() && !is_loading.get() {
            set_text.set(String::new());
            on_send(val);
        }
    };

    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            do_send();
        }
    };

    view! {
        <div class="p-3 border-t border-zinc-800 bg-[#09090b]">
            <div class="flex items-center gap-2 bg-zinc-900 border border-zinc-800 rounded-xl px-3 py-1.5 focus-within:border-zinc-700 transition-colors">
                <input
                    type="text"
                    placeholder="Ask Athena about Pratham's resume, projects, or background..."
                    prop:value=move || text.get()
                    on:input=move |ev| set_text.set(event_target_value(&ev))
                    on:keydown=on_keydown
                    disabled=move || is_loading.get()
                    class="w-full bg-transparent text-xs md:text-sm text-zinc-100 placeholder-zinc-500 focus:outline-none disabled:opacity-50"
                />
                <button
                    type="button"
                    on:click=move |_| do_send()
                    disabled=move || is_loading.get() || text.get().trim().is_empty()
                    class="p-1.5 rounded-lg bg-zinc-100 text-zinc-900 hover:bg-white disabled:opacity-30 disabled:hover:bg-zinc-100 transition-all shrink-0"
                    title="Send Message"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="22" y1="2" x2="11" y2="13"></line>
                        <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
                    </svg>
                </button>
            </div>
            <div class="mt-1 flex justify-between items-center px-1 text-[10px] text-zinc-600 font-mono">
                <span>"Powered by NVIDIA Nemotron 30B & Rust WASM"</span>
                <span>"Press Enter to send"</span>
            </div>
        </div>
    }
}
