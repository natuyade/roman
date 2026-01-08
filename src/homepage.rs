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
        <style>
        r#"
        a[target="_blank"] {
            outline: none;
        }
        
        @keyframes splash {
            46%{
                scale: 110%
            }
            100%{
                scale: 100%
            }
        }

        .title {
            display: flex;
                justify-content: center;
            color: white;
            margin: auto auto;
            margin-top: 128px;
        }
        .icon {
            width: 64px;
            height: 64px;
        }
        
        .splashbox {
        }
        
        .splash {
            display: flex;
                justify-content: center;
            margin: auto auto;
            max-width: 60vw;
            font-family: 'Unifont';
            font-size: 24px;
            color: Yellow;
            text-shadow: 1px 0 6px #838939;
            animation-name: splash;
            animation-duration: 4s;
            animation-iteration-count: infinite;
        }
        
        .schedule_box {
            display: flex;
                justify-content: center;
            margin: auto auto;
            position-top: 100px;
            border: solid;
            border-width: 4px;
            border-color: white;
            background-color: black;
            width: 95vw;
            height: 95vw;
            max-width: 320px;
            max-height: 240px;
        }
        
        .schedule {
            font-family: 'Unifont';
            font-size: 24px;
            color: Yellow;
            text-shadow: 1px 0 6px #838939;
            text-align: center;
        }
        .schedule_img {
            width: 240px;
        }
        "#
        </style>
        <div class="title">
            <img class="icon" src="/image/p2r_logo_wh.webp"></img>
            <h1>"創作小説"</h1>
        </div>
        <div class="splash">
            <p>{splash}</p>
        </div>
        <div class="schedule_box">
            <div class="schedule">
                <p>スケジュール</p>
                //file downloadになる形や外部ソフトの場合<A>ではなく<a>
                <a href="/image/schedule.jpg" target="_blank">
                    <img class="schedule_img" src="/image/schedule.jpg"></img>
                </a>
            </div>
        </div>
    }
}