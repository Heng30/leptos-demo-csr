use leptos::*;

#[component]
pub fn TextareaV1() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    view! {
        <div class="flex-clo gap-2 items-center mt-2">
            <textarea
                class="block round-md border-2 text-gray-900 pl-1 py-1 pr-1"
                prop:value=move || name.get()
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                }
            >

                {name}
            </textarea>
        </div>
    }
}
