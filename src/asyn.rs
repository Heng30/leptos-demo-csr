use anyhow::Result;
use gloo_timers::future::TimeoutFuture;
use leptos::*;
use reqwest::header::{self, HeaderMap, HeaderValue};
use uuid::Uuid;

async fn load_data() -> Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );

    Ok(reqwest::Client::new()
        .get("https://example.com")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?)
}

#[component]
pub fn AsyncV1() -> impl IntoView {
    let once = create_resource(
        || (),
        |_| async move {
            match load_data().await {
                Err(e) => e.to_string(),
                Ok(text) => text,
            }
        },
    );

    view! {
        <div>
            <h1>"My Data"</h1>
            {move || match once.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(data) => {
                    view! {
                        {data}
                        >
                    }
                        .into_view()
                }
            }}

        </div>
    }
}

async fn load_data_v2(value: i32) -> i32 {
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn AsyncV2() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let async_data = create_resource(
        move || count.get(),
        |value| async move { load_data_v2(value).await },
    );

    let is_loading = move || {
        if async_data.loading().get() {
            "Loading"
        } else {
            "Idle"
        }
    };

    view! {
        <div>
            <button class="border-2 text-gray-500" on:click=move |_| set_count.update(|v| *v += 1)>
                "Start"
            </button>
            <h1>"async Data: " {is_loading}</h1>
        </div>
    }
}

#[component]
pub fn AsyncV3() -> impl IntoView {
    let (count, set_count) = create_signal(1);
    let (count2, set_count2) = create_signal(2);

    let async_count = create_resource(
        move || count.get(),
        |value| async move { load_data_v2(value).await },
    );

    let async_count2 = create_resource(
        move || count2.get(),
        |value| async move { load_data_v2(value).await },
    );

    view! {
        <div>
            <button
                class="border-2 text-gray-500"
                on:click=move |_| {
                    set_count.update(|v| *v += 1);
                    set_count2.update(|v| *v += 1);
                }
            >

                "Start Suspense"
            </button>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <h2>"count=" {move || async_count.get().map(|v| view! { <span>{v}</span> })}</h2>
                <h2>"count2=" {move || async_count2.get().map(|v| view! { <span>{v}</span> })}</h2>
            </Suspense>

        </div>
    }
}

#[component]
pub fn AsyncV4() -> impl IntoView {
    let (count, set_count) = create_signal(1);

    view! {
        <div>
            <button
                class="border-2 text-gray-500"
                on:click=move |_| {
                    set_count.update(|v| *v += 1);
                }
            >

                "Start Await"
            </button>

            <Await future=move || load_data_v2(count.get()) let:data>
                <h2>"count=" {*data}</h2>
            </Await>

        </div>
    }
}

#[component]
pub fn AsyncV5() -> impl IntoView {
    let (count, set_count) = create_signal(1);

    let async_count = create_resource(
        move || count.get(),
        |value| async move { load_data_v2(value).await },
    );

    view! {
        <div>
            <button
                class="border-2 text-gray-500"
                on:click=move |_| {
                    set_count.update(|v| *v += 1);
                }
            >

                "Start Transition"
            </button>
            <Transition fallback=move || view! { <p>"Initialize Loading..."</p> }>
                <h2>"count=" {move || async_count.get()}</h2>
            </Transition>

            <p>{move || if async_count.loading().get() { "Working..." } else { "" }}</p>
        </div>
    }
}

async fn add_todo(text: &str) -> Uuid {
    _ = text;
    TimeoutFuture::new(1_000).await;
    Uuid::new_v4()
}

#[component]
pub fn AsyncV6() -> impl IntoView {
    let add_todo = create_action(|input: &String| {
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });

    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<html::Input>();

    view! {
        <div class="flex-col">
            <form
                class="flex gap-2"
                on:submit=move |ev| {
                    ev.prevent_default();
                    let input = input_ref.get().expect("input element");
                    add_todo.dispatch(input.value());
                }
            >

                <label class="flex gap-2">
                    <span>"What do you need to do?"</span>
                    <input class="border-2 text-gray-500" type="text" node_ref=input_ref/>
                </label>
                <button class="border-2" type="submit">
                    "Add Todo"
                </button>
            </form>

            <p>{move || pending.get().then(|| "Loading...")}</p>
            <p>"Submitted: " <code>{move || format!("{:?}", submitted.get())}</code></p>
            <p>"Pending: " <code>{move || format!("{:?}", pending.get())}</code></p>
            <p>"Todo ID: " <code>{move || format!("{:?}", todo_id.get())}</code></p>
        </div>
    }
}

#[component]
pub fn LogginIn<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let fallback = store_value(fallback);
    let children = store_value(children);
    let (count, set_count) = create_signal(0);

    let async_data = create_resource(
        move || count.get(),
        |value| async move { load_data_v2(value).await },
    );

    view! {
        <div>
            <button class="border-2" on:click=move |_| set_count.update(|v| *v += 1)>
                "Start"
            </button>

            <Suspense fallback=|| ()>
                <Show
                    when=move || !async_data.loading().get()
                    fallback=move || fallback.with_value(|fallback| fallback())
                >
                    {children.with_value(|ch| ch())}
                </Show>
            </Suspense>
        </div>
    }
}

#[component]
pub fn AsyncV7() -> impl IntoView {
    view! {
        <LogginIn fallback=move || view! { <p>"Loading"</p> }>
            <p>"Idle..."</p>
        </LogginIn>
    }
}
