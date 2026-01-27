use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::page_swicther;
use crate::page_counter::{Novel, NovelImg, get_message};

use crate::{
    SoundSE,
    load_sound::{LoadSounds, SoundEffects},
};

// 小説ページ
#[component]
pub fn novel_page_2() -> impl IntoView {
    /* countはReadSignal, set_countはWriteSignal */
    let (count, set_count) = signal(0usize);

    let page_num = Novel::Novel2.novel_page().len();

    /* |c|はaddress , *cはその中の実体と思えばいい
    実際は||で参照したものを*cで実体化 */
    let minus_click = move |_| {
        set_count.update(|c| *c = c.saturating_sub(1));
    };

    let plus_click = move |_| {
        set_count.update(|c| *c += 1);
    };
    
    let SoundSE { sevlm, set_sevlm } = use_context::<SoundSE>().unwrap();
    
    let sound_ref = SoundEffects::new();
    let pageflip_ref = sound_ref.pageflip;

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
        {page_swicther!{count, plus_click, minus_click, page_num}}
    }
}
