use leptos::{component, IntoView, RwSignal, SignalGet, SignalUpdate, use_context, view};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};

use crate::css::CssClass::{Nav, NavThemeSelect};
use crate::css::Theme;
use crate::state::GlobalState;

#[component]
pub fn NavBar() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();

    view! {
        <nav class={move || Nav.get_css(state.get().theme)}>
            <ul class="flex justify-between items-center">
                <li class="flex items-center mr-7">
                    <span class="text-xl font-bold italic pl-7">"live-copilot 🤖"</span>
                </li>
                <li>
                    <select class={move || NavThemeSelect.get_css(state.get().theme)}
                        on:change={move |e| update_theme_based_on_selection(state,e)}>
                        <option value="Dark">{"Dark"}</option>
                        <option value="Ig">{"Ig"}</option>
                        <option value="Twitter">{"Tw"}</option>
                    </select>
                </li>
            </ul>
        </nav>
    }
}

/// 切换主题
fn update_theme_based_on_selection(state: RwSignal<GlobalState>, e: Event) {
    let selected_theme = e
        .target()
        .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
        .map(|t| t.value())
        .unwrap_or("Dark".to_string());

    state.update(|state| {
        state.theme = match selected_theme.as_str() {
            "Dark" => Theme::Dark,
            "Ig" => Theme::Ig,
            "Twitter" => Theme::Twitter,
            _ => state.theme,
        }
    });
}
