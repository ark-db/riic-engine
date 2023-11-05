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
    unused_import_braces,
    unused_qualifications
)]

use rayon::{join, scope, spawn};
use riic_fetch::{BaseData, Fetch, OperatorTableDe, OperatorTableSer, Server, TermData};
use std::{thread::spawn as spawn_std, time::Duration};
use ureq::AgentBuilder;

fn main() {
    let client = AgentBuilder::new()
        .https_only(true)
        .timeout(Duration::from_secs(10))
        .user_agent("")
        .build();

    // fetch CN operator data
    let c = client.clone();
    let handle = spawn_std(move || OperatorTableDe::fetch(c, Server::CN).unwrap());

    let skills_fn = || {
        let (mut cn_skills, en_skills) = join(
            || {
                // fetch CN base data
                let BaseData { ops, skills } = BaseData::fetch(client.clone(), Server::CN).unwrap();

                scope(|_| {
                    // transform operator data
                    let ops = OperatorTableSer::create(handle.join().unwrap(), &ops);
                    // save operator data
                    ops.save("../dump/chars.json").unwrap();
                });
                scope(|_| {
                    // fetch + save operator icons
                    ops.get_operator_icons(client.clone(), "../dump/ops/", 60, 25)
                        .unwrap();
                });
                scope(|_| {
                    // fetch + save skill icons
                    skills
                        .get_skill_icons(client.clone(), "../dump/skills/", 60, 50)
                        .unwrap()
                });

                // return base skills
                skills
            },
            || {
                // fetch US base data + return base skills
                BaseData::fetch(client.clone(), Server::US).unwrap().skills
            },
        );

        // combine base skills
        cn_skills.extend(en_skills);

        // save base skills
        cn_skills.save("../dump/skills.json").unwrap();
    };

    let terms_fn = || {
        let (mut cn_terms, en_terms) = join(
            || {
                // fetch CN term data
                let data = TermData::fetch(client.clone(), Server::CN).unwrap();

                // save text style data
                spawn(move || data.styles.save("../dump/text-colors.json").unwrap());

                // return skill terms
                data.terms
            },
            // fetch US term data + return skill terms
            || TermData::fetch(client.clone(), Server::US).unwrap().terms,
        );

        // combine skill terms
        cn_terms.extend(en_terms);

        // save skill terms
        cn_terms.save("../dump/terms.json").unwrap();
    };

    join(skills_fn, terms_fn);
}
