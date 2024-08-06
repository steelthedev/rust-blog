use leptos::*;


#[component]
pub fn NavBar() -> impl IntoView{
    let (visible, set_visible) = create_signal(false);
    let nav_menu_style =move || format!(
        "link-items justify-center m-auto {}",
        if visible.get(){
           "block navbar-transition"
        } else{
            "hidden md:block navbar-transition"
        }
    );
    view! {
        <nav class="navbar bg-transparent p-8 md:flex fixed top-0 left-0 right-0 z-10 transition-all duration-300">
            <div class="flex justify-between">
                <div class="navbar-logo text-main-text m-2 font-bold">
                   <a href="/">
                   <h2 class="nav-logo text-lg"> "A Y O M I P O S I" </h2>
                   </a> 
                </div>
                <button class="md:hidden block text-main-text focus:outline-none"
                on:click=move |_| {
                    set_visible.update(|n| *n = !*n);
                }
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path>
                    </svg>
                </button>
            </div>
        
            <div id="navbarMenu" class={nav_menu_style}>
                <ul class="md:flex">
                    <a href="/">
                        <li class="font-semibold text-main-text m-2">"H O M E"</li>
                    </a>
                
                    // <a href="">
                    //     <li class="font-semibold text-main-text m-2">"A B O U T"</li>
                    // </a>
                
                    <a href="/categories">
                        <li class="font-semibold text-main-text m-2"> "C A T E G O R I E S"</li>
                    </a>
                
        
                    <a href="">
                        <li class="font-semibold text-main-text m-2">"A U T H O R"</li>
                    </a>
                
                </ul>
            </div>

        
            <div class="search-bar justify-end hidden md:block">
                <div class="form-group">
                    <input type="text" placeholder="Search" name="search" class="p-2 rounded-lg" />
                </div>
            </div>
            
        </nav>

    }
}


