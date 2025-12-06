use leptos::*;
use crate::api::ApiClient;
use crate::components::{SearchBar, MediaCard, Modal};

/// Movies page with grid view and search
#[component]
pub fn Movies() -> impl IntoView {
    let (movies, set_movies) = create_signal(Vec::<MovieItem>::new());
    let (loading, set_loading) = create_signal(true);
    let (show_add_modal, set_show_add_modal) = create_signal(false);
    let (search_results, set_search_results) = create_signal(Vec::<MovieSearchResult>::new());
    let (searching, set_searching) = create_signal(false);

    // Load movies
    create_effect(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match ApiClient::get::<Vec<MovieItem>>("/movies").await {
                Ok(data) => set_movies.set(data),
                Err(e) => log::error!("Failed to fetch movies: {}", e),
            }
            set_loading.set(false);
        });
    });

    let handle_search = move |query: String| {
        if query.is_empty() {
            return;
        }

        set_searching.set(true);
        spawn_local(async move {
            match ApiClient::get::<Vec<MovieSearchResult>>(&format!("/search/movie?q={}", query)).await {
                Ok(results) => {
                    set_search_results.set(results);
                    set_show_add_modal.set(true);
                }
                Err(e) => log::error!("Search failed: {}", e),
            }
            set_searching.set(false);
        });
    };

    let add_movie = move |tmdb_id: i64| {
        spawn_local(async move {
            match ApiClient::post::<MovieItem>("/movies", serde_json::json!({ "tmdb_id": tmdb_id })).await {
                Ok(movie) => {
                    set_movies.update(|m| m.push(movie));
                    set_show_add_modal.set(false);
                    set_search_results.set(Vec::new());
                }
                Err(e) => log::error!("Failed to add movie: {}", e),
            }
        });
    };

    view! {
        <div class="movies-page">
            <div class="page-header">
                <h1 class="page-title">"Movies"</h1>
                <p class="page-description">"Manage your movie collection"</p>
            </div>

            <div class="search-section">
                <SearchBar
                    placeholder="Search for movies to add..."
                    on_search=handle_search
                    loading=searching
                />
            </div>

            <Show
                when=move || !loading.get()
                fallback=|| view! { <div class="loading">"Loading movies..."</div> }
            >
                <Show
                    when=move || !movies.get().is_empty()
                    fallback=|| view! {
                        <div class="empty-state">
                            <div class="empty-state-icon">"🎬"</div>
                            <div class="empty-state-title">"No movies yet"</div>
                            <div class="empty-state-message">"Use the search bar above to add movies to your library"</div>
                        </div>
                    }
                >
                    <div class="media-grid">
                        <For
                            each=move || movies.get()
                            key=|movie| movie.id.clone()
                            children=move |movie: MovieItem| {
                                view! {
                                    <MediaCard
                                        title=movie.title
                                        year=movie.year
                                        poster_path=movie.poster_path
                                        status=movie.status
                                        on_click=move |_| {
                                            log::info!("Clicked movie: {}", movie.title);
                                        }
                                    />
                                }
                            }
                        />
                    </div>
                </Show>
            </Show>

            <Show when=move || show_add_modal.get()>
                <Modal
                    title="Add Movie"
                    on_close=move || set_show_add_modal.set(false)
                >
                    <div class="search-results">
                        <For
                            each=move || search_results.get()
                            key=|result| result.tmdb_id
                            children=move |result: MovieSearchResult| {
                                let tmdb_id = result.tmdb_id;
                                view! {
                                    <div class="search-result-item">
                                        <div class="result-info">
                                            <div class="result-title">{result.title.clone()}</div>
                                            <div class="result-year">{result.year}</div>
                                            <div class="result-overview">{result.overview}</div>
                                        </div>
                                        <button
                                            class="btn btn-primary"
                                            on:click=move |_| add_movie(tmdb_id)
                                        >
                                            "Add"
                                        </button>
                                    </div>
                                }
                            }
                        />
                    </div>
                </Modal>
            </Show>
        </div>
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MovieItem {
    pub id: String,
    pub tmdb_id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub poster_path: Option<String>,
    pub status: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MovieSearchResult {
    pub tmdb_id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
}
