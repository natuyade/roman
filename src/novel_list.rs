use leptos::prelude::*;
use leptos_router::components::A;

use crate::{SoundSE, load_sound::{LoadSounds, SoundEffects}};

// 目次ページ
#[component]
pub fn novel_page_list() -> impl IntoView {
    let sound_ref = SoundEffects::new();
    let cursoron_ref = sound_ref.cursoron;
    // unwrapで中のSomeをそのまま渡せる
    let SoundSE{ sevlm, .. } = use_context::<SoundSE>().unwrap();

    view! {
        // load soundlist
        <LoadSounds sound_refs=sound_ref />
            <div class="text-box-pos">
                <div class="text-box">
                    <p>
                        <A
                            attr:class="novel-link"
                            href="/novel_1"
                            on:mouseenter= move |_| {
                                if let Some(audio) = cursoron_ref.get() {
                                    let _ = {
                                        audio.set_volume(sevlm.get() as f64 / 100.0);
                                        audio.load();
                                        audio.play()
                                    };
                                }
                            }
                        >"平凡な生活"</A>
                    </p>
                    <p class="p-margin">
                    <A
                        attr:class="novel-link"
                        href="/novel_2"
                        on:mouseenter= move |_| {
                            if let Some(audio) = cursoron_ref.get() {
                                let _ = {
                                    audio.set_volume(sevlm.get() as f64 / 100.0);
                                    audio.load();
                                    audio.play()
                                };
                            }
                        }
                    >"壊れかけの炒飯"</A>
                    </p>
                    <p class="p-margin">
                    <A
                        attr:class="novel-link"
                        href="/novel_3"
                        on:mouseenter= move |_| {
                            if let Some(audio) = cursoron_ref.get() {
                                let _ = {
                                    audio.set_volume(sevlm.get() as f64 / 100.0);
                                    audio.load();
                                    audio.play()
                                };
                            }
                        }
                    >"毒チワワとコンビニ店員"</A>
                    </p>
                    <p class="list_subtitle">"~のどかな街のマスコット~"</p>
                    <p class="p-margin">
                    <A
                        attr:class="novel-link"
                        href="/test_1"
                        on:mouseenter= move |_| {
                            if let Some(audio) = cursoron_ref.get() {
                                let _ = {
                                    audio.set_volume(sevlm.get() as f64 / 100.0);
                                    audio.load();
                                    audio.play()
                                };
                            }
                        }
                    >"test"</A>
                    </p>
                </div>
            </div>
    }
}
