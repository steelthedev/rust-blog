use leptos::*;



#[component]
pub fn Footer() -> impl IntoView{
    view! {
        <footer class="justify-center">
        <div class="md:flex justify-between m-auto  max-w-4xl text-main-text p-8">
            <div>
                <h6 class="font-semibold text-lg"> "©2024 theayomiposi. All rights reserved."</h6>
            </div>
            <div class="footer-links sm:m-2">
                <a href="#" class="footer-link text-lg"> "Privacy policy" </a>
            </div>
        </div>
     </footer>
    }
}