#!/bin/bash
# Test script to reproduce the human checker plaintext bug
#
# Bug: When human checker confirms a plaintext, the final result shows wrong text
#
# To run: ./tests/scripts/test_human_checker_bug.sh
#
# Prerequisites: 
#   - cargo build (the binary must exist)
#   - expect (for automating stdin) OR run manually
#
# Expected behavior:
#   - Human checker prompt shows: 'hello my baby hello my darling'
#   - After confirming 'y', the plaintext should be: 'hello my baby hello my darling'
#
# Bug behavior:
#   - After confirming 'y', the plaintext is empty or 'Words' instead

set -e

# Build the project first
echo "Building ciphey..."
cargo build --release 2>/dev/null || cargo build

# The encoded input from the bug report
INPUT="WkZoS05XVlhTV2RsYlhkbllqSTFkbUpEUWpGamJtdzFXV2xDTm1KRFFuaGliVlkxWkcxR01BPT0="

echo ""
echo "=============================================="
echo "Human Checker Bug Reproduction Test"
echo "=============================================="
echo ""
echo "Input: $INPUT"
echo ""
echo "Running ciphey with human checker enabled..."
echo "When prompted, type 'y' and press Enter."
echo ""

# Check if expect is available
if command -v expect &> /dev/null; then
    echo "Using expect for automated testing..."
    
    # Create temporary expect script
    EXPECT_SCRIPT=$(mktemp)
    cat > "$EXPECT_SCRIPT" << 'EXPECT_EOF'
#!/usr/bin/expect -f
set timeout 120

# Get the input from environment
set input $env(CIPHEY_INPUT)

# Run ciphey
spawn cargo run --release -- --text $input --no-tui

# Wait for the human checker prompt
expect {
    "Possible plaintext:" {
        # Capture what's shown to the user
        puts "\n>>> Human checker prompt detected"
    }
    timeout {
        puts "\n>>> Timeout waiting for human checker"
        exit 1
    }
    eof {
        puts "\n>>> Unexpected end of output"
        exit 1
    }
}

# Send 'y' to confirm
send "y\r"

# Capture the final output
expect {
    "The plaintext is:" {
        puts "\n>>> Final result detected"
    }
    eof {
        # Expected - program exits after showing result
    }
}

# Wait for the program to finish
expect eof
EXPECT_EOF

    chmod +x "$EXPECT_SCRIPT"
    export CIPHEY_INPUT="$INPUT"
    expect "$EXPECT_SCRIPT"
    rm "$EXPECT_SCRIPT"
else
    echo "expect not found - running in interactive mode"
    echo "Please type 'y' when prompted and check the output manually."
    echo ""
    
    cargo run --release -- --text "$INPUT" --no-tui
fi

echo ""
echo "=============================================="
echo "Test complete. Check the output above."
echo ""
echo "BUG DETECTED if:"
echo "  - The plaintext shown is empty"
echo "  - The plaintext shown is 'Words'"
echo "  - The plaintext doesn't match what was shown in the confirmation prompt"
echo ""
echo "PASS if:"
echo "  - The plaintext matches what was shown in the confirmation prompt"
echo "=============================================="
