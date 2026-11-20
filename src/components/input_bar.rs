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
            <div class="flex items-center gap-2.5 bg-zinc-900 border border-zinc-700 rounded-xl px-3.5 py-2 focus-within:border-emerald-500/80 transition-colors shadow-inner">
                <input
                    type="text"
                    placeholder="Ask Athena about Prathmesh's background, projects, or resume..."
                    prop:value=move || text.get()
                    on:input=move |ev| set_text.set(event_target_value(&ev))
                    on:keydown=on_keydown
                    disabled=move || is_loading.get()
                    class="w-full bg-transparent text-sm sm:text-base text-zinc-100 placeholder-zinc-500 focus:outline-none disabled:opacity-50"
                />
                <button
                    type="button"
                    on:click=move |_| do_send()
                    disabled=move || is_loading.get() || text.get().trim().is_empty()
                    class="p-2 rounded-xl bg-emerald-500 text-zinc-950 hover:bg-emerald-400 disabled:opacity-30 disabled:hover:bg-emerald-500 transition-all shrink-0 font-bold shadow"
                    title="Send Message"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="22" y1="2" x2="11" y2="13"></line>
                        <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
                    </svg>
                </button>
            </div>
            <div class="mt-1.5 flex justify-between items-center px-1 text-xs text-zinc-500 font-mono">
                <span>"Athena AI · Rust WASM"</span>
                <span>"Press Enter to send"</span>
            </div>
        </div>
    }
}
