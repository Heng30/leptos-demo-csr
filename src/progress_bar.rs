use leptos::*;

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
    F: Fn() -> i32 + 'static,
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
    F: Fn() -> i32 + 'static + Clone,
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
