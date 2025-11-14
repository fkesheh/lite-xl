#!/bin/bash
# Test only the new modules we created

echo "Testing buffer module..."
cargo test --lib buffer::tests 2>&1 | grep -E "(test result|running)"

echo -e "\nTesting selection module..."
cargo test --lib selection::tests 2>&1 | grep -E "(test result|running)"

echo -e "\nTesting undo module..."
cargo test --lib undo::tests 2>&1 | grep -E "(test result|running)"

echo -e "\nTesting document module..."
cargo test --lib document::tests 2>&1 | grep -E "(test result|running)"

echo -e "\nTesting integration..."
cargo test --test integration_tests 2>&1 | grep -E "(test result|running)"
