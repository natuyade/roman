use leptos::prelude::*;
use leptos::html::Canvas;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

use crate::menu_icon::draw_menu_icon;

#[component]
pub fn test_1() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    // EffectはDOMがレンダされた時に発動
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let html_canvas: web_sys::HtmlCanvasElement = canvas.clone();

            // 2Dコンテキストを取得
            let ctx = html_canvas
                .get_context("2d")
                /*
                .unwrapを二回実行しているのは
                get_context(&self, context_id: &str) -> Result<Option<::js_sys::Object>, JsValue>
                で,
                    Result=Canvasの呼び出しが成功しているかどうか
                    Option=指定したコンテキスト("2d")が存在しているか
                を処理しているため
                 */
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

                // 描画処理
                draw_menu_icon(&ctx);
        }
    });
    view! {
        <style>
            "
                .menu-button {
                    height: 100vh;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }

                .menu {
                    display: block;
                }

                #toggle:checked ~ .menu {
                    display: none;
                }
            "
        </style>
        <div class="menu-button">
            <input type="checkbox" id="toggle"/>
                <label for="toggle"></label>
            <div class="menu">
                <img src="assets/images/p2r_logo_wh.webp"></img>
            </div>
            <canvas node_ref=canvas_ref width="320" height="320"></canvas>
        </div>
    }
}

