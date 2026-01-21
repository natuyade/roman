use leptos::prelude::*;

use crate::page_counter::{Novel, NovelImg, get_message};

// 小説ページ
#[component]
pub fn novel_page_3() -> impl IntoView {
    /* countはReadSignal, set_countはWriteSignal */
    let (count, set_count) = signal(0usize);

    let page_num = Novel::Novel3.novel_page().len();

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
            <div class="inner-bg">
                <div class="inner">
                    <h1>"『毒チワワとコンビニ店員~のどかな街のマスコット~』"</h1>
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
        // count > 0 のときだけ「前」を表示
        <Show when={move || count.get() > 0}>
            <button class="button left" on:click=minus_click>"prev"</button>
        </Show>
        // count < pages のときだけ「次」を表示
        <Show when={move || count.get() + 1 < page_num}>
        <button class="button right" on:click=plus_click>"next"</button>
        </Show>
    }
}
