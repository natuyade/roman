use leptos::prelude::*;

use crate::page_counter::{Novel, NovelImg, get_message};

#[component]
pub fn novel_page_3() -> impl IntoView {
    let (count, set_count) = signal(0usize);

    let page_num = Novel::Novel3.novel_page().len();

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
        <Show when={move || count.get() > 0}>
            <button class="button left" on:click=minus_click>"prev"</button>
        </Show>
        <Show when={move || count.get() + 1 < page_num}>
        <button class="button right" on:click=plus_click>"next"</button>
        </Show>
    </div>
    }
}
