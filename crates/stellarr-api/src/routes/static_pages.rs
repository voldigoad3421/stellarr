use actix_web::{get, HttpResponse, web};

/// Serve the main HTML dashboard
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(HTML_CONTENT)
}

/// Configure web UI routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

/// Static HTML content with dark theme
const HTML_CONTENT: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Stellarr - Media Management</title>
    <style>
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
    transition: all 0.2s;
}

.stat-card:hover {
    border-color: var(--accent-primary);
    box-shadow: 0 4px 12px var(--shadow);
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

.notice-card {
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 24px;
    text-align: center;
}

.notice-icon {
    font-size: 48px;
    margin-bottom: 16px;
    opacity: 0.7;
}

.notice-title {
    font-size: 20px;
    font-weight: 600;
    margin-bottom: 8px;
    color: var(--text-primary);
}

.notice-message {
    font-size: 14px;
    color: var(--text-secondary);
}
    </style>
</head>
<body>
    <div class="app-container">
        <aside class="sidebar">
            <div class="sidebar-header">
                <h1 class="logo">Stellarr</h1>
                <p class="tagline">Media Management</p>
            </div>
            <nav class="sidebar-nav">
                <a href="/" class="nav-link active">
                    <span class="icon">📊</span>
                    <span class="label">Dashboard</span>
                </a>
                <a href="/movies" class="nav-link">
                    <span class="icon">🎬</span>
                    <span class="label">Movies</span>
                </a>
                <a href="/tv" class="nav-link">
                    <span class="icon">📺</span>
                    <span class="label">TV Shows</span>
                </a>
                <a href="/settings" class="nav-link">
                    <span class="icon">⚙️</span>
                    <span class="label">Settings</span>
                </a>
            </nav>
        </aside>
        <main class="main-content">
            <div class="page-header">
                <h1 class="page-title">Dashboard</h1>
                <p class="page-description">Overview of your media library</p>
            </div>
            
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-label">Movies</div>
                    <div class="stat-value">0</div>
                </div>
                <div class="stat-card">
                    <div class="stat-label">TV Shows</div>
                    <div class="stat-value">0</div>
                </div>
                <div class="stat-card">
                    <div class="stat-label">Episodes</div>
                    <div class="stat-value">0</div>
                </div>
                <div class="stat-card">
                    <div class="stat-label">Active Downloads</div>
                    <div class="stat-value">0</div>
                </div>
            </div>

            <div class="notice-card">
                <div class="notice-icon">🚧</div>
                <div class="notice-title">Full UI Coming Soon</div>
                <div class="notice-message">
                    The simplified dashboard is now active. The full-featured UI with search, media management, 
                    and advanced features is under development. For now, you can use the REST API at /api/*
                </div>
            </div>
        </main>
    </div>
</body>
</html>
"#;
