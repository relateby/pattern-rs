#!/usr/bin/env python3
"""
Interactive demo for gram-codec Python bindings

Prerequisites:
    pip install relateby
    Or from TestPyPI: pip install --index-url https://test.pypi.org/simple/ relateby
"""

import sys

try:
    from relateby.gram import parse_gram, validate_gram, round_trip, version
except ImportError:
    print("❌ relateby.gram not found!")
    print("\nTo install: pip install relateby")
    print("Or from TestPyPI: pip install --index-url https://test.pypi.org/simple/ relateby")
    sys.exit(1)

def print_header(title):
    """Print a formatted section header"""
    print(f"\n{'='*60}")
    print(f"  {title}")
    print('='*60)

def example_parse():
    """Example 1: Parse gram notation"""
    print_header("Example 1: Parse Gram Notation")
    
    examples = [
        "(alice)-[:KNOWS]->(bob)",
        "(a) (b) (c)",
        "[team:Team {name: \"DevRel\"} | (alice), (bob), (charlie)]",
    ]
    
    for gram in examples:
        try:
            result = parse_gram(gram)
            print(f"\n✓ '{gram}'")
            print(f"  Pattern count: {result['pattern_count']}")
            print(f"  Identifiers: {result['identifiers']}")
        except ValueError as e:
            print(f"\n✗ '{gram}'")
            print(f"  Error: {e}")

def example_validate():
    """Example 2: Validate gram notation"""
    print_header("Example 2: Validate Gram Notation")
    
    test_cases = [
        ("(hello)", True),
        ("(a)-->(b)", True),
        ("[team | alice, bob]", True),
        ("(unclosed", False),
        ("{missing_parens}", False),
        ("(a {key: })", False),
    ]
    
    for gram, expected in test_cases:
        is_valid = validate_gram(gram)
        status = "✓" if is_valid == expected else "✗"
        result = "valid" if is_valid else "invalid"
        print(f"{status} '{gram}' → {result}")

def example_round_trip():
    """Example 3: Round-trip test"""
    print_header("Example 3: Round-Trip Test")
    
    test_cases = [
        "(alice:Person {name: \"Alice\"})-[:KNOWS]->(bob:Person {name: \"Bob\"})",
        "[team | (alice), (bob), (charlie)]",
        "(a)-->(b)-->(c)",
    ]
    
    for original in test_cases:
        print(f"\nOriginal:   {original}")
        try:
            serialized = round_trip(original)
            print(f"Serialized: {serialized}")
            
            # Verify still valid
            is_valid = validate_gram(serialized)
            print(f"Still valid: {'✓' if is_valid else '✗'}")
        except ValueError as e:
            print(f"Error: {e}")

def example_complex_patterns():
    """Example 4: Complex patterns"""
    print_header("Example 4: Complex Patterns")
    
    patterns = [
        ("Simple node", "(hello)"),
        ("Node with label", "(a:Person)"),
        ("Node with properties", '(a {name: "Alice", age: 30})'),
        ("Relationship", "(a)-[:KNOWS]->(b)"),
        ("Relationship with properties", '(a)-[:KNOWS {since: 2020}]->(b)'),
        ("Subject pattern", '[team | (alice), (bob)]'),
        ("Nested pattern", "[outer | [inner | (leaf)]]"),
        ("Multiple labels", "(a:Person:Employee:Manager)"),
    ]
    
    for name, gram in patterns:
        try:
            result = parse_gram(gram)
            print(f"✓ {name:30} → {result['pattern_count']} pattern(s)")
        except ValueError:
            print(f"✗ {name:30} → Parse error")

def example_batch_validation():
    """Example 5: Batch validation"""
    print_header("Example 5: Batch Validation")
    
    # Simulate reading multiple gram files
    gram_files = {
        "nodes.gram": "(alice) (bob) (charlie)",
        "relationships.gram": "(alice)-->(bob) (bob)-->(charlie)",
        "complex.gram": "[team | (alice), (bob)]",
        "properties.gram": '(a {name: "Alice", age: 30})',
        "invalid.gram": "(unclosed",
    }
    
    valid_count = 0
    invalid_count = 0
    
    print("\nValidating gram files:")
    for filename, content in gram_files.items():
        is_valid = validate_gram(content)
        status = "✓" if is_valid else "✗"
        result = "valid" if is_valid else "invalid"
        print(f"  {status} {filename:25} {result}")
        
        if is_valid:
            valid_count += 1
        else:
            invalid_count += 1
    
    print(f"\nSummary: {valid_count} valid, {invalid_count} invalid")

def example_error_handling():
    """Example 6: Error handling"""
    print_header("Example 6: Error Handling")
    
    invalid_patterns = [
        "(unclosed",
        "{no_parens}",
        "(a {key: })",
        "((nested))",
    ]
    
    for gram in invalid_patterns:
        try:
            parse_gram(gram)
            print(f"✗ Unexpected success: '{gram}'")
        except ValueError as e:
            print(f"✓ Expected error for '{gram}'")
            print(f"  {str(e)[:60]}...")

def interactive_mode():
    """Interactive REPL for testing gram notation"""
    print_header("Interactive Mode")
    print("\nType gram notation to parse it, or:")
    print("  'quit' or 'exit' to exit")
    print("  'help' for help")
    print("  'examples' to show examples")
    
    while True:
        try:
            gram = input("\ngram> ").strip()
            
            if not gram:
                continue
            
            if gram.lower() in ('quit', 'exit'):
                print("Goodbye!")
                break
            
            if gram.lower() == 'help':
                print("\nCommands:")
                print("  (gram notation)  - Parse and validate")
                print("  examples         - Show example patterns")
                print("  quit/exit        - Exit interactive mode")
                continue
            
            if gram.lower() == 'examples':
                print("\nExample patterns:")
                print("  (hello)")
                print("  (a:Person)")
                print("  (a {name: \"Alice\"})")
                print("  (a)-->(b)")
                print("  (a)-[:KNOWS]->(b)")
                print("  [team | alice, bob]")
                continue
            
            # Try to parse
            try:
                result = parse_gram(gram)
                print(f"✓ Valid! Parsed {result['pattern_count']} pattern(s)")
                if result['identifiers']:
                    print(f"  Identifiers: {result['identifiers']}")
            except ValueError as e:
                print(f"✗ Invalid: {e}")
                
        except KeyboardInterrupt:
            print("\n\nGoodbye!")
            break
        except EOFError:
            print("\n\nGoodbye!")
            break

def main():
    """Run all examples"""
    print("╔════════════════════════════════════════════════════════════╗")
    print("║     Gram Codec Python Bindings - Interactive Demo         ║")
    print("╚════════════════════════════════════════════════════════════╝")
    
    print(f"\nGram Codec Version: {version()}")
    
    # Run examples
    example_parse()
    example_validate()
    example_round_trip()
    example_complex_patterns()
    example_batch_validation()
    example_error_handling()
    
    # Offer interactive mode
    print("\n" + "="*60)
    response = input("\nWould you like to enter interactive mode? (y/n): ").strip().lower()
    if response in ('y', 'yes'):
        interactive_mode()
    else:
        print("\nDone! Run with 'python examples/python/demo.py' to try again.")

if __name__ == "__main__":
    main()

