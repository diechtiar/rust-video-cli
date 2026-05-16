# What I Learned Today – Rust Video CLI developer's log

## Date: May 16, 2026

### Key things I learned:

 - How to properly set up a Rust development environment on Windows using WSL2 + VS Code with Remote WSL. 
 - The importance of a clean project structure (lib.rs + cli.rs + error.rs) instead of putting everything in main.rs. Clean architecture is crucial, but I don't have any experience in rust projects yet. Let this be the starting point.
- That using `thiserror` for error handling instead of fighting with Box<dyn Error> is a smart choice.
- How to configure VS Code for Rust (format on save + clippy via rust-analyzer).
- That getting stuck on environment issues (cc linker, wrong edition, missing dependencies) is completely normal when starting with Rust — and solvable.
- What **procedural macros** are. The comparison to Angular decorators helps with understanding the concept.
- What a **crate** is.
- How to setup a minimal Rust project.

### Biggest realizations:

- Rust development is entirely different to me than frontend development at the first approach, but the tooling (especially rust-analyzer) seems to be very powerful once configured.
- Starting small with vertical slices and proper foundations early makes future work much easier.
- I went from basically 0% Rust knowledge to having a working CLI skeleton with modules, error handling, and logging in one session.

### How I feel:
I missed the feeling of excitement for a long time. This toy project got me bit mentally tired from fighting environment issues, but genuinely excited. Learning new stuff shouldn't be boring.