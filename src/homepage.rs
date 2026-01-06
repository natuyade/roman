use leptos::prelude::*;
use nanorand::{Rng, tls_rng};

use crate::nonsense;


    fn rnd_sp<'a>(NONS: &'a[&'a str]) -> &'a str {
        let mut rnd_gen = tls_rng();
        let splash = rnd_gen.generate_range(0..NONS.len()) as usize;
        /* NONSのsplash(random0..NONS.len())番目を取り出す */
        NONS[splash]
    }

// ホームページ
#[component]
pub fn HomePage() -> impl IntoView {
    let rnd_splash = rnd_sp(&nonsense::NONS);
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
