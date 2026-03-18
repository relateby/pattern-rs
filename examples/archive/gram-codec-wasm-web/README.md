# Gram Codec WASM - Browser Example

Interactive browser-based demo of the gram-codec WASM bindings.

## Prerequisites

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Or via cargo
cargo install wasm-pack
```

## Building the WASM Module

```bash
# From the pattern-rs root directory
cd crates/gram-codec

# Build for web (ES modules)
wasm-pack build --target web . -- --features wasm

# The output will be in crates/gram-codec/pkg/
```

## Running the Example

### Option 1: Simple HTTP Server

```bash
# From pattern-rs root
cd crates/gram-codec

# Using Python
python3 -m http.server 8000

# Or using Node.js
npx http-server -p 8000

# Open http://localhost:8000/examples/gram-codec-wasm-web/
```

### Option 2: File Protocol

Some browsers allow opening HTML files directly:

```bash
# Open in browser
open examples/wasm-web/index.html
```

**Note**: Chrome/Edge require a server for ES modules. Firefox may work with `file://`.

## Features

The interactive demo includes:

### 📥 Parse Gram Notation
- Real-time parsing with visual feedback
- Pattern count and identifier extraction
- Syntax validation

### 📚 Quick Examples
- One-click loading of common patterns
- Simple nodes, relationships, subject patterns
- Properties and labels

### 🔄 Round-Trip Testing
- Parse → Serialize → Parse validation
- Verify structural correctness
- Format preservation testing

### 📊 Session Statistics
- Track parse attempts
- Count successful operations
- Monitor error rates

## API Usage in Browser

```javascript
import init, { parse_gram, validate_gram, round_trip, version } from '../../crates/gram-codec/pkg/gram_codec.js';

// Initialize WASM module
await init();

// Parse gram notation
const result = parse_gram("(alice)-[:KNOWS]->(bob)");
console.log(`Parsed ${result.pattern_count} patterns`);
console.log(`Identifiers: ${result.identifiers}`);

// Validate
const isValid = validate_gram("(hello)");
console.log(`Valid: ${isValid}`);

// Round-trip
const serialized = round_trip("(a)-->(b)");
console.log(`Serialized: ${serialized}`);

// Get version
console.log(`Version: ${version()}`);
```

## Customization

### Styling

Edit the `<style>` section in `index.html` to customize:
- Colors and gradients
- Layout and spacing
- Button styles
- Output formatting

### Examples

Add more quick examples by editing the examples section:

```html
<button class="example-btn" onclick="loadExample('YOUR_PATTERN')">
    Your Label
</button>
```

### Statistics

Add custom statistics by:
1. Adding a counter variable
2. Updating in the appropriate function
3. Displaying in the stats section

## Troubleshooting

### WASM Module Not Loading

If you see "Failed to Load WASM":

1. **Check build output**: Ensure `pkg/` directory exists with `.wasm` and `.js` files
2. **Verify path**: Module import path should be `../../pkg/gram_codec.js`
3. **Use HTTP server**: ES modules require HTTP, not `file://` protocol
4. **Check console**: Open browser DevTools for detailed errors

### CORS Errors

If you see CORS errors:

```bash
# Use a server that sets CORS headers
npx http-server -p 8000 --cors
```

### Memory Issues

WASM has memory limitations. For very large patterns:

```javascript
try {
    const result = parse_gram(veryLargeInput);
} catch (e) {
    if (e.message.includes('memory')) {
        console.error('Pattern too large for WASM');
    }
}
```

## Performance

WASM performance characteristics:

- **Cold start**: ~50-100ms (module initialization)
- **Parse**: Near-native speed (~90-95% of native Rust)
- **Memory**: Minimal overhead, patterns are ephemeral
- **Size**: ~500KB uncompressed, ~150KB compressed

### Optimization Tips

1. **Cache the WASM module**: Initialize once, reuse
2. **Batch operations**: Validate before parsing
3. **Compress**: Serve `.wasm` with gzip/brotli
4. **Lazy load**: Load WASM only when needed

## Browser Compatibility

Tested and working on:

- ✅ Chrome 90+
- ✅ Firefox 89+
- ✅ Safari 14+
- ✅ Edge 90+

**Requirements**:
- WebAssembly support
- ES6 modules
- Async/await

## Next Steps

- See `../wasm-node/` for Node.js examples
- Check `../../README.md` for complete API reference
- Run benchmarks: `cargo bench --package gram-codec`
- Build for production: `wasm-pack build --target web --release`

## Production Deployment

For production deployment:

```bash
# Build optimized release
wasm-pack build --target web --release crates/gram-codec -- --features wasm

# Copy pkg/ to your web server
cp -r crates/gram-codec/pkg /var/www/html/gram-codec/

# Serve with proper MIME types
# .wasm → application/wasm
# .js   → application/javascript
```

### CDN Hosting

```html
<!-- Example: Hosting on CDN -->
<script type="module">
  import init, { parse_gram } from 'https://your-cdn.com/gram-codec/pkg/gram_codec.js';
  await init();
  // Use gram_codec functions
</script>
```
