use leptos::*;

/// Reusable media card component for displaying movies and TV shows
#[component]
pub fn MediaCard(
    /// Media title
    title: String,
    /// Release year
    #[prop(optional)]
    year: Option<i32>,
    /// Poster path (TMDb path)
    #[prop(optional)]
    poster_path: Option<String>,
    /// Status (Missing, Available, Downloading, etc.)
    status: String,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn(ev::MouseEvent) + 'static>>,
) -> impl IntoView {
    let poster_url = poster_path
        .map(|path| format!("https://image.tmdb.org/t/p/w500{}", path))
        .unwrap_or_else(|| "/placeholder-poster.jpg".to_string());

    let status_class = match status.as_str() {
        "Available" | "Downloaded" => "badge-success",
        "Downloading" => "badge-info",
        "Missing" => "badge-warning",
        "Failed" => "badge-error",
        _ => "badge-info",
    };

    let year_str = year.map(|y| format!(" ({})", y)).unwrap_or_default();

    view! {
        <div
            class="media-card card"
            on:click=move |ev| {
                if let Some(ref handler) = on_click {
                    handler(ev);
                }
            }
            style="cursor: pointer;"
        >
            <div class="media-poster">
                <img
                    src=poster_url
                    alt=format!("{} poster", title)
                    style="width: 100%; height: auto; border-radius: 6px 6px 0 0; display: block;"
                    on:error=move |ev| {
                        let img = ev.target().unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap();
                        img.set_src("/placeholder-poster.jpg");
                    }
                />
            </div>
            <div class="media-info" style="padding: 12px;">
                <div class="media-title" style="font-weight: 600; margin-bottom: 4px; line-height: 1.3;">
                    {title}
                    <span class="text-muted" style="font-weight: 400;">{year_str}</span>
                </div>
                <span class={format!("badge {}", status_class)}>
                    {status}
                </span>
            </div>
        </div>
    }
}
