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
  os: AnalyticsEntry[]
  countries: AnalyticsEntry[]
  cities: AnalyticsEntry[]
  utm_sources: AnalyticsEntry[]
  utm_mediums: AnalyticsEntry[]
  utm_campaigns: AnalyticsEntry[]
  activities: ActivityEntry[]
  views_by_date: ViewsByDate[]
}

export interface ActivityEntry {
  id: number
  created_ts: number
  ip?: string
  device?: string
  browser?: string
  os?: string
  country?: string
  city?: string
  referer?: string
  referer_domain?: string
  utm_source?: string
  utm_medium?: string
  utm_campaign?: string
  user_agent?: string
}

export interface ViewsByDate {
  date: string
  count: number
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
