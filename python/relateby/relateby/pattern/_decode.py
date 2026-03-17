"""Decode pipeline from JSON interchange format to native Pattern[Subject].

The Rust gram-codec produces JSON with the "subject" key. This module
converts that raw JSON structure into native Python dataclasses.
"""
from __future__ import annotations

from ._value import value_from_dict, Value
from ._subject import Subject
from ._pattern import Pattern


def pattern_from_dict(d: dict) -> Pattern[Subject]:
    """Recursively construct a Pattern[Subject] from a raw AstPattern dict.

    Expected input shape (from gram_parse_to_json):
    {
      "subject": {
        "identity": str,
        "labels": list[str],
        "properties": dict[str, RawValue]
      },
      "elements": list[dict]
    }
    """
    try:
        subj_data = d["subject"]
        subject = Subject(
            identity=subj_data["identity"],
            labels=set(subj_data.get("labels", [])),
            properties={
                k: value_from_dict(v)
                for k, v in subj_data.get("properties", {}).items()
            },
        )
        elements = [pattern_from_dict(e) for e in d.get("elements", [])]
        return Pattern(value=subject, elements=elements)
    except (KeyError, TypeError, ValueError) as exc:
        raise ValueError(
            f"Failed to decode pattern from dict: {exc}"
        ) from exc
