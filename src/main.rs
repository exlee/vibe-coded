use std::{io, process::exit};

use anyhow::{Context, anyhow};
use vibe_coded::{clean_repo_dir, run_rules};
use regex::RegexSet;
use once_cell::sync::Lazy;

fn main() {
    let mut args = pico_args::Arguments::from_env();
    let url = args.free_from_str::<String>()
        .or_else(|_e| {
            println!("No URL provided. Checking clipboard...");
            get_from_clip()
        })
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e.to_string());
            eprintln!("\nRepo missing, provide argument or keep in clipboard");
            exit(1);
        });
    let clean_before = args.contains(["-c", "--clean"]);
    if clean_before {
        let _ = clean_repo_dir(&url);
    };
    let _ = run_rules(&url);
}


const GIT_URL_WORDS: &[&str] = &[
    "github",
];

fn get_from_clip() -> Result<String, anyhow::Error> {
    let mut clip = arboard::Clipboard::new().context("Can't fetch Clipboard")?;
    let text = clip.get_text().context("No text in clipboard")?.to_lowercase();
    if validate_url(&text) {
        println!("Found git-like URL in clipboard: {}", &text);
        Ok(String::from(text))
    } else {
        Err(anyhow!("Not a recognized URL in Clipboard"))
    }
}
fn validate_url(url: &str) -> bool {
    static REPO_SET: Lazy<RegexSet> = Lazy::new(|| {
        RegexSet::new([
            // GitHub: HTTPS, SSH, and git+ssh
            r"^(https?://|git@|git\+ssh://)github\.com[:/][\w.-]+/[\w.-]+(?:\.git)?/?$",
            
            // GitLab: HTTPS, SSH, and git+ssh
            r"^(https?://|git@|git\+ssh://)gitlab\.com[:/][\w.-]+/[\w.-]+(?:\.git)?/?$",
            
            // Bitbucket: HTTPS, SSH, and git+ssh
            r"^(https?://|git@|git\+ssh://)bitbucket\.org[:/][\w.-]+/[\w.-]+(?:\.git)?/?$",
            
            // SourceForge: HTTPS and SSH
            r"^(https?://|git\+ssh://|[\w.-]+@)git\.code\.sf\.net/p/[\w.-]+/[\w.-]+(?:\.git)?$",
            
            // General Azure DevOps / Launchpad / Gitea patterns (optional/generic)
            r"^(https?://|git@)[\w.-]+[:/][\w.-]+/[\w.-]+(?:\.git)?/?$",
        ]).unwrap()
    });

    REPO_SET.is_match(url)
}
