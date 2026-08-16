-- ShCut Rust - Initial Schema

-- Users
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_ts BIGINT NOT NULL DEFAULT (unixepoch()),
    updated_ts BIGINT NOT NULL DEFAULT (unixepoch()),
    email TEXT NOT NULL UNIQUE,
    nickname TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('admin', 'user', 'view')) DEFAULT 'view'
);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Shortcuts
CREATE TABLE IF NOT EXISTS shortcuts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    creator_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_ts BIGINT NOT NULL DEFAULT (unixepoch()),
    updated_ts BIGINT NOT NULL DEFAULT (unixepoch()),
    name TEXT NOT NULL UNIQUE,
    link TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    visibility TEXT NOT NULL CHECK (visibility IN ('workspace', 'public')) DEFAULT 'workspace',
    view_count INTEGER NOT NULL DEFAULT 0,
    og_title TEXT NOT NULL DEFAULT '',
    og_description TEXT NOT NULL DEFAULT '',
    og_image TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_shortcuts_creator ON shortcuts(creator_id);
CREATE INDEX IF NOT EXISTS idx_shortcuts_name ON shortcuts(name);
CREATE INDEX IF NOT EXISTS idx_shortcuts_created ON shortcuts(created_ts DESC);

-- Tags
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);

-- Shortcut-Tag junction table
CREATE TABLE IF NOT EXISTS shortcut_tags (
    shortcut_id INTEGER NOT NULL REFERENCES shortcuts(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (shortcut_id, tag_id)
);
CREATE INDEX IF NOT EXISTS idx_shortcut_tags_tag ON shortcut_tags(tag_id);

-- Activities (extended analytics)
CREATE TABLE IF NOT EXISTS activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    creator_id INTEGER NOT NULL,
    created_ts BIGINT NOT NULL DEFAULT (unixepoch()),
    type TEXT NOT NULL,
    level TEXT NOT NULL DEFAULT 'info',
    payload TEXT NOT NULL DEFAULT '{}',
    shortcut_id INTEGER,
    referer TEXT,
    user_agent TEXT,
    ip_country TEXT,
    ip_city TEXT,
    utm_source TEXT,
    utm_medium TEXT,
    utm_campaign TEXT,
    duration_ms INTEGER
);
CREATE INDEX IF NOT EXISTS idx_activities_type ON activities(type);
CREATE INDEX IF NOT EXISTS idx_activities_shortcut ON activities(shortcut_id, type);
CREATE INDEX IF NOT EXISTS idx_activities_created ON activities(created_ts DESC);
CREATE INDEX IF NOT EXISTS idx_activities_referer ON activities(referer);
CREATE INDEX IF NOT EXISTS idx_activities_utm ON activities(utm_source, utm_medium, utm_campaign);

-- User settings
CREATE TABLE IF NOT EXISTS user_settings (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value TEXT NOT NULL DEFAULT '',
    UNIQUE(user_id, key)
);

-- Workspace settings
CREATE TABLE IF NOT EXISTS workspace_settings (
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL
);

-- Access tokens
CREATE TABLE IF NOT EXISTS access_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    created_ts BIGINT NOT NULL DEFAULT (unixepoch())
);
CREATE INDEX IF NOT EXISTS idx_access_tokens_user ON access_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_access_tokens_token ON access_tokens(token);
