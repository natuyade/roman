use leptos::prelude::*;

// ホームページ
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <style>
        "
        body{
            display: flex;
                    justify-content: center;
        }
        "
        </style>
        <div>
            <p>"P2R"
            <img src="/image/p2r_logo.webp"></img>
            </p>
        </div>
    }
}
