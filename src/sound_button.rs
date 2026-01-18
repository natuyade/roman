use leptos::prelude::*;

#[component]
pub fn sound_btn() -> impl IntoView {
    let (sounds, set_sounds) = signal(false);

    view! {
        <button
            class="sound_btn"
            on:click=move |_| set_sounds.update(|c| *c = !*c)>
            <Show when=move || !sounds.get()>
                "🔊"
            </Show>
            <Show when=move || sounds.get()>
                "🔇"
            </Show>
            <input type="range"
                min="0"
                max="100"
                step="10"
                value="0"
                />
        </button>
    }
}
