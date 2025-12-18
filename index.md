# File Organizer CLI

A small command-line tool to scan folders and organize files automatically by extension.

## Problem
Folders like Downloads or project directories quickly become messy.
Manually sorting files wastes time and is error-prone.

## Solution
File Organizer automates this task directly from the terminal:
- Scan a folder and show file statistics
- Organize files into subfolders by extension
- Safe `--dry-run` mode to preview changes

## Download
Download the prebuilt binaries from GitHub Releases:

- **Linux aarch64 (required):**
  `file_organizer-aarch64-unknown-linux-musl.tar.gz`
- Linux x86_64 (bonus)

## Installation
```bash
tar -xzf file_organizer-<target>.tar.gz
chmod +x file_organizer
./file_organizer --help