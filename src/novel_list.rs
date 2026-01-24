use leptos::prelude::*;
use leptos_router::components::A;

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
                                let audio_cloned =
                                    audio
                                    /*
                                     * trueで<audio>の中(子要素含む全て)まで複製する
                                     * falseは<audio>(親要素)のみ
                                     */
                                    .clone_node_with_deep(true)
                                    .unwrap()
                                    /*
                                     * JsValueを受け取り型チェックを行わず
                                     * HtmlAudioElementだと仮定して
                                     * 型をHtmlAudioElementに付け替える
                                     */
                                    .unchecked_into::<web_sys::HtmlAudioElement>();

                                audio_cloned.set_volume(sevlm.get() as f64 / 100.0);
                                let _ = audio_cloned.play();
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
                                let audio_cloned =
                                    audio
                                    /*
                                     * trueで<audio>の中(子要素含む全て)まで複製する
                                     * falseは<audio>(親要素)のみ
                                     */
                                    .clone_node_with_deep(true)
                                    .unwrap()
                                    /*
                                     * JsValueを受け取り型チェックを行わず
                                     * HtmlAudioElementだと仮定して
                                     * 型をHtmlAudioElementに付け替える
                                     */
                                    .unchecked_into::<web_sys::HtmlAudioElement>();

                                audio_cloned.set_volume(sevlm.get() as f64 / 100.0);
                                let _ = audio_cloned.play();
                            }
                        }
                    >"毒チワワとコンビニ店員"</A>
                    </p>
                    <p class="list-subtitle">"~のどかな街のマスコット~"</p>
                    <p class="p-margin">
                    <A
                        attr:class="novel-link"
                        href="/test_1"
                        on:mouseenter= move |_| {
                            if let Some(audio) = cursoron_ref.get() {
                                let audio_cloned =
                                    audio
                                    /*
                                     * trueで<audio>の中(子要素含む全て)まで複製する
                                     * falseは<audio>(親要素)のみ
                                     */
                                    .clone_node_with_deep(true)
                                    .unwrap()
                                    /*
                                     * JsValueを受け取り型チェックを行わず
                                     * HtmlAudioElementだと仮定して
                                     * 型をHtmlAudioElementに付け替える
                                     */
                                    .unchecked_into::<web_sys::HtmlAudioElement>();

                                audio_cloned.set_volume(sevlm.get() as f64 / 100.0);
                                let _ = audio_cloned.play();
                            }
                        }
                    >"test"</A>
                    </p>
                </div>
            </div>
    }
}
