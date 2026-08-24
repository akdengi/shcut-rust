#!/usr/bin/env python3
"""
Migration script: Slash NL archive → ShCut Rust database.

Migrates ONLY shortcuts and tags from the Slash archive.
- Does NOT migrate users
- Resets shortcut IDs to start from 1
- Creates all tags and shortcut-tag links
- Handles visibility conversion (PRIVATE→workspace, PUBLIC→public)
- Parses og_metadata JSON into separate fields

Usage:
    python migrate_slash.py --archive slash_prod.db --target data/shcut.db
    python migrate_slash.py --archive slash_prod.db --target data/shcut.db --reset
"""

import sqlite3
import argparse
import json
import sys
from pathlib import Path


def ensure_schema(target_db):
    """Create target schema if tables don't exist."""
    migrations_dir = Path(__file__).parent / "migrations"
    for sql_file in sorted(migrations_dir.glob("*.sql")):
        print(f"  Applying {sql_file.name}...")
        with open(sql_file) as f:
            target_db.executescript(f.read())


def clear_shortcuts_and_tags(target_db):
    """Remove all existing shortcuts, tags, and shortcut_tags."""
    print("Clearing existing shortcuts and tags...")
    target_db.execute("DELETE FROM shortcut_tags")
    target_db.execute("DELETE FROM shortcuts")
    target_db.execute("DELETE FROM tags")
    target_db.commit()
    print("  Done.")


def migrate_shortcuts(source_db, target_db):
    """Migrate shortcuts with sequential IDs starting from 1."""
    print("Migrating shortcuts...")

    source = source_db.execute(
        "SELECT id, creator_id, created_ts, updated_ts, name, link, "
        "title, description, visibility, tag, og_metadata "
        "FROM shortcut WHERE row_status='NORMAL' ORDER BY id"
    )

    new_id = 1
    count = 0

    for row in source:
        (old_id, creator_id, created_ts, updated_ts, name, link,
         title, description, visibility, tag_str, og_metadata_str) = row

        # Visibility conversion: PRIVATE→workspace, PUBLIC→public, WORKSPACE→workspace
        vis = visibility.lower()
        if vis == "private":
            vis = "workspace"
        elif vis == "public":
            vis = "public"
        elif vis == "workspace":
            vis = "workspace"
        else:
            vis = "workspace"

        # Parse og_metadata
        og_title = ""
        og_description = ""
        og_image = ""
        try:
            og = json.loads(og_metadata_str)
            og_title = og.get("title", "")
            og_description = og.get("description", "")
            og_image = og.get("image", "")
        except (json.JSONDecodeError, TypeError):
            pass

        target_db.execute(
            """INSERT INTO shortcuts
               (id, creator_id, created_ts, updated_ts, name, link, title,
                description, visibility, view_count, og_title, og_description, og_image)
               VALUES (?, 1, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, ?)""",
            (new_id, created_ts, updated_ts, name, link,
             title or "", description or "", vis,
             og_title, og_description, og_image)
        )

        # Parse tags (space-separated in Slash DB)
        if tag_str and tag_str.strip():
            for tag_name in tag_str.split():
                tag_name = tag_name.strip()
                if not tag_name:
                    continue
                target_db.execute("INSERT OR IGNORE INTO tags (name) VALUES (?)", (tag_name,))
                tag_row = target_db.execute("SELECT id FROM tags WHERE name = ?", (tag_name,)).fetchone()
                if tag_row:
                    target_db.execute(
                        "INSERT OR IGNORE INTO shortcut_tags (shortcut_id, tag_id) VALUES (?, ?)",
                        (new_id, tag_row[0])
                    )

        new_id += 1
        count += 1

    target_db.commit()
    print(f"  Migrated {count} shortcuts (IDs 1..{count})")
    return count


def migrate_tags_only(source_db, target_db):
    """Create all unique tags from Slash DB."""
    print("Ensuring all tags exist...")
    source = source_db.execute(
        "SELECT DISTINCT tag FROM shortcut WHERE row_status='NORMAL' AND tag != ''"
    )
    count = 0
    for row in source:
        tag_name = row[0].strip()
        if tag_name:
            target_db.execute("INSERT OR IGNORE INTO tags (name) VALUES (?)", (tag_name,))
            count += 1
    target_db.commit()
    print(f"  Created {count} tags")
    return count


def print_summary(target_db):
    """Print summary of target DB."""
    shortcuts = target_db.execute("SELECT COUNT(*) FROM shortcuts").fetchone()[0]
    tags = target_db.execute("SELECT COUNT(*) FROM tags").fetchone()[0]
    st_links = target_db.execute("SELECT COUNT(*) FROM shortcut_tags").fetchone()[0]
    users = target_db.execute("SELECT COUNT(*) FROM users").fetchone()[0]

    print(f"\n=== Migration Summary ===")
    print(f"  Shortcuts: {shortcuts}")
    print(f"  Tags: {tags}")
    print(f"  Shortcut-Tag links: {st_links}")
    print(f"  Users: {users} (unchanged)")

    # Show tag distribution
    tag_dist = target_db.execute(
        "SELECT t.name, COUNT(st.shortcut_id) FROM tags t "
        "LEFT JOIN shortcut_tags st ON t.id = st.tag_id "
        "GROUP BY t.id ORDER BY COUNT(st.shortcut_id) DESC"
    ).fetchall()
    if tag_dist:
        print(f"\n  Tag distribution:")
        for name, cnt in tag_dist:
            print(f"    {name}: {cnt} shortcuts")


def main():
    parser = argparse.ArgumentParser(description="Migrate Slash NL archive to ShCut Rust DB")
    parser.add_argument("--archive", required=True, help="Path to slash_prod.db")
    parser.add_argument("--target", required=True, help="Path to target shcut.db")
    parser.add_argument("--reset", action="store_true", help="Clear existing shortcuts/tags before migration")
    args = parser.parse_args()

    if not Path(args.archive).exists():
        print(f"Error: Archive DB not found: {args.archive}")
        sys.exit(1)

    # Ensure target directory exists
    Path(args.target).parent.mkdir(parents=True, exist_ok=True)

    print(f"Source: {args.archive}")
    print(f"Target: {args.target}")

    source_conn = sqlite3.connect(args.archive)
    target_conn = sqlite3.connect(args.target)

    # Ensure schema exists
    print("\nEnsuring schema...")
    ensure_schema(target_conn)

    if args.reset:
        clear_shortcuts_and_tags(target_conn)

    print("\nStarting migration...")
    try:
        migrate_shortcuts(source_conn, target_conn)
        migrate_tags_only(source_conn, target_conn)
        print_summary(target_conn)
        print("\nMigration completed successfully!")
    except Exception as e:
        print(f"\nMigration failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
    finally:
        source_conn.close()
        target_conn.close()


if __name__ == "__main__":
    main()
