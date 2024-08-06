use leptos::*;


#[component]
pub fn Home() -> impl IntoView{
    view! {
        <crate::components::navbar::NavBar />
        <section class="hero relative h-screen flex items-center justify-center">
            <div class="text-center">
                <h1 class="font-extrabold text-extra-lg">"Inspiration through stories."</h1>
            </div>
        </section>

        // Recent posts

        <section class="justify-center">
            <div class="text-center">
                <h3 class="font-bold text-main-text text-extra-lg mb-4">"Recent posts"</h3>
            </div>

            // Blog items list

            <crate::components::blog_outline::BlogOutline />

            <div class="text-center m-auto mb-5">
                <a href="#" class="px-4 py-3 bg-red-500 text-primary shadow-sm rounded hover:bg-main-text">"View all posts"</a>
            </div>
            
        </section>
        
    }
}