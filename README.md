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
curl -L https://github.com/kawakamimoeki/makesense/releases/download/v0.1.0/makesense-linux-amd64 -o mks
chmod +x mks
sudo mv mks /usr/local/bin/
```


### macOS

```bash
curl -L https://github.com/kawakamimoeki/makesense/releases/download/v0.1.0/makesense-macos-amd64 -o mks
chmod +x mks
sudo mv mks /usr/local/bin/
```

### Windows

```powershell
Invoke-WebRequest https://github.com/kawakamimoeki/makesense/releases/download/v0.1.0/makesense-windows-amd64.exe -OutFile mks.exe
Move-Item .\mks.exe C:\Windows\System32\
```

## Quick Start

1. Login to your Cosense account:
   ```
   mks login your-connect-sid
   ```

2. Fetch JSON data from a project:
   ```
   mks json your-project --pretty
   ```

3. Search within a project:
   ```
   mks search your-project "your search query" --link
   ```

## Detailed Usage

### Global Options

All commands support the following global options:
- `--help`: Show help information for the command
- `--version`: Show the version of Makesense

### Login

Authenticate with Cosense using your `connect.sid` cookie:

```
mks login <your-connect-sid>
```

### JSON

Retrieve JSON data from a project or page:

```
mks json <resource> [options]
```

Options:
- `--pretty` or `-p`: Format the JSON output for better readability
- `--skip <value>` or `-s <value>`: Skip a number of pages (for project JSON)
- `--limit <value>` or `-l <value>`: Limit the number of pages returned (for project JSON)
- `--url` or `-u`: Display the API URL instead of fetching data
- `--query <value>` or `-q <value>`: Specify a search query (for search JSON)

Examples:
```
mks json my-project --pretty
mks json my-project/my-page --url
mks json my-project --query "search term" --limit 10
```

### Pages

List page titles for a project:

```
mks pages <project> [options]
```

Options:
- `--skip <value>` or `-s <value>`: Skip a number of pages
- `--limit <value>` or `-l <value>`: Limit the number of pages returned
- `--url` or `-u`: Display the API URL instead of fetching data
- `--link`: Include page links in the output

Example:
```
mks pages my-project --limit 20 --link
```

### Create

Create a new page with content:

```
mks create <page> <body> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of creating the page

Example:
```
mks create "My New Page" "This is the content of my new page."
```

### Page

View or open a page:

```
mks page <page> [options]
```

Options:
- `--web` or `-w`: Open the page in a web browser
- `--url` or `-u`: Display the API URL instead of fetching the page

Examples:
```
mks page my-project/my-page
mks page my-project/my-page --web
```

### Code

Retrieve code snippets from a page:

```
mks code <page> <name> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the code

Example:
```
mks code my-project/my-page my-code-snippet
```

### Table

Extract table data in CSV format from a page:

```
mks table <page> <name> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of fetching the table

Example:
```
mks table my-project/my-page my-table-name
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
mks icon my-project/my-page
```

### Search

Search within a project:

```
mks search <project> <query> [options]
```

Options:
- `--url` or `-u`: Display the API URL instead of performing the search
- `--link` or `-l`: Include links to the search results

Example:
```
mks search my-project "important topic" --link
```

## License

Distributed under the MIT License. See [LICENSE](LICENSE.txt) for more information.

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/kawakamimoeki/makesense/issues/new) on our GitHub repository.

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/kawakamimoeki">kawakamimoeki</a>
</p>
