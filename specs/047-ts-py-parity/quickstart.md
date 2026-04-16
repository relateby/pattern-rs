# Quickstart: Pattern API Parity Operations

**Feature**: 047-ts-py-parity
**For**: Developers using `@relateby/pattern` (TypeScript) or `relateby.pattern` (Python)

---

## TypeScript

### Installation (no change)

```bash
npm install @relateby/pattern
```

### Programmatic construction

```typescript
import { Pattern } from "@relateby/pattern"

// Leaf node (existing)
const leaf = Pattern.point("a")

// Non-atomic pattern with children (NEW)
const tree = Pattern.pattern("root", [Pattern.point("a"), Pattern.point("b")])

// Pattern from list of values (NEW)
const flat = Pattern.fromList("root", ["a", "b", "c"])
// Equivalent to: Pattern.pattern("root", ["a","b","c"].map(Pattern.point))
```

### Structural predicates (NEW)

```typescript
import { pipe } from "effect"
import { anyValue, allValues, matches, contains } from "@relateby/pattern"

const hasLong = anyValue<string>(s => s.length > 5)
pipe(tree, hasLong)  // false — "root", "a", "b" all ≤ 5 chars

const allShort = allValues<string>(s => s.length <= 5)
pipe(tree, allShort)  // true

const aLeaf = Pattern.point("a")
matches(aLeaf, Pattern.point("a"))   // true
matches(aLeaf, Pattern.point("b"))   // false

pipe(tree, contains(aLeaf))  // true — "a" appears in tree
```

### Paramorphism (NEW)

```typescript
import { para } from "@relateby/pattern"

// Compute tree height
const height = para<string, number>(
  (_p, childHeights) => childHeights.length === 0 ? 0 : 1 + Math.max(...childHeights)
)
height(tree)  // 1 (root → children are leaves)

// Render with depth indentation
const render = para<string, string>(
  (p, childLines) => [p.value, ...childLines.map(l => "  " + l)].join("\n")
)
render(tree)
// root
//   a
//   b
```

### Anamorphism / unfold (NEW)

```typescript
import { pipe } from "effect"
import { unfold } from "@relateby/pattern"

// Build a countdown chain: 3 → 2 → 1 → 0
const countdown = unfold<number, number>(n => [n, n > 0 ? [n - 1] : []])
pipe(3, countdown)
// Pattern(3, [Pattern(2, [Pattern(1, [Pattern(0, [])])])])
```

### Pattern combination / Semigroup (NEW)

```typescript
import { pipe } from "effect"
import { combine } from "@relateby/pattern"

const pat1 = Pattern.pattern("hello", [Pattern.point("a")])
const pat2 = Pattern.pattern(" world", [Pattern.point("b")])

// combine(combineValues)(a)(b)
const merged = pipe(
  pat1,
  combine((x: string, y: string) => x + y)(pat2)
)
// Pattern("hello world", [Pattern("a"), Pattern("b")])
```

### Comonad helpers (NEW)

```typescript
import { depthAt, sizeAt, indicesAt } from "@relateby/pattern"

const nested = Pattern.pattern("r", [
  Pattern.pattern("a", [Pattern.point("x")]),
  Pattern.point("b")
])

depthAt(nested)
// Pattern(2, [Pattern(1, [Pattern(0)]), Pattern(0)])
// root has max subtree depth 2; "a" has depth 1; "x" and "b" have depth 0

sizeAt(nested)
// Pattern(4, [Pattern(2, [Pattern(1)]), Pattern(1)])
// root has 4 nodes; "a" subtree has 2; "x" has 1; "b" has 1

indicesAt(nested)
// Pattern([], [Pattern([0], [Pattern([0,0])]), Pattern([1])])
// root path []; first child path [0]; "x" path [0,0]; second child path [1]
```

---

## Python

### Installation (no change)

```bash
pip install relateby-pattern
```

### Programmatic construction

```python
from relateby.pattern import Pattern, Subject

# Leaf node (existing)
leaf = Pattern.point(Subject.from_id("a"))

# Non-atomic pattern with children (NEW)
tree = Pattern.pattern(
    Subject.from_id("root"),
    [Pattern.point(Subject.from_id("a")), Pattern.point(Subject.from_id("b"))]
)

# Pattern from list of values (NEW)
flat = Pattern.from_list(Subject.from_id("root"), [
    Subject.from_id("a"), Subject.from_id("b"), Subject.from_id("c")
])
```

### Structural predicates (NEW)

```python
has_person = tree.any_value(lambda s: "Person" in s.labels)  # False — no labels set
all_have_id = tree.all_values(lambda s: s.identity != "")     # True

leaf_a = Pattern.point(Subject.from_id("a"))
tree.matches(tree)       # True
tree.matches(leaf_a)     # False
tree.contains(leaf_a)    # True — "a" node is in tree
```

### Paramorphism (NEW)

```python
def height(pat, child_heights):
    return 0 if not child_heights else 1 + max(child_heights)

tree.para(height)  # 1

def render(pat, child_lines):
    lines = [pat.value.identity] + ["  " + l for line in child_lines for l in line.split("\n")]
    return "\n".join(lines)

print(tree.para(render))
# root
#   a
#   b
```

### Anamorphism / unfold (NEW)

```python
def countdown(n):
    return (str(n), [n - 1] if n > 0 else [])

chain = Pattern.unfold(countdown, 3)
# Pattern("3", [Pattern("2", [Pattern("1", [Pattern("0", [])])])])
```

### Pattern combination (NEW)

```python
def combine_subjects(a, b):
    return Subject(
        identity=a.identity or b.identity,
        labels=a.labels | b.labels,
        properties={**b.properties, **a.properties},
    )

pat1 = Pattern.pattern(Subject.from_id("alice"), [Pattern.point(Subject.from_id("x"))])
pat2 = Pattern.pattern(Subject.from_id("bob"),   [Pattern.point(Subject.from_id("y"))])

merged = pat1.combine(pat2, combine_subjects)
# Pattern(Subject("alice"), [Pattern(Subject("x")), Pattern(Subject("y"))])
```

### Comonad helpers (NEW)

```python
from relateby.pattern import Pattern

nested = Pattern.pattern("r", [
    Pattern.pattern("a", [Pattern.point("x")]),
    Pattern.point("b")
])

nested.depth_at()    # Pattern(2, [Pattern(1, [Pattern(0)]), Pattern(0)])
nested.size_at()     # Pattern(4, [Pattern(2, [Pattern(1)]), Pattern(1)])
nested.indices_at()  # Pattern([], [Pattern([0], [Pattern([0,0])]), Pattern([1])])
```

### Graph transforms (NEW)

```python
from relateby.pattern import map_graph, filter_graph, para_graph
from relateby.pattern import StandardGraph

graph = StandardGraph.from_gram("(alice:Person)-[:KNOWS]->(bob:Person)")

# Add a "Visited" label to all nodes
updated = map_graph(graph, {
    "node": lambda p: p.map(lambda s: s.with_label("Visited"))
})

# Remove nodes without a "Person" label
filtered = filter_graph(
    graph,
    lambda cls, p: cls != "node" or "Person" in p.value.labels,
    "delete_container"   # remove relationships to deleted nodes
)

# Compute in-degree for each node
def count_references(query, p, child_results):
    return len([r for r in query.relationships() if query.target(r) == p])

degrees = para_graph(graph, count_references)
# {"alice": 0, "bob": 1}
```

---

## Reference Equivalence

All operations are verified against the Haskell reference (`../pattern-hs/libs/pattern/src/Pattern/Core.hs`):

| New operation | Haskell equivalent |
|---|---|
| `anyValue` | `Pattern.anyValue` (Foldable `any`) |
| `allValues` | `Pattern.allValues` (Foldable `all`) |
| `matches` | `matches = (==)` |
| `contains` | `Pattern.contains` |
| `para` | `Pattern.para` |
| `unfold` | `Pattern.unfold` |
| `combine` | `Semigroup (<>)` |
| `Pattern.pattern()` | `pattern :: v -> [Pattern v] -> Pattern v` |
| `Pattern.fromList()` | `fromList :: v -> [v] -> Pattern v` |
| `depthAt` | `depthAt = extend depth` |
| `sizeAt` | `Pattern.sizeAt` |
| `indicesAt` | `Pattern.indicesAt` |
