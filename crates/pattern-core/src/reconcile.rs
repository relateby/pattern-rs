//! Pattern reconciliation for normalizing duplicate identities.
//!
//! Provides operations to reconcile patterns by resolving duplicate identities
//! and completing partial references. Ported from `Pattern.Reconcile` in the
//! Haskell reference implementation.

use std::collections::{HashMap, HashSet};

use crate::pattern::Pattern;
use crate::subject::{Subject, Symbol};

// -----------------------------------------------------------------------------
// Core Traits
// -----------------------------------------------------------------------------

/// Allows extracting a unique identifier from a value.
pub trait HasIdentity<V, I: Ord> {
    fn identity(v: &V) -> &I;
}

/// Allows merging two values according to a strategy.
pub trait Mergeable {
    type MergeStrategy;

    /// Merge two values. `a` is the "accumulated/existing" value, `b` is the "incoming" one.
    fn merge(strategy: &Self::MergeStrategy, a: Self, b: Self) -> Self;
}

/// Allows checking if one value is a "partial" version of another.
pub trait Refinable {
    /// Returns `true` if `sub` contains a subset of the information in `sup`.
    fn is_refinement_of(sup: &Self, sub: &Self) -> bool;
}

// -----------------------------------------------------------------------------
// Reconciliation Policy
// -----------------------------------------------------------------------------

/// Policy for resolving duplicate identities during reconciliation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconciliationPolicy<S> {
    /// Keep the last occurrence of each identity.
    LastWriteWins,
    /// Keep the first occurrence of each identity.
    FirstWriteWins,
    /// Combine all occurrences using specified strategies for elements and values.
    Merge(ElementMergeStrategy, S),
    /// Fail if any duplicate identities have different content.
    Strict,
}

/// Strategy for merging the children (elements) of a pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementMergeStrategy {
    /// Later element list completely replaces earlier ones.
    ReplaceElements,
    /// Concatenate all element lists in traversal order.
    AppendElements,
    /// Deduplicate elements by identity.
    UnionElements,
}

// -----------------------------------------------------------------------------
// Subject-specific merge types
// -----------------------------------------------------------------------------

/// Strategy for merging Subject content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectMergeStrategy {
    pub label_merge: LabelMerge,
    pub property_merge: PropertyMerge,
}

/// Strategy for merging label sets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelMerge {
    UnionLabels,
    IntersectLabels,
    ReplaceLabels,
}

/// Strategy for merging property maps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyMerge {
    ReplaceProperties,
    ShallowMerge,
    DeepMerge,
}

/// Default merge strategy for Subjects.
pub fn default_subject_merge_strategy() -> SubjectMergeStrategy {
    SubjectMergeStrategy {
        label_merge: LabelMerge::UnionLabels,
        property_merge: PropertyMerge::ShallowMerge,
    }
}

// -----------------------------------------------------------------------------
// Subject trait implementations
// -----------------------------------------------------------------------------

impl HasIdentity<Subject, Symbol> for Subject {
    fn identity(v: &Subject) -> &Symbol {
        &v.identity
    }
}

impl Mergeable for Subject {
    type MergeStrategy = SubjectMergeStrategy;

    fn merge(strategy: &SubjectMergeStrategy, a: Subject, b: Subject) -> Subject {
        let merged_labels = merge_labels(&strategy.label_merge, &a.labels, &b.labels);
        let merged_props = merge_properties(&strategy.property_merge, a.properties, b.properties);
        Subject {
            identity: a.identity,
            labels: merged_labels,
            properties: merged_props,
        }
    }
}

impl Refinable for Subject {
    fn is_refinement_of(sup: &Subject, sub: &Subject) -> bool {
        sup.identity == sub.identity
            && sub.labels.is_subset(&sup.labels)
            && sub
                .properties
                .iter()
                .all(|(k, v)| sup.properties.get(k) == Some(v))
    }
}

fn merge_labels(
    strategy: &LabelMerge,
    l1: &HashSet<String>,
    l2: &HashSet<String>,
) -> HashSet<String> {
    match strategy {
        LabelMerge::UnionLabels => l1.union(l2).cloned().collect(),
        LabelMerge::IntersectLabels => l1.intersection(l2).cloned().collect(),
        LabelMerge::ReplaceLabels => l2.clone(),
    }
}

fn merge_properties(
    strategy: &PropertyMerge,
    p1: HashMap<String, crate::subject::Value>,
    p2: HashMap<String, crate::subject::Value>,
) -> HashMap<String, crate::subject::Value> {
    match strategy {
        PropertyMerge::ReplaceProperties => p2,
        PropertyMerge::ShallowMerge | PropertyMerge::DeepMerge => {
            // p2 values win on conflict (right-bias)
            let mut merged = p1;
            merged.extend(p2);
            merged
        }
    }
}

// -----------------------------------------------------------------------------
// Error types
// -----------------------------------------------------------------------------

/// Error returned by `reconcile` when `Strict` policy detects conflicts.
#[derive(Debug, Clone, PartialEq)]
pub struct ReconcileError {
    pub message: String,
}

// -----------------------------------------------------------------------------
// Core reconcile function
// -----------------------------------------------------------------------------

/// Normalizes a pattern by resolving duplicate identities according to the policy.
///
/// - `LastWriteWins`, `FirstWriteWins`, `Merge` always return `Ok`.
/// - `Strict` returns `Err` if any duplicate identity has different content.
pub fn reconcile<V>(
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    pattern: &Pattern<V>,
) -> Result<Pattern<V>, ReconcileError>
where
    V: HasIdentity<V, Symbol> + Mergeable + Refinable + PartialEq + Clone,
{
    match policy {
        ReconciliationPolicy::Strict => reconcile_strict(pattern),
        _ => Ok(reconcile_non_strict(policy, pattern)),
    }
}

fn reconcile_non_strict<V>(
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    pattern: &Pattern<V>,
) -> Pattern<V>
where
    V: HasIdentity<V, Symbol> + Mergeable + Refinable + Clone,
{
    let occurrence_map = collect_by_identity(pattern);
    let canonical_map: HashMap<Symbol, Pattern<V>> = occurrence_map
        .into_iter()
        .map(|(id, occurrences)| {
            let canonical = reconcile_occurrences(policy, occurrences);
            (id, canonical)
        })
        .collect();

    let (rebuilt, _) = rebuild_pattern(&mut HashSet::new(), &canonical_map, pattern);
    rebuilt
}

fn reconcile_occurrences<V>(
    policy: &ReconciliationPolicy<V::MergeStrategy>,
    occurrences: Vec<Pattern<V>>,
) -> Pattern<V>
where
    V: HasIdentity<V, Symbol> + Mergeable + Clone,
{
    match policy {
        ReconciliationPolicy::LastWriteWins => {
            let v = occurrences.last().unwrap().value.clone();
            let all_elements = merge_elements_union(occurrences.iter().map(|p| &p.elements));
            Pattern {
                value: v,
                elements: all_elements,
            }
        }
        ReconciliationPolicy::FirstWriteWins => {
            let v = occurrences.first().unwrap().value.clone();
            let all_elements = merge_elements_union(occurrences.iter().map(|p| &p.elements));
            Pattern {
                value: v,
                elements: all_elements,
            }
        }
        ReconciliationPolicy::Merge(elem_strat, val_strat) => {
            let merged_val = occurrences
                .iter()
                .skip(1)
                .fold(occurrences[0].value.clone(), |acc, p| {
                    V::merge(val_strat, acc, p.value.clone())
                });
            let merged_elements =
                merge_elements(elem_strat, occurrences.iter().map(|p| &p.elements));
            Pattern {
                value: merged_val,
                elements: merged_elements,
            }
        }
        ReconciliationPolicy::Strict => unreachable!("Strict handled separately"),
    }
}

fn merge_elements_union<'a, V, I>(lists: I) -> Vec<Pattern<V>>
where
    V: HasIdentity<V, Symbol> + Clone,
    I: Iterator<Item = &'a Vec<Pattern<V>>>,
    V: 'a,
{
    merge_elements(&ElementMergeStrategy::UnionElements, lists)
}

fn merge_elements<'a, V, I>(strategy: &ElementMergeStrategy, lists: I) -> Vec<Pattern<V>>
where
    V: HasIdentity<V, Symbol> + Clone,
    I: Iterator<Item = &'a Vec<Pattern<V>>>,
    V: 'a,
{
    let all: Vec<Vec<Pattern<V>>> = lists.cloned().collect();
    match strategy {
        ElementMergeStrategy::ReplaceElements => all.into_iter().last().unwrap_or_default(),
        ElementMergeStrategy::AppendElements => all.into_iter().flatten().collect(),
        ElementMergeStrategy::UnionElements => {
            let mut seen: HashMap<Symbol, Pattern<V>> = HashMap::new();
            for elem in all.into_iter().flatten() {
                let id = V::identity(&elem.value).clone();
                seen.entry(id).or_insert(elem);
            }
            seen.into_values().collect()
        }
    }
}

fn rebuild_pattern<V>(
    visited: &mut HashSet<Symbol>,
    canonical_map: &HashMap<Symbol, Pattern<V>>,
    pattern: &Pattern<V>,
) -> (Pattern<V>, ())
where
    V: HasIdentity<V, Symbol> + Clone,
{
    let v_id = V::identity(&pattern.value).clone();
    let source = canonical_map.get(&v_id).unwrap_or(pattern);
    visited.insert(v_id);

    let mut rebuilt_elems = Vec::new();
    for elem in &source.elements {
        let elem_id = V::identity(&elem.value).clone();
        if !visited.contains(&elem_id) {
            let (rebuilt, _) = rebuild_pattern(visited, canonical_map, elem);
            rebuilt_elems.push(rebuilt);
        }
    }

    (
        Pattern {
            value: source.value.clone(),
            elements: rebuilt_elems,
        },
        (),
    )
}

fn collect_by_identity<V>(pattern: &Pattern<V>) -> HashMap<Symbol, Vec<Pattern<V>>>
where
    V: HasIdentity<V, Symbol> + Clone,
{
    let mut map: HashMap<Symbol, Vec<Pattern<V>>> = HashMap::new();
    collect_recursive(pattern, &mut map);
    map
}

fn collect_recursive<V>(pattern: &Pattern<V>, map: &mut HashMap<Symbol, Vec<Pattern<V>>>)
where
    V: HasIdentity<V, Symbol> + Clone,
{
    let id = V::identity(&pattern.value).clone();
    map.entry(id).or_default().push(pattern.clone());
    for elem in &pattern.elements {
        collect_recursive(elem, map);
    }
}

fn reconcile_strict<V>(pattern: &Pattern<V>) -> Result<Pattern<V>, ReconcileError>
where
    V: HasIdentity<V, Symbol> + Mergeable + Refinable + PartialEq + Clone,
{
    let occurrence_map = collect_by_identity(pattern);
    for occurrences in occurrence_map.values() {
        if occurrences.len() > 1 {
            let first = &occurrences[0];
            for other in &occurrences[1..] {
                if other.value != first.value {
                    return Err(ReconcileError {
                        message: "Duplicate identities with different content".to_string(),
                    });
                }
            }
        }
    }
    Ok(reconcile_non_strict(
        &ReconciliationPolicy::FirstWriteWins,
        pattern,
    ))
}
