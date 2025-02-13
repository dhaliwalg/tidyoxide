# tidyoxide 🧹

A lightning-fast file organization CLI tool written in Rust that automatically sorts and categorizes your messy directories.

## Features

- 🚀 Blazingly fast file operations
- 📁 Smart file categorization based on file types and patterns
- 📊 Generated reports of organizing actions
- ⚡ Zero configuration needed - works out of the box
- 🔧 Customizable rules for organization (coming soon)
- ↩️ Undo operations (coming soon)
- 🔍 Duplicate file detection (coming soon)

## Installation

```bash
cargo install tidyoxide
```

## Usage

Basic usage:
```bash
tidyoxide /path/to/directory
```

This will:
1. Scan the specified directory
2. Categorize all files
3. Create organized subdirectories
4. Move files to appropriate locations
5. Generate a summary report

## Examples

Organize your Downloads folder:
```bash
tidyoxide ~/Downloads
```

Preview what changes would be made without actually moving files:
```bash
tidyoxide --dry-run ~/Desktop
```

## Categories

Files are automatically organized into the following categories:
- 📸 Images (jpg, png, gif, etc.)
- 📄 Documents (pdf, doc, txt, etc.)
- 💻 Code (rs, js, py, etc.)
- 🎵 Audio (mp3, wav, etc.)
- 🎬 Video (mp4, mov, etc.)
- 📦 Archives (zip, rar, etc.)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
