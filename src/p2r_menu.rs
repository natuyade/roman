use leptos::html::Canvas;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::menu_icon::draw_menu_icon;

#[component]
pub fn p2r_menu() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    let (menu, set_menu) = signal(false);
    let (menu_anim, set_menu_anim) = signal(false);

    /*
     * menuの項目が増える可能性が十分にあるため,
     * 変数化しon:clickに割り当てをしている。
     */
    let close_menu = move |_| set_menu.set(false);

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

            if !menu.get() {
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
            <div class="menuwrap">
                <canvas
                    class="menu-icon"
                    class:menu-anim={move || menu_anim.get()}
                    node_ref=canvas_ref width="320" height="320"
                    on:click=move |_| {
                        set_menu_anim.set(true);
                        set_menu.update(|c| *c = !*c);
                    }
                    on:animationend=move |_| set_menu_anim.set(false)
                ></canvas>
                <Show when={move || menu.get()}>
                    <div>
                        <nav>
                            <ul>
                                <li class:li-anim1=move || menu.get()>
                                <div 
                                    class="menu-tab-border"
                                    class:li-anim1=move || menu.get()
                                >
                                    <A attr:class="menu-a" href="/"
                                        on:click=close_menu
                                    >"HOME"</A>
                                </div>
                                </li>
                                <li class:li-anim2=move || menu.get()>
                                <div 
                                    class="menu-tab-border"
                                    class:li-anim2=move || menu.get()
                                >
                                    <A attr:class="menu-a" href="/list"
                                        on:click=close_menu
                                    >"目次"</A>
                                </div>
                                </li>
                                <li class:li-anim3=move || menu.get()>
                                <div
                                    class="menu-tab-border"
                                    class:li-anim3=move || menu.get()
                                >
                                    <A attr:class="menu-a" href="/secret"
                                        on:click=close_menu
                                    >"Coming soon"</A>
                                </div>
                                </li>
                            </ul>
                        </nav>
                    </div>
                </Show>
            </div>
    }
}
