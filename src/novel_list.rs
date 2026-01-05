use leptos::prelude::*;
use leptos_router::components::A;

// 目次ページ
#[component]
pub fn Novel_list() -> impl IntoView {
    view! {
            <style>
            "
            body{
                /* Flexbox を有効化 */
                display: flex;
                    /* 横方向 中央 */
                    justify-content: center;

                    /* 縦方向 中央 */
                    /* align-items: center; */
            }

                .container{
                    margin-top: 100px;
                    width: 50vw;
                    text-align: left;
                }

                .list{
                    background: orange;/* デバッグのための色付けbg */
                    padding: 10px; 
                }
            "
            </style>
            <div class="container">
                <p class="list">
                    "・"<A href="/novel_1">"『平凡な生活』"</A>
                </p>
                <p class="list">
                    "・"<A href="/novel_2">"『壊れかけの炒飯』"</A>
                </p>
                <p class="list">
                    "・"<A href="/novel_3">"novel_3"</A>
                </p>
            </div>
    }
}