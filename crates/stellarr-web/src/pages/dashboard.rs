use leptos::*;
use crate::api::ApiClient;
use crate::components::{SearchBar, MediaCard};

/// Dashboard page with statistics and recent activity
#[component]
pub fn Dashboard() -> impl IntoView {
    let (stats, set_stats) = create_signal(DashboardStats::default());
    let (recent_activity, set_recent_activity) = create_signal(Vec::<ActivityItem>::new());
    let (loading, set_loading) = create_signal(true);

    // Load dashboard data
    create_effect(move |_| {
        spawn_local(async move {
            set_loading.set(true);

            // Fetch statistics
            match ApiClient::get::<DashboardStats>("/stats").await {
                Ok(data) => set_stats.set(data),
                Err(e) => log::error!("Failed to fetch stats: {}", e),
            }

            // Fetch recent activity
            match ApiClient::get::<Vec<ActivityItem>>("/activity/recent").await {
                Ok(data) => set_recent_activity.set(data),
                Err(e) => log::error!("Failed to fetch activity: {}", e),
            }

            set_loading.set(false);
        });
    });

    view! {
        <div class="dashboard">
            <div class="page-header">
                <h1 class="page-title">"Dashboard"</h1>
                <p class="page-description">"Overview of your media library and recent activity"</p>
            </div>

            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-label">"Total Movies"</div>
                    <div class="stat-value">{move || stats.get().total_movies}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-label">"Total Shows"</div>
                    <div class="stat-value">{move || stats.get().total_shows}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-label">"Active Downloads"</div>
                    <div class="stat-value">{move || stats.get().active_downloads}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-label">"Disk Space"</div>
                    <div class="stat-value">{move || format!("{}GB", stats.get().disk_space_gb)}</div>
                </div>
            </div>

            <div class="search-section">
                <h2 class="section-title">"Quick Add"</h2>
                <SearchBar placeholder="Search for movies or TV shows..." on_search=move |query| {
                    log::info!("Quick search: {}", query);
                }/>
            </div>

            <div class="recent-activity">
                <h2 class="section-title">"Recent Activity"</h2>
                <Show
                    when=move || !loading.get()
                    fallback=|| view! { <div class="loading">"Loading..."</div> }
                >
                    <Show
                        when=move || !recent_activity.get().is_empty()
                        fallback=|| view! {
                            <div class="empty-state">
                                <div class="empty-state-icon">"📭"</div>
                                <div class="empty-state-title">"No recent activity"</div>
                                <div class="empty-state-message">"Activity will appear here when you add or download media"</div>
                            </div>
                        }
                    >
                        <div class="list">
                            <For
                                each=move || recent_activity.get()
                                key=|item| item.id.clone()
                                children=move |item: ActivityItem| {
                                    view! {
                                        <div class="list-item">
                                            <div class="activity-info">
                                                <div class="activity-title">{item.title}</div>
                                                <div class="activity-description">{item.description}</div>
                                            </div>
                                            <div class="activity-time">{item.time_ago}</div>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </Show>
                </Show>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DashboardStats {
    pub total_movies: i32,
    pub total_shows: i32,
    pub active_downloads: i32,
    pub disk_space_gb: i32,
}

impl Default for DashboardStats {
    fn default() -> Self {
        Self {
            total_movies: 0,
            total_shows: 0,
            active_downloads: 0,
            disk_space_gb: 0,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ActivityItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub time_ago: String,
}
