# AMADiag Quick Start

This guide is for people who do **not** regularly build software from source code.

If you can download a file, extract it, and open a terminal, you can run AMADiag.

You do **not** need Rust or a compiler if you use the release downloads.

## What AMADiag does

AMADiag reads an Azure Monitor Agent troubleshooting bundle and tells you:

- what likely went wrong
- how serious it is
- what evidence was found
- what to do next

It works with:

- `.zip` bundles
- `.tgz` and `.tar.gz` bundles
- extracted log folders

## The easiest way to get it

Go to the project's **Releases** page and download the package for your operating system.

After downloading:

- Windows: unzip the file and look for `amadiag.exe`
- Linux: extract the archive and look for `amadiag`

If you are not sure where to put it, your **Downloads** folder is fine.

## Fastest possible path

If you only want the shortest version:

1. Download the release for your operating system from **Releases**.
2. Extract it.
3. Open a terminal in that folder.
4. Run one of these:

### Windows

```powershell
.\amadiag.exe tui
```

### Linux

```bash
./amadiag tui
```

5. The TUI opens in a **file browser** with a path bar at the top.
6. Use the arrow keys to highlight your AMA bundle or extracted folder.
7. Press `Enter` to analyze it.

If you prefer to paste a full path instead of browsing, press `Tab` to move to the path bar, paste the path, and press `Enter`.

## What you need before running it

You need one of these:

- an AMA troubleshooter `.zip` file
- an AMA troubleshooter `.tgz` or `.tar.gz` file
- a folder that already contains the extracted troubleshooting files

## Windows: step-by-step

### 1. Download and unzip

- Download the Windows release zip, such as `amadiag-windows-x86_64-v0.4.0.zip`.
- Right-click it.
- Choose **Extract All**.
- Open the extracted folder.

You should see `amadiag.exe`.

### 2. Open a terminal in that folder

An easy way:

- click in the folder path bar in File Explorer
- type `powershell`
- press `Enter`

A PowerShell window will open in the correct folder.

### 3. Start interactive mode

```powershell
.\amadiag.exe tui
```

You will see the file browser.

### 4. Choose how to open your bundle

#### Option A: browse to it

The file browser has two areas you can switch between with `Tab`: the **file list** and the **path bar**.

File list keys:

- `Up` / `Down` = move through files and folders
- `Enter` = open a folder or analyze the selected bundle
- `Backspace` = go to the parent folder
- `Home` / `End` = jump to first or last entry
- `Tab` = move to the path bar
- `t`, `/`, or `\` = activate the path bar
- `h` = show or hide hidden files
- `Esc` or `q` = quit

#### Option B: paste a full path

Press `Tab` to move to the path bar, then paste a path like one of these and press `Enter`:

```powershell
C:\Users\YourName\Downloads\ama-troubleshooter-output.zip
C:\Users\YourName\Downloads\ama-troubleshooter-output.tgz
C:\Users\YourName\Downloads\AMA-Diag-Logs
```

Path bar keys:

- `Enter` = open the path (navigates into directories, analyzes bundles)
- `Tab` = move back to the file list
- `Ctrl+T` = move back to the file list
- `Esc` = quit

### 5. Review the results dashboard

After analysis finishes, the dashboard opens.

Useful keys:

- `Tab`, `Left`, `Right`, `Shift+Tab` = move focus between category navigator, findings, details, and evidence
- `Up` / `Down` = move in the currently focused list
- `Page Up` / `Page Down` = scroll the details or evidence pane
- `Home` / `End` = jump to the first or last item
- `1` = show critical findings only
- `2` = show critical and warning findings
- `3` = show all findings
- `t` = cycle the time filter used by the dashboard
- `m` = open Markdown export
- `j` = open JSON export
- `r` = rerun the last analysis
- `n` or `Esc` = go back to the file browser
- `q` = quit

### 6. Save a report from the TUI

When you press `m` or `j`, an export screen opens.

- edit the suggested output path if you want
- press `Tab` to switch between Markdown and JSON
- press `Enter` to save the file
- if the file already exists, press `Enter` again to confirm overwrite
- press `Esc` to cancel the export screen

## Linux: step-by-step

### 1. Download and extract

Download the Linux release archive, then extract it:

```bash
tar -xzf amadiag-linux-x86_64-v0.4.0.tar.gz
cd amadiag-linux-x86_64-v0.4.0
```

You should see an `amadiag` binary.

### 2. Make sure it can run

```bash
chmod +x ./amadiag
```

### 3. Start interactive mode

```bash
./amadiag tui
```

### 4. Pick a bundle

You can either:

- browse with the file browser and press `Enter` on the bundle, or
- press `Tab` to move to the path bar, paste a path like the examples below, and press `Enter`

```bash
/home/yourname/Downloads/ama-troubleshooter-output.tgz
/home/yourname/Downloads/ama-troubleshooter-output.zip
/home/yourname/Downloads/ama-logs
```

If you move to the path bar by mistake, press `Tab` or `Ctrl+T` to go back to the file list.

## If you prefer a one-line command instead of the TUI

### Windows

```powershell
.\amadiag.exe analyze C:\path\to\bundle.zip
```

### Linux

```bash
./amadiag analyze /path/to/bundle.tgz
```

By default, `analyze` prints a Markdown report to the terminal.

## Saving a report to a file

### Windows

```powershell
.\amadiag.exe analyze C:\path\to\bundle.zip --format json --output report.json
.\amadiag.exe analyze C:\path\to\bundle.zip --format markdown --output report.md
```

### Linux

```bash
./amadiag analyze /path/to/bundle.tgz --format json --output report.json
./amadiag analyze /path/to/bundle.tgz --format markdown --output report.md
```

If you want verbose logs, put `--verbose` before the subcommand:

### Windows

```powershell
.\amadiag.exe --verbose analyze C:\path\to\bundle.zip
```

### Linux

```bash
./amadiag --verbose analyze /path/to/bundle.tgz
```

## Other useful commands

### Validate a bundle without full analysis

```powershell
.\amadiag.exe validate C:\path\to\bundle.zip
```

```bash
./amadiag validate /path/to/bundle.tgz
```

This prints a quick summary of the bundle format, file count, total size, and XML/log/CSV counts.

### List built-in YAML rules

```powershell
.\amadiag.exe rules list
```

```bash
./amadiag rules list
```

## Common problems

### "Path does not exist"

The path is wrong or the file was moved.

Check that:

- the file or folder really exists
- you copied the full path
- the extension is `.zip`, `.tgz`, or `.tar.gz`, unless you are pointing to a directory

### "Unrecognized bundle format"

AMADiag accepts only:

- `.zip`
- `.tgz`
- `.tar.gz`
- directories

### "I want to paste a path, but the app opened a browser"

That is expected. Press `Tab` or `t` to move focus to the path bar and type a path.

### "I moved to the path bar and want the file list back"

Press `Tab` or `Ctrl+T`.

### "The terminal looks strange after closing"

Close the terminal window and open a new one. If that happens often, report it as an issue.

## For project maintainers: how to create a release

The repository includes a GitHub Actions workflow that builds Windows and Linux binaries when you push a version tag.

Example:

```bash
git tag v0.4.0
git push origin v0.4.0
```

That workflow creates a GitHub Release and uploads packaged binaries for users to download.
