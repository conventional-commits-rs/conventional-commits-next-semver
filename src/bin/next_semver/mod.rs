use crate::report::Report;
use clap::{AppSettings, Clap};
use conventional_commits_next_semver::{
    git_commits_in_range, latest_semver_compatible_git_tag, next_version,
    types::{ConventionalCommit, TraversedCommit},
};
use git2::Repository;
use std::{borrow::Cow, process::exit};

mod report;

#[derive(Clap, Debug)]
#[clap(
    author,
    about = "Detect the next semantic release version based on your commit history",
    name = "next-semver",
    setting = AppSettings::ColoredHelp,
    version
)]
struct Args {
    /// A git revision.
    ///
    /// This is used to determine the range of commits that are parsed and used
    /// for the version detection. Any value that `git rev-parse` accepts is
    /// valid here. Has to contain `..`.
    #[clap(long)]
    rev: Option<String>,

    /// Creates and prints a report, giving more insight into the parsing
    /// process.
    #[clap(long, short)]
    report: bool,
    /// If true, returns the report and result as json.
    #[clap(long, requires("report"), short)]
    json: bool,
}

fn run(args: Args) -> anyhow::Result<()> {
    // The tool currently needs to be run inside of a git repository.
    let repo_path = std::env::current_dir()?;
    let repo = Repository::open(repo_path);
    if repo.is_err() {
        eprintln!("Error: working directory is not a git repository!");
        exit(1);
    }
    let repo = repo.unwrap();

    // If the revision is not rightly passed, exit.
    // TODO: does working with Cow's actually do anything useful here?
    let (from, to) = if let Some(rev) = args.rev {
        let splitted = rev.split("..").collect::<Vec<_>>();
        if splitted.len() != 2 {
            eprintln!("Revision string needs to include `..`");
            exit(1);
        }

        let mut from = Cow::Borrowed(splitted[0]);
        let mut to = Cow::Borrowed(splitted[1]);

        // If one of them is empty, substitute with the default value. For `from` this
        // means the latest compatible semver tag. For `to` this means the current HEAD.
        if from.is_empty() {
            // FIXME: why the heck can't I just use `?`. The compiler bitches around that
            // `repo` has to be `'static` ???
            let latest_tag = latest_semver_compatible_git_tag(&repo);
            if latest_tag.is_err() {
                eprintln!("could not find latest semver compatible git tag!");
                exit(1);
            }
            from = Cow::Owned(latest_tag.unwrap().raw);
        }
        if to.is_empty() {
            to = Cow::Owned("HEAD".to_string());
        }

        (from.to_string(), to.to_string())
    } else {
        // No revision has been specified. Use the latest viable tag and HEAD.
        // FIXME: as above
        let latest_tag = latest_semver_compatible_git_tag(&repo);
        if latest_tag.is_err() {
            eprintln!("could not find latest semver compatible git tag!");
            exit(1);
        }

        (latest_tag.unwrap().raw, "HEAD".to_string())
    };
    let rev = format!("{}..{}", &from, &to);

    // If a revision complete revision (from..to) is given, we'll just use that one.
    // If only one of the two is given, we'll fill in the other one.
    let commits = git_commits_in_range(&repo, &rev)?;
    //println!("Found {} commits in range {}", commits.len(), &rev);

    // Parse each of the commits.
    let mut traversed_commits = Vec::new();
    let commits = commits
        .into_iter()
        .map(|oid| repo.find_commit(oid))
        .collect::<Vec<_>>();
    for commit in commits.iter() {
        if let Ok(commit) = commit {
            let oid = commit.id();
            let msg = commit.message();
            if let Some(msg) = msg {
                let parsed_commit = conventional_commits_parser::parse_commit_msg(msg);
                let traversed_commit = match parsed_commit {
                    Ok(parsed) => {
                        TraversedCommit::Conventional(ConventionalCommit::from(oid, parsed))
                    }
                    Err(_) => TraversedCommit::Normal(oid),
                };
                traversed_commits.push(traversed_commit);
            }
        }
    }

    let current_version =
        latest_semver_compatible_git_tag(&repo).expect("failed to get latest version tag");
    let semver_version = current_version.version;
    let parsed_commits = traversed_commits
        .iter()
        .filter(|&c| match c {
            TraversedCommit::Conventional(_) => true,
            _ => false,
        })
        .map(|c| match c {
            TraversedCommit::Conventional(cc) => &cc.msg,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let next_version = next_version(semver_version.clone(), parsed_commits.as_slice())
        .expect("failed to find next version tag");

    if args.report {
        let report = Report {
            next_version,
            commits: traversed_commits,
            from: &from,
            to: &to,
            current_version: semver_version,
        };
        if args.json {
            println!(
                "{}",
                serde_json::to_string(&report).expect("failed to convert report to json")
            );
        } else {
            println!("{:#?}", &report);
        }
    } else {
        println!("{}", next_version);
    }

    Ok(())
}

fn main() {
    let args: Args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Error while executing: {:?}", e);
    }
}
