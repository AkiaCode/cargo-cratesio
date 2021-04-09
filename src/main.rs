use std::env;

use crates_io_api::{ListOptions, Sort};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1].is_empty() {
        return;
    }

    let client = crates_io_api::SyncClient::new(
        "cargo-cratesio (github.com/Akiacode/cargo-cratesio)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();

    let crates = client.crates(ListOptions {
        sort: Sort::Alphabetical,
        per_page: 100,
        page: 1,
        query: Some(args[1].to_string()),
    });

    match crates {
        Ok(c) => {
            if c.crates.is_empty() {
                eprintln!("cargo-cratesio: \x1b[31mNOT FOUND\x1b[39m");
                return;
            }

            println!("\nid: {}, name: {}", c.crates[0].id, c.crates[0].name);
            println!(
                "description: {}",
                c.crates[0]
                    .description
                    .as_ref()
                    .unwrap_or(&"NONE".to_owned())
            );
            println!(
                "license: {}",
                c.crates[0].license.as_ref().unwrap_or(&"NONE".to_owned())
            );
            println!(
                "documentation: {}",
                c.crates[0]
                    .documentation
                    .as_ref()
                    .unwrap_or(&"NONE".to_owned())
            );
            println!(
                "homepage: {}",
                c.crates[0].homepage.as_ref().unwrap_or(&"NONE".to_owned())
            );
            println!(
                "repository: {}",
                c.crates[0]
                    .repository
                    .as_ref()
                    .unwrap_or(&"NONE".to_owned())
            );
            println!("downloads: {}", c.crates[0].downloads);
            println!(
                "recent downloads: {}",
                c.crates[0].recent_downloads.as_ref().unwrap_or(&(0 as u64))
            );
            println!(
                "categories: {}",
                c.crates[0]
                    .categories
                    .as_ref()
                    .unwrap_or(&vec!["NONE".to_string()])
                    .join(", ")
            );
            println!(
                "keywords: {}",
                c.crates[0]
                    .keywords
                    .as_ref()
                    .unwrap_or(&vec!["NONE".to_string()])
                    .join(", ")
            );
            println!(
                "versions: {}",
                c.crates[0]
                    .versions
                    .as_ref()
                    .unwrap_or(&vec![(0 as u64)])
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<String>()
            );
            println!("max version: {}", c.crates[0].max_version);
            println!("created at: {}", c.crates[0].created_at);
            println!("updated at: {}", c.crates[0].updated_at);
            println!(
                "exact match: {}",
                c.crates[0]
                    .exact_match
                    .as_ref()
                    .unwrap_or(&false)
                    .to_string()
            );
        }
        Err(_) => {
            eprintln!("cargo-cratesio: \x1b[31mNOT FOUND\x1b[39m");
        }
    }
}
