# Zed Coolify Extension

A Zed extension for integrating with Coolify deployment platform.

## Features

- List Coolify servers
- List applications
- Deploy applications
- View application logs

## Building

1. Build the extension:
```bash
cargo build --release
```

2. Package the extension:
```bash
# Create extension directory
mkdir -p ~/.config/zed/extensions/dev/coolify

# Copy extension files
cp target/release/libzed_coolify.so ~/.config/zed/extensions/dev/coolify/
cp extension.toml ~/.config/zed/extensions/dev/coolify/

# Create extension package
cd ~/.config/zed/extensions/dev
tar -czf coolify.vsix coolify/
```

3. Install the packaged extension:
```bash
cp ~/.config/zed/extensions/dev/coolify.vsix ~/.config/zed/extensions/
```

## Development

For local development:

1. Create symbolic links:
```bash
mkdir -p ~/.config/zed/extensions/dev/coolify
ln -s "$(pwd)/target/release/libzed_coolify.so" ~/.config/zed/extensions/dev/coolify/
ln -s "$(pwd)/extension.toml" ~/.config/zed/extensions/dev/coolify/
```

2. Restart Zed to load the extension

## Configuration

Add to your Zed settings.json:
```json
{
  "coolify": {
    "api_url": "https://your-coolify-instance.com",
    "api_key": "your-api-key"
  }
}
```

## Usage

- `/pg-schema <table-name>`: Retrieve the schema for the table with the given name.
- `/pg-schema all-tables`: Retrieve the schemas for all tables in the database.

## Releases

Releases are automated using GitHub Actions. To create a new release:

1.  Update the version in `Cargo.toml`.
2.  Run `./scripts/release.sh <version>` to create a git tag and push to GitHub.
3.  The GitHub Actions workflow will automatically build the release binary, create a GitHub release, and upload the extension files.
