use leptos::*;

#[component]
pub fn EHandlingV1() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());

    view! {
        <label>
            <p>"Number:"</p>
            <input
                class="block round-md border-2 text-gray-900 pl-1 py-1 pr-1"
                type="number"
                on:input=on_input
            />
            <p>"You entered " <strong>{value}</strong></p>
        </label>
    }
}

#[component]
pub fn EHandlingV2() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());

    view! {
        <label>
            <p>"Number:"</p>
            <input
                class="block round-md border-2 text-gray-900 pl-1 py-1 pr-1"
                type="number"
                on:input=on_input
            />
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="text-red-500">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}

                        </ul>
                    </div>
                }
            }>

                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
