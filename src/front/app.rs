use crate::front::{navbar::*, settings::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/sermon.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        
        // sets the document title
        <Title text="SerMon UI"/>

        <Router>
            <main class="flex flex-col h-screen">
                <Navbar arrangement=GroupArrangement::MiddleOut>
                    <NavGroup slot>
                        <A href="" class="rounded-full hover:bg-gray-300">
                            <img src="/icon_small.png" />
                        </A>
                    </NavGroup>
                    <NavGroup slot> <Breadcrumbs /> </NavGroup>
                    <NavGroup slot> <Menu /> </NavGroup>
                    <NavGroup slot> "" </NavGroup>
                </Navbar>
                <div class="flex-1">
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
                            <Route path="general" view=move |cx| view! { cx, <GeneralSettings/> }/>
                            <Route path="profile" view=move |cx| view! { cx, <UserProfile/> }/>
                            <Route path="metrics-sharing" view=move |cx| view! { cx, <MetricsSharing/> }/>
                            <Route path="" view=move |cx| view! { cx, <Redirect path="general"/> }/>
                        </Route>
                    </Routes>
                </div>
                <footer class="flex p-4 w-full bottom-0 justify-center items-center">
                    <p class="text-gray-900">"Service Monitor (SerMon) | 🤖"</p>
                </footer>
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


#[derive(Params, PartialEq, Debug)]
struct DeviceParams {
    id: usize,
}

#[component]
fn DeviceProfile(cx: Scope) -> impl IntoView {
    let params = use_params::<DeviceParams>(cx);
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());

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
