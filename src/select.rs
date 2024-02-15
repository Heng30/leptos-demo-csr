use leptos::*;

#[component]
pub fn SelectV1() -> impl IntoView {
    let (value, set_value) = create_signal("B".to_string());

    view! {
        <div class="flex-clo gap-2 items-center mt-2">
            <select
                class="block round-md border-2 text-gray-900 pl-1 py-1 pr-1"
                on:change=move |ev| {
                    set_value.set(event_target_value(&ev));
                }
            >

                <SelectOptionV1 is="A" value/>
                <SelectOptionV1 is="B" value/>
                <SelectOptionV1 is="C" value/>
            </select>
        </div>
    }
}

#[component]
pub fn SelectOptionV1(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option class="text-gray-500" value=is selected=move || value.get() == is>
            {is}
        </option>
    }
}
