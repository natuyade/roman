use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

use crate::homepage::HomePage;
use crate::novel_list::Novel_list;
use crate::novels::novel_1::Novel_1;
use crate::novels::novel_2::Novel_2;

mod novels;

mod homepage;
mod novel_list;
mod page_counter;
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
                    <Route path=path!("/list") view=Novel_list/>
                    <Route path=path!("/novel_1") view=Novel_1/>
                    <Route path=path!("/novel_1/:page") view=Novel_1/>
                    <Route path=path!("/novel_2") view=Novel_2/>
                    <Route path=path!("/novel_2/:page") view=Novel_2/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
