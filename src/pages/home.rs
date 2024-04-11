use leptos::{component, create_signal, IntoView, ReadSignal, RwSignal, SignalGet, SignalUpdate, use_context, view, WriteSignal};
use crate::css::CssClass::{HomeButton, HomeContainer, HomeH2, HomeP};
use crate::css::Theme;
use crate::state::GlobalState;

#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    let (count, set_count) = create_signal(1);
    view! {
        <div class={move || HomeContainer.get_css(state.get().theme)}>
            <h2 class={move || HomeH2.get_css(state.get().theme)}>"Welcome to Leptos with Tailwind"</h2>
            <p class={move || HomeP.get_css(state.get().theme)}>"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class={move || HomeButton.get_css(state.get().theme)}
                on:click=move |_| home_button_on_click(count,set_count,state)
            >
                "Something's here | "
                {move || if count.get() == 1 {
                    "Click me!".to_string()
                } else {
                    count.get().to_string()
                }}
                " | Some more text"
            </button>
        <img src="https://www.notion.so/image/https%3A%2F%2Fprod-files-secure.s3.us-west-2.amazonaws.com%2Fe59a7428-887d-4578-8d2c-0b3ff4a78350%2Fe196cc81-f740-4bf4-a2e7-7767fb4803cf%2FUntitled.png?table=block&id=a7f484d2-c783-452d-bf95-338897542617&spaceId=e59a7428-887d-4578-8d2c-0b3ff4a78350&width=2000&userId=18003b6e-3f70-4a92-b6b6-8b2500cee160&cache=v2" alt="Leptos Logo" />
        </div>
    }
}

/// 增加计数器 + 切换主题
fn home_button_on_click(count: ReadSignal<i32>, set_count: WriteSignal<i32>, state: RwSignal<GlobalState>) {
    set_count.update(|count| *count += 1);
    let current_count = count.get();
    if current_count % 2 == 0 {
        state.update(|state| state.theme = Theme::Twitter);
    } else {
        state.update(|state| state.theme = Theme::Ig);
    }
}

