# GeminiOxide

A Rust GUI application that provides an interface to Google's Gemini AI, with support for text prompts and image inputs.

## Features

- Clean, intuitive GUI built with egui
- Text-based prompts to Gemini AI
- Image support (clipboard paste, screenshots, file paths)
- Markdown rendering for AI responses
- Cross-platform support

## Setup

### Prerequisites

- Rust (latest stable version)
- A Google Gemini API key

### Getting Your API Key

1. Visit [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Create a new API key
3. Copy the key for use in the next step

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/isandrocks/GeminiOxide.git
   cd GeminiOxide
   ```

2. Set up your `.env` file:
   ```bash
   # Create a .env file in the project root
   echo GEMINI_API_KEY=your-actual-api-key-here > .env
   ```
   
   Or copy from the example:
   ```bash
   cp .env.example .env
   # Edit .env and set GEMINI_API_KEY=your-actual-api-key-here
   ```

3. Build the application:
   ```bash
   cargo build --release
   ```

   The API key from your `.env` file will be compiled into the binary. You only need to distribute the `.exe` file - no separate `.env` file needed at runtime!

4. Run the application:
   ```bash
   cargo run --release
   # or run the executable directly from target/release/
   ```

## Usage

1. **Text Prompts**: Type your question or prompt in the text field and press Enter or click Generate
2. **Add Images**: Click "Add Image" to access screenshot or clipboard paste options
3. **View Responses**: AI responses are displayed with markdown formatting

## Security

🔒 **Important Security Information**

This application handles API keys and should be used securely:

### API Key Safety
- Keep your `.env` file with `GEMINI_API_KEY` in the project root during development
- The API key is read from `.env` at build time and embedded into your binary
- **NEVER** commit your `.env` file to version control (it should be in `.gitignore`)
- **NEVER** share your compiled `.exe` file publicly - it contains your API key
- Build separate binaries for different environments/users with their own API keys
- Rotate your API keys regularly
- Restrict API key usage in Google Cloud Console

### Security Features
- API key compiled into binary at build time (no runtime `.env` file needed)
- Pre-commit hooks to prevent accidental secret commits
- Comprehensive security documentation

For detailed security guidelines, see [SECURITY.md](SECURITY.md).

## Development

### Quick Start
```bash
# Set up development environment
./scripts/dev.sh setup

# Run the application
./scripts/dev.sh run

# Run all development commands
./scripts/dev.sh help
```

### Manual Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Security Audit
```bash
# Run comprehensive security audit
./scripts/security-audit.sh

# Check dependencies for vulnerabilities (requires cargo-audit)
cargo audit

# Search for potential secrets in code
git log -p | grep -i -E "(api.?key|secret|token|password)"
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure security guidelines are followed
5. Submit a pull request

Please review [SECURITY.md](SECURITY.md) before contributing.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Troubleshooting

### Common Issues

**"GEMINI_API_KEY environment variable not set"**
- Ensure you've copied `.env.example` to `.env`
- Set your actual API key in the `.env` file
- Restart the application

**"Please set a valid GEMINI_API_KEY"**
- Your API key may be invalid or placeholder text
- Verify your API key at Google AI Studio
- Check for any extra spaces or characters

**Build errors on Windows**
- Ensure you have the required build tools installed
- Some dependencies may require Visual Studio Build Tools