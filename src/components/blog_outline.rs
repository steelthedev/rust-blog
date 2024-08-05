use leptos::{component, view, IntoView};



#[component]
pub fn BlogOutline() -> impl IntoView{
    view! {
        <div class="max-w-4xl mx-auto p-6 mb-3">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">


        //blog item
            <div class="bg-white shadow-lg rounded-lg overflow-hidden">
                <img class="w-full h-48 object-cover" src="https://avatars.githubusercontent.com/u/65210097?v=4" alt="Image 1" />
                <div class="p-6">
                    <div class="text-gray-600 text-sm mb-2 flex items-center">
                        <span>05 Dec, 2021</span>
                        <span class="mx-2">"—"</span>
                        <span>02 min read</span>
                    </div>
                    <h2 class="text-lg font-semibold text-main-text mb-3"> The AGI hype train is running out of steam</h2>
                    <p class="text-gray-600"> The AGI hype train has hit some heavy traffic. While futurists and fundraisers used to make bullish predictions about artificial general intelligence...</p>
                </div>
            </div>

            // blog item
            <div class="bg-white shadow-lg rounded-lg overflow-hidden">
                <img class="w-full h-48 object-cover" src="https://avatars.githubusercontent.com/u/65210097?v=4" alt="Image 2" />
                <div class="p-6">
                    <div class="text-gray-600 text-sm mb-2 flex items-center">
                        <span>17 Nov, 2021</span>
                        <span class="mx-2">"-"</span>
                        <span>02 min read</span>
                    </div>
                    <h2 class="text-lg font-semibold text-main-text mb-3">Creating an object that travels at 1% the speed of light?</h2>
                    <p class="text-gray-600">Light is fast. In fact, it is the fastest thing that exists, and a law of the universe is that nothing can move faster than light. Light travels...</p>
                </div>
            </div>
        

           
        </div>
    </div>

    }
}