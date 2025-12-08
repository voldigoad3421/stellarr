use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::*;

/// Main application component with sidebar navigation and dark theme
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/stellarr-web.css"/>
        <Title text="Stellarr - Media Management"/>
        <Style>
            {DARK_THEME_CSS}
        </Style>
        <Router>
            <div class="app-container">
                <Sidebar/>
                <main class="main-content">
                    <Routes>
                        <Route path="/" view=Dashboard/>
                        <Route path="/movies" view=Movies/>
                        <Route path="/tv" view=TvShows/>
                        <Route path="/activity" view=Activity/>
                        <Route path="/settings" view=Settings/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

/// Sidebar navigation component
#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <div class="sidebar-header">
                <h1 class="logo">"Stellarr"</h1>
                <p class="tagline">"Media Management"</p>
            </div>
            <nav class="sidebar-nav">
                <A href="/" class="nav-link" active_class="active">
                    <span class="icon">"📊"</span>
                    <span class="label">"Dashboard"</span>
                </A>
                <A href="/movies" class="nav-link" active_class="active">
                    <span class="icon">"🎬"</span>
                    <span class="label">"Movies"</span>
                </A>
                <A href="/tv" class="nav-link" active_class="active">
                    <span class="icon">"📺"</span>
                    <span class="label">"TV Shows"</span>
                </A>
                <A href="/activity" class="nav-link" active_class="active">
                    <span class="icon">"⬇️"</span>
                    <span class="label">"Activity"</span>
                </A>
                <A href="/settings" class="nav-link" active_class="active">
                    <span class="icon">"⚙️"</span>
                    <span class="label">"Settings"</span>
                </A>
            </nav>
        </aside>
    }
}

/// Dark theme CSS styles
const DARK_THEME_CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

:root {
    --bg-primary: #0d1117;
    --bg-secondary: #161b22;
    --bg-tertiary: #21262d;
    --bg-hover: #30363d;
    --border-color: #30363d;
    --text-primary: #e6edf3;
    --text-secondary: #8b949e;
    --text-muted: #6e7681;
    --accent-primary: #58a6ff;
    --accent-hover: #79c0ff;
    --success: #3fb950;
    --warning: #f0883e;
    --error: #f85149;
    --shadow: rgba(0, 0, 0, 0.3);
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif;
    background-color: var(--bg-primary);
    color: var(--text-primary);
    line-height: 1.6;
}

.app-container {
    display: flex;
    min-height: 100vh;
}

.sidebar {
    width: 240px;
    background-color: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    position: fixed;
    height: 100vh;
    overflow-y: auto;
}

.sidebar-header {
    padding: 24px 20px;
    border-bottom: 1px solid var(--border-color);
}

.logo {
    font-size: 24px;
    font-weight: 700;
    color: var(--accent-primary);
    margin-bottom: 4px;
}

.tagline {
    font-size: 12px;
    color: var(--text-muted);
    font-weight: 500;
}

.sidebar-nav {
    padding: 16px 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.nav-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: 6px;
    color: var(--text-secondary);
    text-decoration: none;
    transition: all 0.2s;
    font-size: 14px;
    font-weight: 500;
}

.nav-link:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
}

.nav-link.active {
    background-color: var(--bg-tertiary);
    color: var(--accent-primary);
}

.nav-link .icon {
    font-size: 18px;
    width: 20px;
    text-align: center;
}

.main-content {
    flex: 1;
    margin-left: 240px;
    padding: 32px 40px;
    max-width: 1400px;
}

.page-header {
    margin-bottom: 32px;
}

.page-title {
    font-size: 32px;
    font-weight: 700;
    margin-bottom: 8px;
    color: var(--text-primary);
}

.page-description {
    font-size: 16px;
    color: var(--text-secondary);
}

.card {
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 20px;
    transition: all 0.2s;
}

.card:hover {
    border-color: var(--accent-primary);
    box-shadow: 0 4px 12px var(--shadow);
}

.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 20px;
    margin-bottom: 32px;
}

.stat-card {
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 24px;
    text-align: center;
}

.stat-label {
    font-size: 14px;
    color: var(--text-secondary);
    margin-bottom: 8px;
    text-transform: uppercase;
    font-weight: 600;
}

.stat-value {
    font-size: 36px;
    font-weight: 700;
    color: var(--accent-primary);
}

.media-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 20px;
}

.btn {
    padding: 10px 20px;
    border-radius: 6px;
    border: none;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: 8px;
}

.btn-primary {
    background-color: var(--accent-primary);
    color: var(--bg-primary);
}

.btn-primary:hover {
    background-color: var(--accent-hover);
}

.btn-secondary {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
}

.btn-secondary:hover {
    background-color: var(--bg-hover);
}

.search-bar {
    display: flex;
    gap: 12px;
    margin-bottom: 24px;
}

.search-input {
    flex: 1;
    padding: 12px 16px;
    border-radius: 6px;
    border: 1px solid var(--border-color);
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
    font-size: 14px;
}

.search-input:focus {
    outline: none;
    border-color: var(--accent-primary);
}

.list {
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    overflow: hidden;
}

.list-item {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.list-item:last-child {
    border-bottom: none;
}

.list-item:hover {
    background-color: var(--bg-tertiary);
}

.progress-bar {
    width: 100%;
    height: 8px;
    background-color: var(--bg-tertiary);
    border-radius: 4px;
    overflow: hidden;
}

.progress-fill {
    height: 100%;
    background-color: var(--accent-primary);
    transition: width 0.3s;
}

.form-group {
    margin-bottom: 20px;
}

.form-label {
    display: block;
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 8px;
}

.form-input {
    width: 100%;
    padding: 10px 14px;
    border-radius: 6px;
    border: 1px solid var(--border-color);
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
    font-size: 14px;
}

.form-input:focus {
    outline: none;
    border-color: var(--accent-primary);
}

.modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal {
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    max-width: 600px;
    width: 90%;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: 0 8px 32px var(--shadow);
}

.modal-header {
    padding: 24px;
    border-bottom: 1px solid var(--border-color);
}

.modal-title {
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
}

.modal-body {
    padding: 24px;
}

.modal-footer {
    padding: 20px 24px;
    border-top: 1px solid var(--border-color);
    display: flex;
    gap: 12px;
    justify-content: flex-end;
}

.badge {
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 600;
    display: inline-block;
}

.badge-success {
    background-color: rgba(63, 185, 80, 0.15);
    color: var(--success);
}

.badge-warning {
    background-color: rgba(240, 136, 62, 0.15);
    color: var(--warning);
}

.badge-error {
    background-color: rgba(248, 81, 73, 0.15);
    color: var(--error);
}

.badge-info {
    background-color: rgba(88, 166, 255, 0.15);
    color: var(--accent-primary);
}

.empty-state {
    text-align: center;
    padding: 60px 20px;
    color: var(--text-secondary);
}

.empty-state-icon {
    font-size: 64px;
    margin-bottom: 16px;
    opacity: 0.5;
}

.empty-state-title {
    font-size: 20px;
    font-weight: 600;
    margin-bottom: 8px;
    color: var(--text-primary);
}

.empty-state-message {
    font-size: 14px;
    margin-bottom: 24px;
}
"#;
