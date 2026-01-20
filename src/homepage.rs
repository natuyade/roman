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

        /*
         * scaleを110%等にしたい場合
         * 110%にするとページの大きさに干渉しページscaleが意図せず拡張されるため
         * 元のscaleを拡大し対応させる
         */
        @keyframes splash {
            0%{
                transform: translateY(0px);
                scale: 90%
            }
            32%, 40%{
            transform: translateY(-10px);
                scale: 100%
            }
            100%{
                transform: translateY(0px);
                scale: 90%
            }
        }

        .wrapper {
            display: flex;
                justify-content: center;
                align-items: center;
            position: relative;
        }

        .title {
            position: absolute;
                top: 96px;
                color: white;
        }

        .splash {
            position: absolute;
                top: 176px;
            font-family: 'Unifont';
            font-size: 32px;
            color: Yellow;
            text-shadow: 0 0 12px #838939;
            animation-name: splash;
            animation-duration: 5s;
            animation-iteration-count: infinite;
        }

        .schedule-box {
            position: absolute;
                top: 320px;
            border: solid;
            border-width: 4px;
            border-color: white;
            background-color: black;
            width: 95vw;
            height: 240px;
            max-width: 320px;
        }

        .schedule {
            font-family: 'Unifont';
            font-size: 24px;
            color: Yellow;
            text-align: center;
        }

        .schedule-img {
            width: 90vw;
            max-width: 240px;
        }
        .schedule-img:hover {
            opacity: 0.8;
        }

        "#
        </style>
            <div class="wrapper">
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
