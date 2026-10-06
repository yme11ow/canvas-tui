# canvas-tui

A terminal user interface for [Canvas LMS](https://www.instructure.com/canvas), written in Rust with [Ratatui](https://ratatui.rs).

Browse your active courses and their sections from the terminal, with vim-style keybindings.

![screenshot](docs/screenshot.png)

> **Status:** early development. You can browse courses and their navigation tabs, but you can't open tab contents yet. See the [Roadmap](#roadmap).

---

## For users

### Features

- Lists all of your **active** Canvas courses
- Shows each course's navigation tabs as a scrolling tab bar, with **Modules, Assignments, Announcements and Grades** pinned first
- Caches tabs per course so moving between courses is fast
- Vim-style keybindings

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) **1.85 or newer** (the project uses the 2024 edition)
- A Canvas account and a personal access token

### Getting a Canvas access token

1. Log in to Canvas in your browser.
2. Go to **Account → Settings**.
3. Scroll to **Approved Integrations** and click **+ New Access Token**.
4. Give it a purpose (e.g. `canvas-tui`), then click **Generate Token**.
5. Copy the token now. Canvas won't show it again.

> Some schools turn off token generation for students. If you can't find the button, ask your school's IT department.

Your token gives full access to your Canvas account, so treat it like a password. Don't commit it or share it.

### Installation

```sh
git clone <repo-url>
cd canvas-tui
cargo install --path .
```

Or run it without installing:

```sh
cargo run --release
```

### Configuration

canvas-tui reads two environment variables:

| Variable       | Description                                              | Example                          |
| -------------- | -------------------------------------------------------- | -------------------------------- |
| `CANVAS_URL`   | Your school's Canvas URL (a trailing slash is fine)      | `https://canvas.myschool.edu`    |
| `CANVAS_TOKEN` | The personal access token you generated above           | `1234~abcdef...`                 |

The easiest way to set them is a `.env` file in the directory you run the app from:

```env
CANVAS_URL=https://canvas.myschool.edu
CANVAS_TOKEN=your-token-here
```

You can also export them in your shell instead. The app exits with an error at startup if either variable is missing.

### Keybindings

| Key              | Action                              |
| ---------------- | ----------------------------------- |
| `j` / `↓`        | Next course                         |
| `k` / `↑`        | Previous course                     |
| `l` / `]`        | Next tab                            |
| `h` / `[`        | Previous tab                        |
| `r`              | Refresh (re-fetch the current course's tabs) |
| `q`              | Quit                                |

---

## For developers

### Building and running

```sh
cargo build
cargo run
```

You need a `.env` file (see [Configuration](#configuration)) because the app fetches your courses on startup.

### Project structure

```
src/
├── main.rs              # Entry point: loads .env, builds the client, runs the event loop
├── app.rs               # App state, key handling, and the per-course tab cache
├── api/
│   ├── client.rs        # Blocking Canvas REST client (bearer auth + Link-header pagination)
│   └── models.rs        # Serde models for Canvas API responses (Course, Tabs)
└── ui/
    ├── layout.rs        # Two-pane layout (courses | tabs)
    ├── theme.rs
    ├── components/      # Reusable widgets: pane, list, tabs
    └── views/
        └── courses.rs   # Course list and tab bar state and rendering
```

### How it works

- **API client.** `CanvasClient` uses `reqwest`'s blocking client. Canvas paginates list endpoints and puts the next page in the `Link` header, so `get_paginated` follows `rel="next"` until it runs out of pages.
- **State.** `App` holds the client and the UI state. When the selected course changes, `load_tabs` checks a `HashMap<course_id, Vec<Tabs>>` cache before calling the API. `r` clears the cache.
- **Rendering.** Each view has a state struct with `handle_key` and `render` methods. The tab bar works out how many tabs fit in the pane's width and scrolls to keep the selected tab visible, showing `◀` / `▶` when more tabs are off-screen.

### Adding a Canvas endpoint

1. Add a model to `src/api/models.rs`. Use `Option<T>` for fields Canvas may leave out (e.g. on restricted courses).
2. Add a method to `CanvasClient` that calls `self.get_paginated(...)` for list endpoints.
3. Wire it into `App` and add a view under `src/ui/views/`.

See the [Canvas REST API docs](https://canvas.instructure.com/doc/api/) for available endpoints.

### Contributing

Issues and pull requests are welcome. Before opening a PR, please run:

```sh
cargo fmt
cargo clippy
```

---

## Roadmap

<!-- Fill these in. Check items off with [x] as they ship. -->

- [x] List active courses
- [x] Show course navigation tabs
- [ ] 
- [ ] 
- [ ] 

---

## License

Licensed under the [MIT License](LICENSE).
