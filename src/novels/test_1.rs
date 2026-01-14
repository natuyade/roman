use leptos::prelude::*;
use leptos_router::components::A;
use leptos::html::Canvas;

use crate::menu_icon::draw_menu_icon;

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
                } else {
                        ctx.set_fill_style_str("rgba(92, 38, 92, 1)");
                        ctx.fill_rect(0.0, 0.0, 320.0, 320.0);
                        ctx.set_fill_style_str("rgba(248, 191, 33, 1)");
                        draw_menu_icon(&ctx);
                    //draw_menu_icon_false(&ctx);
                }
        }
    });
    view! {
        <style>
            "
                @keyframes toggle-menu {
                    0%{
                        top: 4px;
                    },
                    100%{
                        top: 0;
                    }
                }
                
                .menuwrap {
                    height: 100vh;
                }
                
                .menu-icon {
                    position: fixed;
                        top: 0;
                        right: 0;
                    width: 48px;
                    height: 48px;
                    animation: toggle-menu 300ms linear forwards;
                    z-index: 9999;
                }
                .menu-icon:hover {
                    opacity: 0.8;
                }
            "
        </style>
        <div class="menuwrap">
                    <Show when={move || !checked.get()}>
                        <canvas class="menu-icon" node_ref=canvas_ref width="320" height="320"
                            on:click=move |_| {
                                set_checked.update(|c| *c = !*c)
                    }></canvas>
                    </Show>
                    <Show when={move || checked.get()}>
                        <canvas class="menu-icon" node_ref=canvas_ref width="320" height="320"
                            on:click=move |_| {
                                set_checked.update(|c| *c = !*c)
                    }></canvas>
                    </Show>
                        <Show when={move || !checked.get()}>
                            <div class="menu">
                                <nav>
                                    <A href="/">"HOME"</A>
                                    <A href="/list">"目次"</A>
                                    <A href="/test_1">"test"</A>
                                </nav>
                            </div>
                        </Show>
        </div>
    }
}

