// 共通スタイル
pub fn global_style() -> &'static str {
    "
    @font-face {
        font-family: 'Unifont';
        src: url('/image/unifont-17.0.03.otf') format('opentype');
        font-weight: normal;
        font-style: normal;
        font-display: swap;
    }
    
    html, body {
        margin: 0;
        padding: 0;
        background: #16080D;
        /* 背景を固定 */
        background-attachment: fixed;
        cursor: url('/image/default.webp') 0 0, crosshair;
    }
    
    .novelbg {
        background-image: url('/image/penguin.webp');
        background-attachment: fixed;
        background-size: cover;
    }

    nav {
        background-color: rgba(92, 38, 92, 1);
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        z-index: 9999;
    }
    nav a {
        margin-right: 10px;
        color: rgba(248, 191, 33, 1);
    }

    .center_bg {
        margin: 0 auto;
        background: #ffffffff;
        width: 660px;
        height: 100vh;
    }
    
    .inner {
        max-width: 650px;
        margin: 0 auto;
        width: 650px;
        height: auto;
    }

    .novel {
        height: auto;
    }
    
    .illust {
        width: 100vw;
        max-width: 650px;
        height: auto;
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
