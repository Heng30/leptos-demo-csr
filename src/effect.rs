use leptos::*;

#[component]
pub fn EffectV1() -> impl IntoView {
    let (a, set_a) = create_signal(0);

    // should not sync different reactive values in this scope
    create_effect(move |pre_value| {
        log::debug!("pre-a={:?}, a={}", pre_value, a.get());
        a.get()
    });

    view! {
        <div class="flex gap-2">
            <button on:click=move |_| set_a.update(|v| *v += 1) class="border-2 text-gray-500">
                "Increment"
            </button>
            <p>{a}</p>
        </div>
    }
}

#[component]
pub fn WatchV1() -> impl IntoView {
    let (a, set_a) = create_signal(1);

    let stop = watch(
        move || a.get(),
        move |value, pre_value, _| {
            log::debug!("watch-pre-a={:?}, watch-a={}", pre_value, value);
        },
        false,
    );

    view! {
        <div class="flex gap-2">
            <button
                on:click=move |_| {
                    if a.get() % 3 == 0 {
                        stop();
                    }
                    set_a.update(|v| *v += 1)
                }

                class="border-2 text-gray-500"
            >
                "Watch Increment"
            </button>
            <p>{a}</p>
        </div>
    }
}
