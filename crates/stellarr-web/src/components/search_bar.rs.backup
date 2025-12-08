use leptos::*;
use leptos::ev::KeyboardEvent;

/// Search bar component with optional loading state
#[component]
pub fn SearchBar(
    /// Placeholder text
    #[prop(optional, into)]
    placeholder: String,
    /// Search handler callback
    on_search: Box<dyn Fn(String) + 'static>,
    /// Loading state
    #[prop(optional)]
    loading: ReadSignal<bool>,
) -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let input_ref = create_node_ref::<html::Input>();

    let handle_search = move |_| {
        let search_query = query.get().trim().to_string();
        if !search_query.is_empty() {
            on_search(search_query);
        }
    };

    let handle_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            handle_search(ev::MouseEvent::new("click").unwrap());
        }
    };

    let loading_value = loading.unwrap_or_else(|| create_signal(false).0);

    view! {
        <div class="search-bar">
            <input
                type="text"
                class="search-input"
                placeholder=placeholder
                prop:value=move || query.get()
                on:input=move |ev| set_query.set(event_target_value(&ev))
                on:keydown=handle_keydown
                node_ref=input_ref
                disabled=move || loading_value.get()
            />
            <button
                class="btn btn-primary"
                on:click=handle_search
                disabled=move || loading_value.get() || query.get().trim().is_empty()
            >
                <Show
                    when=move || loading_value.get()
                    fallback=|| view! { <span>"Search"</span> }
                >
                    <span>"Searching..."</span>
                </Show>
            </button>
        </div>
    }
}
