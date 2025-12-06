# Stellarr Web Frontend

A modern, dark-themed Leptos-based web frontend for the Stellarr media management system.

## Overview

This crate provides a fully-featured web UI for managing movies, TV shows, downloads, and system configuration. Built with Leptos 0.7, it offers a reactive, single-page application experience with server-side rendering capabilities.

## Architecture

### File Structure

```
src/
├── lib.rs                    # Main library entry point
├── app.rs                    # App shell with sidebar navigation
├── api.rs                    # Backend API client
├── components/
│   ├── mod.rs               # Component exports
│   ├── media_card.rs        # Reusable media card component
│   ├── search_bar.rs        # Search input component
│   └── modal.rs             # Modal dialog component
└── pages/
    ├── mod.rs               # Page exports
    ├── dashboard.rs         # Dashboard with stats and activity
    ├── movies.rs            # Movie library management
    ├── tv_shows.rs          # TV show library with seasons/episodes
    ├── activity.rs          # Download tracking and history
    └── settings.rs          # System configuration
```

## Features

### App Shell (app.rs)
- **Fixed sidebar navigation** with links to all major sections
- **Dark theme** with custom CSS variables
- **Responsive layout** with flexbox
- **Active route highlighting**
- Clean, modern GitHub-inspired design

### Pages

#### Dashboard (pages/dashboard.rs)
- Statistics cards: Total Movies, Total Shows, Active Downloads, Disk Space
- Quick add search bar for media
- Recent activity feed
- Real-time data loading with Leptos signals

#### Movies (pages/movies.rs)
- Grid view of movie posters
- Search integration with TMDB
- Add movie modal with search results
- Status badges (Available, Downloading, Missing, Failed)
- Click to view details

#### TV Shows (pages/tv_shows.rs)
- Grid view of show posters
- Expandable seasons view in modal
- Episode list with status indicators
- Season progress tracking (downloaded/total episodes)
- Episode codes (S01E05 format)

#### Activity (pages/activity.rs)
- Active downloads section with progress bars
- Real-time progress updates (2-second polling)
- Download speed and ETA display
- Download history section
- Status badges for different states

#### Settings (pages/settings.rs)
- Tabbed interface for different configuration sections
- **Indexers**: Add/edit/delete torrent and usenet indexers
- **Download Clients**: Configure qBittorrent, Transmission, etc.
- **Quality Profiles**: Manage quality preferences
- **General**: System-wide settings

### Components

#### MediaCard (components/media_card.rs)
- Displays poster, title, year, and status
- Fallback for missing posters
- Click handler support
- Status-based badge styling

#### SearchBar (components/search_bar.rs)
- Input field with button
- Enter key support
- Loading state handling
- Disabled state when searching

#### Modal (components/modal.rs)
- Full-screen backdrop
- Closable via backdrop click or close button
- Header with title
- Scrollable content area
- Stop propagation on modal content clicks

### API Client (api.rs)

Comprehensive REST API client with methods for:

**Core HTTP Methods:**
- `get<T>()` - GET requests
- `post<T>()` - POST requests
- `put<T>()` - PUT requests
- `delete<T>()` - DELETE requests
- `delete_no_response()` - DELETE without response body

**Convenience Methods:**
- `fetch_movies()` - Get all movies
- `fetch_series()` - Get all TV series
- `search_movies(query)` - Search TMDB for movies
- `search_tv_shows(query)` - Search TMDB for TV shows
- `add_movie(tmdb_id, quality_profile)` - Add movie to library
- `add_series(tmdb_id, quality_profile)` - Add series to library
- `fetch_active_downloads()` - Get active downloads
- `fetch_download_history()` - Get download history
- `fetch_stats()` - Get dashboard statistics
- `fetch_recent_activity()` - Get recent activity feed
- `fetch_indexers()` - Get configured indexers
- `fetch_download_clients()` - Get configured download clients
- `fetch_quality_profiles()` - Get quality profiles
- `trigger_movie_search(id)` - Trigger manual search for movie
- `trigger_series_search(id)` - Trigger manual search for series
- `delete_movie(id)` - Remove movie from library
- `delete_series(id)` - Remove series from library

## Design System

### Color Palette (Dark Theme)

```css
--bg-primary: #0d1117      /* Main background */
--bg-secondary: #161b22    /* Cards, sidebar */
--bg-tertiary: #21262d     /* Inputs, hover states */
--bg-hover: #30363d        /* Active hover */
--border-color: #30363d    /* Borders */
--text-primary: #e6edf3    /* Primary text */
--text-secondary: #8b949e  /* Secondary text */
--text-muted: #6e7681      /* Muted text */
--accent-primary: #58a6ff  /* Links, primary actions */
--accent-hover: #79c0ff    /* Hover state */
--success: #3fb950         /* Success states */
--warning: #f0883e         /* Warning states */
--error: #f85149          /* Error states */
```

### Typography

- Font family: `-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans'`
- Base line height: `1.6`
- Page titles: `32px`, weight `700`
- Card titles: `20px`, weight `700`
- Body text: `14px`

### Layout

- **Sidebar width**: `240px` (fixed)
- **Main content**: `max-width: 1400px`, padding `32px 40px`
- **Grid gaps**: `20px` for media grids
- **Card padding**: `20px - 24px`
- **Border radius**: `6px - 12px` depending on element

## Usage

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# WASM target for browser
cargo build --target wasm32-unknown-unknown
```

### Running with Trunk

```bash
# Install trunk
cargo install trunk

# Serve with hot reload
trunk serve

# Build for production
trunk build --release
```

## Dependencies

- **leptos 0.7** - Reactive web framework
- **leptos_router 0.7** - Client-side routing
- **leptos_meta 0.7** - Meta tags management
- **gloo-net 0.6** - HTTP client for WASM
- **serde** - Serialization/deserialization
- **serde_json** - JSON support
- **stellarr-core** - Shared domain models (workspace dependency)

## Browser Compatibility

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Any browser with WASM support

## Development Notes

### State Management

Uses Leptos signals for reactive state:
- `create_signal()` for local component state
- `create_effect()` for side effects and data loading
- `spawn_local()` for async operations

### Routing

Client-side routing with Leptos Router:
- `/` - Dashboard
- `/movies` - Movie library
- `/tv` - TV show library
- `/activity` - Downloads and activity
- `/settings` - Configuration

### Error Handling

All API calls return `Result<T, String>` with error messages logged to console.

### Performance

- Lazy loading of data on page mount
- Polling intervals for active downloads (2 seconds)
- Optimistic UI updates where possible
- Efficient re-renders with Leptos fine-grained reactivity

## Future Enhancements

- [ ] Real-time WebSocket updates instead of polling
- [ ] Drag-and-drop poster uploads
- [ ] Bulk actions for media items
- [ ] Advanced filtering and sorting
- [ ] Keyboard shortcuts
- [ ] Mobile-responsive sidebar collapse
- [ ] Light theme option
- [ ] Accessibility improvements (ARIA labels)
- [ ] Progressive Web App (PWA) support

## License

MIT
