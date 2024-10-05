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
curl -L https://github.com/kawakamimoeki/cs/releases/download/v0.1.15/cs-linux-amd64 -o cs
chmod +x cs
sudo mv cs /usr/local/bin/
```


### macOS

```bash
curl -L https://github.com/kawakamimoeki/cs/releases/download/v0.1.15/cs-macos-amd64 -o cs
chmod +x cs
sudo mv cs /usr/local/bin/
```

### Windows

```powershell
Invoke-WebRequest https://github.com/kawakamimoeki/cs/releases/download/v0.1.15/cs-windows-amd64.exe -OutFile cs.exe
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

3. Find pages:
   ```
   cs find "your search query" --link
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
cs switch <project>
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the page
- `--web` or `-w`: Open the project in a web browser

### List

List page titles for a project:

```
cs ls [options]
```

Options:
- `--skip <value>` or `-s <value>`: Skip a number of pages
- `--limit <value>` or `-l <value>`: Limit the number of pages returned
- `--url` or `-u`: Display the API URL instead of fetching data
- `--link`: Include page links in the output
- `--json` or `-j`: Display the API URL instead of fetching the page

Example:
```
cs ls --limit 20 --link
```

### Page

View or open a page:

```
cs view <page> [options]
```

Options:
- `--body` or `b`: Add body to page
- `--web` or `-w`: Open the page in a web browser
- `--url` or `-u`: Display the API URL instead of fetching the page
- `--json` or `-j`: Display the API URL instead of fetching the page

Examples:
```
cs view my-page
cs view my-page --web
```

### Get resource

Get resource from a page:

```
cs get <name> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the resource
- `--resource` or `-t`: Specify the type of resource to retrieve
  - `code`: Retrieve code snippets
  - `table`: Retrieve tables
  - `icon`: Retrieve icons

Example:
```
cs get my-page/my-code-snippet --type code
cs get my-page/my-table-name --type table
cs get my-page --type icon
```

### Find

Find pages:

```
cs find <query> [options]
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
