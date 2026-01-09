use leptos::prelude::*;
use leptos_router::components::A;

// 目次ページ
#[component]
pub fn novel_page_list() -> impl IntoView {
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
                
                a {
                }
                
                .text_box_pos {
                    margin-top: 100px;
                
                }
                
                .text_box {
                    display: flex;
                        justify-content: center;
                        align-items: center;
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
                
                .novel_link {
                    font-family: 'Unifont';
                    color: Yellow;
                    text-decoration: none;
                }
                .novel_link:hover {
                    color: orange;
                }
            "
            </style>
            <div class="text_box_pos">
                <div class="text_box">
                    <p>
                        <A attr:class="novel_link" href="/novel_1">"『平凡な生活』"</A>
                    </p>
                    <p>
                        <A attr:class="novel_link" href="/novel_2">"『壊れかけの炒飯』"</A>
                    </p>
                    <p>
                        <A attr:class="novel_link" href="/novel_3">"novel_3"</A>
                    </p>
                </div>
            </div>
    }
}