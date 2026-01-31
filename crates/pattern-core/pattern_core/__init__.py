"""
Python bindings for pattern-core.

This module provides Python-friendly bindings for pattern-core, enabling
Python developers to programmatically construct and operate on Pattern and Subject instances.
"""

# Re-export all classes from the compiled Rust module
from .pattern_core import (
    Value,
    Subject,
    Pattern,
    ValidationRules,
    ValidationError,
    StructureAnalysis,
)

__all__ = [
    "Value",
    "Subject",
    "Pattern",
    "ValidationRules",
    "ValidationError",
    "StructureAnalysis",
]
