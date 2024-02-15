use leptos::*;

#[component]
pub fn CtrlflowV1() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value.get() & 1 == 1;

    view! {
        <div class="flex gap-2 items-center">
            <p>{move || if is_odd() { "Odd" } else { "Even" }}</p>
            <button
                class="hover:bg-red-400 rounded-md bg-blue-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                on:click=move |_| set_value.update(|v| *v += 1)
            >
                "Increment"
            </button>
        </div>
    }
}

#[component]
pub fn CtrlflowV2() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let message = move || (value.get() & 1 == 1).then(|| "Odd");

    view! {
        <div class="flex gap-2 items-center">
            <p>{message}</p>
            <button
                class="hover:bg-red-400 rounded-md bg-blue-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                on:click=move |_| set_value.update(|v| *v += 1)
            >
                "Increment"
            </button>
        </div>
    }
}

#[component]
pub fn CtrlflowV3() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value.get() & 1 == 1;

    view! {
        <div class="flex gap-2 items-center">
            <Show when=is_odd fallback=|| view! { <p>"Even"</p> }>
                <p>"Odd"</p>
            </Show>

            <button
                class="hover:bg-red-400 rounded-md bg-blue-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                on:click=move |_| set_value.update(|v| *v += 1)
            >
                "Increment"
            </button>
        </div>
    }
}

#[component]
pub fn CtrlflowV4() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value.get() & 1 == 1;

    view! {
        <div class="flex gap-2 items-center">
            <button
                class="hover:bg-red-400 rounded-md bg-blue-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                on:click=move |_| set_value.update(|v| *v += 1)
            >
                "Increment"
            </button>

            {move || match is_odd() {
                true if value.get() == 1 => view! { <pre>"One"</pre> }.into_view(),
                false if value.get() == 2 => view! { <p>"Two"</p> }.into_view(),
                _ => view! { <span>{value.get()}</span> }.into_view(),
            }}

        </div>
    }
}
