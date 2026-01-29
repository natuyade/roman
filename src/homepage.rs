use leptos::prelude::*;

use crate::{nonsense, pre_date};
use crate::pre_date::PREDATE;

/* 何か反応(signalの値更新やui,Aなどでの更新)がないと初期値を渡してしまうため
Effectを使っての採用 */
fn rnd_sp(splash_num: &[&str]) -> String {
    let splash = fastrand::usize(0..splash_num.len());
    splash_num[splash].to_string()
}

// ホームページ
#[component]
pub fn HomePage() -> impl IntoView {
    let (splash, set_splash) = signal(rnd_sp(&nonsense::NONS));

    let schedule_date = PREDATE[0];
    let schedule_img = PREDATE[1];

    Effect::new(move || {
        set_splash.set(rnd_sp(&nonsense::NONS));
    });

    view! {
            <div class="schedule-wrapper">
                <div class="title">
                    <h1>"P2R創作小説"</h1>
                </div>
                <div class="splash">
                    <p>{ splash }</p>
                </div>
                <div class="schedule-box">
                    <div class="schedule">
                        <p style="color: yellow">スケジュール</p>
                        <p style="color: white">"playoff"<span style="color: lime">"{ "{schedule_date}" }"</span></p>
                        //file downloadになる形や外部サイトの場合<A>ではなく<a>
                        <a href=schedule_img target="_blank">
                            <img class="schedule-img" src=schedule_img ></img>
                        </a>
                    </div>
                </div>
            </div>
    }
}
