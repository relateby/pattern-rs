"""
PEP 517 build backend for unified relateby package.
Builds pattern-core and gram-codec via maturin, assembles one wheel with
relateby.pattern and relateby.gram (no top-level pattern_core/gram_codec).
"""
from __future__ import annotations

import base64
import re
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path
from typing import Any


def _repo_root() -> Path:
    """Repository root (pattern-rs)."""
    current = Path(__file__).resolve()
    for candidate in current.parents:
        if (candidate / "crates" / "pattern-core" / "Cargo.toml").exists():
            return candidate
    raise RuntimeError("Could not locate repository root containing crates/pattern-core")


def _project_dir() -> Path:
    """Unified package dir (python/packages/relateby)."""
    current = Path(__file__).resolve()
    for candidate in current.parents:
        if (candidate / "pyproject.toml").exists() and (candidate / "relateby").exists():
            return candidate
    raise RuntimeError("Could not locate project directory containing pyproject.toml and relateby/")


def _read_version() -> str:
    """Read version from pyproject.toml."""
    pyproject = _project_dir() / "pyproject.toml"
    text = pyproject.read_text()
    m = re.search(r'version\s*=\s*["\']([^"\']+)["\']', text)
    if not m:
        return "0.1.0"
    return m.group(1)


def _read_project_name() -> str:
    """Read distribution name from pyproject.toml."""
    pyproject = _project_dir() / "pyproject.toml"
    text = pyproject.read_text()
    m = re.search(r'name\s*=\s*["\']([^"\']+)["\']', text)
    if not m:
        return "relateby-pattern"
    return m.group(1)


def _wheel_distribution_name(name: str) -> str:
    """Normalize distribution name for wheel filenames and dist-info directories."""
    return re.sub(r"[-.]+", "_", name)


def _run_maturin(manifest_path: Path, cwd: Path) -> None:
    import os

    env = os.environ.copy()
    env["PYO3_PYTHON"] = sys.executable
    maturin = shutil.which("maturin")
    if maturin is None:
        command = [
            sys.executable,
            "-m",
            "maturin",
            "build",
            "--release",
            "--manifest-path",
            str(manifest_path),
            "--features",
            "python",
        ]
    else:
        command = [
            maturin,
            "build",
            "--release",
            "--manifest-path",
            str(manifest_path),
            "--features",
            "python",
        ]
    subprocess.run(
        command,
        check=True,
        cwd=cwd,
        env=env,
    )


def _extract_so_from_wheel(whl_path: Path, out_dir: Path) -> list[Path]:
    """Extract compiled extension modules from wheel into out_dir."""
    extracted: list[Path] = []
    with zipfile.ZipFile(whl_path, "r") as zf:
        for name in zf.namelist():
            if not (name.endswith(".so") or name.endswith(".pyd")):
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
    h = base64.urlsafe_b64encode(hashlib.sha256(data).digest()).rstrip(b"=").decode("ascii")
    return ("sha256", h)


def _iter_sdist_files(repo: Path, project_dir: Path) -> list[tuple[Path, str]]:
    """Return source files to include in the sdist."""
    files: list[tuple[Path, str]] = []

    def add_file(path: Path, arc: str) -> None:
        if path.is_file():
            files.append((path, arc.replace("\\", "/")))

    add_file(project_dir / "pyproject.toml", "pyproject.toml")
    add_file(project_dir / "README.md", "README.md")
    add_file(repo / "Cargo.toml", "Cargo.toml")

    for path in (project_dir / "relateby").rglob("*"):
        if path.is_file() and (path.suffix in {".py", ".pyi"} or path.name == "py.typed"):
            add_file(path, f"relateby/{path.relative_to(project_dir / 'relateby')}")

    for path in (project_dir / "relateby_build").rglob("*.py"):
        add_file(path, f"relateby_build/{path.relative_to(project_dir / 'relateby_build')}")

    for crate_name in ("pattern-core", "gram-codec"):
        crate_dir = repo / "crates" / crate_name
        for path in crate_dir.rglob("*"):
            if not path.is_file():
                continue
            if any(part in {"target", ".pytest_cache", "__pycache__"} for part in path.parts):
                continue
            if path.suffix in {".rs", ".toml", ".md", ".lock"} or path.name in {"Cargo.toml", "README.md"}:
                add_file(path, f"crates/{crate_name}/{path.relative_to(crate_dir)}")

    benches_dir = repo / "benches"
    for path in benches_dir.rglob("*"):
        if not path.is_file():
            continue
        if any(part in {"target", "__pycache__"} for part in path.parts):
            continue
        if path.suffix in {".rs", ".toml", ".md", ".lock"} or path.name == "Cargo.toml":
            add_file(path, f"benches/{path.relative_to(benches_dir)}")

    return files


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
    project_name = _read_project_name()
    wheel_dist_name = _wheel_distribution_name(project_name)

    pattern_manifest = repo / "crates" / "pattern-core" / "Cargo.toml"
    gram_manifest = repo / "crates" / "gram-codec" / "Cargo.toml"
    if not pattern_manifest.exists() or not gram_manifest.exists():
        raise RuntimeError(
            "Unified build must be run from the repository: crates/pattern-core and crates/gram-codec must exist"
        )

    # Build both crates (maturin puts wheels in workspace target/wheels)
    wheels_dir = repo / "target" / "wheels"
    wheels_dir.mkdir(parents=True, exist_ok=True)
    for old_wheel in wheels_dir.glob("*.whl"):
        old_wheel.unlink()

    _run_maturin(pattern_manifest, repo)
    _run_maturin(gram_manifest, repo)

    pattern_whl, gram_whl = _find_wheels(wheels_dir)
    if not pattern_whl or not gram_whl:
        raise RuntimeError(
            f"Expected pattern_core and gram_codec wheels in {wheels_dir}"
        )

    # Use tag from one of the wheels (same platform)
    tag = _wheel_tag_from_whl(pattern_whl)
    wheel_basename = f"{wheel_dist_name}-{version}-{tag}.whl"
    wheel_path = Path(wheel_directory) / wheel_basename

    # Staging for wheel contents
    import tempfile

    with tempfile.TemporaryDirectory() as stage:
        stage_p = Path(stage)
        native_dir = stage_p / "relateby" / "_native"
        native_dir.mkdir(parents=True)

        # Extract compiled extension modules from each wheel into staging
        _extract_so_from_wheel(pattern_whl, stage_p)
        _extract_so_from_wheel(gram_whl, stage_p)
        # Move compiled extension modules into relateby/_native/
        for extension in (".so", ".pyd"):
            for module_path in stage_p.glob(f"**/*{extension}"):
                if module_path.parent != native_dir:
                    shutil.move(str(module_path), str(native_dir / module_path.name))

        # Copy relateby package tree, including shipped type information.
        relateby_src = project_dir / "relateby"
        for source in relateby_src.rglob("*"):
            if not source.is_file():
                continue
            if source.suffix not in {".py", ".pyi"} and source.name != "py.typed":
                continue
            rel = source.relative_to(relateby_src)
            dest = stage_p / "relateby" / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, dest)

        # Ensure _native has __init__.py
        (native_dir / "__init__.py").write_text("# Native extensions: pattern_core, gram_codec\n")

        # Build METADATA from pyproject
        metadata_content = _generate_metadata(project_dir, version)
        dist_info = stage_p / f"{wheel_dist_name}-{version}.dist-info"
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
            rel = f"{wheel_dist_name}-{version}.dist-info/" + f.name
            algo, h = _hash_file(f)
            size = f.stat().st_size
            record_lines.append(f"{rel},{algo}={h},{size}\n")
        record_lines.append(f"{wheel_dist_name}-{version}.dist-info/RECORD,,\n")
        (dist_info / "RECORD").write_text("".join(record_lines), encoding="utf-8")

        # Zip into wheel
        with zipfile.ZipFile(wheel_path, "w", zipfile.ZIP_DEFLATED) as zf:
            for f in (stage_p / "relateby").rglob("*"):
                if f.is_file():
                    arc = "relateby/" + str(f.relative_to(stage_p / "relateby")).replace("\\", "/")
                    zf.write(f, arc)
            for f in dist_info.iterdir():
                zf.write(f, f"{wheel_dist_name}-{version}.dist-info/" + f.name)

    return wheel_basename


def _load_pyproject(project_dir: Path) -> dict[str, Any]:
    """Load pyproject.toml as TOML (stdlib tomllib on 3.11+, else tomli)."""
    pyproject = project_dir / "pyproject.toml"
    text = pyproject.read_text(encoding="utf-8")
    if sys.version_info >= (3, 11):
        import tomllib
        return tomllib.loads(text)  # type: ignore[attr-defined]
    import tomli
    return tomli.loads(text)


def _generate_metadata(project_dir: Path, version: str) -> str:
    """METADATA content from pyproject.toml, including optional-dependencies."""
    data = _load_pyproject(project_dir)
    project = data.get("project", {})
    name = project.get("name", "relateby-pattern")
    desc = project.get("description", "Combined Python distribution for relateby.pattern and relateby.gram")
    requires_python = project.get("requires-python", ">=3.8")

    lines = [
        "Metadata-Version: 2.1",
        f"Name: {name}",
        f"Version: {version}",
        f"Summary: {desc}",
        "License: Apache-2.0",
        f"Requires-Python: {requires_python}",
    ]

    # Optional dependencies (extras) for pip install relateby-pattern[dev], etc.
    optional = project.get("optional-dependencies", {})
    for extra, deps in optional.items():
        lines.append(f"Provides-Extra: {extra}")
        for dep in deps:
            if isinstance(dep, str):
                lines.append(f"Requires-Dist: {dep}; extra == {repr(extra)}")

    return "\n".join(lines) + "\n"


def build_sdist(
    sdist_directory: str,
    config_settings: dict[str, Any] | None = None,
) -> str:
    """Build sdist. Source tree must be built from repo (crates available)."""
    import tarfile

    repo = _repo_root()
    project_dir = _project_dir()
    version = _read_version()
    project_name = _read_project_name()
    wheel_dist_name = _wheel_distribution_name(project_name)
    tarball_name = f"{wheel_dist_name}-{version}.tar.gz"
    tarball_path = Path(sdist_directory) / tarball_name

    with tarfile.open(tarball_path, "w:gz", format=tarfile.PAX_FORMAT) as tf:
        root = f"{wheel_dist_name}-{version}"
        for path, arc in _iter_sdist_files(repo, project_dir):
            tf.add(path, f"{root}/{arc}")

    return tarball_name


def get_requires_for_build_wheel(config_settings: dict[str, Any] | None = None) -> list[str]:
    return ["maturin>=1.0,<2.0"]


def get_requires_for_build_sdist(config_settings: dict[str, Any] | None = None) -> list[str]:
    return []
