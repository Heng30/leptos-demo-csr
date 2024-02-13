use leptos::*;

#[component]
pub fn InputV1() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    view! {
        <div class="flex-clo gap-2 items-center mt-2">
            <span class="text-gray-500">"Name: "</span>
            <input
                type="text"
                class="block round-md border-2 text-gray-900 pl-1 py-1 pr-1"
                prop:placeholder="Input name"
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                }
            />

            <span class="text-red-500">"Name is: " {name}</span>
        </div>
    }
}

#[component]
pub fn InputV2() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = input_element.get().unwrap().value();
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit class="flex-clo gap-2 items-center mt-2">
            <span class="text-gray-500">"Name: "</span>
            <input
                type="text"
                class="block round-md border-2 text-gray-900 pl-1 py-1 pr-1"
                prop:placeholder="Input name"
                value=name
                node_ref=input_element
            />
            <input
                type="submit"
                value="Submit"
                class="block round-md border-2 pl-1 py-1 pr-1 mt-2 rounded-md"
            />
        </form>
        <span class="text-red-500">"Name is: " {name}</span>
    }
}
