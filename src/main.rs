use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

use crate::globalcss::global_style;
use crate::homepage::HomePage;
use crate::novel_list::Novel_list;
use crate::novels::novel_1::Novel_1;
use crate::novels::novel_2::Novel_2;

mod novels;

mod globalcss;
mod homepage;
mod novel_list;
mod page_counter;
mod text_data;

// ルートApp
#[component]
fn App() -> impl IntoView {
    view! {
        <style>{ global_style() }</style>
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/list">"目次"</A>
            </nav>
            <main>
                <Routes fallback=|| "Page not found.">
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
