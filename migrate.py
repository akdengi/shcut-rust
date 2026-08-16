#!/usr/bin/env python3
"""
Migration script from original Go version to ShCut Rust.

Usage:
    python migrate.py --source /path/to/original.db --target /path/to/new.db

This script migrates:
- Users (with role conversion: ADMIN -> admin, USER -> user)
- Shortcuts (with tag normalization and visibility conversion)
- Activities (analytics data)

IMPORTANT: Password hashes are NOT compatible between versions.
Original uses bcrypt, ShCut Rust uses Argon2id.
After migration, all users must reset their passwords.
"""

import sqlite3
import argparse
import json
import sys
from pathlib import Path


def migrate_users(source_db, target_db):
    """Migrate users with role conversion."""
    print("Migrating users...")
    
    source = source_db.execute("SELECT id, created_ts, updated_ts, email, nickname, password_hash, role FROM user")
    count = 0
    
    for row in source:
        user_id, created_ts, updated_ts, email, nickname, password_hash, role = row
        role_lower = role.lower()
        
        target_db.execute(
            """INSERT INTO users (id, created_ts, updated_ts, email, nickname, password_hash, role)
               VALUES (?, ?, ?, ?, ?, ?, ?)""",
            (user_id, created_ts, updated_ts, email, nickname, password_hash, role_lower)
        )
        count += 1
    
    target_db.commit()
    print(f"  Migrated {count} users")
    return count


def migrate_shortcuts(source_db, target_db):
    """Migrate shortcuts with tag normalization."""
    print("Migrating shortcuts...")
    
    source = source_db.execute(
        "SELECT id, creator_id, created_ts, updated_ts, name, link, title, description, visibility, tag, og_metadata FROM shortcut"
    )
    count = 0
    
    for row in source:
        (shortcut_id, creator_id, created_ts, updated_ts, name, link, title, 
         description, visibility, tag_str, og_metadata_str) = row
        
        visibility_lower = visibility.lower()
        if visibility_lower == 'private':
            visibility_lower = 'workspace'
        
        og_title = ''
        og_description = ''
        og_image = ''
        try:
            og_data = json.loads(og_metadata_str)
            og_title = og_data.get('title', '')
            og_description = og_data.get('description', '')
            og_image = og_data.get('image', '')
        except:
            pass
        
        target_db.execute(
            """INSERT INTO shortcuts (id, creator_id, created_ts, updated_ts, name, link, title, 
               description, visibility, view_count, og_title, og_description, og_image)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, ?)""",
            (shortcut_id, creator_id, created_ts, updated_ts, name, link, title or '',
             description or '', visibility_lower, og_title, og_description, og_image)
        )
        
        if tag_str and tag_str.strip():
            tags = tag_str.split()
            for tag_name in tags:
                tag_name = tag_name.strip()
                if not tag_name:
                    continue
                
                target_db.execute("INSERT OR IGNORE INTO tags (name) VALUES (?)", (tag_name,))
                tag_row = target_db.execute("SELECT id FROM tags WHERE name = ?", (tag_name,)).fetchone()
                if tag_row:
                    tag_id = tag_row[0]
                    target_db.execute(
                        "INSERT OR IGNORE INTO shortcut_tags (shortcut_id, tag_id) VALUES (?, ?)",
                        (shortcut_id, tag_id)
                    )
        
        count += 1
    
    target_db.commit()
    print(f"  Migrated {count} shortcuts")
    return count


def migrate_activities(source_db, target_db):
    """Migrate activities (analytics data)."""
    print("Migrating activities...")
    
    source = source_db.execute(
        "SELECT id, creator_id, created_ts, type, level, payload FROM activity"
    )
    count = 0
    
    for row in source:
        activity_id, creator_id, created_ts, activity_type, level, payload_str = row
        
        shortcut_id = None
        referer = None
        user_agent = None
        
        try:
            payload = json.loads(payload_str)
            shortcut_id = payload.get('shortcut_id')
            referer = payload.get('referer')
            user_agent = payload.get('user_agent')
        except:
            pass
        
        target_db.execute(
            """INSERT INTO activities (id, creator_id, created_ts, type, level, payload, 
               shortcut_id, referer, user_agent)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)""",
            (activity_id, creator_id, created_ts, activity_type, level.lower(), 
             payload_str, shortcut_id, referer, user_agent)
        )
        count += 1
    
    target_db.commit()
    print(f"  Migrated {count} activities")
    return count


def migrate_settings(source_db, target_db):
    """Migrate user and workspace settings."""
    print("Migrating settings...")
    
    source = source_db.execute("SELECT user_id, key, value FROM user_setting")
    count = 0
    for row in source:
        user_id, key, value = row
        target_db.execute(
            "INSERT OR IGNORE INTO user_settings (user_id, key, value) VALUES (?, ?, ?)",
            (user_id, key, value)
        )
        count += 1
    
    source = source_db.execute("SELECT key, value FROM workspace_setting")
    for row in source:
        key, value = row
        if 'license' in key.lower():
            continue
        target_db.execute(
            "INSERT OR IGNORE INTO workspace_settings (key, value) VALUES (?, ?)",
            (key, value)
        )
        count += 1
    
    target_db.commit()
    print(f"  Migrated {count} settings")
    return count


def main():
    parser = argparse.ArgumentParser(description='Migrate database to ShCut Rust')
    parser.add_argument('--source', required=True, help='Path to original SQLite database')
    parser.add_argument('--target', required=True, help='Path to target ShCut Rust database')
    parser.add_argument('--reset', action='store_true', help='Reset target database before migration')
    args = parser.parse_args()
    
    if not Path(args.source).exists():
        print(f"Error: Source database not found: {args.source}")
        sys.exit(1)
    
    print(f"Connecting to source: {args.source}")
    source_conn = sqlite3.connect(args.source)
    
    if args.reset and Path(args.target).exists():
        print(f"Resetting target database: {args.target}")
        Path(args.target).unlink()
    
    print(f"Connecting to target: {args.target}")
    target_conn = sqlite3.connect(args.target)
    
    migration_sql = Path(__file__).parent / 'migrations' / '001_initial.sql'
    if migration_sql.exists():
        print("Creating target schema...")
        with open(migration_sql) as f:
            target_conn.executescript(f.read())
    
    print("\nStarting migration...")
    try:
        migrate_users(source_conn, target_conn)
        migrate_shortcuts(source_conn, target_conn)
        migrate_activities(source_conn, target_conn)
        migrate_settings(source_conn, target_conn)
        
        print("\nMigration completed successfully!")
        print(f"\nTarget database: {args.target}")
        
        users = target_conn.execute("SELECT COUNT(*) FROM users").fetchone()[0]
        shortcuts = target_conn.execute("SELECT COUNT(*) FROM shortcuts").fetchone()[0]
        activities = target_conn.execute("SELECT COUNT(*) FROM activities").fetchone()[0]
        tags = target_conn.execute("SELECT COUNT(*) FROM tags").fetchone()[0]
        
        print(f"\nSummary:")
        print(f"  Users: {users}")
        print(f"  Shortcuts: {shortcuts}")
        print(f"  Activities: {activities}")
        print(f"  Tags: {tags}")
        
    except Exception as e:
        print(f"\nMigration failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
    finally:
        source_conn.close()
        target_conn.close()


if __name__ == '__main__':
    main()
