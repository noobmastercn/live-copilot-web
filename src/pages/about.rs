use crate::css::CssClass::{AboutH1, AboutP};
use crate::state::GlobalState;
use leptos::{component, use_context, view, IntoView, RwSignal, SignalGet};

#[component]
pub fn About() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <div class="text-center p-10 flex flex-col items-center">
            <h1 class={move || AboutH1.get_css(state.get().theme)}>{"About Me"}</h1>
            <p class={move || AboutP.get_css(state.get().theme)}>{"Here you can learn more about us."}</p>
            <a class="mx-auto my-7" href="https://github.com/noobmastercn">
                <img src="https://github-readme-stats.shadownoob.com/api/top-langs/?username=noobmastercn&layout=compact&langs_count=7&hide=javascript,html,css&bg_color=30,1e3c72,2a5298&title_color=fff&text_color=fff&hide_border=true" />
            </a>
            <a class="mx-auto my-7" href="https://github.com/noobmastercn">
                <img src="https://github-readme-stats.shadownoob.com/api?username=noobmastercn&show_icons=true&bg_color=30,2a5298,1e3c72&title_color=fff&text_color=fff&icon_color=ff8c00&hide_border=true" />
            </a>
        </div>
    }
}
