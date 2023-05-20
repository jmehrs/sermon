mod general_settings;
mod metrics_sharing;
mod user_profile;

use leptos::*;
use leptos_router::*;

pub use general_settings::*;
pub use metrics_sharing::*;
pub use user_profile::*;

#[component]
fn SettingCard(
    cx: Scope,
    link: &'static str,
    name: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! { cx,
        <A class="w-80 h-32 border-2 p-2 rounded-lg" href=link>
            <p class="text-xl">{name}</p>
            <p class="text-base">{description}</p>
        </A>
    }
}

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row h-full w-full">
            <div class="max-w-xs flex flex-col m-4 space-y-4 justify-start justify-items-center">
                <SettingCard link="general" name="General" description="Configure the application"/>
                <SettingCard link="profile" name="Profile" description="Configure your profile"/>
                <SettingCard
                    link="metrics-sharing"
                    name="Metrics sharing"
                    description="Enable/disable sharing of metrics"
                />
            </div>
            <div class="flex w-full m-4 border-2 p-2 rounded-lg">
                <Outlet/>
            </div>
        </div>
    }
}
