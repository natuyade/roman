use leptos::prelude::*;
use leptos_router::components::A;

// 目次ページ
#[component]
pub fn Novel_list() -> impl IntoView {
    view! {
            <style>
            r#"

            body{
                /* 画面全体を高さにする */
                min-height: 100vh;

                /* Flexbox を有効化 */
                display: flex;

                /* 横方向 中央 */
                justify-content: center;

                /* 縦方向 中央 */
                align-items: center;
        }
                /*
                containerで箱を作り、
                marginをautoにしどちらも均等な余白をつくることで
                中央に整列できる。
                */
                .container{
                    width: 50vw;
                    text-align: left;
                }

                .list{
                    top: 100px;
                    background: orange;/* debug_bg */
                    padding: 10px; 
                }

            "#
            </style>
            <div class="container">
                <p class="list">
                    "・"<A href="/novel_1">"『平凡な生活』"</A>
                </p>
                <p class="list">
                    "・"<A href="/novel_2">"TESTTTT"</A>
                </p>
                <p class="list">
                    "・"<A href="/novel_3">"novel_3"</A>
                </p>
            </div>
    }
}