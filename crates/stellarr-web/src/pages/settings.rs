use leptos::*;
use crate::api::ApiClient;

/// Settings page with configuration forms
#[component]
pub fn Settings() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("indexers".to_string());

    view! {
        <div class="settings-page">
            <div class="page-header">
                <h1 class="page-title">"Settings"</h1>
                <p class="page-description">"Configure Stellarr"</p>
            </div>

            <div class="settings-tabs">
                <button
                    class=move || if active_tab.get() == "indexers" { "tab-button active" } else { "tab-button" }
                    on:click=move |_| set_active_tab.set("indexers".to_string())
                >
                    "Indexers"
                </button>
                <button
                    class=move || if active_tab.get() == "download-clients" { "tab-button active" } else { "tab-button" }
                    on:click=move |_| set_active_tab.set("download-clients".to_string())
                >
                    "Download Clients"
                </button>
                <button
                    class=move || if active_tab.get() == "quality" { "tab-button active" } else { "tab-button" }
                    on:click=move |_| set_active_tab.set("quality".to_string())
                >
                    "Quality Profiles"
                </button>
                <button
                    class=move || if active_tab.get() == "general" { "tab-button active" } else { "tab-button" }
                    on:click=move |_| set_active_tab.set("general".to_string())
                >
                    "General"
                </button>
            </div>

            <div class="settings-content">
                <Show when=move || active_tab.get() == "indexers">
                    <IndexersSettings />
                </Show>
                <Show when=move || active_tab.get() == "download-clients">
                    <DownloadClientsSettings />
                </Show>
                <Show when=move || active_tab.get() == "quality">
                    <QualitySettings />
                </Show>
                <Show when=move || active_tab.get() == "general">
                    <GeneralSettings />
                </Show>
            </div>
        </div>
    }
}

/// Indexers configuration section
#[component]
fn IndexersSettings() -> impl IntoView {
    let (indexers, set_indexers) = create_signal(Vec::<Indexer>::new());
    let (show_add_form, set_show_add_form) = create_signal(false);

    // Load indexers
    create_effect(move |_| {
        spawn_local(async move {
            match ApiClient::get::<Vec<Indexer>>("/indexers").await {
                Ok(data) => set_indexers.set(data),
                Err(e) => log::error!("Failed to fetch indexers: {}", e),
            }
        });
    });

    view! {
        <section class="settings-section">
            <div class="section-header">
                <h2 class="section-title">"Indexers"</h2>
                <button
                    class="btn btn-primary"
                    on:click=move |_| set_show_add_form.set(true)
                >
                    "+ Add Indexer"
                </button>
            </div>

            <div class="indexers-list">
                <For
                    each=move || indexers.get()
                    key=|indexer| indexer.id.clone()
                    children=move |indexer: Indexer| {
                        view! {
                            <div class="card indexer-card">
                                <div class="indexer-header">
                                    <h3 class="indexer-name">{indexer.name}</h3>
                                    <span class={format!("badge badge-{}", if indexer.enabled { "success" } else { "warning" })}>
                                        {if indexer.enabled { "Enabled" } else { "Disabled" }}
                                    </span>
                                </div>
                                <div class="indexer-details">
                                    <div class="detail-item">
                                        <span class="detail-label">"Type:"</span>
                                        <span class="detail-value">{indexer.protocol}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="detail-label">"URL:"</span>
                                        <span class="detail-value">{indexer.base_url}</span>
                                    </div>
                                </div>
                                <div class="card-actions">
                                    <button class="btn btn-secondary">"Test"</button>
                                    <button class="btn btn-secondary">"Edit"</button>
                                    <button class="btn btn-secondary">"Delete"</button>
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            <Show when=move || show_add_form.get()>
                <div class="add-form card">
                    <h3>"Add Indexer"</h3>
                    <div class="form-group">
                        <label class="form-label">"Name"</label>
                        <input type="text" class="form-input" placeholder="Indexer name" />
                    </div>
                    <div class="form-group">
                        <label class="form-label">"Protocol"</label>
                        <select class="form-input">
                            <option>"Torrent"</option>
                            <option>"Usenet"</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label class="form-label">"Base URL"</label>
                        <input type="text" class="form-input" placeholder="https://indexer.example.com" />
                    </div>
                    <div class="form-group">
                        <label class="form-label">"API Key"</label>
                        <input type="password" class="form-input" placeholder="API Key" />
                    </div>
                    <div class="form-actions">
                        <button class="btn btn-primary">"Save"</button>
                        <button
                            class="btn btn-secondary"
                            on:click=move |_| set_show_add_form.set(false)
                        >
                            "Cancel"
                        </button>
                    </div>
                </div>
            </Show>
        </section>
    }
}

/// Download clients configuration section
#[component]
fn DownloadClientsSettings() -> impl IntoView {
    let (clients, set_clients) = create_signal(Vec::<DownloadClientConfig>::new());

    // Load download clients
    create_effect(move |_| {
        spawn_local(async move {
            match ApiClient::get::<Vec<DownloadClientConfig>>("/download-clients").await {
                Ok(data) => set_clients.set(data),
                Err(e) => log::error!("Failed to fetch download clients: {}", e),
            }
        });
    });

    view! {
        <section class="settings-section">
            <div class="section-header">
                <h2 class="section-title">"Download Clients"</h2>
                <button class="btn btn-primary">"+ Add Client"</button>
            </div>

            <div class="clients-list">
                <For
                    each=move || clients.get()
                    key=|client| client.id.clone()
                    children=move |client: DownloadClientConfig| {
                        view! {
                            <div class="card client-card">
                                <div class="client-header">
                                    <h3 class="client-name">{client.name}</h3>
                                    <span class={format!("badge badge-{}", if client.enabled { "success" } else { "warning" })}>
                                        {if client.enabled { "Enabled" } else { "Disabled" }}
                                    </span>
                                </div>
                                <div class="client-details">
                                    <div class="detail-item">
                                        <span class="detail-label">"Type:"</span>
                                        <span class="detail-value">{client.client_type}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="detail-label">"Host:"</span>
                                        <span class="detail-value">{format!("{}:{}", client.host, client.port)}</span>
                                    </div>
                                </div>
                                <div class="card-actions">
                                    <button class="btn btn-secondary">"Test"</button>
                                    <button class="btn btn-secondary">"Edit"</button>
                                    <button class="btn btn-secondary">"Delete"</button>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </section>
    }
}

/// Quality profiles configuration section
#[component]
fn QualitySettings() -> impl IntoView {
    view! {
        <section class="settings-section">
            <div class="section-header">
                <h2 class="section-title">"Quality Profiles"</h2>
                <button class="btn btn-primary">"+ Add Profile"</button>
            </div>

            <div class="profiles-list">
                <div class="card">
                    <h3>"HD"</h3>
                    <p class="text-secondary">"1080p and 720p releases"</p>
                    <div class="card-actions">
                        <button class="btn btn-secondary">"Edit"</button>
                    </div>
                </div>
                <div class="card">
                    <h3>"4K"</h3>
                    <p class="text-secondary">"2160p releases only"</p>
                    <div class="card-actions">
                        <button class="btn btn-secondary">"Edit"</button>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// General settings section
#[component]
fn GeneralSettings() -> impl IntoView {
    view! {
        <section class="settings-section">
            <h2 class="section-title">"General Settings"</h2>

            <div class="card">
                <div class="form-group">
                    <label class="form-label">"Media Root Directory"</label>
                    <input type="text" class="form-input" placeholder="/media" />
                    <small class="form-help">"Root directory for all media files"</small>
                </div>

                <div class="form-group">
                    <label class="form-label">"API Port"</label>
                    <input type="number" class="form-input" placeholder="3000" />
                </div>

                <div class="form-group">
                    <label class="form-label">
                        <input type="checkbox" />
                        " Enable automatic updates"
                    </label>
                </div>

                <div class="form-group">
                    <label class="form-label">
                        <input type="checkbox" />
                        " Enable notifications"
                    </label>
                </div>

                <button class="btn btn-primary">"Save Settings"</button>
            </div>
        </section>
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Indexer {
    pub id: String,
    pub name: String,
    pub protocol: String,
    pub base_url: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DownloadClientConfig {
    pub id: String,
    pub name: String,
    pub client_type: String,
    pub host: String,
    pub port: u16,
    pub enabled: bool,
}
