<h1 align="center">CS</h1>

<p align="center">
  <strong>An unofficial CLI tool for seamless interaction with the Cosense API</strong>
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#detailed-usage">Detailed Usage</a> •
  <a href="#contributing">Contributing</a> •
  <a href="#license">License</a>
</p>

## Installation

### Linux

```bash
curl -L https://github.com/kawakamimoeki/cs/releases/download/v0.1.10/cs-linux-amd64 -o cs
chmod +x cs
sudo mv cs /usr/local/bin/
```


### macOS

```bash
curl -L https://github.com/kawakamimoeki/cs/releases/download/v0.1.10/cs-macos-amd64 -o cs
chmod +x cs
sudo mv cs /usr/local/bin/
```

### Windows

```powershell
Invoke-WebRequest https://github.com/kawakamimoeki/cs/releases/download/v0.1.10/cs-windows-amd64.exe -OutFile cs.exe
Move-Item .\cs.exe C:\Windows\System32\
```

## Quick Start

1. Login to your Cosense account:
   ```
   cs login your-connect-sid
   ```

2. Set a project:
   ```
   cs project your-project
   ```

3. Search within a project:
   ```
   cs search "your search query" --link
   ```

## Detailed Usage

### Global Options

All commands support the following global options:
- `--help`: Show help information for the command

### Login

Authenticate with Cosense using your `connect.sid` cookie:

```
cs login <your-connect-sid>
```

### Project

Set a project:

```
cs project <project>
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the page
- `--web` or `-w`: Open the project in a web browser

### List

List page titles for a project:

```
cs list [options]
```

Options:
- `--skip <value>` or `-s <value>`: Skip a number of pages
- `--limit <value>` or `-l <value>`: Limit the number of pages returned
- `--url` or `-u`: Display the API URL instead of fetching data
- `--link`: Include page links in the output
- `--json` or `-j`: Display the API URL instead of fetching the page

Example:
```
cs list --limit 20 --link
```

### Page

View or open a page:

```
cs page <page> [options]
```

Options:
- `--body` or `b`: Add body to page
- `--web` or `-w`: Open the page in a web browser
- `--url` or `-u`: Display the API URL instead of fetching the page
- `--json` or `-j`: Display the API URL instead of fetching the page

Examples:
```
cs page my-page
cs page my-page --web
```

### Code

Retrieve code snippets from a page:

```
cs code <name> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the code

Example:
```
cs code my-page/my-code-snippet
```

### Table

Extract table data in CSV format from a page:

```
cs table <name> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the table

Example:
```
cs table my-page/my-table-name
```

### Icon

Fetch the icon of a page:

```
cs icon <page> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the icon

Example:
```
cs icon my-page
```

### Search

Search within a project:

```
cs search <query> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of performing the search
- `--link` or `-l`: Include links to the search results
- `--web` or `-w`: Open the search results in a web browser
- `--json` or `-j`: Display the API URL instead of performing the search

Example:
```
cs search "important topic" --link
```

## License

Distributed under the MIT License. See [LICENSE](LICENSE.txt) for more information.

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/kawakamimoeki/cs/issues/new) on our GitHub repository.

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/kawakamimoeki">kawakamimoeki</a>
</p>
