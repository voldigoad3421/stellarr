use leptos::*;
use crate::api::ApiClient;

/// Activity/Downloads page with active downloads and history
#[component]
pub fn Activity() -> impl IntoView {
    let (active_downloads, set_active_downloads) = create_signal(Vec::<Download>::new());
    let (download_history, set_download_history) = create_signal(Vec::<Download>::new());
    let (loading, set_loading) = create_signal(true);

    // Load downloads
    create_effect(move |_| {
        spawn_local(async move {
            set_loading.set(true);

            // Fetch active downloads
            match ApiClient::get::<Vec<Download>>("/downloads/active").await {
                Ok(data) => set_active_downloads.set(data),
                Err(e) => log::error!("Failed to fetch active downloads: {}", e),
            }

            // Fetch download history
            match ApiClient::get::<Vec<Download>>("/downloads/history").await {
                Ok(data) => set_download_history.set(data),
                Err(e) => log::error!("Failed to fetch download history: {}", e),
            }

            set_loading.set(false);
        });
    });

    // Poll for download updates every 2 seconds
    create_effect(move |_| {
        set_interval(
            move || {
                spawn_local(async move {
                    if let Ok(data) = ApiClient::get::<Vec<Download>>("/downloads/active").await {
                        set_active_downloads.set(data);
                    }
                });
            },
            std::time::Duration::from_secs(2),
        );
    });

    view! {
        <div class="activity-page">
            <div class="page-header">
                <h1 class="page-title">"Activity"</h1>
                <p class="page-description">"Monitor downloads and recent activity"</p>
            </div>

            <section class="active-downloads-section">
                <h2 class="section-title">"Active Downloads"</h2>
                <Show
                    when=move || !loading.get()
                    fallback=|| view! { <div class="loading">"Loading..."</div> }
                >
                    <Show
                        when=move || !active_downloads.get().is_empty()
                        fallback=|| view! {
                            <div class="empty-state">
                                <div class="empty-state-icon">"⬇️"</div>
                                <div class="empty-state-title">"No active downloads"</div>
                                <div class="empty-state-message">"Downloads will appear here when you add media"</div>
                            </div>
                        }
                    >
                        <div class="downloads-list">
                            <For
                                each=move || active_downloads.get()
                                key=|dl| dl.id.clone()
                                children=move |download: Download| {
                                    view! {
                                        <DownloadItem download=download />
                                    }
                                }
                            />
                        </div>
                    </Show>
                </Show>
            </section>

            <section class="download-history-section">
                <h2 class="section-title">"Download History"</h2>
                <Show
                    when=move || !download_history.get().is_empty()
                    fallback=|| view! {
                        <div class="empty-state">
                            <div class="empty-state-message">"No download history yet"</div>
                        </div>
                    }
                >
                    <div class="list">
                        <For
                            each=move || download_history.get()
                            key=|dl| dl.id.clone()
                            children=move |download: Download| {
                                view! {
                                    <div class="list-item">
                                        <div class="download-info">
                                            <div class="download-title">{download.title}</div>
                                            <div class="download-meta text-muted">
                                                {download.size_string()} " - "
                                                {download.completed_at.unwrap_or_default()}
                                            </div>
                                        </div>
                                        <span class={format!("badge badge-{}", download.status_class())}>
                                            {download.status}
                                        </span>
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>
            </section>
        </div>
    }
}

/// Individual download item component with progress bar
#[component]
fn DownloadItem(download: Download) -> impl IntoView {
    view! {
        <div class="download-item card">
            <div class="download-header">
                <div class="download-title">{download.title.clone()}</div>
                <span class={format!("badge badge-{}", download.status_class())}>
                    {download.status.clone()}
                </span>
            </div>
            <div class="download-progress">
                <div class="progress-info">
                    <span class="progress-percent">{format!("{:.1}%", download.progress)}</span>
                    <span class="progress-speed">{download.speed_string().unwrap_or_default()}</span>
                    <span class="progress-eta">{download.eta_string().unwrap_or_default()}</span>
                </div>
                <div class="progress-bar">
                    <div
                        class="progress-fill"
                        style:width=format!("{}%", download.progress)
                    />
                </div>
            </div>
            <div class="download-meta text-muted">
                {download.downloaded_string()} " of " {download.size_string()}
            </div>
        </div>
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Download {
    pub id: String,
    pub title: String,
    pub status: String,
    pub progress: f64,
    pub size_bytes: Option<i64>,
    pub downloaded_bytes: Option<i64>,
    pub download_speed: Option<i64>,
    pub eta_seconds: Option<i64>,
    pub completed_at: Option<String>,
}

impl Download {
    fn status_class(&self) -> String {
        match self.status.as_str() {
            "Downloading" => "info".to_string(),
            "Completed" => "success".to_string(),
            "Failed" => "error".to_string(),
            "Paused" => "warning".to_string(),
            _ => "info".to_string(),
        }
    }

    fn size_string(&self) -> String {
        self.size_bytes
            .map(format_bytes)
            .unwrap_or_else(|| "Unknown".to_string())
    }

    fn downloaded_string(&self) -> String {
        self.downloaded_bytes
            .map(format_bytes)
            .unwrap_or_else(|| "0 B".to_string())
    }

    fn speed_string(&self) -> Option<String> {
        self.download_speed.map(|speed| format!("{}/s", format_bytes(speed)))
    }

    fn eta_string(&self) -> Option<String> {
        self.eta_seconds.map(format_duration)
    }
}

fn format_bytes(bytes: i64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

fn format_duration(seconds: i64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m", seconds / 60)
    } else {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}
