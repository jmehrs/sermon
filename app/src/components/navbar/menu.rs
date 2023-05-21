use leptos::*;
use leptos_router::*;

#[component]
pub fn Menu(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-row">
            <A href="" class="flex items-center p-2 text-gray-900 rounded-md hover:bg-gray-300">
                "Home"
            </A>
            <A href="devices" class="flex items-center p-2 text-gray-900 rounded-md hover:bg-gray-300">
                "Devices"
            </A>
            <A href="services" class="flex items-center p-2 text-gray-900 rounded-md hover:bg-gray-300">
                "Services"
            </A>
            <A href="settings" class="flex items-center p-2 text-gray-900 rounded-md hover:bg-gray-300">
                "Settings"
            </A>
        </div>
    }
}
