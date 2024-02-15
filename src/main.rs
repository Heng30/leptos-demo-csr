use leptos::*;

mod counter;
mod list;
mod progress_bar;
mod input;
mod textarea;
mod select;
mod ctrlflow;
mod error_handling;
mod parent_child_talk;
mod effect;
mod asyn;
mod global_state;
mod router;

use counter::*;
use list::*;
use input::*;
use textarea::*;
use select::*;
use ctrlflow::*;
use error_handling::*;
use parent_child_talk::*;
use effect::*;
use asyn::*;
use global_state::*;
use router::*;

#[component]
pub fn App() -> impl IntoView {
    let html = "<p> This HTML will be injected.</p>";

    view! {
        <div class="container flex items-center mt-20 flex-col">
            // <Counter init_value=5/>

            // <div inner_html=html class="text-gray-400"></div>

            // <div class="text-gray-300">
            // <span> "Size of `String` type: " </span>
            // <SizeOf<String> />
            // <span> "byte!" </span>
            // </div>

            // <div class="flex-clo gap-2 items-center mt-2" >
            // <ListV1 />
            // <ListV2 />
            // <ListV3 init_len=3 />
            // <ListV4 />
            // </div>

            // <div class="flex-clo gap-2 items-center mt-2" >
            // <InputV1 />
            // <InputV2 />
            // <TextareaV1 />
            // <SelectV1 />
            // </div>

            // <div class="flex gap-2 items-center mt-2" >
            // <CtrlflowV1 />
            // <CtrlflowV2 />
            // <CtrlflowV3 />
            // <CtrlflowV4 />
            // </div>

            // <div class="flex gap-2 items-center mt-2">
            // <EHandlingV1/>
            // <EHandlingV2/>
            // </div>

            // <div class="flex gap-2 items-center mt-2">
            // <ParentChildTalkV1/>
            // <ParentChildTalkV2/>
            // <ParentChildTalkV3/>
            // <ParentChildTalkV4/>
            // <ParentChildTalkV5/>
            // </div>

            // <div class="flex-col gap-2 items-center mt-2">
            // <ParentChildTalkV6/>
            // <ParentChildTalkV7/>
            // </div>

            // <div class="flex gap-2 items-center mt-2">
            // <EffectV1/>
            // <WatchV1/>
            // </div>

            // <div class="flex gap-2 items-center mt-2">
            // <AsyncV1/>
            // <AsyncV2/>
            // <AsyncV3/>
            // <AsyncV4/>
            // <AsyncV5/>
            // <AsyncV6/>
            // <AsyncV7/>
            // </div>

            // <div class="flex gap-2 items-center mt-2">
            // <GlobalStateV1/>
            // </div>

            <div class="flex gap-2 items-center mt-2">
                <RouterV1/>
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
