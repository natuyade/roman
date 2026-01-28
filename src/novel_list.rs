use leptos::prelude::*;
use leptos_router::components::A;

use crate::play_sound;
use wasm_bindgen::JsCast;
use crate::SoundSE;
use crate::load_sound::{LoadSounds, SoundEffects};

// 目次ページ
#[component]
pub fn novel_page_list() -> impl IntoView {
    let sound_ref = SoundEffects::new();
    let cursoron_ref = sound_ref.cursoron;
    // unwrapで中のSomeをそのまま渡せる
    let SoundSE { sevlm, .. } = use_context::<SoundSE>().unwrap();

    view! {
        // load soundlist
        <LoadSounds sound_refs=sound_ref />
            <div class="text-box-pos">
                <div class="text-box">
                    <p>
                        <A
                            attr:class="novel-link"
                            href="/novel_1"
                            on:mouseenter=move |_| play_sound!{cursoron_ref, sevlm}
                        >"平凡な生活"</A>
                    </p>
                    <p class="p-margin">
                    <A
                        attr:class="novel-link"
                        href="/novel_2"
                        on:mouseenter=move |_| play_sound!{cursoron_ref, sevlm}
                    >"壊れかけの炒飯"</A>
                    </p>
                    <p class="p-margin">
                    <A
                        attr:class="novel-link"
                        href="/novel_3"
                        on:mouseenter=move |_| play_sound!{cursoron_ref, sevlm}
                    >"ペンギンgaku園"</A>
                    </p>
                    <p class="list-subtitle">"~リストバンド戦争~"</p>
                </div>
            </div>
    }
}
