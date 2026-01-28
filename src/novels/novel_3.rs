use leptos::prelude::*;

use crate::page_counter::{Novel, NovelImg, get_message};

use crate::play_sound;
use wasm_bindgen::JsCast;
use crate::SoundSE;
use crate::load_sound::{LoadSounds, SoundEffects};

// 小説ページ
#[component]
pub fn novel_page_3() -> impl IntoView {
    
        let sound_ref = SoundEffects::new();
        let pageflip_ref = sound_ref.pageflip;
        let SoundSE { sevlm, .. } = use_context::<SoundSE>().unwrap();
        
        /* countはReadSignal, set_countはWriteSignal */
        let (count, set_count) = signal(0usize);
    
        let page_num = Novel::Novel3.novel_page().len();
        
    view! {
        <LoadSounds sound_refs = sound_ref />
        
        <Show when={move || count.get() > 0}>
            <button
                class="button left"
                on:click=
                    move |_| {
                        play_sound!{pageflip_ref, sevlm};
                        set_count.update(|c| *c = c.saturating_sub(1))
                    }
            >
                "prev"
            </button>
        </Show>
        <Show when={move || count.get() + 1 < page_num}>
            <button
                class="button right"
                on:click=
                    move |_| {
                        play_sound!{pageflip_ref, sevlm};
                        set_count.update(|c| *c += 1);
                    }
            >
                "next"
            </button>
        </Show>
        
        <div class="novelbg">
            <div class="inner-bg">
                <div class="inner">
                    <h1>"『ペンギンgaku園 ---リストバンド戦争---』"</h1>
                    <Show
                        when= move || NovelImg::Novel3.nimgpath(count.get()).is_some()
                        fallback=|| ()
                    >{
                        view!{
                            <img
                                class="illust"
                                src = move || NovelImg::Novel3.nimgpath(count.get())
                            />
                        }
                    }
                    </Show>

                    <p class="novel">{ move || get_message(Novel::Novel3 , count.get()) }</p>

                </div>
            </div>
        </div>
    }
}
