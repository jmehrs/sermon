use leptos::*;


#[component]
pub fn GeneralSettings(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col w-full space-y-4">
            <div>
                <p class="text-2xl">"General Settings"</p>
                <p class="text-base border-b-2">"Here be Warnings?"</p>
            </div>
        </div>
    }
}
