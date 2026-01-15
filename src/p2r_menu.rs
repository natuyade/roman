use leptos::html::Canvas;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::menu_icon::draw_menu_icon;

#[component]
pub fn p2r_menu() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    let (checked, set_checked) = signal(true);
    let (animate, set_animate) = signal(false);
    
    /* 
     * menuの項目が増える可能性が十分にあるため,
     * 変数化しon:clickに割り当てをしている。
     */
    let close_menu = move |_| set_checked.set(true);
    
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
            }
        }
    });
    view! {
            <div class="manuwrap">
                <canvas
                    class="menu-icon"
                    class:menu-anim={move || animate.get()}
                    node_ref=canvas_ref width="320" height="320"
                    on:click={move |_| {
                        set_animate.set(true);
                        set_checked.update(|c| *c = !*c);
                    }}
                    on:animationend=move |_| set_animate.set(false)
                ></canvas>
                <Show when={move || !checked.get()}>
                    <div>
                        <nav class="menu">
                            <A href="/"
                                on:click=close_menu
                            >"HOME"</A>
                            <A href="/list"
                            on:click=close_menu
                            >"目次"</A>
                            <A href="/test_1"
                            on:click=close_menu
                            >"test"</A>
                        </nav>
                    </div>
                </Show>
            </div>
    }
}
