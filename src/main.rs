use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! { <Counter init_value=5/> }
}

#[component]
pub fn ProgessBar<F>(#[prop(default = 50)] max: u16, progress: F) -> impl IntoView
where
    F: Fn() -> i32 + 'static + Copy,
{
    view! {
        <div class="flex gap-2 items-center mt-2">
            <progress max=max value=move || progress().abs()></progress>
            <span>{progress}</span>
        </div>
    }
}

#[component]
pub fn ProgessBarV2(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <div class="flex gap-2 items-center mt-2">
            <progress max=max value=move || progress.get().abs()></progress>
            <span>{progress}</span>
        </div>
    }
}

#[component]
pub fn ProgessBarV3<F>(
    #[prop(default = 100)] max: u16,
    #[prop(optional)] progress: Option<Box<F>>,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static
{
    progress.map(|progress| {
        let value = Signal::derive(*progress);

        view! {
            <div class="flex gap-2 items-center mt-2">
                <progress max=max value=value></progress>
                <span>{value}</span>
            </div>
        }
    })
}

#[component]
pub fn ProgessBarV4<F>(
    #[prop(default = 100)] max: u16,
    #[prop(optional)] progress: Option<Box<F>>,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static + Clone
{
    progress.map(|progress| {
        let value = dyn_clone::clone_box(&*progress);

        view! {
            <div class="flex gap-2 items-center mt-2">
                <progress max=max value=progress></progress>
                <span>{value}</span>
            </div>
        }
    })
}

#[component]
pub fn SizeOf<T: Sized>(#[prop(optional)] _ty: std::marker::PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}

#[component]
pub fn Counter(init_value: i32) -> impl IntoView {
    let (value, set_value) = create_signal(init_value);
    let double_value = move || value.get() * 2;

    let html = "<p> This HTML will be injected.</p>";

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

            <div inner_html=html class="text-gray-400"></div>

            <div class="text-gray-300">
                <span> "Size of `String` type: " </span>
                <SizeOf<String> />
                <span> "byte!" </span>
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

    mount_to_body(|| view! { <App/> })
}
