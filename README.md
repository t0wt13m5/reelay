# reelay - RSS Feed Reader

A fast and simple command-line RSS feed reader written in Rust with SQLite storage.

## Features

### Current Features

- **Fetch RSS feeds** from any URL and display in the console
- **SQLite integration** for persistent feed storage
- **Feed subscription management** - add, view, and delete feeds
- **List all feeds** with IDs and subscription status
- **Show feed contents** - display all items from a specific feed
- **Delete feeds** with confirmation prompt (or force delete)
- **Automatic item storage** - feed items are saved to database during fetch

### Planned Features

- **Update notifications** for subscribed feeds
- **HTML conversion** for viewing feeds in your default browser
- **Feed categorization** and tagging system
- **Export/import** functionality for feed lists
- **Search functionality** within feed items

## Development Status

⚠️ **This project is under active development.** New features are being added regularly, and the API may change. The current version is functional but considered alpha quality.

## Contributing & Feedback

All contributions and feature requests are welcome! If you have ideas for improvements or encounter any issues:

- **Email**: t0wt13m5@gmail.com
- **Issues**: Open an issue on this repository
- **Pull Requests**: Feel free to submit PRs for bug fixes or new features

Thank you for your patience as I continue to develop this tool!

## Installation

### From crates.io

```bash
cargo install reelay
```

### From source

### WIP \*\* TO BE RELEASED SOON

## Usage

reelay uses subcommands to manage RSS feeds:

```bash
reelay <COMMAND>
```

### Commands

#### Fetch RSS feeds

```bash
# Fetch and store an RSS feed
reelay fetch "https://feeds.bbci.co.uk/news/rss.xml"
```

#### List all feeds

```bash
# Show all stored feeds with their IDs
reelay list

# Show only subscribed feeds
reelay list --subscribed
```

#### Show feed contents

```bash
# Display all items from feed with ID 1
reelay show 1
```

#### Delete feeds

```bash
# Delete feed with confirmation prompt
reelay delete 1

# Force delete without confirmation
reelay delete 1 --force
```

### Global Options

- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

```bash
# Fetch BBC News feed
reelay fetch "https://feeds.bbci.co.uk/news/rss.xml"

# List all feeds to see their IDs
reelay list

# View contents of feed ID 1
reelay show 1

# Delete feed ID 2 with confirmation
reelay delete 2

# Delete feed ID 3 without confirmation
reelay delete 3 --force

# Get help for a specific command
reelay show --help
```

## Database

reelay automatically creates a SQLite database (`feeds.db`) in the current directory to store:

- Feed metadata (URL, title, subscription status, last updated)
- Feed items (title, link, publication date, description)

The database is created automatically when you run your first `fetch` command.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
