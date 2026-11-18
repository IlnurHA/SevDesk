# SevDesk

Multiple desktops **with their own files** for Windows.

Windows virtual desktops share one Desktop folder. SevDesk lets you keep several
Desktop folders and switch between them. Each "desktop" is just a directory, and
switching points the Windows Desktop to that directory, so the icons and files on
your desktop change with it.

> Status: early prototype (2022). Windows only.

## How it works

1. Every desktop is a folder on disk.
2. To switch, SevDesk sets the `Desktop` value under
   `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders`
   to the chosen folder.
3. It then restarts `explorer.exe` so the change shows up at once. The taskbar will blink for a moment.

There are two kinds of desktops:

| Kind | Where it lives | Created with |
|------|----------------|--------------|
| **Common** | `<base>\desktops\<name>` (managed by SevDesk) | `create_desk <name>` |
| **Specific** | Any existing folder you point to | `create_specific_desk <name> <path>` |

A special common desktop named `blank` is created automatically the first time
you switch to it.

## Requirements

- Windows 10/11
- [Rust](https://rustup.rs/) (stable) to build from source

## Build

```powershell
git clone https://github.com/IlnurHA/SevDesk.git
cd SevDesk
cargo build --release
# binary: target\release\several_desktops.exe
```

> **Note:** rustup's default Windows toolchain (`x86_64-pc-windows-msvc`) needs
> the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
> with the **"Desktop development with C++"** workload. Without it, `cargo build`
> fails with `link.exe not found`.

## First run

On first launch SevDesk asks for a **base path**, the folder where it keeps its
data and common desktops. If your current Windows Desktop isn't known to
SevDesk yet, it offers to register it as a specific desktop so you can always
switch back to it.

> **Tip:** before you start, add your current Desktop as a specific desktop,
> for example `create_specific_desk home C:\Users\<you>\Desktop`. That way you
> can always run `cd home` to get back to normal.

## Usage

SevDesk runs as an interactive console. Type a command and press Enter.

| Command | Description |
|---------|-------------|
| `change_desk <name>` / `cd <name>` | Switch to a desktop |
| `create_desk <name>` | Create a common desktop in `<base>\desktops` |
| `create_specific_desk <name> <path>` | Register an existing folder as a desktop |
| `remove_desk <name>` | Remove a desktop. **Deletes the files** of a common desktop; for a specific desktop, only removes it from SevDesk |
| `bind <bind> <desk>` | Create a short alias for a desktop |
| `unbind <bind>` | Remove an alias |
| `<bind>` | Any unknown word is treated as a bind, and SevDesk switches to its desktop |
| `desk_list` / `dl` | List all desktops |
| `bind_list` / `bl` | List all binds |
| `open <name>` | Open the desktop folder in Explorer |
| `add_to_autostart <cmd>;<cmd>;...` | Start SevDesk with Windows and run these commands on startup |
| `remove_from_autostart` | Remove SevDesk from autostart |
| `clear` | Clear the console |
| `help` | Show help |

Desktop and bind names must be a single word (no spaces).

### Hotkeys

While SevDesk is running, press **Left Ctrl + Left Shift + Alt + `<digit>`** to
use the bind named `<digit>`.

```text
bind 1 work
bind 2 games
# Left Ctrl + Left Shift + Alt + 2  ->  switches to "games"
```

### Autostart example

```text
add_to_autostart cd blank;clear
```

On Windows login, SevDesk starts, switches to `blank` and clears the console.

## Data files

| File | Location | Content |
|------|----------|---------|
| `base_data_file.txt` | Working directory of the process | Path to the base folder |
| `specific_desktops_data_file.txt` | `<base>` | `name>>path` per line |
| `binds.txt` | `<base>` | `bind desk` per line |
| `desktops\` | `<base>` | Common desktops |

## Restoring the default Desktop

If something goes wrong, set the registry value back by hand:

```powershell
Set-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\User Shell Folders" `
  -Name Desktop -Value "$env:USERPROFILE\Desktop"
Stop-Process -Name explorer -Force
```

## Project structure

```text
src/
  main.rs             startup, first-run setup
  command_handler.rs  command parsing and execution
  model.rs            Action enum, SpecificDesktop
  logic.rs            desktop create/switch/remove
  regchange.rs        Windows registry (desktop path, autostart)
  data_manager.rs     reading and writing data files
  shortcuts.rs        global hotkeys (rdev) and main loop
  fs_lib.rs, tools.rs helpers
```

## Known limitations

- Windows only, and switching restarts Explorer.
- Hotkeys are fixed and support only digits 0–9.
- See the issues and TODOs in the source for planned work.
