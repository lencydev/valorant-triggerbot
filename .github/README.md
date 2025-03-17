<div align = "center">
  <img src = "../assets/header.png" alt = "header" />

  <br />

  <a href = "https://github.com/lencydev/valorant-triggerbot/releases/latest"><img src = "https://img.shields.io/github/v/release/lencydev/valorant-triggerbot?style=flat-square&color=5864F9" alt = "Release" /></a>
  <a href = "https://github.com/lencydev/valorant-triggerbot/blob/main/LICENSE"><img src = "https://img.shields.io/github/license/lencydev/valorant-triggerbot?style=flat-square&color=5864F9" alt = "License" /></a>
  <a href = "https://github.com/lencydev/valorant-triggerbot/releases"><img src = "https://img.shields.io/github/downloads/lencydev/valorant-triggerbot/total?style=flat-square&color=5864F9" alt = "Downloads" /></a>
  <a href = "https://github.com/lencydev/valorant-triggerbot/forks?include=active,archived,inactive,network&page=1&period=&sort_by=last_updated"><img src = "https://img.shields.io/github/forks/lencydev/valorant-triggerbot?style=flat-square&color=5864F9" alt = "Forks" /></a>
  <a href = "https://github.com/lencydev/valorant-triggerbot/stargazers"><img src = "https://img.shields.io/github/stars/lencydev/valorant-triggerbot?style=flat-square&color=5864F9" alt = "Stars" /></a>
</div>

# Usage
1. Download the latest release from the [releases](https://github.com/lencydev/valorant-triggerbot/releases) page.
2. Run the exe file and click the `Enable` button to enable the triggerbot.

### Valorant Settings
These are the in-game settings required for the triggerbot to work.
- `General > Accessibility > Enemy Highlight Color:` Select `Purple`.
- `Controls > Equipment > Weapons > Fire:` Set the secondary key to `K`.

### Default Settings
These are the default and recommended settings.
- `Resolution:` 1920x1080 (If your in-game resolution is different, change it)
- `Trigger Keys:` Left Shift
- `Trigger Delay:` 50ms
- `Trigger Area:` 5.0
- `Target Color:` Purple (RGB: 250, 100, 250)
- `Color Tolerance:` 70

# Building from Source
- If you prefer to build the application yourself, follow these steps:

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Clone the repository:
   ```bash
   git clone https://github.com/lencydev/valorant-triggerbot.git
   cd valorant-triggerbot
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. After the compilation:
   - Navigate to the `target/release` directory.
   - Find the `valorant-triggerbot.exe` file.
   - Run exe to start the application.

# Disclaimer
> [!WARNING]
> Currently, this application is not detectable by Vanguard.<br />
> However, this may change in the future as Riot Games continuously updates their security measures.

- The author is not responsible for any consequences resulting from the use of this software.
- Using this application violates the game's terms of service and may result in your account being banned.

# Support
If you need help, you can message me on [discord](https://discord.com/users/313738210729656332).

<hr />

<div align = "center">
  <video src = "https://github.com/user-attachments/assets/c2a3a180-24ae-4ed2-9c6d-b27732631dc2" />
</div>
