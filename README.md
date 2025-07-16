# unnamed-rss-manager - RSS Feed Reader

A fast and simple command-line RSS feed reader written in Rust.

## Features

### Current Features

- Fetch and display in the console RSS feeds from any URL

### Planned Features

- **SQLite integration** for persistent feed storage
- **Feed subscription management** - add, remove, and organize feeds
- **Update notifications** for subscribed feeds
- **HTML conversion** for viewing feeds in your default browser
- **Feed categorization** and tagging system
- **Export/import** functionality for feed lists

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
cargo install unnamed-rss-manager
```

### From source

### WIP \*\* TO BE RELEASED SOON

## Usage

Fetch and display an RSS feed:

```bash
unnamed-rss-manager --fetch "https://feeds.bbci.co.uk/news/rss.xml"
```

Or using the short flag:

```bash
unnamed-rss-manager -f "https://feeds.bbci.co.uk/news/rss.xml"
```

## Options

- `-f, --fetch <URL>`: The URL to fetch the RSS feed from
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

```bash
# Fetch BBC News
unnamed-rss-manager --fetch "https://feeds.bbci.co.uk/news/rss.xml"

# Fetch Reddit RSS
unnamed-rss-manager --fetch "https://www.reddit.com/.rss"

# Get help
unnamed-rss-manager --help
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
