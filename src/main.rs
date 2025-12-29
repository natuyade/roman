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
        animation: rainbow-scroll 5s linear infinite;
        background-attachment: fixed;
        cursor: url('/image/icon.ico') 0 0, crosshair;
    }
    nav { margin-bottom: 20px; }
    nav a { margin-right: 10px; color: white; }
    "
}

fn get_message(count: i32) -> String {
    match count {
        0 => text_data::MAIN_TEXT[0].to_string(),
        1 => text_data::MAIN_TEXT[1].to_string(),
        2 => text_data::MAIN_TEXT[2].to_string(),
        3 => text_data::MAIN_TEXT[3].to_string(),
        4 => text_data::MAIN_TEXT[4].to_string(),
        5 => text_data::MAIN_TEXT[5].to_string(),
        _ => format!("Clicked {} times!", count),
    }
}

fn get_image(count: i32) -> &'static str {
    match count {
        0 => "/image/temmie.webp",
        1 => "/image/test.webp",
        _ => "/image/default.webp",
    }
}

// ホームページ
#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = signal(0);

    let plus_click = move |_| {
        let next = count.get() + 1;
        set_count.set(if next > 4 { 0 } else { next });
    };

    let minus_click = move |_| {
        let next = count.get() - 1;
        set_count.set(if next < -4 { 0 } else { next });
    };

    view! {
        <div>
            <p>{ move || get_message(count.get()) }</p>
            <p>
                <img
                    src=move || get_image(count.get())
                    alt="dynamic-image"
                    style="max-width: 200px; height: auto;"
                />
            </p>
            <button on:click=plus_click>"Plus"</button>
            <button on:click=minus_click>"Minus"</button>
            <p><A href="/dice">"Go to Dice Page"</A></p>
        </div>
    }
}

#[component]
fn Menu() -> impl IntoView {
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
                width: 50vw;
                height: 100vh;
                background: transparent;
                border: none;
            }

            .left {
                left: 0;
            }

            .right {
                right: 0;
            }
            "#}
        </style>

        <div>
            <h1>"『平凡な生活』"</h1>
            <p style="white-space: pre-line;">{ move || get_message(count.get()) }</p>

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
    let (count, _) = signal(0);
    view! {
        <style>{ global_style()}
            r#"
            nav {
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                z-index: 9999;
            }"#
        </style>
        <Router>
            <Show when=move || {count.get() == 0}>
            <nav>
                <A href="/">"Home"</A>
                <A href="/menu">"Menu"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/menu") view=Menu/>
                </Routes>
            </main>
            </Show>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
