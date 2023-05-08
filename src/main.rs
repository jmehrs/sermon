use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <header> "Header" </header>
            <nav> "Nav" </nav>
            <main>
                <Routes>
                    <Route path="" view=move |cx| view! { cx, <Home/> }/>
                    <Route path="devices" view=move |cx| view! { cx, <Devices/> }>
                        <Route path=":id" view=move |cx| view! { cx, <DeviceProfile/> }/>
                        <Route path="" view=move |cx| view! { cx, <p> "Select a device for more info" </p> }/>
                    </Route>
                    <Route path="services" view=move |cx| view! { cx, <Services/> }>
                        <Route path="snmp" view=move |cx| view! { cx, <SnmpService/> }/>
                        <Route path="logs" view=move |cx| view! { cx, <LogsService/> }/>
                        <Route path="" view=move |cx| view! { cx, <p> "Select a service for more info" </p> }/>
                    </Route>
                    <Route path="settings" view=move |cx| view! { cx, <Settings/> }>
                        <Route path="profile" view=move |cx| view! { cx, <UserProfile/> }/>
                        <Route path="metrics-sharing" view=move |cx| view! { cx, <MetricsSharing/> }/>
                        <Route path="" view=move |cx| view! { cx, <p> "Select a setting for more info" </p> }/>
                    </Route>
                    <Route path="*" view=move |cx| view! { cx, <p> "/!\\ Page not found /!\\" </p> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2> "Home" </h2>
    }
}

#[component]
fn Devices(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2> "Devices" </h2>
        <Outlet/>  // Insert nested child route here
    }
}

#[component]
fn Services(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2> "Services" </h2>
        <Outlet/>  // Insert nested child route here
    }
}

#[component]
fn Settings(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2> "Settings" </h2>
        <Outlet/>  // Insert nested child route here
    }
}

#[derive(Params, PartialEq, Debug)]
struct DeviceParams {
    id: usize
}

#[component]
fn DeviceProfile(cx: Scope) -> impl IntoView {
    let params = use_params::<DeviceParams>(cx);
    let id = move || { 
        params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default())
    };

    view! { cx,
        <h2> "Device Number" {id} </h2>
    }
}

#[component]
fn SnmpService(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2> "SNMP Service" </h2>
    }
}

#[component]
fn LogsService(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2> "Logs Service" </h2>
    }
}

#[component]
fn UserProfile(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2> "User Profile" </h2>
    }
}

#[component]
fn MetricsSharing(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2> "Web Sharing" </h2>
    }
}
