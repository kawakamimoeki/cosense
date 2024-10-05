<h1 align="center">Makesense</h1>

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
curl -L https://github.com/kawakamimoeki/makesense/releases/download/v0.1.3/makesense-linux-amd64 -o mks
chmod +x mks
sudo mv mks /usr/local/bin/
```


### macOS

```bash
curl -L https://github.com/kawakamimoeki/makesense/releases/download/v0.1.3/makesense-macos-amd64 -o mks
chmod +x mks
sudo mv mks /usr/local/bin/
```

### Windows

```powershell
Invoke-WebRequest https://github.com/kawakamimoeki/makesense/releases/download/v0.1.3/makesense-windows-amd64.exe -OutFile mks.exe
Move-Item .\mks.exe C:\Windows\System32\
```

## Quick Start

1. Login to your Cosense account:
   ```
   mks login your-connect-sid
   ```

2. Set a project:
   ```
   mks project your-project
   ```

3. Search within a project:
   ```
   mks search "your search query" --link
   ```

## Detailed Usage

### Global Options

All commands support the following global options:
- `--help`: Show help information for the command

### Login

Authenticate with Cosense using your `connect.sid` cookie:

```
mks login <your-connect-sid>
```

### Project

Set a project:

```
mks project <project>
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the page
- `--web` or `-w`: Open the project in a web browser

### List

List page titles for a project:

```
mks list [options]
```

Options:
- `--skip <value>` or `-s <value>`: Skip a number of pages
- `--limit <value>` or `-l <value>`: Limit the number of pages returned
- `--url` or `-u`: Display the API URL instead of fetching data
- `--link`: Include page links in the output
- `--json` or `-j`: Display the API URL instead of fetching the page

Example:
```
mks list --limit 20 --link
```

### Page

View or open a page:

```
mks page <page> [options]
```

Options:
- `--body` or `b`: Add body to page
- `--web` or `-w`: Open the page in a web browser
- `--url` or `-u`: Display the API URL instead of fetching the page
- `--json` or `-j`: Display the API URL instead of fetching the page

Examples:
```
mks page my-page
mks page my-page --web
```

### Code

Retrieve code snippets from a page:

```
mks code <name> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the code

Example:
```
mks code my-page/my-code-snippet
```

### Table

Extract table data in CSV format from a page:

```
mks table <name> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the table

Example:
```
mks table my-page/my-table-name
```

### Icon

Fetch the icon of a page:

```
mks icon <page> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the icon

Example:
```
mks icon my-page
```

### Search

Search within a project:

```
mks search <query> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of performing the search
- `--link` or `-l`: Include links to the search results
- `--web` or `-w`: Open the search results in a web browser
- `--json` or `-j`: Display the API URL instead of performing the search

Example:
```
mks search "important topic" --link
```

## License

Distributed under the MIT License. See [LICENSE](LICENSE.txt) for more information.

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/kawakamimoeki/makesense/issues/new) on our GitHub repository.

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/kawakamimoeki">kawakamimoeki</a>
</p>
