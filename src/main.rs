use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::globalcss::global_style;
use crate::p2r_menu::p2r_menu;
use crate::homepage::HomePage;
use crate::novel_list::NovelPageList;
use crate::novels::novel_1::NovelPage1;
use crate::novels::novel_2::NovelPage2;
use crate::novels::test_1::Test1;

mod novels;

mod globalcss;
mod p2r_menu;
mod homepage;
mod nonsense;
mod novel_list;
mod page_counter;
mod text_data;
mod menu_icon;

#[allow(non_snake_case)]
// ルートApp
#[component]
fn App() -> impl IntoView {
    view! {
        <style>{ global_style() }</style>
        <Router>
                { p2r_menu() }
            <div>
                <main>
                    <Routes fallback=|| "Page not found.">
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/list") view=NovelPageList/>
                        <Route path=path!("/novel_1") view=NovelPage1/>
                        <Route path=path!("/novel_2") view=NovelPage2/>
                        <Route path=path!("/test_1") view=Test1/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
