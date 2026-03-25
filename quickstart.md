# AMADiag Quick Start

This guide is for people who do **not** regularly build software from source code.

If you can download a file, unzip it, and open a terminal, you can run AMADiag.

You do **not** need Rust or a compiler if you use the release downloads.

## What AMADiag does

AMADiag reads an Azure Monitor Agent troubleshooting bundle and tells you:

- what likely went wrong
- how serious it is
- what evidence was found
- what to do next

It works with:

- `.zip` bundles
- `.tgz` / `.tar.gz` bundles
- extracted log folders

## The easiest way to get it

Go to the project's **Releases** page and download the package for your operating system.

You should see release files like:

- `amadiag-windows-x86_64-v0.2.0.zip`
- `amadiag-linux-x86_64-v0.2.0.tar.gz`

After downloading:

- Windows: unzip the file and look for `amadiag.exe`
- Linux: extract the archive and look for `amadiag`

If you are not sure where to put it, your **Downloads** folder is fine.

## Fastest possible path

If you only want the shortest version:

1. Download the release for your operating system from **Releases**
2. Unzip or extract it
3. Open a terminal in that folder
4. Run:

### Windows

```powershell
.\amadiag.exe tui
```

### Linux

```bash
./amadiag tui
```

5. Paste the path to your AMA bundle and press `Enter`

## What you need before running it

You need one of these:

- an AMA troubleshooter `.zip` file
- an AMA troubleshooter `.tgz` or `.tar.gz` file
- a folder that already contains the extracted troubleshooting files

## Windows: step-by-step

### 1. Download and unzip

- Download the Windows release zip
- Right-click it
- Choose **Extract All**
- Open the extracted folder

You should see `amadiag.exe`.

### 2. Open a terminal in that folder

An easy way:

- click in the folder path bar in File Explorer
- type `powershell`
- press `Enter`

A PowerShell window will open in the correct folder.

### 3. Start the interactive mode

Run:

```powershell
.\amadiag.exe tui
```

You will see the interactive screen.

### 4. Enter your bundle path

Example paths:

```powershell
C:\Users\YourName\Downloads\ama-troubleshooter-output.zip
C:\Users\YourName\Downloads\ama-troubleshooter-output.tgz
C:\Users\YourName\Downloads\AMA-Diag-Logs
```

Paste the path into the TUI and press `Enter`.

### 5. Move around the results

Useful keys:

- `Up` / `Down` = move between findings
- `Tab`, `Left`, `Right` = switch panes
- `Page Up` / `Page Down` = scroll details
- `Home` / `End` = jump to first or last finding
- `m` = export Markdown report
- `j` = export JSON report
- `n` or `Esc` = go back to path entry
- `q` = quit

## Linux: step-by-step

### 1. Download and extract

Download the Linux release archive, then extract it:

```bash
tar -xzf amadiag-linux-x86_64.tar.gz
cd amadiag-linux-x86_64
```

You should see an `amadiag` binary.

### 2. Make sure it can run

```bash
chmod +x ./amadiag
```

### 3. Start the interactive mode

```bash
./amadiag tui
```

### 4. Enter your bundle path

Example paths:

```bash
/home/yourname/Downloads/ama-troubleshooter-output.tgz
/home/yourname/Downloads/ama-troubleshooter-output.zip
/home/yourname/Downloads/ama-logs
```

Paste the path into the TUI and press `Enter`.

## If you prefer a one-line command instead of the TUI

### Windows

```powershell
.\amadiag.exe analyze C:\path\to\bundle.zip
```

### Linux

```bash
./amadiag analyze /path/to/bundle.tgz
```

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

## Common problems

### "Path does not exist"

The path is wrong or the file was moved.

Check:

- the file or folder really exists
- you copied the full path
- the file extension is `.zip`, `.tgz`, or `.tar.gz`

### "Unrecognized bundle format"

AMADiag only accepts:

- `.zip`
- `.tgz`
- `.tar.gz`
- directories

### Nothing happens after typing a path

Press `Enter` after pasting the path.

### The terminal looks strange after closing

Close the terminal window and open a new one. If that happens often, report it as an issue.

## For project maintainers: how to create a release

The repository includes a GitHub Actions workflow that builds Windows and Linux binaries when you push a version tag.

Example:

```bash
git tag v0.2.0
git push origin v0.2.0
```

That workflow creates a GitHub Release and uploads packaged binaries for users to download.
