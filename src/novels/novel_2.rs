use leptos::prelude::*;

use crate::page_counter::{Novel, NovelImg, get_message};

use crate::play_sound;
use wasm_bindgen::JsCast;
use crate::SoundSE;
use crate::load_sound::{LoadSounds, SoundEffects};

// 小説ページ
#[component]
pub fn novel_page_2() -> impl IntoView {
    
    let sound_ref = SoundEffects::new();
    let pageflip_ref = sound_ref.pageflip;
    let SoundSE { sevlm, set_sevlm } = use_context::<SoundSE>().unwrap();
    
    let (count, set_count) = signal(0usize);

    view! {
        <div class="novelbg">
            <div class="inner-bg">
                <div class="inner">
                    <h1>"『壊れかけの炒飯』"</h1>
                    <Show
                        when= move || NovelImg::Novel2.nimgpath(count.get()).is_some()
                        fallback=|| ()
                    >{
                        view!{
                            <img
                                class="illust"
                                src = move || NovelImg::Novel2.nimgpath(count.get())
                            />
                        }
                    }
                    </Show>

                    <p class="novel">{ move || get_message(Novel::Novel2 , count.get()) }</p>

                </div>
            </div>
        </div>
    }
}
