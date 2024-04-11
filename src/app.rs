use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::nav_bar::NavBar;
use crate::css::CssClass::{Container, MyFooter};
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::about::About;
use crate::pages::home::Home;
use crate::state::GlobalState;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let state = create_rw_signal(GlobalState::default());
    provide_context(state);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-ssr-template.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            // .into_view()
        }>
             <div class={move || Container.get_css(state.get().theme)}>
                <NavBar/>
                <main class="flex-grow">
                    <Routes>
                        <Route path="/" view=Home ssr=SsrMode::Async />
                        <Route path="/about" view=About ssr=SsrMode::Async />
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}




#[component]
fn Footer() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <footer class={move || MyFooter.get_css(state.get().theme)}>
            <p class="font-bold">{"© 2024 lilpum. All rights reserved."}</p>
        </footer>
    }
}


