use crate::{SoundSE, load_sound::{LoadSounds, SoundEffects}};
use leptos::prelude::*;

#[component]
pub fn sounds_vlm() -> (ReadSignal<usize>, WriteSignal<usize>) {
    signal(0usize)
}

pub fn setting_menu_tab() -> impl IntoView {
    let (settings, set_settings) = signal(false);
    let (settings_anim, set_settings_anim) = signal(false);

    let (vlmcache, set_vlmcache) = signal(0usize);

    let SoundSE { sevlm, set_sevlm } = use_context::<SoundSE>().unwrap();

    let sound_ref = SoundEffects::new();
    let cursoron_ref = sound_ref.cursoron;

    view! {
        // load soundlist
        <LoadSounds sound_refs=sound_ref />

        <div class="settings_wrapper">
            <img src="assets/images/setting.webp"
                class="settings_icon"
                class:setting_anim={move || settings_anim.get()}
                on:click=move |_| {
                    set_settings_anim.set(true);
                    set_settings.update(|c| *c = !*c)
                }
                on:animationend=move |_| set_settings_anim.set(false)
            />
            <Show when=move || settings.get()>
            <div class="stng_container">
                <div class="settings">
                <div class="settings_tab">
                    <h1 class="settings_text">"設定"</h1>
                </div>
                <div class="sounds_stng">
                    <img class="close_button"
                        src="assets/images/close.webp"
                        on:click=move |_| set_settings.set(false)
                    />
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        value="0"
                        class="serange"
                        on:mouseenter= move |_| {
                            if let Some(audio) = cursoron_ref.get() {
                                let _ = {
                                    audio.set_volume(sevlm.get() as f64 / 100.0);
                                    audio.load();
                                    audio.play()
                                };
                            }
                        }
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
