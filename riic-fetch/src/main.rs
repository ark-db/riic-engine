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

use anyhow::{Error, Result};
use reqwest::Client;
use riic_fetch::{
    BaseData, Fetch, GetIcons, OperatorTableDe, OperatorTableSer, Server, SkillTable, TermData,
};
use std::{path::Path, sync::Arc, time::Duration};
use tokio::{
    task::{spawn, spawn_blocking, JoinHandle},
    try_join,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .https_only(true)
        .timeout(Duration::from_secs(60))
        .use_rustls_tls()
        .build()
        .expect("failed to build HTTP client");

    let ops_handle = spawn(OperatorTableDe::fetch(client.clone(), Server::CN));

    let c0 = client.clone();

    let skills_handle = spawn(async move {
        let c1 = client.clone();
        let cn_handle: JoinHandle<Result<_>> = spawn(async {
            // fetch CN base data
            let BaseData { ops, skills } = BaseData::fetch(c1.clone(), Server::CN).await?;
            let (ops, skills) = (Arc::new(ops), Arc::new(skills));

            let o = ops.clone();
            let h0 = spawn(async move {
                // transform operator data
                let ser = OperatorTableSer::create(ops_handle.await??, &o);
                // save operator data
                ser.save("../dump/chars.json")
            });

            let c2 = c1.clone();
            let h1 = spawn(async move {
                // fetch + save operator icons
                let mut set = ops.get_icons(c1.clone(), Path::new("../dump/ops/"), 60, 25);
                while let Some(res) = set.join_next().await {
                    res??;
                }
                Ok::<_, Error>(())
            });

            let s = skills.clone();
            let h2 = spawn(async move {
                // fetch + save skill icons
                let mut set = s.get_icons(c2, Path::new("../dump/skills/"), 60, 50);
                while let Some(res) = set.join_next().await {
                    res??;
                }
                Ok::<_, Error>(())
            });

            // return base skills
            Ok((skills, (h0, h1, h2)))
        });

        let c2 = client.clone();
        let en_handle: JoinHandle<Result<_>> = spawn(async {
            // fetch US base data
            let data = BaseData::fetch(c2, Server::US).await?;
            // return base skills
            Ok(data.skills)
        });

        let (cn_skills, en_skills) = try_join!(cn_handle, en_handle)?;
        let ((cn_skills, (h0, h1, h2)), en_skills) = (cn_skills?, en_skills?);

        // combine base skills
        let skills = SkillTable::combine(&cn_skills, &en_skills);

        // save base skills
        skills.save("../dump/skills.json")?;

        Ok::<_, Error>((h0, h1, h2))
    });

    let c1 = c0.clone();

    let terms_handle = spawn(async move {
        let c2 = c1.clone();
        let cn_handle: JoinHandle<Result<_>> = spawn(async {
            // fetch CN term data
            let data = TermData::fetch(c2, Server::CN).await?;

            // save text style data
            let h = spawn_blocking(move || data.styles.save("../dump/text-colors.json"));

            // return skill terms
            Ok((data.terms, h))
        });

        let c3 = c0.clone();
        let en_handle: JoinHandle<Result<_>> = spawn(async {
            // fetch US term data
            let data = TermData::fetch(c3, Server::US).await?;
            // return skill terms
            Ok(data.terms)
        });

        let (cn_terms, en_terms) = try_join!(cn_handle, en_handle)?;
        let ((mut cn_terms, h), en_terms) = (cn_terms?, en_terms?);

        // combine skill terms
        cn_terms.extend(en_terms);

        // save skill terms
        cn_terms.save("../dump/terms.json")?;

        Ok::<_, Error>(h)
    });

    // wait for all tasks to finish and catch errors
    let (skills_result, terms_result) = try_join!(skills_handle, terms_handle)?;
    let (h0, h1, h2) = skills_result?;
    let h3 = terms_result?;
    let (h0, h1, h2, h3) = try_join!(h0, h1, h2, h3)?;
    (h0?, h1?, h2?, h3?);

    Ok(())
}
