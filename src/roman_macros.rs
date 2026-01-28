#[macro_export]
macro_rules! play_sound {
    ($e: expr, $field: ident) => {
        if let Some(audio) = $e.get() {
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
