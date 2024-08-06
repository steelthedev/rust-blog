use leptos::*;
use leptos_router::*;

use crate::pages::{categories::Categories, home::Home};

#[component]
pub fn App() -> impl IntoView {
  view! {
    <Router>

      <crate::components::navbar::NavBar />
      <main>
        <Routes>
          <Route path="/" view=Home />
          <Route path="/categories" view=Categories />
        </Routes>
      </main>
      <crate::components::footer::Footer />
    </Router>
  }
}


