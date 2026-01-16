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
        margin: 16px;
        color: rgba(248, 191, 33, 1);
        text-decoration: none;
    }
    nav a:hover {
        opacity: 0.8;
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
