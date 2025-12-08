use leptos::*;

/// Modal dialog component
#[component]
pub fn Modal(
    /// Modal title
    #[prop(into)]
    title: String,
    /// Close handler
    on_close: Box<dyn Fn() + 'static>,
    /// Modal content
    children: Children,
) -> impl IntoView {
    let handle_backdrop_click = move |ev: ev::MouseEvent| {
        // Only close if clicking the backdrop itself, not its children
        if let Some(target) = ev.target() {
            if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                if element.class_list().contains("modal-backdrop") {
                    on_close();
                }
            }
        }
    };

    view! {
        <div class="modal-backdrop" on:click=handle_backdrop_click>
            <div class="modal" on:click=|ev: ev::MouseEvent| ev.stop_propagation()>
                <div class="modal-header">
                    <h2 class="modal-title">{title}</h2>
                    <button
                        class="modal-close"
                        on:click=move |_| on_close()
                        style="background: none; border: none; color: var(--text-secondary); font-size: 24px; cursor: pointer; padding: 0; margin-left: auto;"
                    >
                        "×"
                    </button>
                </div>
                <div class="modal-body">
                    {children()}
                </div>
            </div>
        </div>
    }
}
