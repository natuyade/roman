use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::{
    SoundSE,
    load_sound::{LoadSounds, SoundEffects},
};

#[component]
pub fn sounds_vlm() -> (ReadSignal<usize>, WriteSignal<usize>) {
    signal(0usize)
}

pub fn setting_menu_tab() -> impl IntoView {
    let (settings, set_settings) = signal(false);
    let (settings_anim, set_settings_anim) = signal(false);
    let (tab_anim, set_tab_anim) = signal(false);

    let (vlmcache, set_vlmcache) = signal(0usize);

    let SoundSE { sevlm, set_sevlm } = use_context::<SoundSE>().unwrap();

    let sound_ref = SoundEffects::new();
    let cursoron_ref = sound_ref.cursoron;

    view! {
        // load soundlist
        <LoadSounds sound_refs=sound_ref />

        <div class="settings-wrapper">
            <img src="assets/images/setting.webp"
                class="settings-icon"
                class:setting-anim={move || settings_anim.get()}
                on:click=move |_| {
                    if !settings.get() {
                        set_settings.set(true)
                    } else {
                        set_tab_anim.set(true)
                    }
                    set_settings_anim.set(true)
                }
                on:animationend=move |_| set_settings_anim.set(false)
            />
            <Show when=move || settings.get()>
            <div class="stng-container">
                <div class="stng-bg"
                    on:click=move |_| set_tab_anim.set(true)
                >
                </div>
                <div
                    class="settings"
                    class:settings-tab-anim-open=move || settings.get()
                    class:settings-tab-anim-close=move || tab_anim.get()
                    on:animationend=move |_| 
                        if tab_anim.get() {
                            set_tab_anim.set(false);
                            set_settings.set(false)
                        }
                >
                <div class="settings-tab">
                    <h1 class="settings-text">"設定"</h1>
                </div>
                <div class="sounds-stng">
                    <img class="close-button"
                        src="assets/images/close.webp"
                        on:click=move |_| set_tab_anim.set(true)
                    />
                    <div 
                        class="serange-wrapper"
                        on:mouseenter=move |_| {
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
                    >
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        value="0"
                        class="serange"
                        on:click=move |_| set_vlmcache.set(sevlm.get())

                        /*
                        * on:inputだけの場合、イベントが発火し.target()した際に
                        * Rust側はターゲットの要素(今回でいう<input>)がどんな型なのかが分からず曖昧になる。
                        * :targetを追加することでこれはhtmlのinput要素ですよと伝えることができ
                        * ev.target()=今回はHtmlInputElement(<input>)になる。
                        * .target()=その対象の, .value()=中の値
                        */
                        on:input:target=move |ev| {
                            // parse()でusizeに変換, resultなのでunwrap()
                            set_sevlm.set(ev.target().value().parse::<usize>().unwrap())
                        }

                        /*
                        * prop:はsignal(volume)の値をDOMのproperty(実際の値)に反映させる
                        * HTMLのvalue属性の値はDOMでの初期値になる。
                        * 実際にブラウザ上で変化する値はDOMの属性ではなくDOMのpropertyの値
                        */
                        prop:value=sevlm
                    />
                    </div>
                    <button
                        on:click=move |_| {
                            if sevlm.get() > 0{
                                set_sevlm.set(0)
                            } else {
                                set_sevlm.set(vlmcache.get())
                            }
                        }>
                        <Show when=move || {sevlm.get() > 0}>
                            "🔊"
                        </Show>
                        <Show when=move || sevlm.get() == 0>
                            "🔇"
                        </Show>
                        "SE Volume "{ move || sevlm.get() }"%"
                    </button>
                    </div>
                </div>
            </div>
            </Show>
        </div>
    }
}
