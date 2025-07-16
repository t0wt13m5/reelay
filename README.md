# unnamed-rss-manager - RSS Feed Reader

A fast and simple command-line RSS feed reader written in Rust.

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
