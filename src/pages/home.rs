use crate::css::CssClass::{
    FeatureCardContainer, FeatureCardDescription, FeatureCardTitle, FeatureTips, HomeButton,
    HomeContainer, HomeH2, HomeP, Hr,
};
use crate::css::Theme;
use crate::state::GlobalState;
use leptos::{
    component, create_signal, use_context, view, IntoView, ReadSignal, RwSignal, SignalGet,
    SignalUpdate, WriteSignal,
};

#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    let (count, set_count) = create_signal(1);
    view! {
        <div class={move || HomeContainer.get_css(state.get().theme)}>
            <h2 class={move || HomeH2.get_css(state.get().theme)}>"Live Copilot：直播流的终极助手"</h2>
            <p class={move || HomeP.get_css(state.get().theme)}>
                 "Live Copilot 是一款多功能媒体工具，旨在提供包括视频下载、实时内容抓取和翻译等服务。不论是想要无水印下载抖音和TikTok视频，还是需要抓取和分析实时数据，或是寻求多语言翻译以提高内容可及性，Live Copilot 都能满足您的需求。"
            </p>
        <hr class={move || Hr.get_css(state.get().theme)} />
        <ul class="feature-list">
            <FeatureCard title="下载视频".into() description="支持从TikTok、DouYin等平台下载高清无水印视频，方便内容重新分发或个人收藏。".into() />
            <FeatureCard title="抓取评论".into() description="实时抓取直播或视频评论，为内容创作者提供即时的观众反馈和互动数据。".into() />
            <FeatureCard title="获取直播信息".into() description="详细获取直播间信息，包括推流地址、观众统计、实时用户评论、用户入场和礼物信息。".into() />
            <FeatureCard title="实时翻译".into() description="实时获取并翻译主播的语言内容，支持多种语言，帮助内容跨越语言障碍，触达更广泛的全球观众。".into() />
        </ul>

            <button
                class={move || HomeButton.get_css(state.get().theme)}
                on:click=move |_| home_button_on_click(count,set_count,state)
            >
                "立即下载Live Copilot"
            </button>
        </div>
    }
}

/// 增加计数器 + 切换主题
fn home_button_on_click(
    count: ReadSignal<i32>,
    set_count: WriteSignal<i32>,
    state: RwSignal<GlobalState>,
) {
    set_count.update(|count| *count += 1);
    let current_count = count.get();
    if current_count % 2 == 0 {
        state.update(|state| state.theme = Theme::Twitter);
    } else {
        state.update(|state| state.theme = Theme::Ig);
    }
}

#[component]
pub fn FeatureCard(title: String, description: String) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <div
        class={move || FeatureCardContainer.get_css(state.get().theme)}
        on:click=move |_| {web_sys::window().unwrap().alert_with_message("点击了特性卡片");}
        >
            <div class="absolute top-0 left-0 mt-4 mr-4 text-sm text-white bg-black px-2 py-1 rounded bg-transparent">
                <p class ={move || FeatureTips.get_css(state.get().theme)}>"点击查看教程"</p>
            </div>
            <h3 class={move || FeatureCardTitle.get_css(state.get().theme)}>{title}</h3>
            <p class={move || FeatureCardDescription.get_css(state.get().theme)}>{description}</p>
        </div>
    }
}
