#!/bin/bash

# Development utility script for GeminiOxide
# This script provides common development commands

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 GeminiOxide Development Utility${NC}"
echo "==============================================="

case "${1:-help}" in
    "build")
        echo -e "${BLUE}Building project...${NC}"
        cargo build
        ;;
    "run")
        echo -e "${BLUE}Running project...${NC}"
        if [ ! -f ".env" ]; then
            echo -e "${YELLOW}Warning: .env file not found. Copying from .env.example${NC}"
            cp .env.example .env
            echo -e "${YELLOW}Please edit .env and set your GEMINI_API_KEY${NC}"
        fi
        cargo run
        ;;
    "test")
        echo -e "${BLUE}Running tests...${NC}"
        cargo test
        ;;
    "check")
        echo -e "${BLUE}Checking code...${NC}"
        cargo check
        ;;
    "format")
        echo -e "${BLUE}Formatting code...${NC}"
        cargo fmt
        ;;
    "lint")
        echo -e "${BLUE}Running linter...${NC}"
        cargo clippy -- -D warnings
        ;;
    "security")
        echo -e "${BLUE}Running security audit...${NC}"
        ./scripts/security-audit.sh
        ;;
    "release")
        echo -e "${BLUE}Building release version...${NC}"
        cargo build --release
        ;;
    "clean")
        echo -e "${BLUE}Cleaning build artifacts...${NC}"
        cargo clean
        ;;
    "setup")
        echo -e "${BLUE}Setting up development environment...${NC}"
        if [ ! -f ".env" ]; then
            cp .env.example .env
            echo -e "${GREEN}✅ Created .env file from .env.example${NC}"
            echo -e "${YELLOW}Please edit .env and set your GEMINI_API_KEY${NC}"
        else
            echo -e "${GREEN}✅ .env file already exists${NC}"
        fi
        echo -e "${BLUE}Installing dependencies...${NC}"
        cargo build
        echo -e "${GREEN}✅ Setup complete! You can now run: $0 run${NC}"
        ;;
    "help"|*)
        echo "Usage: $0 <command>"
        echo ""
        echo "Available commands:"
        echo "  setup     - Set up development environment"
        echo "  build     - Build the project"
        echo "  run       - Run the project"
        echo "  test      - Run tests"
        echo "  check     - Check code for errors"
        echo "  format    - Format code with rustfmt"
        echo "  lint      - Run clippy linter"
        echo "  security  - Run security audit"
        echo "  release   - Build release version"
        echo "  clean     - Clean build artifacts"
        echo "  help      - Show this help message"
        ;;
esac