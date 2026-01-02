use leptos::prelude::*;

use crate::page_counter::{Novel, NovelImg, get_message};

// 小説ページ
#[component]
pub fn Novel_1() -> impl IntoView {
    let (count, set_count) = signal(0usize);

    let plus_click = move |_| {
        let next = count.get() + 1;
        set_count.set(if next > 5 { 0 } else { next });
    };

    let minus_click = move |_| {
        let next = count.get().saturating_sub(1);
        set_count.set(next);
    };

    view! {
        <style>

            {r#"

            .button {
                position: fixed;
                top: 0;
                height: 100vh;
                background: transparent;
                color: transparent;
                border: none;
            }
            
            /* hoverで触れている時だけ可視化 */
            .button:hover {
                background-color: rgba(0,0,0,0.2);
                color: rgba(72, 72, 72, 1);
                transition:
                    background-color 0.8s,
                    color 0.8s;
            }

            .left {
                left: 0;
                width: 25vw;
            }

            .right {
                right: 0;
                width: 25vw;
            }

            "#}

        </style>

        <div>
            <h1>"『平凡な生活』"</h1>
            <img
                src = move || NovelImg::Novel1.path(count.get()).to_str()
                style = "
                width: 650px;
                "
            />
            <p style="white-space: pre-line;">{ move || get_message(Novel::Novel1 , count.get()) }</p>

            // count > 0 のときだけ「前」を表示
            <Show when=move || {count.get() > 0}>
                <button class="button left" on:click=minus_click>"前"</button>
            </Show>

            // count < 5 のときだけ「次」を表示
            <Show when=move || {count.get() < 5}>
                <button class="button right" on:click=plus_click>"次"</button>
            </Show>

        </div>
    }
}
