use leptos::*;

#[derive(Default, Clone, Debug)]
struct GlobalState {
    count: u32,
    name: String,
}

#[component]
pub fn GlobalStateV1() -> impl IntoView {
    let state = create_rw_signal(GlobalState {
        count: 0,
        name: "foo".to_string(),
    });

    provide_context(state);

    view! {
        <div class="flex">
            <Counter/>
        </div>
    }
}

#[component]
fn Counter() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");
    let (count, set_count) = create_slice(state, |state| state.count, |state, n| state.count = n);

    let (name, set_name) =
        create_slice(state, |state| state.name.clone(), |state, n| state.name = n);

    view! {
        <div class="flex gap-2">
            <button class="border-2" on:click=move |_| { set_count.set(count.get() + 1) }>
                "Increment Global Count"
            </button>

            <button
                class="border-2"
                on:click=move |_| { set_name.set(format!("{}-{}", name.get(), count.get())) }
            >
                "Change Name"
            </button>

            <span>"count = " {count}</span>
            <span>"name = " {name}</span>
        </div>
    }
}
