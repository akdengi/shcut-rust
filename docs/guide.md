# User Guide

## Getting Started

### Login

1. Go to `http://your-server:5231`
2. Enter your email and password
3. Click "Sign in"

### Registration

If enabled by admin, new users can register at `/auth/register`. New accounts get **view** role by default — admin must upgrade to **user** to allow creating shortcuts.

---

## Creating Shortcuts

1. Click **"+ New Shortcut"** button
2. Fill in:
   - **Short name** — the URL path (e.g., `google` → `/s/google`)
   - **Target URL** — where to redirect
   - **Title** / **Description** — optional
   - **Tags** — click to select from existing tags
   - **Visibility** — `workspace` (private) or `public`
3. Click **"Create"**

Your shortcut is available at `http://your-server:5231/s/<name>`

---

## Managing Shortcuts

### Edit

- **Cards view:** hover over card → click pencil icon
- **Table view:** click pencil icon in Actions column

### Delete (admin only)

- Click trash icon → confirm deletion

### View Stats

- Click the chart icon → opens analytics page with:
  - Total views, devices, browsers, OS, countries
  - Referrers and UTM parameters
  - Views by date chart
  - Activity log with IP, device, browser, referrer

---

## Tags

### Using Tags

- Select tags when creating/editing shortcuts
- Filter shortcuts by tag using the dropdown above the list
- Click a tag on any shortcut card to filter by that tag

### Managing Tags (admin only)

Go to **Settings → Tag Management**:
- Create new tags
- Rename existing tags (updates all shortcuts)
- Delete tags (removes from all shortcuts)

---

## Analytics

### Viewing Analytics

Click the stats icon on any shortcut to see:
- **Devices** — Desktop, Mobile, Tablet
- **Browsers** — Chrome, Firefox, Safari, Edge
- **OS** — Windows, macOS, Linux, Android, iOS
- **Countries** — detected from IP address
- **Referrers** — where visitors come from
- **UTM** — marketing campaign tracking
- **Activity Log** — detailed per-visit data

### Configuring Analytics (admin only)

Go to **Settings → Analytics Settings**:
- **Analytics** — master on/off toggle
- **Geolocation** — country/city from IP (adds ~100ms)
- **UTM Parameters** — track marketing tags
- **Referrer Tracking** — track traffic sources

> Disabling geolocation speeds up redirects.

---

## User Management (admin only)

Go to **Settings → User Management**:

| Role | Permissions |
|------|------------|
| **admin** | Full access — create, edit, delete, settings |
| **user** | Create and edit own shortcuts |
| **view** | Read-only — view shortcuts and stats |

- Edit user: click "Edit" → change nickname, email, role
- Delete user: click "Delete" → confirm
- New users default to **view** role

---

## Workspace Settings (admin only)

Go to **Settings → Workspace Settings**:
- **Company Name** — shown in header and login page
- **Logo** — upload image file (PNG, JPG, GIF, SVG, WebP, max 2MB)

---

## Tips

- **Quick copy:** hover over shortcut card → click the link icon to copy `/s/name` URL
- **Tag filter:** click any tag badge to filter shortcuts by that tag
- **Dark mode:** click the sun/moon icon in the header
- **Pagination:** choose "All" in the dropdown to show all shortcuts, or select a number for paginated view
