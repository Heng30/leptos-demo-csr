use leptos::*;

#[component]
fn ChildButtonV1(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button class="border-2 text-gray-500" on:click=move |_| setter.update(|v| *v = !*v)>
            "Toggle"
        </button>
    }
}

#[component]
pub fn ParentChildTalkV1() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    view! {
        <p>"Toggle? " {toggled}</p>
        <ChildButtonV1 setter=set_toggled/>
    }
}

#[component]
fn ChildButtonV2(#[prop(into)] on_click: Callback<ev::MouseEvent>) -> impl IntoView {
    view! {
        <button class="border-2 text-gray-500" on:click=move |ev| on_click.call(ev)>
            "Toggle"
        </button>
    }
}

#[component]
pub fn ParentChildTalkV2() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    view! {
        <p>"Toggle? " {toggled}</p>
        <ChildButtonV2 on_click=move |_| set_toggled.update(|v| *v = !*v)/>
    }
}

#[component]
fn ChildButtonV3<F>(on_click: F) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    view! {
        <button class="border-2 text-gray-500" on:click=on_click>
            "Toggle"
        </button>
    }
}

#[component]
pub fn ParentChildTalkV3() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    view! {
        <p>"Toggle? " {toggled}</p>
        <ChildButtonV3 on_click=move |_| set_toggled.update(|v| *v = !*v)/>
    }
}

#[component]
fn ChildButtonV4() -> impl IntoView {
    view! { <button class="border-2 text-gray-500">"Toggle"</button> }
}

#[component]
pub fn ParentChildTalkV4() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    view! {
        <p>"Toggle? " {toggled}</p>

        // This will add on:click event listener to each child element
        <ChildButtonV4 on:click=move |_| set_toggled.update(|v| *v = !*v)/>
    }
}

#[component]
fn ChildButtonV5() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");

    view! {
        <button class="border-2 text-gray-500" on:click=move |_| setter.update(|v| *v = !*v)>
            "Toggle"
        </button>
    }
}

#[component]
pub fn ParentChildTalkV5() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    provide_context(set_toggled);

    view! {
        <p>"Toggle? " {toggled}</p>
        <ChildButtonV5/>
    }
}

#[component]
fn TakesChildrenV1<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}

        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
pub fn ParentChildTalkV6() -> impl IntoView {
    view! {
        <TakesChildrenV1 render_prop=|| {
            view! { <p>"Hi, there"</p> }
        }>"Some text. " <span>"A span"</span></TakesChildrenV1>
    }
}

#[component]
fn TakesChildrenV2(children: Children) -> impl IntoView {
    let children = children().nodes.into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();

    view! { <ul>{children}</ul> }
}

#[component]
pub fn ParentChildTalkV7() -> impl IntoView {
    view! { <TakesChildrenV2>"A" "B" "C"</TakesChildrenV2> }
}
