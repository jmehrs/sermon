mod breadcrumbs;
mod menu;

pub use breadcrumbs::*;
use leptos::*;
pub use menu::*;

fn middleout(cx: Scope, nav_group: Vec<NavGroup>) -> impl IntoView {
    let len = nav_group.len();
    let middle = len / 2;
    if len % 2 == 0 {
        let left = &nav_group[..middle];
        let right = &nav_group[middle..];
        view! { cx,
            <div class="flex flex-row w-1/2 items-center">
            {left
                .into_iter()
                .map(|group| view! {cx, <div class="flex items-center">{(group.children)(cx).into_view(cx)}</div>})
                .collect_view(cx)}
            </div>
            <div class="flex flex-row justify-end w-1/2 items-center">
            {right
                .into_iter()
                .map(|group| view! {cx, <div class="flex items-center">{(group.children)(cx).into_view(cx)}</div>})
                .collect_view(cx)}
            </div>
        }
    } else {
        let left = &nav_group[..middle];
        let right = &nav_group[middle + 1..];
        let mid = &[&nav_group[middle]];
        view! { cx,
            <div class="flex flex-row w-1/2 items-center">
            {left
                .into_iter()
                .map(|group| view! {cx, <div class="flex items-center">{(group.children)(cx).into_view(cx)}</div>})
                .collect_view(cx)}
            </div>
            <div class="flex ml-auto mr-auto items-center">
            {mid
                .into_iter()
                .map(|group| view! {cx, <div class="flex items-center">{(group.children)(cx).into_view(cx)}</div>})
                .collect_view(cx)}
            </div>
            <div class="flex flex-row justify-end w-1/2 items-center">
            {right
                .into_iter()
                .map(|group| view! {cx, <div class="flex items-center">{(group.children)(cx).into_view(cx)}</div>})
                .collect_view(cx)}
            </div>
        }
    }
}

pub enum GroupArrangement {
    MiddleOut,
}

impl GroupArrangement {
    fn strategy(&self, cx: Scope, nav_group: Vec<NavGroup>) -> impl IntoView {
        match *self {
            GroupArrangement::MiddleOut => middleout(cx, nav_group),
        }
    }
}

#[slot]
pub struct NavGroup {
    children: ChildrenFn,
}

#[component]
pub fn Navbar(
    cx: Scope,
    arrangement: GroupArrangement,
    #[prop(default=vec![])] nav_group: Vec<NavGroup>,
) -> impl IntoView {
    view! { cx,
        <header class="sticky top-0 z-50 w-full backdrop-blur border-b">
            <div class="py-4 px-4">
                <div class="relative flex items-center">
                    {arrangement.strategy(cx, nav_group)}
                </div>
            </div>
        </header>
    }
}
