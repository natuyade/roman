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

    html, body {
        margin: 0;
        padding: 0;
        background: #16080D;
        /* 背景を固定 */
        background-attachment: fixed;
        cursor: url('assets/images/cursorpg.webp') 0 0, auto;
    }

    a {
        cursor: url('assets/images/cursorpg.webp') 0 0, pointer;
    }
    button {
        cursor: url('assets/images/cursorpg.webp') 0 0, pointer;
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
        z-index: 9998;
        display: flex;
            align-items: center;
            flex-direction: column;
        background-color: rgba(92, 38, 92, 1);
        border-bottom: 2px solid #000000;
    }
    nav a {
        padding: 16px;
        color: rgba(248, 191, 33, 1);
        text-decoration: none;
    }
    nav a:hover {
        opacity: 0.8;
    }
    
    .settings {
        position: fixed;
        width: 100vw;
        height: 100vh;
        background-color: #124124;
        z-index: 9997;
        display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
    }
    .settings_icon {
        position: fixed;
        top: 0;
        left: 0;
        width: 64px;
        height: 64px;
        z-index: 9999;
    }

    .sound_btn {
        z-index: 9998;
    }
    .volume_value {
        color: white;
        z-index: 9998;
    }

    .serange {
        /* defaultのappearanceを削除 */
        appearance: none;
        /* focusされた際のoutlineを削除 */
        outline: none;
        /* 操作中のズーム,スクロールを無効 */
        touch-action: none;
        width: 25%;
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
    "
}
