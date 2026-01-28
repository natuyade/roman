// 共通スタイル
pub fn global_style() -> &'static str {
    "
    @font-face {
        font-family: 'Unifont';
        src: url('assets/fonts/unifont-17.0.03.otf') format('opentype');
        font-weight: normal;
        font-style: normal;
        font-display: swap;
    }

    @keyframes toggle-menu {
        0%{
            top: 4px;
        },
        100%{
            top: 0;
        }
    }
    @keyframes toggle-setting {
        0%{
            top: 4px;
            transform: rotateY(200deg);
        },
        100%{
            top: 0;
            transform: rotateY(360deg);
        }
    }
    @keyframes toggle-menu-tab {
        0%{
            transform: translateX(100%);
        },
        100%{
            transform: translateX(0%);
        }
    }
    @keyframes settingstab-anim-open {
        0%{
            scale: 0%;
        }
        100%{
            scale: 100%;
        }
    }
    @keyframes settingstab-anim-close {
        0%{
            scale: 100%;
        }
        100%{
            scale: 0%;
        }
    }

    html, body {
        margin: 0;
        padding: 0;
        width: 100%;
        height: 100%;
        background: #16080D;
        /* 背景を固定 */
        background-attachment: fixed;
        cursor: url('assets/images/cursorpg.webp') 0 0, auto;
        user-select: none;
    }

    a {
        cursor: url('assets/images/cursorpg.webp') 0 0, pointer;
    }
    button {
        cursor: url('assets/images/cursorpg.webp') 0 0, pointer;
    }

    .menuicons {
        width:100%;
    }

    .menu-icon {
        position: fixed;
            top: 0;
            right: 0;
        width: 48px;
        height: 48px;
        z-index: 9999;
    }
    .menu-icon:hover {
        opacity: 0.8;
    }
    .menu-anim {
        animation-name: toggle-menu;
        animation-duration: 0.2s;
    }

    nav {
        position: fixed;
        top: 0;
        width: 100%;
        z-index: 9997;
    }
    ul {
        list-style: none;
        margin: 0;
        padding: 0;
    }
    li {
        position: relative;
        width: 100vw;
        height: 60px;
        background-color: rgb(92, 38, 92);
    }
    li:hover {
        background-color: rgb(73 38 73);
    }
    .menu-tab-border {
        box-sizing: border-box;
        bottom: 0;
        width: 100%;
        height: 60px;
        border-bottom: 3px double #242424;
        display: table;
    }
    .menu-a {
        display: table-cell;
        vertical-align: middle;
        text-align: center;
        width: 100%;
        color: rgba(248, 191, 33, 1);
        text-decoration: none;
    }
    .li-anim1 {
        animation-name: toggle-menu-tab;
        animation-duration: 0.4s;
    }
    .li-anim2 {
        animation-name: toggle-menu-tab;
        animation-duration: 0.6s;
    }
    .li-anim3 {
        animation-name: toggle-menu-tab;
        animation-duration: 0.8s;
    }

    .settings-wrapper {
        position: fixed;
        z-index: 9998;
    }
    .stng-container {
        position: relative;
        width: 100vw;
        height: 100vh;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .stng-bg {
        width: 100%;
        height: 100%;
        background-color: rgba(0,0,0,0.7);
    }
    .close-button {
        position: absolute;
        top: 0;
        right: 0;
        width: 48px;
        height: 48px;
    }
    .close-button:hover {
        opacity: 0.8;
    }

    .settings-icon {
        position: fixed;
        top: 0;
        right: 3rem;
        width: 48px;
        height: 48px;
        z-index: 9999;
    }
    .settings-icon:hover {
        opacity: 0.8;
    }
    .setting-anim {
        animation-name: toggle-setting;
        animation-duration: 0.2s;
    }
    .settings {
        position: absolute;
        width: 80%;
        height: 64%;
        max-width: 720px;
        max-height: 680px;
        background-color: #bca4ba;
        border-radius: 16px;
    }
    .settings-tab {
        position: absolute;
        width: 100%;
        height: 48px;
        background-color: #9426d9;
        border-radius: 16px 16px 0 0;
        display: flex;
            justify-content: center;
            align-items: center;
    }

    .settings-tab-anim-open {
        animation-name: settingstab-anim-open;
        animation-duration: 0.15s;
    }
    .settings-tab-anim-close {
        animation-name: settingstab-anim-close;
        animation-duration: 0.15s;
    }

    .settings-text {
        margin: 0;
        font-size: 1.5em;
        color: #f3f0f4;
    }

    .sounds-stng {
        width: 100%;
        height: 100%;
    display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;

    }

    .sound-btn {
    }
    .volume-value {
        color: white;
    }

    .serange-wrapper {
        height: 28px;
    }

    .serange {
        /* defaultのappearanceを削除 */
        appearance: none;
        /* focusされた際のoutlineを削除 */
        outline: none;
        /* 操作中のズーム,スクロールを無効 */
        touch-action: none;
        width: 70vw;
        max-width: 256px;
        background: #cefdd1;
        height: 4px;
        border-radius: 8px;
        cursor: url('assets/images/cursorpg.webp') 0 0, pointer;
    }

    /* WebKit 系 */
    .serange::-webkit-slider-thumb {
        -webkit-appearance: none;
        height: 22px;
        width: 22px;
        background-color: white;
        border-radius: 50%;
        border: 2px solid #727272;
    }
    .serange::-webkit-slider-thumb:hover {
        background-color: #ebdfec;
    }
    .serange:active::-webkit-slider-thumb {
        background-color: #afb0b1;
    }
    .serange:focus::-webkit-slider-thumb {
        background-color: #afb0b1;
    }

    /* Gecko 系 */
    .serange::-moz-range-thumb {
        /* border-boxでpaddingとborderがwidth,height)に含まれる */
        box-sizing: border-box;
        /* borderが初期でついているため消去 */
        border: none;
        height: 22px;
        width: 22px;
        background-color: #white;
        border-radius: 50%;
        border: 2px solid #727272;
    }
    .serange::-moz-range-thumb:hover {
        background-color: #ebdfec;
    }
    .serange:active::-moz-range-thumb {
        background-color: #afb0b1;
    }
    .serange:focus::-moz-range-thumb {
        background-color: #afb0b1;
    }

    .novelbg {
        background-image: url('assets/images/novelbg.webp');
        background-attachment: fixed;
        background-size: cover;
    }

    .inner-bg {
        position: relative;
            top: 0;
            margin: 0 auto;
        background: #d6d0bd;
        width: 100vw;
        height: 100vh;
        max-width: 720px;
        overflow-y: auto;
    }

    .inner {
        position: absolute;
        display: flex;
            flex-direction: column;
        padding: 10px;
    }

    .novel {
        color: #491e04;
        text-shadow: 1px 1px 1px #c6bb9f;
        white-space: pre-line;
    }

    .illust {
        width: 100%;
        max-width: 700px;
        border: solid;
        border-width: 1px;
    }

    .button {
        position: fixed;
        top: 0;
        height: 100vh;
        border: none;
        background: transparent;
        color: transparent;
        cursor: pointer;
        transition: background-color 0.8s, color 0.8s;
        cursor: url('assets/images/cursorpg.webp') 0 0, pointer;
    }

    /* hoverで触れている時だけ可視化 */
    .button:hover {
        background-color: rgba(0,0,0,0.1);
        color: rgba(72, 72, 72, 0.8);
    }

    .left {
        left: 0;
        width: 24vw;
    }
    .right {
        right: 0;
        width: 24vw;
    }

    /* homepage */

    a[target=_blank] {
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

    .schedule-wrapper {
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
            top: 360px;
        border: solid;
        border-width: 4px;
        border-color: white;
        background-color: black;
        width: 95vw;
        height: 280px;
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

    /* novellist */
    .text-box-pos {
        /* Flexbox を有効化 */
        display: flex;
            /* 横方向 中央 */
            justify-content: center;
            /* 縦方向 中央 */
            align-items: center;
        height: 100vh;
    }

    .text-box {
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        padding: 0px;
        border: solid;
        border-width: 4px;
        border-color: white;
        background-color: black;
        width: 50vw;
        height: 70vh;
        max-width: 256px;
        max-height: 400px;
    }

    .text-box p {
        margin: 0;
    }
    .text-box .p-margin {
        margin-top: 16px;
    }

    .novel-link {
        font-family: 'Unifont';
        color: Yellow;
        text-decoration: none;
    }
    .novel-link:hover {
        color: orange;
    }
    .list-subtitle {
        text-align: center;
        width: 100%;
        font-size: 12px;
        color: lightgreen;
    }
    "
}
