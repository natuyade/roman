use leptos::prelude::*;

use crate::{page_counter::{Novel, NovelImg, get_message}};


// 小説ページ
#[component]
pub fn Novel_2() -> impl IntoView {

    /* countはReadSignal, set_countはWriteSignal */
    let (count, set_count) = signal(0usize);

    let page_num = Novel::Novel2.novel_page().len();
    //let navigate = use_navigate();

    /* |c|はaddress , *cはその中の実体と思えばいい 
        実際は||で参照したものを*cで実体化 */
    let minus_click = move |_| {
        set_count.update(|c| *c = c.saturating_sub(1));
    };

    let plus_click = move |_| {
        set_count.update(|c| *c += 1);
    };


    view! {
    <div class="novelbg">
        <div class="inner">
            <div>
                <h1>"『壊れかけの炒飯』"</h1>
            </div>
                <Show 
                    when= move || NovelImg::Novel2.nimgpath(count.get()).is_some()
                    fallback=|| view!{}
                >{
                    view!{
                        <img
                            class="illust"
                            src = move || NovelImg::Novel2.nimgpath(count.get())
                        />
                    }
                }
                </Show>
            <div class="novel">
                <p style="white-space: pre-line;">{ move || get_message(Novel::Novel2 , count.get()) }</p>

                // count > 0 のときだけ「前」を表示
                <Show when={move || count.get() > 0}>
                    <button class="button left" on:click=minus_click>"prev"</button>

                </Show>

                // count < pages のときだけ「次」を表示
                <Show when={move || count.get() + 1 < page_num}>
                    <button class="button right" on:click=plus_click>"next"</button>
                </Show>

            </div>
        </div>
    </div>
    }
}