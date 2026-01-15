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
            0%{
                scale: 90%
            }
            46%{
                scale: 100%
            }
            100%{
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
            top: 128px;
            color: white;
        }
        .icon {
            width: 64px;
            height: 64px;
        }
        
        .splash {
            position: absolute;
                top: 256px;
            font-family: 'Unifont';
            font-size: 32px;
            color: Yellow;
            text-shadow: 0 0 12px #838939;
            animation-name: splash;
            animation-duration: 4s;
            animation-iteration-count: infinite;
        }
        
        .schedule-box {
            position: absolute;
                top: 480px;
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
            text-align: center;
        }
        
        .schedule-img {
            width: 240px;
        }
        .schedule-img:hover {
            opacity: 0.8;
        }
        
        "#
        </style>
            <div class="wrapper">
                <div class="title">
                    <img class="icon" src="assets/images/p2r_logo_wh.webp"></img>
                    <h1>"創作小説"</h1>
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