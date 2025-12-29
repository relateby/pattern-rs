#!/bin/bash
# Examples of using act to test GitHub Actions workflows locally

echo "📋 List all workflows and jobs:"
echo "   act -l"
echo ""

echo "🔨 Run specific jobs:"
echo "   act -j build          # Build job (both targets)"
echo "   act -j test           # Test job"
echo "   act -j lint           # Lint job"
echo "   act -j format         # Format job"
echo ""

echo "🚀 Run all jobs (simulate push event):"
echo "   act push"
echo ""

echo "📊 Run with verbose output:"
echo "   act -v -j build"
echo ""

echo "🎯 Run specific matrix target:"
echo "   act -j build --matrix target:wasm32-unknown-unknown"
echo ""

echo "💡 Tip: First run will download Docker images (may take a few minutes)"
