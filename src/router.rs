use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct IdParams {
    id: Option<usize>,
}

#[derive(Params, PartialEq)]
struct SearchParams {
    q: Option<String>,
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <p>"Home"</p>
        <div class="flex gap-2">
            <Outlet/>
        </div>
    }
}

#[component]
pub fn InnerHome() -> impl IntoView {
    view! { <p>"Inner Home"</p> }
}

#[component]
pub fn Page() -> impl IntoView {
    let params = use_params::<IdParams>();
    let query_map = use_query_map();

    let id = move || params.with(|p| p.as_ref().map(|v| v.id).unwrap_or_default());
    let q = move || query_map.with(|qm| qm.get("q").cloned());

    view! { <p>"Page id="{id} " q="{q}</p> }
}

#[component]
pub fn PageEmpty() -> impl IntoView {
    view! {
        <p>"Page Empty"</p>
        <input type="text" class="border-2"/>
    }
}

#[component]
pub fn UserHome() -> impl IntoView {
    let navigate = use_navigate();
    view! {
        <div class="flex gap-2">
            <A href="1?q=Foo&f=bar">"/users/1"</A>
            <A href="2">"/users/2"</A>
            <button class="border-2" on:click=move |_| { navigate("/users", Default::default()) }>
                "Navigate to /users"
            </button>
        </div>
        <p>"User Home Page "</p>
        <Outlet/>
    }
}

#[component]
pub fn UserInfo() -> impl IntoView {
    let params = use_params::<IdParams>();
    let query = use_query::<SearchParams>();
    let query_map = use_query_map();

    let id = move || params.with(|p| p.as_ref().map(|v| v.id).unwrap_or_default());
    let q = move || query.with(|q| q.as_ref().map(|v| v.q.clone()).unwrap_or_default());
    let f = move || query_map.with(|qm| qm.get("f").cloned());

    view! { <p>"User Info Page: id=" {id} , "q=" {q} , "f=" {f}</p> }
}

#[component]
pub fn User() -> impl IntoView {
    view! { <p>"User Page "</p> }
}

#[component]
pub fn RouterV1() -> impl IntoView {
    view! {
        <div class="flex-col">
            <div class="mb-8">
                <FormV1/>
            </div>

            <div class="flex gap-2">
                <a href="/">"/"</a>
                <a href="/1">"/1"</a>
                <a href="/1/address">"/1/address"</a>
                <a href="/page">"/page/home"</a>
                <a href="/users">"/users"</a>
                <a href="/not/found">"/not/fount"</a>
            </div>

            <div>
                <Router>
                    <Routes>
                        <Route path="/" view=Home>
                            <Route path="" view=|| view! { <p>"Home Empty..."</p> }/>
                            <HomeInfoRouters/>
                        </Route>

                        <Route
                            path="/page"
                            view=|| {
                                view! {
                                    <p>"Page Home"</p>
                                    <Outlet/>
                                }
                            }
                        >

                            <Route path="" view=PageEmpty/>
                            <Route path=":id" view=Page/>
                        </Route>
                        <Route path="/users" view=UserHome>
                            <Route path=":id" view=UserInfo/>
                            <Route path="" view=User/>
                        </Route>
                        <Route path="/*any" view=|| view! { <h1>"Not Found..."</h1> }/>
                    </Routes>
                </Router>
            </div>
        </div>
    }
}

#[component(transparent)]
fn HomeInfoRouters() -> impl IntoView {
    view! {
        <Route
            path=":id"
            view=|| {
                view! {
                    <p>"Home Id"</p>
                    <Outlet/>
                }
            }
        >

            <Route path="" view=|| view! { <p>"Home Id Page"</p> }/>
            <Route path="address" view=|| view! { <p>"Home Id Address"</p> }/>
        </Route>
    }
}

#[component]
fn FormV1() -> impl IntoView {
    let (id, set_id) = create_signal(0);
    let (q, set_q) = create_signal(String::default());

    view! {
        <div class="flex gap-2">
            <span>"Jump to page:"</span>
            <input
                type="number"
                class="border-2"
                value=id
                on:input=move |ev| {
                    set_id.set(event_target_value(&ev).parse::<i32>().unwrap_or_default())
                }
            />
        </div>

        <div class="flex gap-2">
            <p>"Query: "</p>
            <Form class="flex gap-2" method="GET" action=move || { format!("/page/{}", id.get()) }>
                <input
                    type="search"
                    name="q"
                    class="border-2"
                    value=q
                    on:input=move |ev| { set_q.set(event_target_value(&ev)) }
                />
                <input type="submit" class="border-2"/>
            </Form>
        </div>
    }
}
