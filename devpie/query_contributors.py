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
contributors_url = f'https://api.github.com/repos/{owner}/{repo}/contributors'

headers = {
    'Authorization': f'token {token}'
}

# Make a request to the GitHub API to get contributors data
response = requests.get(contributors_url, headers=headers)
contributors = response.json()

# Format the CSV file name with the owner and repo values
csv_file_name = f'{owner}_{repo}_contributors.csv'

# Open a CSV file to write the data
with open(csv_file_name, mode='w', newline='') as file:
    writer = csv.writer(file)
    # Write the header row
    writer.writerow(["Login", "ID", "Type", "Contributions"])

    # Write contributor data
    for contributor in contributors:
        login = contributor['login']
        id = contributor['id']
        type = contributor['type']
        contributions = contributor['contributions']

        writer.writerow([login, id, type, contributions])