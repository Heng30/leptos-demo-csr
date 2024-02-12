use leptos::*;

#[component]
pub fn Counter(init_value: i32) -> impl IntoView {
    let (value, set_value) = create_signal(init_value);

    let clear = move |_| {
        set_value.set(0);
        log::debug!("clear");
    };
    let decrement = move |_| {
        set_value.update(|value| *value -= 1);
        log::warn!("decrement");
    };

    let increment = move |_| {
        set_value.update(|value| *value += 1);
        log::info!("increment");
    };

    view! {
        <div class="container flex items-center mt-20 flex-col">
            <div>
                <span class="text-gray-500">"Value: " {move || value.get().to_string()} "!"</span>
            </div>

            <div class="flex gap-2 items-center mt-2">
                <button
                    on:click=clear
                    class="hover:bg-red-400 rounded-md bg-blue-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                >
                    "Clear"
                </button>
                <button
                    on:click=decrement
                    class="hover:bg-yellow-400 rounded-md bg-yellow-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                >
                    "-1"
                </button>
                <button
                    on:click=increment
                    class="hover:bg-green-400 rounded-md bg-green-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                >
                    "+1"
                </button>
            </div>
        </div>
    }
}

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <Counter init_value=5/> })
}
