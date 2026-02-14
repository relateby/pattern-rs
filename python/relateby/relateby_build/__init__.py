"""
PEP 517 build backend for unified relateby package.
Builds pattern-core and gram-codec via maturin, assembles one wheel with
relateby.pattern and relateby.gram (no top-level pattern_core/gram_codec).
"""
from __future__ import annotations

import re
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path
from typing import Any


def _repo_root() -> Path:
    """Repository root (pattern-rs)."""
    # This file is python/relateby/relateby_build/__init__.py
    return Path(__file__).resolve().parent.parent.parent.parent


def _project_dir() -> Path:
    """Unified package dir (python/relateby)."""
    return Path(__file__).resolve().parent.parent


def _read_version() -> str:
    """Read version from pyproject.toml."""
    pyproject = _project_dir() / "pyproject.toml"
    text = pyproject.read_text()
    m = re.search(r'version\s*=\s*["\']([^"\']+)["\']', text)
    if not m:
        return "0.1.0"
    return m.group(1)


def _run_maturin(manifest_path: Path, cwd: Path) -> None:
    import os

    env = os.environ.copy()
    env["PYO3_PYTHON"] = sys.executable
    subprocess.run(
        [
            sys.executable,
            "-m",
            "maturin",
            "build",
            "--release",
            "--manifest-path",
            str(manifest_path),
            "--features",
            "python",
        ],
        check=True,
        cwd=cwd,
        env=env,
    )


def _extract_so_from_wheel(whl_path: Path, out_dir: Path) -> list[Path]:
    """Extract .so from wheel into out_dir; return list of extracted paths."""
    extracted: list[Path] = []
    with zipfile.ZipFile(whl_path, "r") as zf:
        for name in zf.namelist():
            if not name.endswith(".so"):
                continue
            base = name.split("/")[-1]
            zf.extract(name, out_dir)
            src = out_dir / name
            dest = out_dir / base
            if src != dest:
                if dest.exists():
                    dest.unlink()
                src.rename(dest)
                # Remove empty parent dirs if any
                try:
                    src.parent.rmdir()
                except OSError:
                    pass
            extracted.append(dest)
    return extracted


def _find_wheels(wheels_dir: Path) -> tuple[Path | None, Path | None]:
    """Find pattern_core and gram_codec wheels. Returns (pattern_whl, gram_whl)."""
    pattern_whl = None
    gram_whl = None
    for f in wheels_dir.iterdir():
        if f.suffix != ".whl":
            continue
        name = f.stem.split("-")[0]
        if name == "pattern_core":
            pattern_whl = f
        elif name == "gram_codec":
            gram_whl = f
    return pattern_whl, gram_whl


def _wheel_tag_from_whl(whl_path: Path) -> str:
    """Get wheel tag from filename, e.g. pattern_core-0.1.0-cp312-cp312-macosx_11_0_arm64.whl -> cp312-cp312-macosx_11_0_arm64."""
    # Format: {name}-{ver}-{tag}.whl
    stem = whl_path.stem
    parts = stem.split("-")
    if len(parts) >= 3:
        return "-".join(parts[2:])
    return "py3-none-any"


def _hash_file(path: Path) -> tuple[str, str]:
    """Return (algo, hash) for RECORD."""
    import hashlib

    data = path.read_bytes()
    h = hashlib.sha256(data).hexdigest()
    return ("sha256", h)


def build_wheel(
    wheel_directory: str,
    config_settings: dict[str, Any] | None = None,
    metadata_directory: str | None = None,
) -> str:
    """Build one wheel for relateby with relateby.pattern and relateby.gram."""
    config_settings = config_settings or {}
    repo = _repo_root()
    project_dir = _project_dir()
    version = _read_version()

    pattern_manifest = repo / "crates" / "pattern-core" / "Cargo.toml"
    gram_manifest = repo / "crates" / "gram-codec" / "Cargo.toml"
    if not pattern_manifest.exists() or not gram_manifest.exists():
        raise RuntimeError(
            "Unified build must be run from the repository: crates/pattern-core and crates/gram-codec must exist"
        )

    # Build both crates (maturin puts wheels in workspace target/wheels)
    _run_maturin(pattern_manifest, repo)
    _run_maturin(gram_manifest, repo)

    wheels_dir = repo / "target" / "wheels"
    pattern_whl, gram_whl = _find_wheels(wheels_dir)
    if not pattern_whl or not gram_whl:
        raise RuntimeError(
            f"Expected pattern_core and gram_codec wheels in {wheels_dir}"
        )

    # Use tag from one of the wheels (same platform)
    tag = _wheel_tag_from_whl(pattern_whl)
    wheel_basename = f"relateby-{version}-{tag}.whl"
    wheel_path = Path(wheel_directory) / wheel_basename

    # Staging for wheel contents
    import tempfile

    with tempfile.TemporaryDirectory() as stage:
        stage_p = Path(stage)
        native_dir = stage_p / "relateby" / "_native"
        native_dir.mkdir(parents=True)

        # Extract .so from each wheel into _native
        _extract_so_from_wheel(pattern_whl, stage_p)
        _extract_so_from_wheel(gram_whl, stage_p)
        # Move .so files into relateby/_native/
        for so in stage_p.glob("**/*.so"):
            if so.parent != native_dir:
                shutil.move(str(so), str(native_dir / so.name))

        # Copy relateby package tree (__init__.py files)
        relateby_src = project_dir / "relateby"
        for py in relateby_src.rglob("*.py"):
            rel = py.relative_to(relateby_src)
            dest = stage_p / "relateby" / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(py, dest)

        # Ensure _native has __init__.py
        (native_dir / "__init__.py").write_text("# Native extensions: pattern_core, gram_codec\n")

        # Build METADATA from pyproject
        metadata_content = _generate_metadata(project_dir, version)
        dist_info = stage_p / f"relateby-{version}.dist-info"
        dist_info.mkdir()
        (dist_info / "METADATA").write_text(metadata_content, encoding="utf-8")

        # WHEEL file
        (dist_info / "WHEEL").write_text(
            "Wheel-Version: 1.0\nGenerator: relateby_build\nRoot-Is-Pure-Lib: false\nTag: " + tag + "\n",
            encoding="utf-8",
        )

        # RECORD
        record_lines: list[str] = []
        for f in sorted((stage_p / "relateby").rglob("*")):
            if f.is_file():
                rel = "relateby/" + str(f.relative_to(stage_p / "relateby")).replace("\\", "/")
                algo, h = _hash_file(f)
                size = f.stat().st_size
                record_lines.append(f"{rel},{algo}={h},{size}\n")
        for f in sorted(dist_info.iterdir()):
            rel = f"relateby-{version}.dist-info/" + f.name
            algo, h = _hash_file(f)
            size = f.stat().st_size
            record_lines.append(f"{rel},{algo}={h},{size}\n")
        record_lines.append(f"relateby-{version}.dist-info/RECORD,,\n")
        (dist_info / "RECORD").write_text("".join(record_lines), encoding="utf-8")

        # Zip into wheel
        with zipfile.ZipFile(wheel_path, "w", zipfile.ZIP_DEFLATED) as zf:
            for f in (stage_p / "relateby").rglob("*"):
                if f.is_file():
                    arc = "relateby/" + str(f.relative_to(stage_p / "relateby")).replace("\\", "/")
                    zf.write(f, arc)
            for f in dist_info.iterdir():
                zf.write(f, f"relateby-{version}.dist-info/" + f.name)

    return wheel_basename


def _generate_metadata(project_dir: Path, version: str) -> str:
    """Minimal METADATA content from pyproject.toml."""
    pyproject = project_dir / "pyproject.toml"
    text = pyproject.read_text()
    name = "relateby"
    desc = "Unified Python package for Pattern data structures and Gram notation (relateby.pattern and relateby.gram)"
    for line in text.split("\n"):
        if line.strip().startswith("description"):
            m = re.search(r'=\s*["\']([^"\']+)["\']', line)
            if m:
                desc = m.group(1)
            break
    return f"""Metadata-Version: 2.1
Name: {name}
Version: {version}
Summary: {desc}
License: Apache-2.0
Requires-Python: >=3.8
"""


def build_sdist(
    sdist_directory: str,
    config_settings: dict[str, Any] | None = None,
) -> str:
    """Build sdist. Source tree must be built from repo (crates available)."""
    import tarfile

    project_dir = _project_dir()
    version = _read_version()
    tarball_name = f"relateby-{version}.tar.gz"
    tarball_path = Path(sdist_directory) / tarball_name

    with tarfile.open(tarball_path, "w:gz", format=tarfile.PAX_FORMAT) as tf:
        root = f"relateby-{version}"
        for path in [project_dir / "pyproject.toml", project_dir / "README.md"]:
            if path.exists():
                tf.add(path, f"{root}/{path.name}")
        for path in (project_dir / "relateby").rglob("*"):
            if path.is_file():
                arc = f"{root}/relateby/{path.relative_to(project_dir / 'relateby')}"
                tf.add(path, arc.replace("\\", "/"))
        # Include build backend so sdist can build
        for path in (project_dir / "relateby_build").rglob("*.py"):
            arc = f"{root}/relateby_build/{path.relative_to(project_dir / 'relateby_build')}"
            tf.add(path, arc.replace("\\", "/"))

    return tarball_name


def get_requires_for_build_wheel(config_settings: dict[str, Any] | None = None) -> list[str]:
    return ["maturin>=1.0,<2.0"]


def get_requires_for_build_sdist(config_settings: dict[str, Any] | None = None) -> list[str]:
    return []
