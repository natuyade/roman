use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_navigate};

use crate::page_counter::{Novel, NovelImg, get_message};

// 小説ページ
#[component]
pub fn Novel_1() -> impl IntoView {

    let params = use_params_map();

    let pgurl = move || {params
        .read()
        .get("page")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0)};

    /* pcountはReadSignal, countはWriteSignal */
    let (count, set_count) = signal(pgurl);

    let pages = Novel::Novel1.page().len();
    let navigate = use_navigate();

    /* |c|はaddress , *cはその中の実体と思えばいい 
        実際は||で参照したものを*cで実体化 */
    let plus_click = move |_| {
        set_count.update(|c| *c += 1);

        navigate(
            &format!("/novel_1/{}", count.get() + 1),
            Default::default()
        );
    };

    let minus_click = move |_| {
        set_count.update(|c| *c = c.saturating_sub(1));

        navigate(
            &format!("/novel_1/{}", count.get() + 1),
            Default::default()
        );
    };

    view! {
        <div>
            <h1>"『平凡な生活』"</h1>
            <img
                src = move || NovelImg::Novel1.path(count.get())
                style = "
                width: 650px;
                "
            />
            <p style="white-space: pre-line;">{ move || get_message(Novel::Novel1 , count.get()) }</p>

            // count > 0 のときだけ「前」を表示
            <Show when=move || {count.get() > 0}>
                <button class="button left" on:click=minus_click>"前"</button>
            </Show>

            // count < pages のときだけ「次」を表示
            <Show when=move || count.get() + 1 < pages>
                <button class="button right" on:click=plus_click>"次"</button>
            </Show>

        </div>
    }
}
