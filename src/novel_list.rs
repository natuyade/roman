use leptos::prelude::*;
use leptos_router::components::A;

// 目次ページ
#[component]
pub fn novel_page_list() -> impl IntoView {
    view! {
            <style>
            "
                .text-box-pos {
                    position: relative;
                    /* Flexbox を有効化 */
                    display: flex;
                        /* 横方向 中央 */
                        justify-content: center;
                        /* 縦方向 中央 */
                        align-items: center;
                    height: 100vh;
                }
                
                .text-box {
                    display: flex;
                        justify-content: center;
                        flex-direction: column;
                    padding: 0px;
                    text-align: center;
                    border: solid;
                    border-width: 4px;
                    border-color: white;
                    background-color: black;
                    width: 50vw;
                    height: 70vh;
                    max-width: 256px;
                    max-height: 400px;
                }
                
                .novel-link {
                    font-family: 'Unifont';
                    color: Yellow;
                    text-decoration: none;
                }
                .novel-link:hover {
                    color: orange;
                }
            "
            </style>
        <body>
            <div class="text-box-pos">
                <div class="text-box">
                    <p>
                        <A attr:class="novel-link" href="/novel_1">"『平凡な生活』"</A>
                    </p>
                    <p>
                        <A attr:class="novel-link" href="/novel_2">"『壊れかけの炒飯』"</A>
                    </p>
                    <p>
                        <A attr:class="novel-link" href="/novel_3">"novel_3"</A>
                    </p>
                </div>
            </div>
        </body>
    }
}