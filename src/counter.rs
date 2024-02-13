use leptos::*;
use crate::progress_bar::{ProgessBar, ProgessBarV2, ProgessBarV3, ProgessBarV4};

#[component]
pub fn Counter(init_value: i32) -> impl IntoView {
    let (value, set_value) = create_signal(init_value);
    let double_value = move || value.get() * 2;

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
            <div class="flex gap-2 items-center mt-2">
                <span
                    class="text-gray-500"
                    class=("text-red-500", move || value.get().abs() % 2 == 1)
                    style:background-color=move || "pink"
                    style:border-radius="4px"
                    style:padding="4px 4px"
                >

                    "Value: "
                    {move || value.get().to_string()}
                    "!"
                </span>

            </div>

            <div>
                <div class="flex gap-2 items-center mt-2">
                    <progress max="100" value=move || value.get().abs()></progress>
                    <span>{value}</span>
                </div>

                <ProgessBar progress=move || value.get()/>
                <ProgessBarV2 progress=Signal::derive(double_value)/>
                <ProgessBarV3 progress=Box::new(move || value.get())/>
                <ProgessBarV4 progress=Box::new(move || value.get())/>
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
