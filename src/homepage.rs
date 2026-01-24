use leptos::prelude::*;

use crate::nonsense;

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
                        <p>スケジュール</p>
                        //file downloadになる形や外部サイトの場合<A>ではなく<a>
                        <a href="/assets/images/schedule.jpg" target="_blank">
                            <img class="schedule-img" src="/assets/images/schedule.jpg"></img>
                        </a>
                    </div>
                </div>
            </div>
    }
}
