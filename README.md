# Panaseeya
An aesthetic and no-brainer place to catalog and access your games, in real-ish time, from Steam, GOG, Epic, Amazon, as well as your private collection.

The key strategy is going to be to build a database managment/data lake/media procurement software, like Heroic, from the outputs of the CLI tools for Steam (SteamCMD), Gog (most commonly gogdl and gog-cli), Epic (most commonly with Legendary) and Amazon (with Nile). The 'flow' of this app is that the app calls game library lists from the CLI tools for your logged in accounts (and you can log in via the tool) with cover images and such. The game identifies identical titles and groups them together as available on multiple platforms (as on heroic). If a game exists on a package manager or a site, like dwarf fortress, I want that accounted for, and I want to also be albe to log both physical and digital copies of games that you own that are not on exchanges, or could be Roms.

## Status

The first piece exists: a headless catalog core with a command-line program.
It keeps one local catalog, imports an Epic library through
[Legendary](https://github.com/legendary-gl/legendary), records games you add by
hand, and groups copies of the same title. There is no graphical interface,
cover art, or Steam, GOG, or Amazon import yet.

The Epic import has so far been tested only against a hand-built sample of
Legendary's output, not a real library.

## Usage

Build with a stable Rust toolchain:

```
cargo build --release
```

Record a game no store knows about (`--medium` is `physical`, `digital`, or
`rom`; `--source` is any label you like):

```
panaseeya add "Chrono Trigger" --source shelf --medium rom
```

Import your Epic library. Install Legendary and sign in with `legendary auth`
first; Panaseeya runs `legendary list --json` and reads the result:

```
panaseeya import legendary
```

Or import a document you saved earlier with `legendary list --json > epic.json`:

```
panaseeya import legendary --file epic.json
```

List everything, one line per title with every place you own it, or as JSON:

```
panaseeya list
panaseeya list --json
```

The catalog is a single SQLite file, `catalog.db`, in your per-user data
directory. Choose another file with `--db <path>`, or another directory with
the `PANASEEYA_DATA_DIR` environment variable. Set `PANASEEYA_LEGENDARY_BIN` if
Legendary is not on your `PATH`.

Titles are grouped when they contain the same words, ignoring case and
punctuation: "Half-Life" and "half life" are one title, while "Portal" and
"Portal 2" are two.

## Vision

Basically I want this to be like heroic, but with a lot more data. I want it to be a nicely presented app with images of games and coverart and such, as well as the ability to easily access DLC and mods in one place (so perhaps a nexus mods connection as well). It would be cool to see total playtime on identical titles for example to see how much you really play (which would be extra cool if consoles had APIs or CLI tools, so you could know the true total, but i don't know if they do). 

