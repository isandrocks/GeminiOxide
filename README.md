# AI Interface

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
   git clone https://github.com/isandrocks/AI_interface.git
   cd AI_interface
   ```

2. Set up your environment variables:
   ```bash
   cp src/.env.example .env
   # Edit .env and set GEMINI_API_KEY=your-actual-api-key-here
   ```

3. Build and run:
   ```bash
   cargo run
   ```

## Usage

1. **Text Prompts**: Type your question or prompt in the text field and press Enter or click Generate
2. **Add Images**: Click "Add Image" to access screenshot or clipboard paste options
3. **View Responses**: AI responses are displayed with markdown formatting

## Security

🔒 **Important Security Information**

This application handles API keys and should be used securely:

### API Key Safety
- **NEVER** commit your `.env` file or API keys to version control
- Use environment variables for all sensitive configuration
- Rotate your API keys regularly
- Restrict API key usage in Google Cloud Console

### Security Features
- Automatic `.env` file gitignore protection
- Pre-commit hooks to prevent accidental secret commits
- Runtime API key validation
- Comprehensive security documentation

For detailed security guidelines, see [SECURITY.md](SECURITY.md).

## Development

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
# Check dependencies for vulnerabilities
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