extern crate clap;
extern crate regex;

use clap::{Arg, ArgAction, Command};

mod git;

const NAME: &str = "generate-local-changelog";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NO_MERGES_FOUND_MESSAGE: &str = "- No merges found";
const INITIAL_RELEASE: &str = "- Initial release! 🎉";

fn render_changelog_header() {
    println!("# Changelog\n");
}

fn render_changelog(path: String, unreleased: bool, incomplete: bool) {
    let repo = git::get_repo(path).unwrap();

    let remote_origin = git::get_remote_origin(&repo);

    let git_url = git::format_remote_origin_as_url(remote_origin);

    let mut tags = git::get_repo_tags(&repo);

    if unreleased {
        tags.splice(0..0, vec!["HEAD".to_string()]);
    }

    for index in 0..tags.len() {
        let tag_a = tags.get(index).unwrap();

        println!(
            "## [{}]({}/tree/{}) - ({})\n",
            tag_a,
            git_url,
            tag_a,
            git::get_tag_date(&repo, tag_a)
        );

        if index == tags.len() - 1 {
            println!("{}", INITIAL_RELEASE);
        } else {
            let tag_b = tags.get(index + 1).unwrap();

            let merges = git::get_merges_between_tags(&repo, tag_b, tag_a, incomplete);

            println!(
                "[Full Changelog]({}/compare/{}...{})\n",
                git_url, tag_b, tag_a
            );

            if !merges.is_empty() {
                for merge in merges {
                    println!("{}", merge);
                }
            } else {
                println!("{}", NO_MERGES_FOUND_MESSAGE);
            }
        }

        println!();
    }
}

fn render_changelog_footer() {
    println!("_This changelog was generated with **[generate-local-changelog](https://github.com/neogeek/generate-local-changelog)**_")
}

fn main() {
    let mut cli = Command::new(NAME)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .version(VERSION)
        .arg(
            Arg::new("path")
                .default_value(".")
                .help("Path of git repo to generate CHANGELOG for"),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::SetTrue)
                .help("Show the version number and then exit"),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .action(ArgAction::SetTrue)
                .help("Show this help message and then exit"),
        )
        .arg(
            Arg::new("unreleased")
                .short('u')
                .long("unreleased")
                .action(ArgAction::SetTrue)
                .help("Includes unreleased merges"),
        )
        .arg(
            Arg::new("include-incomplete-merges")
                .short('i')
                .long("include-incomplete-merges")
                .action(ArgAction::SetTrue)
                .help("Includes incomplete merges"),
        );

    let version_output = cli.render_version();
    let help_output = cli.render_help();

    let matches = cli.get_matches();

    let unreleased = matches.get_flag("unreleased");
    let incomplete = matches.get_flag("include-incomplete-merges");

    if matches.get_flag("version") {
        print!("{}", version_output);
    } else if matches.get_flag("help") {
        print!("{}", help_output);
    } else {
        let path: &String = matches.get_one("path").expect("Path is required.");

        render_changelog_header();
        render_changelog(path.to_string(), unreleased, incomplete);
        render_changelog_footer();
    }
}
