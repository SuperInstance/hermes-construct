# Projects to Skip (Do Not Migrate)

**Generated:** 2026-01-09
**Agent:** Project Assessment Agent (Agent 1)

---

## Summary

| Category | Count | Reason |
|----------|-------|--------|
| System Folders | 15 | Windows/system folders |
| Non-Code Folders | 20 | Documents, media, personal files |
| Duplicates | 5 | Already archived elsewhere |
| Empty/Minimal | 10 | No meaningful content |
| Dependencies | 3 | node_modules, build artifacts |
| **TOTAL** | **53** | |

---

## System Folders (Skip - Windows/System)

| Folder | Path | Reason |
|--------|------|--------|
| AppData | /mnt/c/users/casey/AppData | Windows application data |
| Application Data | /mnt/c/users/casey/Application Data | Windows application data |
| Contacts | /mnt/c/users/casey/Contacts | Windows contacts folder |
| Cookies | /mnt/c/users/casey/Cookies | Browser cookies |
| Desktop | /mnt/c/users/casey/Desktop | User desktop |
| Downloads | /mnt/c/users/casey/Downloads | Downloads folder |
| Favorites | /mnt/c/users/casey/Favorites | Browser favorites |
| Links | /mnt/c/users/casey/Links | Windows shortcuts |
| Local Settings | /mnt/c/users/casey/Local Settings | Windows settings |
| Music | /mnt/c/users/casey/Music | Media folder |
| My Documents | /mnt/c/users/casey/My Documents | Documents link |
| NetHood | /mnt/c/users/casey/NetHood | Network neighborhood |
| PrintHood | /mnt/c/users/casey/PrintHood | Printer shortcuts |
| Saved Games | /mnt/c/users/casey/Saved Games | Game saves |
| Searches | /mnt/c/users/casey/Searches | Windows searches |
| SendTo | /mnt/c/users/casey/SendTo | Windows send to menu |
| Start Menu | /mnt/c/users/casey/Start Menu | Windows start menu |
| Templates | /mnt/c/users/casey/Templates | Windows templates |
| Videos | /mnt/c/users/casey/Videos | Media folder |
| Recent | /mnt/c/users/casey/Recent | Recent items |
| NTUSER* | /mnt/c/users/casey/NTUSER* | Windows registry files |

---

## Non-Code Folders (Skip - Personal Documents)

| Folder | Path | Reason |
|--------|------|--------|
| Documents | /mnt/c/users/casey/Documents | Personal documents |
| OneDrive | /mnt/c/users/casey/OneDrive | Cloud storage |
| Muse Hub | /mnt/c/users/casey/Muse Hub | Application folder |
| VirtualBox VMs | /mnt/c/users/casey/VirtualBox VMs | Virtual machine files |
| writing | /mnt/c/users/casey/writing | Creative writing, not code |

---

## Empty/Placeholder Projects (Skip - No Content)

| Folder | Path | Reason |
|--------|------|--------|
| agentz | /mnt/c/users/casey/agentz | Empty, no code |
| ansel | /mnt/c/users/casey/ansel | Empty, no code |
| FrontendHub | /mnt/c/users/casey/FrontendHub | Empty |
| CodeGeeXProjects | /mnt/c/users/casey/CodeGeeXProjects | Empty container |
| gamifiedai | /mnt/c/users/casey/gamifiedai | Empty |
| go | /mnt/c/users/casey/go | Empty Go workspace |
| venv | /mnt/c/users/casey/venv | Python virtual environment |
| holo | /mnt/c/users/casey/holo | Needs investigation - subfolder with actual code |
| Guacamole | /mnt/c/users/casey/Guacamole | Application folder, not project |
| What is this file doing here | /mnt/c/users/casey/What is this file doing here | Unknown/accidental |

---

## Archive Folders (Already Archived)

| Folder | Path | Reason |
|--------|------|--------|
| project archive | /mnt/c/users/casey/project archive | Already archived |
| activelogearlier | /mnt/c/users/casey/activelogearlier | Earlier version, archived |
| wslbackup | /mnt/c/users/casey/project archive/wslbackup | WSL backup archive |

---

## Dependencies and Build Artifacts (Skip - Can Be Regenerated)

| Folder | Path | Reason |
|--------|------|--------|
| node_modules | Various | npm dependencies (regenerate with `npm install`) |
| target | Various | Rust build artifacts |
| .next | Various | Next.js build cache |
| dist | Various | Build output |
| build | Various | Build output |
| __pycache__ | Various | Python bytecode cache |
| venv | Various | Python virtual environment |

---

## Files (Not Folders) to Skip

These are files in the root directory that are not projects:

| File | Reason |
|------|--------|
| *.txt | Various text notes and ideas |
| *.md (in root) | Planning documents, not projects |
| *.bat | Batch scripts |
| *.exe | Executable files |
| *.lnk | Windows shortcuts |
| NTUSER* | Windows registry files |
| *.p12 | Certificate file |
| *.zip | Archive files |

---

## Special Cases Requiring Manual Review

| Item | Path | Notes |
|------|------|-------|
| activelog2 | /mnt/c/users/casey/activelog2 | Large (56K files), needs review |
| activelog-deploy | /mnt/c/users/casey/activelog-deploy | Large (5K+ files), deployment artifacts |
| holo | /mnt/c/users/casey/holo | Has subfolder `holos_toolkit` with actual git repo |
| podcastz | /mnt/c/users/casey/podcastz | Maps to SuperInstance/SuperInstanceCore |

---

## Recommendations

1. **Delete after verification:** System folders can be ignored
2. **Archive separately:** Personal documents should be backed up separately
3. **Review large folders:** activelog2 and activelog-deploy may need special handling
4. **Clean up build artifacts:** Add to .gitignore and don't migrate
5. **Keep documentation:** Root .md files contain valuable planning info
