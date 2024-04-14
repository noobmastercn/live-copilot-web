#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum Theme {
    #[default]
    Dark,
    Ig,
    Twitter,
}

pub(crate) enum CssClass {
    // 首页的样式
    Container,
    // 导航栏的样式
    Nav,
    NavLink,
    NavThemeSelect,

    // 分割线的样式
    Hr,

    // 主页的样式
    HomeContainer,
    HomeH2,
    HomeP,
    HomeButton,

    // 页脚的样式
    MyFooter,

    // 特性卡片的样式
    FeatureCardContainer,
    FeatureCardTitle,
    FeatureCardDescription,
    FeatureTips,

    // 关于页面的样式
    AboutH1,
    AboutP,

    // 404 页面的样式
    ErrorContainer,
    ErrorPageH1,
    ErrorPageP,
    ErrorPageA,
}

impl CssClass {
    pub fn get_css(&self, theme: Theme) -> &'static str {
        match theme {
            Theme::Dark => {
                match self {
                    CssClass::Container => "flex flex-col min-h-screen bg-gray-900",
                    CssClass::Nav => "bg-gray-800 text-gray-300",
                    CssClass::NavLink => "p-2 nav-link font-bold text-lg hover:bg-gray-700 rounded-lg transition-all duration-300",
                    CssClass::MyFooter => "mt-auto bg-transparent text-gray-500 text-center py-4",
                    CssClass::HomeContainer => "my-0 mx-auto max-w-3xl text-center",
                    CssClass::HomeH2 => "p-6 text-4xl font-bold text-gray-300",
                    CssClass::HomeP => "px-10 pb-10 text-left text-lg text-gray-400",
                    CssClass::HomeButton => "bg-gray-700 hover:bg-gray-600 active:bg-gray-500 px-6 py-3 text-white rounded-full shadow-md transition-all duration-300",
                    CssClass::AboutH1 => "text-3xl font-bold text-gray-300 my-4",
                    CssClass::AboutP => "mt-2 text-lg text-gray-400",
                    CssClass::ErrorContainer => "m-auto text-center p-20 bg-gray-800 rounded-lg shadow-xl",
                    CssClass::ErrorPageH1 => "text-9xl font-bold text-white mb-8",
                    CssClass::ErrorPageP => "text-2xl text-gray-300 mb-8",
                    CssClass::ErrorPageA => "px-8 py-3 text-lg font-semibold rounded-full bg-gray-700 text-white hover:bg-gray-600 transition-colors duration-300",
                    CssClass::NavThemeSelect => "mr-7 w-13 py-2 bg-transparent font-bold text-md hover:bg-gray-700 rounded-lg transition-all duration-300",
                    CssClass::FeatureCardContainer => "relative flex flex-col bg-gray-700 border border-gray-600 rounded-xl shadow-lg m-4 p-8 hover:bg-gray-600 transition duration-300",
                    CssClass::FeatureCardTitle => "text-2xl font-semibold text-white mb-4",
                    CssClass::FeatureCardDescription => "text-gray-400 text-base",
                    CssClass::Hr => "border-t border-gray-700 shadow mb-3 mx-7",
                    CssClass::FeatureTips => "gradient-purple-green text-md font-bold",
                }
            }
            Theme::Ig => {
                match self {
                    CssClass::Container => "flex flex-col min-h-screen bg-gradient-to-r from-indigo-100 via-purple-100 to-pink-100",
                    CssClass::Nav => "bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 text-white",
                    CssClass::NavLink => "p-2 nav-link font-bold text-lg hover:bg-indigo-700 rounded-lg transition-all duration-300",
                    CssClass::MyFooter => "mt-auto bg-transparent text-pink-500 text-center py-4",
                    CssClass::HomeContainer => "my-0 mx-auto max-w-3xl text-center",
                    CssClass::HomeH2 => "p-6 text-4xl font-bold",
                    CssClass::HomeP => "px-10 pb-10 text-left text-lg",
                    CssClass::HomeButton => "bg-gradient-to-r from-pink-500 to-yellow-500 hover:from-pink-600 hover:to-yellow-600 active:to-pink-700 active:from-yellow-500 px-6 py-3 text-white rounded-full shadow-md transform active:scale-95 transition-all duration-300",
                    CssClass::AboutH1 => "text-3xl font-bold my-4",
                    CssClass::AboutP => "mt-2 text-lg",
                    CssClass::ErrorContainer => "m-auto text-center p-20 bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 rounded-lg shadow-xl",
                    CssClass::ErrorPageH1 => "text-9xl font-bold text-white mb-8",
                    CssClass::ErrorPageP => "text-2xl text-white mb-8",
                    CssClass::ErrorPageA => "px-8 py-3 text-lg font-semibold rounded-full bg-white text-indigo-500 hover:bg-gray-200 transition-colors duration-300",
                    CssClass::NavThemeSelect => "mr-7 w-13 py-2 bg-transparent hover:bg-pink-700 font-bold text-md rounded-lg transition-all duration-300",
                    CssClass::FeatureCardContainer => "relative flex flex-col bg-gradient-to-r from-pink-500 to-yellow-500 rounded-2xl shadow-xl m-6 p-8 hover:from-pink-600 hover:to-yellow-600 transition duration-300 ease-in-out",
                    CssClass::FeatureCardTitle => "text-3xl font-bold text-white mb-4",
                    CssClass::FeatureCardDescription => "text-pink-200 text-lg",
                    CssClass::Hr => "border-t border-pink-300 shadow mb-3 mx-7",
                    CssClass::FeatureTips => "gradient-pink-white text-md font-bold",
                }
            }
            Theme::Twitter => {
                match self {
                    CssClass::Container => "flex flex-col min-h-screen bg-blue-50",
                    CssClass::Nav => "bg-blue-600 text-white",
                    CssClass::NavLink => "p-2 nav-link font-medium text-lg hover:bg-blue-700 rounded transition-all duration-300",
                    CssClass::MyFooter => "mt-auto bg-transparent text-blue-500 text-center py-4",
                    CssClass::HomeContainer => "my-0 mx-auto max-w-4xl text-center",
                    CssClass::HomeH2 => "p-6 text-3xl font-bold",
                    CssClass::HomeP => "px-10 pb-10 text-left text-lg",
                    CssClass::HomeButton => "bg-blue-500 hover:bg-blue-600 active:bg-blue-700 px-6 py-3 text-white rounded-full shadow-md transition-all duration-300",
                    CssClass::AboutH1 => "text-4xl font-bold my-4",
                    CssClass::AboutP => "mt-2 text-lg",
                    CssClass::ErrorContainer => "flex flex-col items-center justify-center h-screen bg-blue-50",
                    CssClass::ErrorPageH1 => "text-6xl font-bold text-blue-600 mb-8",
                    CssClass::ErrorPageP => "text-xl text-blue-600 mb-8",
                    CssClass::ErrorPageA => "px-8 py-3 text-lg font-semibold rounded-full bg-white text-blue-500 hover:bg-gray-200 transition-colors duration-300",
                    CssClass::NavThemeSelect => "mr-7 w-13 py-2 bg-transparent font-medium text-md hover:bg-blue-700 rounded transition-all duration-300",
                    CssClass::FeatureCardContainer => "relative flex flex-col bg-blue-500 rounded-2xl shadow-xl m-6 p-8 hover:bg-blue-600 transition duration-300 ease-in-out",
                    CssClass::FeatureCardTitle => "text-3xl font-bold text-white mb-4",
                    CssClass::FeatureCardDescription => "text-blue-200 text-lg",
                    CssClass::Hr => "border-t border-blue-300 shadow mb-3 mx-7",
                    CssClass::FeatureTips => "gradient-pink-lightblue text-md font-bold",
                }
            }
        }
    }
}
