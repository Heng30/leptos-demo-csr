use leptos::*;

mod counter;
mod list;
mod progress_bar;
mod input;

use counter::*;
use list::*;
use input::*;

#[component]
pub fn App() -> impl IntoView {
    let html = "<p> This HTML will be injected.</p>";

    view! {
        <div class="container flex items-center mt-20 flex-col">
            <Counter init_value=5/>

            <div inner_html=html class="text-gray-400"></div>

            <div class="text-gray-300">
                <span> "Size of `String` type: " </span>
                <SizeOf<String> />
                <span> "byte!" </span>
            </div>

            <div class="flex-clo gap-2 items-center mt-2" >
                <ListV1 />
                <ListV2 />
                <ListV3 init_len=3 />
                <ListV4 />
            </div>

            <div class="flex-clo gap-2 items-center mt-2" >
                <InputV1 />
                <InputV2 />
            </div>
        </div>
    }
}

#[component]
pub fn SizeOf<T: Sized>(#[prop(optional)] _ty: std::marker::PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}
