use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

mod text_data;

// 共通スタイル
fn global_style() -> &'static str {
    "
    html, body {
        margin: 0;
        padding: 0;
        min-height: 100vh;
        background: #bfd9daff;
        background-size: 100% 100%;
        background-attachment: fixed;
        cursor: url('/image/default.webp') 0 0, crosshair;
    }
    nav { margin-bottom: 20px; }
    nav a { margin-right: 10px; color: white; }
    "
}

//Copyの内部処理にCloneが自動で付与されるようになっているが
//deriveのルール上Copy単体で書いてもCloneは使えない
//Cloneを覚えていないとCopyが使えない上位魔法だと思えばいい。

//ノベルリスト
#[derive(Copy, Clone)]
enum Novel {
    Novel1,
    Novel2,
}

//挿絵リスト
#[derive(Copy, Clone)]
enum NovelImg {
    Novel1,
    Novel2,
}

fn get_message(novel: Novel, count: i32) -> String {
    match novel {
        Novel::Novel1 => match count {
            0 => text_data::NOVEL1[0].to_string(),
            1 => text_data::NOVEL1[1].to_string(),
            2 => text_data::NOVEL1[2].to_string(),
            3 => text_data::NOVEL1[3].to_string(),
            4 => text_data::NOVEL1[4].to_string(),
            5 => text_data::NOVEL1[5].to_string(),
            _ => format!("Not found Clicked {} times!", count),
        },

        Novel::Novel2 => match count {
            0 => text_data::NOVEL2[0].to_string(),
            1 => text_data::NOVEL2[1].to_string(),
            2 => text_data::NOVEL2[2].to_string(),
            3 => text_data::NOVEL2[3].to_string(),
            4 => text_data::NOVEL2[4].to_string(),
            5 => text_data::NOVEL2[5].to_string(),
            _ => format!("Not found Clicked {} times!", count),
        },
    }
}

fn get_image(img: NovelImg, count: i32) -> &'static str {
    match img {
        NovelImg::Novel1 => match count {
            1 => "/image/ouch.webp",
            _ => "",
            //_ => "/image/default.webp",
        },
        NovelImg::Novel2 => match count {
            0 => "/image/temmie.webp",
            _ => "",
            //_ => "/image/default.webp",
        },
    }
}

// ホームページ
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
        </div>
    }
}

// 目次ページ
#[component]
fn List() -> impl IntoView {
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

// 小説ページ
#[component]
fn Novel_1() -> impl IntoView {
    let (count, set_count) = signal(0);

    let plus_click = move |_| {
        let next = count.get() + 1;
        set_count.set(if next > 5 { 0 } else { next });
    };

    let minus_click = move |_| {
        let next = count.get() - 1;
        set_count.set(if next < 0 { 0 } else { next });
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
                src = move || get_image(NovelImg::Novel1, count.get())
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

#[component]
fn Novel_2() -> impl IntoView {
    let (count, set_count) = signal(0);

    let plus_click = move |_| {
        let next = count.get() + 1;
        set_count.set(if next > 5 { 0 } else { next });
    };

    let minus_click = move |_| {
        let next = count.get() - 1;
        set_count.set(if next < 0 { 0 } else { next });
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
            <h1>"『TESTTT』"</h1>
            <img
                src = move || get_image(NovelImg::Novel2, count.get())
                style = "
                width: 650px;
                "
            />
            <p style="white-space: pre-line;">{ move || get_message(Novel::Novel2 , count.get()) }</p>

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

// ルートApp
#[component]
fn App() -> impl IntoView {
    view! {
        <style>{ global_style()}
            r#"
            nav {
                background-color: rgba(92, 38, 92, 1);
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                z-index: 9999;
            }"#
        </style>
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/list">"目次"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/list") view=List/>
                    <Route path=path!("/novel_1") view=Novel_1/>
                    <Route path=path!("/novel_2") view=Novel_2/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
