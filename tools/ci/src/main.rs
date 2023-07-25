//! Modified from [Bevy's CI runner](https://github.com/bevyengine/bevy/tree/main/tools/ci/src)

use bitflags::bitflags;
use xshell::{cmd, Shell};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Flags: u32 {
        const FORMAT = 0b01;
        const CLIPPY = 0b10;
    }
}

const CLIPPY_FLAGS: [&str; 8] = [
    "-Aclippy::type_complexity",
    "-Wclippy::doc_markdown",
    "-Wclippy::redundant_else",
    "-Wclippy::match_same_arms",
    "-Wclippy::semicolon_if_nothing_returned",
    "-Wclippy::explicit_iter_loop",
    "-Wclippy::map_flatten",
    "-Dwarnings",
];

fn main() {
    // When run locally, results may differ from actual CI runs triggered by
    // .github/workflows/ci.yml
    // - Official CI runs latest stable
    // - Local runs use whatever the default Rust is locally

    let arguments = [
        ("lints", Flags::FORMAT | Flags::CLIPPY),
        ("format", Flags::FORMAT),
        ("clippy", Flags::CLIPPY),
    ];

    let what_to_run = if let Some(arg) = std::env::args().nth(1).as_deref() {
        if let Some((_, check)) = arguments.iter().find(|(str, _)| *str == arg) {
            *check
        } else {
            println!(
                "Invalid argument: {arg:?}.\nEnter one of: {}.",
                arguments[1..]
                    .iter()
                    .map(|(s, _)| s)
                    .fold(arguments[0].0.to_owned(), |c, v| c + ", " + v)
            );
            return;
        }
    } else {
        Flags::all()
    };

    let sh = Shell::new().unwrap();

    if what_to_run.contains(Flags::FORMAT) {
        // Check if any code needs to be formatted
        cmd!(sh, "cargo fmt --all -- --check")
            .run()
            .expect("Please run 'cargo fmt --all'");
    }

    if what_to_run.contains(Flags::CLIPPY) {
        // Check if Clippy has any complains
        cmd!(
            sh,
            "cargo clippy --workspace --all-targets --all-features -- {CLIPPY_FLAGS...}"
        )
        .run()
        .expect("Please fix clippy errors");
    }
}
