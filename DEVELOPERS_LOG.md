# Developers Log – Rust Video CLI

This file serves as a personal development journal. It records progress, key learnings, decisions, and reflections throughout the project.

_This log is intentionally kept personal and reflective. It helps track both technical progress and the learning journey_

---

## 2026-05-17

### What was done:
- Refactored `HttpClient` into a proper reusable struct with `get()` and `post()` methods
- Added support for custom headers via `with_header()`
- Added cookie/session support using `.cookie_store(true)`
- Added proxy support (`Proxy::all`) for debugging with mitmproxy
- Successfully routed traffic through mitmproxy and inspected requests/responses
- Improved `new()` constructor to return `Result<Self, AppError>`

### Key learnings:
- How to properly handle proxy configuration in `reqwest`
- Importance of `.danger_accept_invalid_certs(true)` when testing with mitmproxy (temporary)
- Better understanding of `Self`, associated functions, and the builder pattern
- `&str` vs `String` and when to use each

### Reflections:
Being able to see real traffic in mitmproxy is nice for my mostly non-visual type of mental processing. It makes future work on login flows much more tangible.

Rust is slowing getting comfy somewhere in my brain, but hey, it's been just two days working with it. Many concepts encountered up to this moment start to get clearer from session to session. Self, associated functions, strong function typing, focus on functional programming encouraged by implicit return (_think about returned value, instead of focusing on data flow_), and more.

---

## 2026-05-16

### What was done:
- Initialized the project with `clap`
- Created clean module structure (`cli`, `error`, `http`, `logging`)
- Added `thiserror` for proper error handling
- Added `tracing` + `tracing-subscriber` for structured logging
- Set up WSL2 + VS Code development environment
- Created initial `HttpClient` with basic `fetch_url` functionality

### Key learnings:
- Rust development feels very different from frontend work at first, but the tooling (especially rust-analyzer) becomes extremely powerful once configured
- Procedural macros (e.g. `#[derive(Parser)]`, `#[derive(Error)]`) are similar to Angular decorators + code generation
- Using `thiserror` is much better than manually implementing `From` and `Display`
- Environment issues (missing `build-essential`, wrong `edition`, missing features) are normal when starting with Rust

### Biggest realizations:
- Starting with small vertical slices and solid foundations early pays off significantly later
- Clean module structure from the beginning makes the project much more maintainable
- I went from ~0% Rust knowledge to having a working CLI with modules, error handling, logging, and HTTP capabilities surprisingly quickly

### How I feel:
Excited. After a long time, I feel genuinely motivated while coding again. The initial friction with the environment was tiring, but overcoming it and seeing the project take shape is very rewarding.

---

## Key Decisions Log

| Date       | Decision                              | Reason |
|------------|---------------------------------------|--------|
| 2026-05-16 | Use `thiserror` instead of `Box<dyn Error>` | Better error messages and maintainability |
| 2026-05-17 | Make `HttpClient::new()` return `Result` | More idiomatic and allows proper error handling |
| 2026-05-17 | Use `Proxy::all()` instead of `Proxy::http()` | Needed for HTTPS traffic when using mitmproxy the |
| 2026-05-17 | Add `.danger_accept_invalid_certs(true)` temporarily | Required to work with mitmproxy during development |

---


