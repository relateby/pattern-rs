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
    """A self-describing entity with identity, labels, and properties.

    The canonical value type used inside ``Pattern[Subject]``.  All
    builder methods are immutable — they return a new ``Subject``
    instance and leave the original unchanged.

    Attributes:
        identity: Unique string identifier for the subject.
        labels: Set of string labels (type tags) attached to the subject.
        properties: Mapping of property names to their values.
    """

    identity: str
    labels: set[str] = field(default_factory=set)
    properties: dict[str, "Value"] = field(default_factory=dict)

    @classmethod
    def from_id(cls, identity: str) -> "Subject":
        """Create a Subject with only an identity (no labels or properties).

        Args:
            identity: The unique identifier for the new subject.

        Returns:
            A Subject with the given identity and empty labels/properties.
        """
        return cls(identity=identity)

    def with_label(self, label: str) -> "Subject":
        """Return a new Subject with ``label`` added to the label set.

        Args:
            label: The label string to add.

        Returns:
            A new Subject identical to ``self`` except that ``label`` is
            included in ``labels``.
        """
        return Subject(
            identity=self.identity,
            labels=self.labels | {label},
            properties=dict(self.properties),
        )

    def with_property(self, name: str, value: "Value") -> "Subject":
        """Return a new Subject with ``name`` set to ``value`` in properties.

        Args:
            name: The property key.
            value: The property value.

        Returns:
            A new Subject identical to ``self`` except that
            ``properties[name]`` is set to ``value``.
        """
        return Subject(
            identity=self.identity,
            labels=set(self.labels),
            properties={**self.properties, name: value},
        )
