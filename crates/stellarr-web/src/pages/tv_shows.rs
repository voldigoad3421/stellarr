use leptos::*;
use crate::api::ApiClient;
use crate::components::{SearchBar, MediaCard, Modal};

/// TV Shows page with expandable seasons and episodes
#[component]
pub fn TvShows() -> impl IntoView {
    let (shows, set_shows) = create_signal(Vec::<TvShow>::new());
    let (loading, set_loading) = create_signal(true);
    let (selected_show, set_selected_show) = create_signal(Option::<String>::None);
    let (show_details, set_show_details) = create_signal(Option::<TvShowDetails>::None);

    // Load TV shows
    create_effect(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match ApiClient::get::<Vec<TvShow>>("/series").await {
                Ok(data) => set_shows.set(data),
                Err(e) => log::error!("Failed to fetch TV shows: {}", e),
            }
            set_loading.set(false);
        });
    });

    let load_show_details = move |show_id: String| {
        set_selected_show.set(Some(show_id.clone()));
        spawn_local(async move {
            match ApiClient::get::<TvShowDetails>(&format!("/series/{}", show_id)).await {
                Ok(details) => set_show_details.set(Some(details)),
                Err(e) => log::error!("Failed to fetch show details: {}", e),
            }
        });
    };

    view! {
        <div class="tv-shows-page">
            <div class="page-header">
                <h1 class="page-title">"TV Shows"</h1>
                <p class="page-description">"Manage your TV show collection"</p>
            </div>

            <div class="search-section">
                <SearchBar
                    placeholder="Search for TV shows to add..."
                    on_search=move |query| {
                        log::info!("Search TV shows: {}", query);
                    }
                />
            </div>

            <Show
                when=move || !loading.get()
                fallback=|| view! { <div class="loading">"Loading TV shows..."</div> }
            >
                <Show
                    when=move || !shows.get().is_empty()
                    fallback=|| view! {
                        <div class="empty-state">
                            <div class="empty-state-icon">"📺"</div>
                            <div class="empty-state-title">"No TV shows yet"</div>
                            <div class="empty-state-message">"Use the search bar above to add TV shows to your library"</div>
                        </div>
                    }
                >
                    <div class="media-grid">
                        <For
                            each=move || shows.get()
                            key=|show| show.id.clone()
                            children=move |show: TvShow| {
                                let show_id = show.id.clone();
                                view! {
                                    <MediaCard
                                        title=show.title
                                        year=show.year
                                        poster_path=show.poster_path
                                        status=show.status
                                        on_click=move |_| load_show_details(show_id.clone())
                                    />
                                }
                            }
                        />
                    </div>
                </Show>
            </Show>

            <Show when=move || selected_show.get().is_some()>
                <Modal
                    title=show_details.get().map(|d| d.title.clone()).unwrap_or_default()
                    on_close=move || {
                        set_selected_show.set(None);
                        set_show_details.set(None);
                    }
                >
                    <Show when=move || show_details.get().is_some()>
                        {move || {
                            show_details.get().map(|details| view! {
                                <div class="show-details">
                                    <div class="show-overview">{details.overview}</div>
                                    <div class="seasons-list">
                                        <For
                                            each=move || details.seasons.clone()
                                            key=|season| season.season_number
                                            children=move |season: Season| {
                                                let (expanded, set_expanded) = create_signal(false);
                                                view! {
                                                    <div class="season-item">
                                                        <div
                                                            class="season-header"
                                                            on:click=move |_| set_expanded.update(|e| *e = !*e)
                                                        >
                                                            <span class="season-title">{season.name.clone()}</span>
                                                            <span class="season-status">{format!("{}/{} episodes", season.downloaded_count, season.episode_count)}</span>
                                                        </div>
                                                        <Show when=move || expanded.get()>
                                                            <div class="episodes-list">
                                                                <For
                                                                    each=move || season.episodes.clone()
                                                                    key=|ep| ep.episode_number
                                                                    children=move |episode: Episode| {
                                                                        view! {
                                                                            <div class="episode-item">
                                                                                <div class="episode-number">{episode.episode_code()}</div>
                                                                                <div class="episode-title">{episode.title}</div>
                                                                                <div class="episode-status">
                                                                                    <span class={format!("badge badge-{}", episode.status_class())}>
                                                                                        {episode.status}
                                                                                    </span>
                                                                                </div>
                                                                            </div>
                                                                        }
                                                                    }
                                                                />
                                                            </div>
                                                        </Show>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            })
                        }}
                    </Show>
                </Modal>
            </Show>
        </div>
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TvShow {
    pub id: String,
    pub tmdb_id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub poster_path: Option<String>,
    pub status: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TvShowDetails {
    pub id: String,
    pub title: String,
    pub overview: Option<String>,
    pub seasons: Vec<Season>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Season {
    pub season_number: i32,
    pub name: String,
    pub episode_count: i32,
    pub downloaded_count: i32,
    pub episodes: Vec<Episode>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Episode {
    pub episode_number: i32,
    pub season_number: i32,
    pub title: String,
    pub status: String,
}

impl Episode {
    fn episode_code(&self) -> String {
        format!("S{:02}E{:02}", self.season_number, self.episode_number)
    }

    fn status_class(&self) -> String {
        match self.status.as_str() {
            "Downloaded" => "success".to_string(),
            "Downloading" => "info".to_string(),
            "Missing" => "warning".to_string(),
            "Failed" => "error".to_string(),
            _ => "info".to_string(),
        }
    }
}
