use leptos::{component, view, IntoView};



#[component]
pub fn Categories() -> impl IntoView{
    view! {
        <section class="justify-center p-40">
            <div class="text-center">
                <h3 class="font-bold text-main-text text-extra-lg mb-4">Categories</h3>
            </div>

            <div class="max-w-6xl mx-auto mb-3">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 p-10">

                    <div class="bg-white shadow-lg rounded-lg overflow-hidden p-10  text-center h-72">
                    <div class="content mt-20">
                        <h2 class="font-bold text-2xl">"RUST"</h2>
                        <span class="mt-8 font-semibold text-md">"Total 2 posts"</span>
                    </div>
                    </div>

                    <div class="bg-white shadow-lg rounded-lg overflow-hidden p-10  text-center h-72">
                        <div class="content mt-20">
                        <h2 class="font-bold text-2xl">"RUST"</h2>
                        <span class="mt-8 font-semibold text-md">"Total 2 posts"</span>
                        </div>
                    </div>

                    <div class="bg-white shadow-lg rounded-lg overflow-hidden p-10  text-center h-72">
                        <div class="content mt-20">
                        <h2 class="font-bold text-2xl">"RUST"</h2>
                        <span class="mt-8 font-semibold text-md">"Total 2 posts"</span>
                        </div>
                    </div>
                
                
                </div>
            </div>

        
            
        </section>
    }
}