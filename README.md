# Zed Aider Extension

A Zed extension for integrating with Aider deployment platform.

## Features

- List files in the project
- Ask Aider for code changes

## Building

1. Build the extension:
```bash
cargo build --release
```

2. Package the extension:
```bash
# Create extension directory
mkdir -p ~/.config/zed/extensions/dev/aider

# Copy extension files
cp target/release/libzed_aider.so ~/.config/zed/extensions/dev/aider/
cp extension.toml ~/.config/zed/extensions/dev/aider/

# Create extension package
cd ~/.config/zed/extensions/dev
tar -czf aider.vsix aider/
```

3. Install the packaged extension:
```bash
cp ~/.config/zed/extensions/dev/aider.vsix ~/.config/zed/extensions/
```

## Development

For local development:

1. Create symbolic links:
```bash
mkdir -p ~/.config/zed/extensions/dev/aider
ln -s "$(pwd)/target/release/libzed_aider.so" ~/.config/zed/extensions/dev/aider/
ln -s "$(pwd)/extension.toml" ~/.config/zed/extensions/dev/aider/
```

2. Restart Zed to load the extension

## Configuration

Add to your Zed settings.json:
```json
{
  "aider": {
    "api_url": "https://your-aider-instance.com",
    "api_key": "your-api-key"
  }
}
```

## Usage

- `aider.listFiles`: List files in the current workspace. Run this command from the Zed command palette.
- `aider.askAider`: Ask Aider for code changes. Run this command from the Zed command palette and provide a prompt for Aider.

## Releases

Releases are automated using GitHub Actions. To create a new release:

1.  Update the version in `Cargo.toml`.
2.  Run `./scripts/release.sh <version>` to create a git tag and push to GitHub.
3.  The GitHub Actions workflow will automatically build the release binary, create a GitHub release, and upload the extension files.
