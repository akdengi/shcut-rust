// ─── Auth ────────────────────────────────────────────────

export interface User {
  id: number
  created_ts: string
  updated_ts: string
  email: string
  nickname: string
  role: 'admin' | 'user'
}

export interface AuthResponse {
  token: string
  user: User
}

// ─── Shortcuts ───────────────────────────────────────────

export interface Shortcut {
  id: number
  creator_id: number
  created_ts: string
  updated_ts: string
  name: string
  link: string
  title: string
  description: string
  visibility: 'workspace' | 'public'
  view_count: number
  og_title: string
  og_description: string
  og_image: string
}

export interface ShortcutWithTags extends Shortcut {
  tags: string[]
}

export interface ShortcutCreatePayload {
  name: string
  link: string
  title?: string
  description?: string
  visibility?: 'workspace' | 'public'
  tags?: string[]
  og_title?: string
  og_description?: string
  og_image?: string
}

export interface ShortcutUpdatePayload extends Partial<ShortcutCreatePayload> {}

// ─── Collections ─────────────────────────────────────────

export interface Collection {
  id: number
  creator_id: number
  created_ts: string
  updated_ts: string
  name: string
  title: string
  description: string
  visibility: 'workspace' | 'public'
}

export interface CollectionWithShortcuts extends Collection {
  shortcut_ids: number[]
}

export interface CollectionCreatePayload {
  name: string
  title?: string
  description?: string
  visibility?: 'workspace' | 'public'
  shortcut_ids?: number[]
}

export interface CollectionUpdatePayload extends Partial<CollectionCreatePayload> {}

// ─── Tags ────────────────────────────────────────────────

export interface Tag {
  id: number
  name: string
}

// ─── Analytics ───────────────────────────────────────────

export interface AnalyticsEntry {
  name: string
  count: number
}

export interface ShortcutAnalytics {
  view_count: number
  references: AnalyticsEntry[]
  devices: AnalyticsEntry[]
  browsers: AnalyticsEntry[]
  countries: AnalyticsEntry[]
  utm_sources: AnalyticsEntry[]
  utm_mediums: AnalyticsEntry[]
  utm_campaigns: AnalyticsEntry[]
}

// ─── Pagination ──────────────────────────────────────────

export interface PaginatedResponse<T> {
  items: T[]
  total: number
  page: number
  per_page: number
  total_pages: number
}

// ─── Query params ────────────────────────────────────────

export interface ShortcutListParams {
  page?: number
  per_page?: number
  tag?: string
  search?: string
  visibility?: 'workspace' | 'public'
}
