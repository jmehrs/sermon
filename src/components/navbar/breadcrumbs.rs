use leptos::*;
use leptos_router::*;

#[derive(Clone, PartialEq)]
struct PathParts {
    name: String,
    path: String,
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn parse_path(path: String) -> Vec<PathParts> {
    path.split_terminator("/")
        .skip(1)
        .fold(vec![], |mut acc, name| {
            let path = (&acc)
                .into_iter()
                .map(|p| p.path.to_owned())
                .chain([name.to_owned()])
                .collect::<Vec<String>>()
                .join("/");
            acc.push(PathParts {
                name: capitalize(&name.replace("-", " ")),
                path,
            });
            acc
        })
}

#[component]
pub fn Breadcrumbs(cx: Scope) -> impl IntoView {
    let pathname = use_location(cx).pathname;
    let path = create_memo(cx, move |_| parse_path(pathname()));

    view! { cx,
        <div class="flex flex-row ml-2">
            {move || {
                path()
                    .into_iter()
                    .map(|v| {
                        view! { cx,
                            <span class="flex items-center">
                                <span class="p-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-right" viewBox="0 0 16 16">
                                        <path fill-rule="evenodd" d="M1 8a.5.5 0 0 1 .5-.5h11.793l-3.147-3.146a.5.5 0 0 1 .708-.708l4 4a.5.5 0 0 1 0 .708l-4 4a.5.5 0 0 1-.708-.708L13.293 8.5H1.5A.5.5 0 0 1 1 8z"/>
                                    </svg>
                                </span>
                                <A class="p-2 text-gray-900 rounded-md hover:bg-gray-300" href=v.path>{v.name}</A>
                            </span>
                        }
                    })
                    .collect_view(cx)
            }}
        </div>
    }
}
