# Diagnostics Modeling Exploration

This directory captures an early modeling exploration for pato diagnostic output using
`data/hello.gram` as the running example.

## Purpose

The goal was to explore multiple gram-native ways to model:

- source anchors / problem sites
- diagnostic kinds
- concrete problem occurrences
- remediations and options
- human-readable explanation

The variants here range from nested/container-oriented shapes to graph-native and rule-driven
shapes.

## Files

- `diagnostics-variant-nested.gram`: nested/container-style report
- `diagnostics-variant-attached-flat.gram`: flat attached composition using unary patterns
- `diagnostics-variant-detached-reference.gram`: flat normalized form using explicit references
- `diagnostics-variant-attached-multi-anchor.gram`: attached composition spanning multiple anchors
- `diagnostics-variant-problem-edge.gram`: problem occurrence modeled as an edge
- `diagnostics-variant-problem-node.gram`: problem occurrence modeled as a node
- `diagnostics-variant-problem-node-annotated.gram`: problem node with remediation as annotation
- `diagnostics-variant-problem-node-rule-driven.gram`: rule-driven compact model
- `diagnostics-variant-problem-node-rule-driven-commented.gram`: rule-driven compact model with explanatory comments
- `hello.cst.sexp`: raw CST sexp for `../hello.gram`

## Conclusion

The currently preferred direction is:

- `diagnostics-variant-problem-node-rule-driven-commented.gram`

Why this variant won:

- the diagnostic/remediation knowledge base is reusable and scales well
- per-source reports stay compact and parameterized
- rule-based refactoring tools can follow stable diagnostic and remediation identifiers
- comments provide rich contextual explanation without making prose canonical data
- pato can synthesize the verbose comments or strip them while preserving the same structured report

In short:

- structure carries the canonical facts
- rule/remediation templates carry reusable knowledge
- comments carry contextualized human explanation

All `.gram` files in this directory are intended to be verifiable with `gram-lint`.
