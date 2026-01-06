# Kairo: smart URL routing

Kairo is a tool that helps you open links in your preferred app in a smart and flexible way. It's built for people who use different browsers or browser profiles to separate work, personal, and development tasks, giving you full control over where each link goes.

When you open a link, Kairo shows a small window listing all the apps on your system that can handle it. You can see their names and icons, pick one instantly, or set Kairo to remember your choice for next time. It can also act as your system's default link handler, ensuring every link you click goes through Kairo first.

For power users, Kairo includes a CLI that provides the same features without leaving the terminal.

Kairo aims to be platform-independent, working across different desktop environments such as GNOME, KDE, Hyprland, and more. It currently supports Linux, with macOS support planned for upcoming releases.

## Installation

### Arch Linux

Use one of the official AUR packages:

- [kairo](https://aur.archlinux.org/packages/kairo) if you prefer to build from source
- [kairo-bin](https://aur.archlinux.org/packages/kairo-bin) if you prefer to install the pre-built binaries

```bash
yay -S kairo
```

### Debian / Ubuntu

Download and install the provided `.deb` package in the [latest release](https://github.com/aelesbao/kairo/releases/latest).

## Set it as the default browser

Use the following command to set `kairo` as your default URL handler:

```bash
xdg-mime default kairo.desktop x-scheme-handler/http
xdg-mime default kairo.desktop x-scheme-handler/https
```

## Use it in `tmux`

You can integrate the `kairo` CLI interface with the [tmux-fzf-url](https://github.com/wfxr/tmux-fzf-url) plugin:

```tmux
# Quickly open urls using fzf
set -g @plugin 'wfxr/tmux-fzf-url'
# Open tmux-fzf-url in a popup
set -g @fzf-url-fzf-options '--tmux center,50%,50% --multi --exit-0 --no-preview --no-border'
# Open using kairo
set -g @fzf-url-open 'tmux display-popup -E kairo open'
```

## References

This project was inspired by [Junction](https://junction.sonny.re/). I tried it for a while on GNOME and liked the way it worked. However, I couldn’t find a similar tool that worked across other environments, so I decided to build one that would be portable, modern, and accessible to Linux and macOS users. Kairo is still a work in progress, but is growing toward that goal.

## License

Copyright 2025 Augusto Elesbão

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
