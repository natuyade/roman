use leptos::{html::Audio, prelude::*};

#[derive(Clone)]
/* 
 * se<audio>の参照をまとめて持つための箱
 * struct(構造体)が呼ばれたら中身の情報をまとめて渡せる
*/
pub struct SoundEffects {
    pub cursoron: NodeRef<Audio>,
    pub cardflip: NodeRef<Audio>,
}

/* 
 * implはstruct君に立ち振る舞い,レシピを教える
 * ここではSoundEffects君に,
 * SoundEffects::new()が呼ばれたときは,初期化してどこにも紐づいていない`参照(NodeRef)`を用意してねというだけ
 */
impl SoundEffects {
    pub fn new() -> Self {
        Self {
        /* 
         * new()は中身が初期化されてどこにも紐づいていない`参照`という形にするだけ.
         * <audio>がレンダリングされたときに対応したnode_refと自動的に紐づく
         */
            cursoron: NodeRef::new(),
            cardflip: NodeRef::new(),
        }
    }
}

#[component]
pub fn load_sounds(
    // ここでのSoundEffectsはsound_refsの型(x: 型)
    sound_refs: SoundEffects
) -> impl IntoView {
    view! {
        /* 
         * audioは同時に操作される可能性があるので.clone()するが
         * NodeRef自体がCopytraitなので書く必要はない。
         */
        <audio node_ref=sound_refs.cursoron>
            <source src="assets/sounds/cursoron.mp3" type="audio/mp3"/>
            <source src="assets/sounds/cursoron.ogg" type="audio/ogg"/>
            <source src="assets/sounds/cursoron.wav" type="audio/wav"/>
        </audio>
        <audio node_ref=sound_refs.cardflip>
            <source src="assets/sounds/cardflip.mp3" type="audio/mp3"/>
            <source src="assets/sounds/cardflip.ogg" type="audio/ogg"/>
            <source src="assets/sounds/cardflip.wav" type="audio/wav"/>
        </audio>
    }
}
