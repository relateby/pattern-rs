"""
PEP 517 build backend for relateby-gram (single-crate: gram-codec only).
Produces a wheel that installs into the relateby namespace (relateby.gram).
"""
from __future__ import annotations

import re
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path
from typing import Any

# Which crate to build; must match crate artifact name (pattern_core / gram_codec)
COMPONENT = "gram"
CRATE_DIR = "gram-codec"
WHEEL_NAME_PREFIX = "gram_codec"


def _repo_root() -> Path:
    """Repository root (pattern-rs)."""
    # This file is python/relateby-gram/relateby_build/__init__.py
    return Path(__file__).resolve().parent.parent.parent.parent


def _project_dir() -> Path:
    """Project dir (python/relateby-gram)."""
    return Path(__file__).resolve().parent.parent


def _read_version() -> str:
    pyproject = _project_dir() / "pyproject.toml"
    text = pyproject.read_text()
    m = re.search(r'version\s*=\s*["\']([^"\']+)["\']', text)
    return m.group(1) if m else "0.1.0"


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
                try:
                    src.parent.rmdir()
                except OSError:
                    pass
            extracted.append(dest)
    return extracted


def _wheel_tag_from_whl(whl_path: Path) -> str:
    stem = whl_path.stem
    parts = stem.split("-")
    if len(parts) >= 3:
        return "-".join(parts[2:])
    return "py3-none-any"


def _hash_file(path: Path) -> tuple[str, str]:
    import hashlib
    h = hashlib.sha256(path.read_bytes()).hexdigest()
    return ("sha256", h)


def _load_pyproject(project_dir: Path) -> dict[str, Any]:
    text = (project_dir / "pyproject.toml").read_text(encoding="utf-8")
    if sys.version_info >= (3, 11):
        import tomllib
        return tomllib.loads(text)  # type: ignore[attr-defined]
    import tomli
    return tomli.loads(text)


def _generate_metadata(project_dir: Path, version: str, dist_name: str) -> str:
    data = _load_pyproject(project_dir)
    project = data.get("project", {})
    name = project.get("name", "relateby-gram")
    desc = project.get("description", "")
    requires_python = project.get("requires-python", ">=3.8")
    lines = [
        "Metadata-Version: 2.1",
        f"Name: {name}",
        f"Version: {version}",
        f"Summary: {desc}",
        "License: Apache-2.0",
        f"Requires-Python: {requires_python}",
    ]
    optional = project.get("optional-dependencies", {})
    for extra, deps in optional.items():
        lines.append(f"Provides-Extra: {extra}")
        for dep in deps:
            if isinstance(dep, str):
                lines.append(f"Requires-Dist: {dep}; extra == {repr(extra)}")
    return "\n".join(lines) + "\n"


def build_wheel(
    wheel_directory: str,
    config_settings: dict[str, Any] | None = None,
    metadata_directory: str | None = None,
) -> str:
    repo = _repo_root()
    project_dir = _project_dir()
    version = _read_version()
    manifest = repo / "crates" / CRATE_DIR / "Cargo.toml"
    if not manifest.exists():
        raise RuntimeError(
            f"Build must be run from the repository: crates/{CRATE_DIR} must exist"
        )

    _run_maturin(manifest, repo)
    wheels_dir = repo / "target" / "wheels"
    whl = None
    for f in wheels_dir.iterdir():
        if f.suffix == ".whl" and f.stem.split("-")[0] == WHEEL_NAME_PREFIX:
            whl = f
            break
    if not whl:
        raise RuntimeError(
            f"Expected {WHEEL_NAME_PREFIX} wheel in {wheels_dir}"
        )

    tag = _wheel_tag_from_whl(whl)
    dist_name = "relateby_gram"  # PEP 427: hyphen -> underscore
    wheel_basename = f"{dist_name}-{version}-{tag}.whl"
    wheel_path = Path(wheel_directory) / wheel_basename

    import tempfile
    with tempfile.TemporaryDirectory() as stage:
        stage_p = Path(stage)
        native_dir = stage_p / "relateby" / "_native"
        native_dir.mkdir(parents=True)

        _extract_so_from_wheel(whl, stage_p)
        for so in stage_p.glob("**/*.so"):
            if so.parent != native_dir:
                shutil.move(str(so), str(native_dir / so.name))

        relateby_src = project_dir / "relateby"
        for py in relateby_src.rglob("*.py"):
            rel = py.relative_to(relateby_src)
            dest = stage_p / "relateby" / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(py, dest)

        (native_dir / "__init__.py").write_text(
            "# Native extension(s): gram_codec\n", encoding="utf-8"
        )

        metadata_content = _generate_metadata(project_dir, version, dist_name)
        dist_info = stage_p / f"{dist_name}-{version}.dist-info"
        dist_info.mkdir()
        (dist_info / "METADATA").write_text(metadata_content, encoding="utf-8")
        (dist_info / "WHEEL").write_text(
            "Wheel-Version: 1.0\nGenerator: relateby_build\nRoot-Is-Pure-Lib: false\nTag: " + tag + "\n",
            encoding="utf-8",
        )

        record_lines: list[str] = []
        for f in sorted((stage_p / "relateby").rglob("*")):
            if f.is_file():
                rel = "relateby/" + str(f.relative_to(stage_p / "relateby")).replace("\\", "/")
                algo, h = _hash_file(f)
                record_lines.append(f"{rel},{algo}={h},{f.stat().st_size}\n")
        for f in sorted(dist_info.iterdir()):
            rel = f"{dist_name}-{version}.dist-info/" + f.name
            algo, h = _hash_file(f)
            record_lines.append(f"{rel},{algo}={h},{f.stat().st_size}\n")
        record_lines.append(f"{dist_name}-{version}.dist-info/RECORD,,\n")
        (dist_info / "RECORD").write_text("".join(record_lines), encoding="utf-8")

        with zipfile.ZipFile(wheel_path, "w", zipfile.ZIP_DEFLATED) as zf:
            for f in (stage_p / "relateby").rglob("*"):
                if f.is_file():
                    arc = "relateby/" + str(f.relative_to(stage_p / "relateby")).replace("\\", "/")
                    zf.write(f, arc)
            for f in dist_info.iterdir():
                zf.write(f, f"{dist_name}-{version}.dist-info/" + f.name)

    return wheel_basename


def build_sdist(
    sdist_directory: str,
    config_settings: dict[str, Any] | None = None,
) -> str:
    import tarfile
    project_dir = _project_dir()
    version = _read_version()
    dist_name = "relateby-gram"
    tarball_name = f"{dist_name}-{version}.tar.gz"
    tarball_path = Path(sdist_directory) / tarball_name
    root = f"{dist_name}-{version}"

    with tarfile.open(tarball_path, "w:gz", format=tarfile.PAX_FORMAT) as tf:
        for path in [project_dir / "pyproject.toml", project_dir / "README.md"]:
            if path.exists():
                tf.add(path, f"{root}/{path.name}")
        for path in (project_dir / "relateby").rglob("*"):
            if path.is_file():
                arc = f"{root}/relateby/{path.relative_to(project_dir / 'relateby')}"
                tf.add(path, arc.replace("\\", "/"))
        for path in (project_dir / "relateby_build").rglob("*.py"):
            arc = f"{root}/relateby_build/{path.relative_to(project_dir / 'relateby_build')}"
            tf.add(path, arc.replace("\\", "/"))

    return tarball_name


def get_requires_for_build_wheel(config_settings: dict[str, Any] | None = None) -> list[str]:
    return ["maturin>=1.0,<2.0", "tomli>=2.0; python_version<'3.11'"]


def get_requires_for_build_sdist(config_settings: dict[str, Any] | None = None) -> list[str]:
    return []
