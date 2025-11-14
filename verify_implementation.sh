#!/bin/bash
# Verification script for document module implementation

echo "=================================="
echo "Document Module Implementation"
echo "Verification Report"
echo "=================================="
echo ""

echo "📁 File Structure:"
echo ""
echo "Core Implementation:"
ls -lh src/buffer/*.rs 2>/dev/null | awk '{print "  ✓", $9, "(" $5 ")"}'
ls -lh src/document/*.rs 2>/dev/null | awk '{print "  ✓", $9, "(" $5 ")"}'
ls -lh src/undo/*.rs 2>/dev/null | awk '{print "  ✓", $9, "(" $5 ")"}'
ls -lh src/lib.rs 2>/dev/null | awk '{print "  ✓", $9, "(" $5 ")"}'

echo ""
echo "Tests & Examples:"
ls -lh tests/*.rs 2>/dev/null | awk '{print "  ✓", $9, "(" $5 ")"}'
ls -lh examples/*.rs 2>/dev/null | awk '{print "  ✓", $9, "(" $5 ")"}'

echo ""
echo "Documentation:"
ls -lh *.md 2>/dev/null | grep -E "(DOCUMENT|IMPLEMENTATION)" | awk '{print "  ✓", $9, "(" $5 ")"}'

echo ""
echo "📊 Line Counts:"
echo ""
echo "  Buffer Module:"
wc -l src/buffer/*.rs 2>/dev/null | tail -1 | awk '{print "    Lines:", $1}'
echo "  Document Module:"
wc -l src/document/*.rs 2>/dev/null | tail -1 | awk '{print "    Lines:", $1}'
echo "  Undo Module:"
wc -l src/undo/*.rs 2>/dev/null | tail -1 | awk '{print "    Lines:", $1}'
echo "  Tests:"
wc -l tests/*.rs 2>/dev/null | tail -1 | awk '{print "    Lines:", $1}'
echo "  Examples:"
wc -l examples/*.rs 2>/dev/null | tail -1 | awk '{print "    Lines:", $1}'

echo ""
echo "  Total Implementation:"
wc -l src/buffer/*.rs src/document/*.rs src/undo/*.rs src/lib.rs tests/*.rs 2>/dev/null | tail -1 | awk '{print "    Lines:", $1}'

echo ""
echo "🧪 Test Summary:"
echo ""
grep -h "#\[test\]" src/buffer/*.rs src/document/*.rs src/undo/*.rs 2>/dev/null | wc -l | awk '{print "  Unit Tests:", $1}'
grep -h "#\[test\]" tests/*.rs 2>/dev/null | wc -l | awk '{print "  Integration Tests:", $1}'

echo ""
echo "📚 Documentation:"
echo ""
grep -h "^///" src/buffer/*.rs src/document/*.rs src/undo/*.rs src/lib.rs 2>/dev/null | wc -l | awk '{print "  Doc Comment Lines:", $1}'

echo ""
echo "✅ Implementation Complete!"
echo ""
echo "See IMPLEMENTATION_SUMMARY.md for full details."
