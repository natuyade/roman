use leptos::prelude::*;

#[component]
pub fn test_1() -> impl IntoView {
    view! {
        <style>
            "   
                .menu-button {
                    height: 100vh;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }
                
                .menu {
                    display: block;
                }
                
                #toggle:checked ~ .menu {
                    display: none;
                }
            "
        </style>
        <div class="menu-button">
            <input type="checkbox" id="toggle"/>
                <label for="toggle"></label>
            <div class="menu">
                <img src="assets/images/p2r_logo_wh.webp"></img>
            </div>
        </div>
    }
}