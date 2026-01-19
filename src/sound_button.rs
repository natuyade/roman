use leptos::prelude::*;

#[component]
pub fn sound_btn() -> impl IntoView {
    let (vlmcache, set_vlmcache) = signal(0f32);
    let (volume, set_volume) = signal(0f32);

    view! {
        <button
            class="sound_btn"
            on:click=move |_| {
                if volume.get() > 0.0{
                    set_volume.set(0.0)
                } else {
                    set_volume.set(vlmcache.get())
                }
            }>
            <Show when=move || {volume.get() > 0.0}>
                "🔊"
            </Show>
            <Show when=move || volume.get() == 0.0>
                "🔇"
            </Show>
        </button>
            <input type="range"
                min="0.0"
                max="1.0"
                step="0.01"
                value="0"
                class="volume_slide"
                id="sound_btn"
                on:click=move |_| set_vlmcache.set(volume.get())

                /*
                 * on:inputだけの場合、イベントが発火し.target()した際に
                 * Rust側はターゲットの要素(今回でいう<input>)がどんな型なのかが分からず曖昧になる。
                 * :targetを追加することでこれはhtmlのinput要素ですよと伝えることができ
                 * ev.target()=今回はHtmlInputElement(<input>)になる。
                 * .target()=その対象の, .value()=中の値
                 */
                on:input:target=move |ev| {
                    // parse()でf32に変換(audioタグのvolumeが0.0~1.0のため), resultなのでunwrap()
                    set_volume.set(ev.target().value().parse::<f32>().unwrap())
                }

                /*
                 * prop:はsignal(volume)の値をDOMのproperty(実際の値)に反映させる
                 * HTMLのvalue属性の値はDOMでの初期値になる。
                 * 実際にブラウザ上で変化する値はDOMの属性ではなくDOMのpropertyの値
                 */
                prop:value=volume
                />

                // for debug
            <p class="volume_value">"Volume "{ move || volume.get() * 100.0 }"%"</p>
    }
}
