use leptos::prelude::*;

// ホームページ
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <style>
        "
        .title{
            margin-top: 128px;
            display: flex;
                    justify-content: center;
        }
        .icon{
        }
        "
        </style>
        <div class="title">
            <img class="icon" src="/image/p2r_logo.webp"></img>
            <h1>"創作小説"</h1>
        </div>
    }
}
