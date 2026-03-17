"""Native Python Subject type.

A self-describing entity with identity, labels, and properties.
Implemented as a @dataclass with structural equality.
Builder methods are immutable (return new instances).
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from ._value import Value


@dataclass
class Subject:
    identity: str
    labels: set[str] = field(default_factory=set)
    properties: dict[str, "Value"] = field(default_factory=dict)

    @classmethod
    def from_id(cls, identity: str) -> "Subject":
        return cls(identity=identity)

    def with_label(self, label: str) -> "Subject":
        return Subject(
            identity=self.identity,
            labels=self.labels | {label},
            properties=dict(self.properties),
        )

    def with_property(self, name: str, value: "Value") -> "Subject":
        return Subject(
            identity=self.identity,
            labels=set(self.labels),
            properties={**self.properties, name: value},
        )
