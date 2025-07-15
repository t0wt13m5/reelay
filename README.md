# Seer - RSS Feed Reader

A fast and simple command-line RSS feed reader written in Rust.

## Installation

### From crates.io

```bash
cargo install seer
```

### From source

```bash
git clone https://github.com/t0wt13m5/seer
cd seer
cargo install --path .
```

## Usage

Fetch and display an RSS feed:

```bash
seer --fetch "https://feeds.bbci.co.uk/news/rss.xml"
```

Or using the short flag:

```bash
seer -f "https://feeds.bbci.co.uk/news/rss.xml"
```

## Options

- `-f, --fetch <URL>`: The URL to fetch the RSS feed from
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples

```bash
# Fetch BBC News
seer --fetch "https://feeds.bbci.co.uk/news/rss.xml"

# Fetch Reddit RSS
seer --fetch "https://www.reddit.com/.rss"

# Get help
seer --help
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
