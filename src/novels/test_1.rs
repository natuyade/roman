use leptos::{prelude::*, task::spawn_local};
use leptos::html::Canvas;

use crate::menu_icon::{draw_menu_icon, draw_menu_icon_false};

#[component]
pub fn test_1() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();
    
    let (checked, set_checked) = signal(true);
    
    // EffectはDOMがレンダされた時に発動
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let html_canvas: web_sys::HtmlCanvasElement = canvas.clone();

            // 2Dコンテキストを取得
            use wasm_bindgen::JsCast;
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
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

                // 描画リフレッシュ
                ctx.clear_rect(0.0, 0.0, 320.0, 320.0);
                
                if checked.get() {
                        ctx.set_fill_style_str("white");
                        ctx.fill_rect(0.0, 0.0, 320.0, 320.0);
                        ctx.set_fill_style_str("black");
                        draw_menu_icon(&ctx);
                        spawn_local(async move {
                            gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
                            
                            spawn_local(async move {
                                gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
                    });
                } else {
                        ctx.set_fill_style_str("green");
                        ctx.fill_rect(0.0, 0.0, 320.0, 320.0);
                        ctx.set_fill_style_str("white");
                        draw_menu_icon(&ctx);
                        spawn_local(async move {
                            gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
                            
                            spawn_local(async move {
                                gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
                    });
                    //draw_menu_icon_false(&ctx);
                }
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
                
                .menu-icon {
                    display: block;
                    width: 32px;
                    height: 32px;
                    background-image: `draw_menu_icon(&ctx)`;
                    background-size: cover;
                }

                #toggle:checked ~ .menu {
                    display: none;
                }
                
                #toggle:checked ~ canvas {
                    rotate: 90deg; 
                }
            "
        </style>
        <div class="menu-button">
                <canvas node_ref=canvas_ref width="320" height="320" on:click=move |_| {
                    set_checked.update(|c| *c = !*c)
                    }></canvas>
            <div class="menu">
                <img src="assets/images/p2r_logo_wh.webp"></img>
            </div>
        </div>
    }
}

