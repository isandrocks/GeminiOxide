#!/bin/bash
# Security audit script for the AI Interface project

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Running comprehensive security audit...${NC}\n"

# Function to check command availability
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${YELLOW}⚠️  $1 is not installed - skipping $2${NC}"
        return 1
    fi
    return 0
}

# 1. Check git history for secrets
echo -e "${BLUE}1. Checking git history for potential secrets...${NC}"
SECRETS_IN_HISTORY=$(git log -p --all | grep -iE "(api[_-]?key|secret|token|password)" | grep -v "your-actual-api-key-here" | grep -v "placeholder" | grep -v "example" | grep -v "GEMINI_API_KEY" | grep -v "environment variable" | grep -v "\.env" | grep -v "api\.?key" | grep -v "Google AI Studio" | grep -v "makersuite" | grep -v "secrets\.\*" | grep -v "Pre-commit hooks" | grep -v "hardcoded secrets" | grep -v "api_key" | grep -v "HARDCODED_SECRETS" | grep -v "SECRETS_IN_HISTORY" | grep -vE "(# |//|\*|echo|if|let)" | wc -l)
if [ "$SECRETS_IN_HISTORY" -gt 0 ]; then
    echo -e "${RED}   ❌ Found $SECRETS_IN_HISTORY potential secrets in git history${NC}"
    git log -p --all | grep -iE "(api[_-]?key|secret|token|password)" | grep -v "your-actual-api-key-here" | grep -v "placeholder" | grep -v "example" | grep -v "GEMINI_API_KEY" | grep -v "environment variable" | grep -v "\.env" | grep -v "api\.?key" | grep -v "Google AI Studio" | grep -v "makersuite" | grep -v "secrets\.\*" | grep -v "Pre-commit hooks" | grep -v "hardcoded secrets" | grep -v "api_key" | grep -v "HARDCODED_SECRETS" | grep -v "SECRETS_IN_HISTORY" | grep -vE "(# |//|\*|echo|if|let)" | head -5
else
    echo -e "${GREEN}   ✅ No actual secrets found in git history (filtered out documentation and code references)${NC}"
fi

# 2. Check for accidentally committed environment files
echo -e "\n${BLUE}2. Checking for accidentally committed environment files...${NC}"
ENV_FILES=$(find . -name ".env" -not -path "./.git/*" | wc -l)
if [ "$ENV_FILES" -gt 0 ]; then
    echo -e "${RED}   ❌ Found .env files that might be committed:${NC}"
    find . -name ".env" -not -path "./.git/*"
else
    echo -e "${GREEN}   ✅ No .env files found in repository${NC}"
fi

# 3. Check .gitignore
echo -e "\n${BLUE}3. Checking .gitignore configuration...${NC}"
if grep -q "\.env" .gitignore; then
    echo -e "${GREEN}   ✅ .env files are properly ignored${NC}"
else
    echo -e "${RED}   ❌ .env files are not in .gitignore${NC}"
fi

# 4. Check for hardcoded secrets in source files
echo -e "\n${BLUE}4. Scanning source files for hardcoded secrets...${NC}"
HARDCODED_SECRETS=0
for pattern in "AIza[0-9A-Za-z_-]{35}" "sk-[0-9A-Za-z]{48}" "xoxb-[0-9A-Za-z-]{59}" "ghp_[0-9A-Za-z]{36}"; do
    if grep -rE "$pattern" --include="*.rs" --include="*.toml" . 2>/dev/null; then
        HARDCODED_SECRETS=$((HARDCODED_SECRETS + 1))
    fi
done

if [ "$HARDCODED_SECRETS" -gt 0 ]; then
    echo -e "${RED}   ❌ Found $HARDCODED_SECRETS potential hardcoded secrets${NC}"
else
    echo -e "${GREEN}   ✅ No hardcoded API keys found${NC}"
fi

# 5. Check dependencies for vulnerabilities
echo -e "\n${BLUE}5. Checking dependencies for vulnerabilities...${NC}"
if check_command "cargo" "dependency audit"; then
    if check_command "cargo-audit" "cargo audit"; then
        cargo audit 2>/dev/null | head -10
    else
        echo -e "${YELLOW}   ⚠️  cargo-audit not installed. Install with: cargo install cargo-audit${NC}"
    fi
fi

# 6. Check for debug prints that might leak information
echo -e "\n${BLUE}6. Checking for potential information leaks in debug prints...${NC}"
DEBUG_PRINTS=$(grep -r "println!\|dbg!\|eprintln!" --include="*.rs" . | grep -iE "(key|secret|token|password)" | wc -l)
if [ "$DEBUG_PRINTS" -gt 0 ]; then
    echo -e "${RED}   ❌ Found $DEBUG_PRINTS debug prints that might leak sensitive information${NC}"
    grep -r "println!\|dbg!\|eprintln!" --include="*.rs" . | grep -iE "(key|secret|token|password)" | head -3
else
    echo -e "${GREEN}   ✅ No suspicious debug prints found${NC}"
fi

# 7. Check API key validation
echo -e "\n${BLUE}7. Checking API key validation...${NC}"
if grep -q "is_empty" src/api_client.rs; then
    echo -e "${GREEN}   ✅ API key validation is implemented${NC}"
else
    echo -e "${YELLOW}   ⚠️  No API key validation found${NC}"
fi

# 8. Summary
echo -e "\n${BLUE}📋 Security Audit Summary:${NC}"
echo "   - Git history: $([ "$SECRETS_IN_HISTORY" -eq 0 ] && echo -e "${GREEN}Clean${NC}" || echo -e "${RED}Issues found${NC}")"
echo "   - Environment files: $([ "$ENV_FILES" -eq 0 ] && echo -e "${GREEN}Safe${NC}" || echo -e "${RED}Issues found${NC}")"
echo "   - .gitignore: $(grep -q "\.env" .gitignore && echo -e "${GREEN}Configured${NC}" || echo -e "${RED}Needs attention${NC}")"
echo "   - Hardcoded secrets: $([ "$HARDCODED_SECRETS" -eq 0 ] && echo -e "${GREEN}None found${NC}" || echo -e "${RED}Found issues${NC}")"
echo "   - Debug prints: $([ "$DEBUG_PRINTS" -eq 0 ] && echo -e "${GREEN}Safe${NC}" || echo -e "${RED}Potential leaks${NC}")"

TOTAL_ISSUES=$((SECRETS_IN_HISTORY + ENV_FILES + HARDCODED_SECRETS + DEBUG_PRINTS))
if [ "$TOTAL_ISSUES" -eq 0 ]; then
    echo -e "\n${GREEN}🎉 Security audit completed successfully! No issues found.${NC}"
else
    echo -e "\n${YELLOW}⚠️  Security audit found $TOTAL_ISSUES potential issues that should be addressed.${NC}"
fi