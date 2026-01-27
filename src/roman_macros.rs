#[macro_export]
macro_rules! play_sound {
    ($e: expr, $field: ident) => {
        move |_| {
        if let Some(audio) = $e.get() {
            let audio_cloned =
                audio
                .clone_node_with_deep(true)
                .unwrap()
                .unchecked_into::<web_sys::HtmlAudioElement>();
            /*
             * macro_rules!では呼び出し時に与えられたトークンを元にコードを展開する
             * なのでマクロのコード外にある値や識別子を使う場合,
             * マクロを定義するときに引数として渡す必要がある
             */ 
            audio_cloned.set_volume($field.get() as f64 / 100.0);
            let _ = audio_cloned.play();
        }
        }
    }
}

#[macro_export]
macro_rules! page_swicther {
    ($rdsig: expr, $plus: expr, $minus: expr, $nowpg: expr) => {
        view!{
            // count > 0 のときだけ「前」を表示
            <Show when={move || $rdsig.get() > 0}>
                <button
                    class="button left"
                    on:click=$minus
                >
                    "prev"
                </button>
            </Show>
            // count < pages のときだけ「次」を表示
            <Show when={move || $rdsig.get() + 1 < $nowpg}>
                <button
                    class="button right"
                    on:click=$plus
                >
                    "next"
                </button>
            </Show>
        }
    };
}