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
3. Expand **"Open Graph"** section for social preview:
   - **OG Title** / **OG Description** — custom social preview text
   - **OG Image** — enter an image URL or upload a file (PNG, JPG, GIF, WebP, max 2MB)
4. Click **"Create"**

Your shortcut is available at `http://your-server:5231/s/<name>`

---

## Using UTM Parameters

UTM (Urchin Tracking Module) parameters help track where your traffic comes from. Add them to your short link URL.

### Format

```
http://your-server:5231/s/<name>?utm_source=<source>&utm_medium=<medium>&utm_campaign=<campaign>
```

### Parameters

| Parameter | Description | Example |
|-----------|-------------|---------|
| `utm_source` | Traffic source | `telegram`, `twitter`, `email`, `google` |
| `utm_medium` | Marketing medium | `post`, `banner`, `newsletter`, `cpc` |
| `utm_campaign` | Campaign name | `spring_sale`, `product_launch`, `weekly_digest` |

### Examples

**Telegram post:**
```
http://your-server:5231/s/my-product?utm_source=telegram&utm_medium=post&utm_campaign=spring2025
```

**Email newsletter:**
```
http://your-server:5231/s/my-product?utm_source=email&utm_medium=newsletter&utm_campaign=weekly
```

**Twitter ad:**
```
http://your-server:5231/s/my-product?utm_source=twitter&utm_medium=cpc&utm_campaign=awareness
```

### Where to see UTM data

Go to shortcut → Stats → UTM Sources, UTM Mediums, and UTM Campaigns sections show aggregated data for each parameter.

---

## Understanding Analytics

### What each metric means

| Metric | What it tracks | How it's collected |
|--------|---------------|-------------------|
| **Devices** | Desktop, Mobile, Tablet | From User-Agent header (always works) |
| **Browsers** | Chrome, Firefox, Safari, Edge | From User-Agent header (always works) |
| **OS** | Windows, macOS, Linux, Android, iOS | From User-Agent header (always works) |
| **Countries** | Visitor's country | From IP address via ip-api.com (requires geolocation enabled) |
| **Referrers** | Website visitor came from | From Referer header (requires referrer tracking enabled) |
| **UTM** | Marketing campaign tags | From URL parameters (requires UTM tracking enabled) |

### Why Referrers shows 0

Referrers only appear when someone clicks a link to your short URL from another website:
- Clicking a link on Twitter → referrer = `twitter.com`
- Clicking a link in email → referrer = email client
- Typing URL directly in browser → referrer = "Direct" (not counted)

To test referrers: create a link to your short URL on any webpage and click it.

### Why Countries shows 0

Countries require geolocation to be enabled in Settings → Analytics Settings. When disabled, country data is not collected.

---

## Managing Shortcuts

### Edit

- **Cards view:** hover over card → click pencil icon
- **Table view:** click pencil icon in Actions column

### Delete (admin only)

- Click trash icon → confirm deletion in dialog

### View Stats

- Click the chart icon → opens analytics page with:
  - Date range filter (from/to)
  - Total views, devices, browsers, OS, countries
  - Referrers and UTM parameters (sources, mediums, campaigns)
  - Views by date chart
  - Activity log with IP, device, browser, referrer, UTM data (paginated, 10 per page)

---

## Tags

### Using Tags

- Select tags when creating/editing shortcuts
- Filter shortcuts by tag using the dropdown above the list
- Click a tag on any shortcut card to filter by that tag

### Managing Tags

Go to **Settings → Tag Management**:
- Create new tags
- Rename existing tags (updates all shortcuts)
- Delete tags with confirmation dialog (removes from all shortcuts)

---

## User Management (admin only)

Go to **Settings → User Management**:

| Role | Permissions |
|------|------------|
| **admin** | Full access — create, edit, delete, settings |
| **user** | Create and edit own shortcuts |
| **view** | Read-only — view shortcuts and stats |

- Edit user: click "Edit" → change nickname, email, role
- Reset password: click "Edit" → enter "New Password" (admin can reset for user/view roles)
- Delete user: click "Delete" → confirm
- New users default to **view** role

---

## Password Management

### Change Your Password

1. Go to **Settings → Profile**
2. Scroll to **Change Password** section
3. Enter your current password
4. Enter and confirm your new password (min 6 characters)
5. Click **"Change Password"**

### Forgot Password (requires SMTP)

If SMTP is configured by the admin, users can reset their password:

1. Go to the login page
2. Click **"Forgot password?"**
3. Enter your email address
4. Check your inbox for a reset link (valid for 1 hour)
5. Click the link and set a new password

### Admin Reset Password

Admin can set a new password for any non-admin user:

1. Go to **Settings → User Management**
2. Click **"Edit"** next to the user
3. Enter a new password in the **"New Password"** field
4. Click **"Save"**

The user can then log in with the new password.

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
- **Analytics period:** use date range filter on stats page to analyze specific time periods
- **Reset analytics:** admin can reset all analytics data for a shortcut from the stats page
- **Close settings:** click the X button in top-right corner to return to main page
- **Loading indicators:** buttons show "Saving...", "Creating...", etc. while requests are in progress
