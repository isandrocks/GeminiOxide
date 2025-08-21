# Security Guidelines

## API Key Management

This application requires a Gemini API key to function. Follow these security practices:

### Environment Variables
- **NEVER** commit API keys directly to the repository
- Use environment variables to store sensitive information
- Copy `.env.example` to `.env` and set your actual API key:
  ```bash
  cp src/.env.example .env
  # Edit .env and set GEMINI_API_KEY=your-actual-api-key-here
  ```

### Required Environment Variables
- `GEMINI_API_KEY`: Your Google Gemini API key (obtain from [Google AI Studio](https://makersuite.google.com/app/apikey))

### Best Practices

#### Development
1. Keep your `.env` file local and never commit it
2. Use `.env.example` to document required environment variables
3. Rotate API keys regularly
4. Use different API keys for development and production

#### Production
1. Use secure environment variable management (e.g., Docker secrets, Kubernetes secrets)
2. Enable API key restrictions in Google Cloud Console
3. Monitor API usage for unusual activity
4. Implement rate limiting and error handling

### File Security
The following files are automatically ignored by git:
- `.env` - Contains actual secrets
- `.env.*` - Any environment files (except .env.example)
- `*.key`, `*.pem`, `*.p12`, `*.pfx` - Certificate and key files
- `secrets.*` - Any files starting with "secrets"
- `config.local.*` - Local configuration files

### Security Checklist
- [ ] API key stored in environment variable only
- [ ] `.env` file is gitignored
- [ ] No hardcoded secrets in source code
- [ ] API key has appropriate restrictions in Google Cloud Console
- [ ] Regular security audits of dependencies (`cargo audit`)

### Reporting Security Issues
If you discover a security vulnerability, please:
1. **DO NOT** create a public issue
2. Email the maintainer directly with details
3. Allow time for the issue to be addressed before public disclosure

### Security Audit Commands
```bash
# Check for accidentally committed secrets
git log -p | grep -i -E "(api.?key|secret|token|password)"

# Audit dependencies
cargo audit

# Check for sensitive files
find . -name "*.env" -o -name "*.key" -o -name "*secret*"
```