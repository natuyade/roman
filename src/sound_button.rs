use leptos::{html::Audio, prelude::*};

#[component]
pub fn sound_btn() -> impl IntoView {
    let (vlmcache, set_vlmcache) = signal(0usize);
    let (sevlm, set_sevlm) = signal(0usize);
    let audio_ref: NodeRef<Audio> = NodeRef::new();
    view! {
            <audio node_ref=audio_ref>
                <source src="assets/sounds/button40.OGG" type="audio/ogg"/>
                <source src="assets/sounds/button40.mp3" type="audio/mp3"/>
            </audio>
            <input type="range"
                min="0"
                max="100"
                step="1"
                value="0"
                class="volume_slide"
                id="sound_btn"
                on:mouseenter= move |_| {
                    if let Some(audio) = audio_ref.get() {
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
                    class="sound_btn"
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
                    "Volume "{ move || sevlm.get() }"%"
                </button>
    }
}
