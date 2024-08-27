import requests # type: ignore
import csv
import yaml # type: ignore
import os

# Path to the GitHub CLI configuration file
config_path = os.path.expanduser('~/.config/gh/hosts.yml')

# Read the GitHub API token from the configuration file
with open(config_path, 'r') as file:
    config = yaml.safe_load(file)
    token = config['github.com']['oauth_token']

# GitHub repository details
owner = 'Web3-Builders-Alliance'
repo = 'soda'
commits_url = f'https://api.github.com/repos/{owner}/{repo}/commits'

headers = {
    'Authorization': f'token {token}'
}

# Make a request to the GitHub API to get commit data
response = requests.get(commits_url, headers=headers)
commits = response.json()

# Format the CSV file name with the owner and repo values
csv_file_name = f'{owner}_{repo}_commits.csv'

# Open a CSV file to write the data
with open(csv_file_name, mode='w', newline='') as file:
    writer = csv.writer(file)
    # Write the header row
    writer.writerow(["SHA", "Author", "Author ID", "Committer", "Committer ID", "Date", "Message", "Lines Added", "Lines Deleted", "Verified"])

    # Write commit data
    for commit in commits:
        sha = commit['sha']
        author = commit['commit']['author']['name']
        author_id = commit['author']['id'] if commit['author'] else None
        committer = commit['commit']['committer']['name']
        committer_id = commit['committer']['id'] if commit['committer'] else None
        date = commit['commit']['author']['date']
        message = commit['commit']['message']

        # Fetch detailed commit information
        commit_url = f'https://api.github.com/repos/{owner}/{repo}/commits/{sha}'
        commit_response = requests.get(commit_url, headers=headers)
        commit_details = commit_response.json()

        # Extract lines added and deleted
        stats = commit_details.get('stats', {})
        lines_added = stats.get('additions', 0)
        lines_deleted = stats.get('deletions', 0)

        # Extract verification details
        verification = commit_details['commit'].get('verification', {})
        verified = verification.get('verified', False)

        # Skip empty lines
        if not sha.strip() or not author.strip() or not committer.strip() or not date.strip() or not message.strip():
            continue

        writer.writerow([sha, author, author_id, committer, committer_id, date, message, lines_added, lines_deleted, verified])

print("Commit data has been written to commits.csv")