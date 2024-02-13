use leptos::*;

#[component]
pub fn ListV1() -> impl IntoView {
    let values = vec![0, 1, 2, 3, 4, 5];
    view! {
        <ul class="flex gap-2 items-center justify-center mt-2">
            {values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}
        </ul>
    }
}

#[component]
pub fn ListV2() -> impl IntoView {
    let values = (0..10).map(|i| create_signal(i));
    let buttons = values
        .map(|(v, set_v)| {
            view! {
                <li>
                    <button
                        class="hover:bg-blue-400 rounded-md bg-blue-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                        on:click=move |_| set_v.update(|n| *n += 1)
                    >
                        {v}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! { <ul class="flex gap-2 items-center justify-center mt-2">{buttons}</ul> }
}

#[component]
pub fn ListV3(init_len: usize) -> impl IntoView {
    let mut next_id = init_len;
    let values: Vec<_> = (0..init_len).map(|id| (id, create_signal(id))).collect();
    let (counters, set_counters) = create_signal(values);

    let add_counter = move |_| {
        let sig = create_signal(next_id);
        set_counters.update(move |v| {
            v.push((next_id, sig));
        });
        next_id += 1;
    };

    view! {
        <div class="flex gap-2 items-center justify-center mt-2">
            <button
                class="hover:bg-green-400 rounded-md bg-green-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                on:click=add_counter
            >
                "Add Counter"
            </button>

            <ul class="flex gap-2 items-center justify-center">
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(key, (count, set_count))| {
                        view! {
                            <li class="flex gap-1">
                                <button
                                    class="hover:bg-blue-400 rounded-md bg-blue-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>

                                <button
                                    class="hover:bg-red-400 rounded-md bg-red-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                                    on:click=move |_| {
                                        set_counters
                                            .update(|counters| counters.retain(|(id, _)| *id != key))
                                    }
                                >

                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />

            </ul>
        </div>
    }
}

#[derive(Debug, Clone)]
struct DataEntry {
    key: String,
    value: i32,
}

#[component]
pub fn ListV4() -> impl IntoView {
    let mut inc_key = 0;

    let (data, set_data) = create_signal(vec![
        DataEntry {
            key: "foo".to_string(),
            value: 1,
        },
        DataEntry {
            key: "bar".to_string(),
            value: 2,
        },
        DataEntry {
            key: "baz".to_string(),
            value: 3,
        },
    ]);

    view! {
        <div class="flex gap-2 items-center justify-center mt-2">
            <button
                class="hover:bg-blue-400 rounded-md bg-blue-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                on:click=move |_| {
                    set_data
                        .update(|data| {
                            for row in data {
                                row.value *= 2;
                            }
                        })
                }
            >

                "Update Values"
            </button>

            <button
                class="hover:bg-green-400 rounded-md bg-green-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                on:click=move |_| {
                    set_data
                        .update(|data| {
                            data.push(DataEntry {
                                key: format!("key-{inc_key}"),
                                value: 1,
                            });
                            inc_key += 1;
                        })
                }
            >

                "Push Value"
            </button>

            <button
                class="hover:bg-red-400 rounded-md bg-red-500 text-white text-sm font-medium pl-2 pr-3 py-2 shadow-sm"
                on:click=move |_| {
                    set_data
                        .update(|data| {
                            data.pop();
                        });
                }
            >

                "Pop Value"
            </button>

            <For
                each=move || data.get().into_iter().enumerate()
                key=|(_, state)| state.key.clone()
                children=move |(index, _)| {
                    let value = create_memo(move |_| {
                        data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                    });
                    view! { <p class="text-gray-500">{value}</p> }
                }
            />

        </div>
    }
}
