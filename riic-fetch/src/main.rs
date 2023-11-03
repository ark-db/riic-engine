#![warn(clippy::pedantic, future_incompatible, nonstandard_style, unused)]
#![deny(
    let_underscore_drop,
    macro_use_extern_crate,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_crate_dependencies,
    unused_import_braces,
    unused_qualifications
)]

use anyhow as _;
use rayon::{join, spawn};
use riic_fetch as _;
use std::thread::spawn as spawn_std;
use std::time::Duration;
use ureq::AgentBuilder;

fn fetch() {
    todo!()
}

fn process<T, U>(_a: T, _b: U) {
    todo!()
}

fn save() {
    todo!()
}

fn main() {
    let _client = AgentBuilder::new()
        .https_only(true)
        .timeout(Duration::from_secs(10))
        .user_agent("")
        .build();

    let handle = spawn_std(|| fetch()); // fetch CN chars

    let skills_fn = || {
        let (cn_skills, en_skills) = join(
            || {
                let (chars, skills) = (fetch(), fetch()); // fetch CN base

                // use scope instead of spawn if there are ownership issues
                spawn(move || {
                    let _chars = process(chars, handle.join()); // add rarity info to chars
                    save(/* chars */); // save chars
                });
                spawn(|| {
                    // fetch + save character icons
                    // use parallel iterator
                });
                spawn(|| {
                    // fetch + save skill icons
                    // use parallel iterator
                });

                skills // return base skills
            },
            || {
                fetch() /* .skills */ // fetch EN base + return base skills
            },
        );
        let _skills = process(cn_skills, en_skills); // combine base skills
        save(/* skills */) // save base skills
    };

    let terms_fn = || {
        let (cn_terms, en_terms) = join(
            || {
                let res = fetch(); // fetch CN misc

                // use scope instead of spawn if there are ownership issues
                spawn(|| save(/* res.styles */)); // save text styles

                res /* .terms */ // return skill terms
            },
            || fetch(), /* .terms */ // fetch EN misc + return skill terms
        );
        let _terms = process(cn_terms, en_terms); // combine skill terms
        save(/* terms */); // save skill terms
    };

    join(skills_fn, terms_fn);
}
