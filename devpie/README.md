# DevPie Equity Calculation Script

This script allows users to interact with the GitHub API to retrieve information about a repository's code commits, and to calculate an equity or revenue split based on that history.

## Current Algorithm

Author:
- 100 points awarded to the author for a commit.
- 25 points for every line of code added.
- 50 points for every line of code deleted.

Committer:
- 100 points awarded to the committer for a commit.
- 25 points for every line of code added.
- 50 points for every line of code deleted.

## Prerequisites

- Python 3.x
- A GitHub account

## Setting up the GitHub API Key

1. Login to the gh cli locally, you will need your github oauth_token in your ~/.config/gh/hosts.yml file.

Follow this if you need help: https://docs.github.com/en/github-cli/github-cli/quickstart

## Steps to run:

1. Change the "owner" and "repo" values to whatever repository you are looking to parse in the query_commits.py script.

2. Run the query_commits.py script. 

3. Run the parse_commits_csv.py script. (A file for all "*_commits.csv" files in your local directory will be parsed/created)

4. View your *_distribution.png for your repo

---

NOTES: 

1. The first version of the equity algorithm only takes into acccount the /commits endpoint for a repository. Future versions will take into consideration the /contributors and /activity endpoints to create a better picture of all the contributions to a repository, and not just its commits (which can sometimes miss other contributions).

2. GitHub as an author or committer (such as when an automated workflow performs some action/commit), are excluded from the equity calculation.

3. This project assumes, and requires a team of honorable, trustworthy, upstanding developers who are dedicated to producing something of value and doing so with integrity and strength of moral character. 

It is possible to act unscrupulously and attempt to gain more equity than you are rightly owed. If this is the case, then perhaps you are not a person to be admitted into the project for equity consideration. If their exist multiple people who attempt to game the final equity calculation for their own benefit(s), then perhaps it is not a team worth being a member of. 

There is no guarantee against bad actors, and there never will be. The importance of the humans involved matters.

It does not pay to attempt to rig the system in your favor, but the room to-do-so therefore necessitates rigorous, thorough, and continued participation and vigilance from its members; in order to protect their own equity stake, and ensure fair distribution amongst the team in is entirety.

--- 

## Future Considerations to the Equity Algorithm

Scaffolding, and boilerplate code:
1. Initialization of front-end boilerplate can add 10,000+ lines of code (easily), with a signle CLI command. This type of contribution should not have a line-count multiplier added to it, and instead should be regarded as a lesser, lone commit, where the points for the commit are 10 (normal is 100) for the author and 10 for the committer. If these boilerplate or scaffold commits are allowed to be counted as normal they would severely alter the eventual pie in favor of the lucky author.

Instead, commits such as "git init" or "npx create-solana-dapp@latest" to produce front end boilerplate should be their OWN COMMIT, with no other alterations or additions from the developer. These commits should be titled (or within the PR/commit message) with the words [BOILERPLATE, SCAFFOLDING, SCAFFOLD, or SCAFF]. The parser script will then know to treat these commits as separate, giving then a nominal point value. Again, these type of boilerplate commits should be PR'd/committed separately, with no further alterations, in order to not alter the outcome of the final pie.

2. The addition of certain other types of files, such as readme files, config files, addition of new packages, etc., should possibly be treated differently as well, in order to not skew the addition of boilerplate (or non-CORE code). Likewise, perhaps core developer additions in the form of .rs, .py, .js, or other files should perhaps be weighted more heavily by the algorithm.

3. DevPie is built for and targeted at a team of software developers and engineers for their contributions (and possibly Project Management if GitHub's Projects and boards are utilized for project management tasks (A more feature-rich PM platform is likely required here (jira, kanban, tools etc.))). However, this leave out the UI/Art & Design/Legal/PR/Advertising/PM/Executive teams (if they exist in your organization), mostly if not entirely unrepresented. Therefore, it is likely that a separate, tangential equity deal would need to be reached by the organization as a whole, where allocations for each of the afformentioned teams have been determined, and the outcome of the DevPie algorithm determines the developer equity as a smaller part of the enitre pie.

For instance, artwork committed to a repository may show as only +1 line or +1 file committed, and may be incredibly valuable to the project, but not adequately represented by the aglorithm. Outside considerations for this form of contribution and resulting equity will be needed by the wider organization/team.

An organization may determine that 50% of the overall pie goes to the developer team, and the DevPie algorithm then determines how that 50% (of the grand total), is allocated, but the other sectors/teams of the business/organization have their own base allocations that collectively equal the other 50% of the overall pie.

4. The /commits endpoint does not adequately show all contributions, contributors, nor activity to a repository. If just the /commits endpoint is considered, then some people will be left out of the final pie. The /contributors and /activity endpoints and their data should be considered as well.

---

## Commit Rules

In general, to ensure the integrity of the pie, stricter rules and procedures will need to be set and adhered-to by the team. Blind merging into main is not recommended, and manual review of code commits by the project's members/maintainers is necessary to ensure a quality codebase, and fair equity.

1. Teams should institute mandatory branch protection rules, and PR commit/reviewer rules, where > 1 reviewer is required, and the author may not self-review their own PR's.

2. All PR's should allow maintainer edits during reviews. (Keep the box checked)

3. All boilerplate code (ANY CODE THAT YOU ADD BUT DIDN'T ACTUALLY WRITE), should be committed as its OWN PR, with NO ADDITIONAL CHANGES from the authoring developer. This includes things such as "git init" or addition of a front-end boilerplate framework, addition of packages and dependencies (this may be handled differently in the future by the algorithm if possible to avoid unnecessary headaches for developers committing such files), etc..

4. Other commit rules intended to protect the integrity of the project and the final pie will be added here for consideration as they become apparently necessary.