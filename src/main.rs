use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

// 共通スタイル
fn global_style() -> &'static str {
    "
    @keyframes rainbow-scroll {
        0% { background-position: 0% 0%; }
        100% { background-position: 0% 100%; }
    }
    html, body {
        margin: 0;
        padding: 0;
        min-height: 100vh;
        background: linear-gradient(
            180deg,
            #ff0000, #ff8000, #ffff00, #00ff00,
            #00ffff, #0000ff, #8000ff, #ff0080, #ff0000
        );
        background-size: 100% 900%;
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
        0 => "Hello, Leptos!".to_string(),
        1 => "The text has changed once!".to_string(),
        2 => "Click again to reset.".to_string(),
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

// ダイス専用ページ
#[component]
fn DicePage() -> impl IntoView {
    let (dice, set_dice) = signal(fastrand::i32(1..=6));

    let roll = move |_| {
        set_dice.set(fastrand::i32(1..=6));
    };

    view! {
        <div>
            <h1>"🎲 Dice Page"</h1>
            <p style="font-size: 4rem;">{ move || dice.get() }</p>
            <button on:click=roll>"Roll Dice"</button>
            <p><A href="/">"Back to Home"</A></p>
        </div>
    }
}

// ルートApp
#[component]
fn App() -> impl IntoView {
    view! {
        <style>{ global_style() }</style>
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/dice">"Dice"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/dice") view=DicePage/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
