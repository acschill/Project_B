"""
Common utilities for the Project B CLI.

Safe, dependency-light helpers for:
- logging setup
- config loading (YAML and JSON; YAML is optional)
- stdout printing (human vs JSON)
- simple state persistence (save/load JSON)
- PID file helpers
- environment summary
"""

from __future__ import annotations
import json
import logging
import os
import pathlib
import sys
import time
from typing import Any, Dict, Optional

LOG = logging.getLogger("project_b.cli")

# Default config search order (first found wins)
DEFAULT_CONFIG_LOCATIONS = [
    os.environ.get("PROJECT_B_CONFIG", "") or "",
    os.path.expanduser("~/.project_b/config.yaml"),
    os.path.expanduser("~/.project_b/config.json"),
    os.path.join(os.getcwd(), "project_b", "config", "default.yaml"),
    os.path.join(os.getcwd(), "project_b", "config", "default.json"),
]

# ---- Filesystem helpers -----------------------------------------------------


def ensure_dirs() -> None:
    """Create local working dirs used by the CLI."""
    for d in (".runtime", "logs", "var"):
        pathlib.Path(d).mkdir(parents=True, exist_ok=True)


def pid_file() -> str:
    return ".runtime/pb.pid"


def write_pid() -> None:
    ensure_dirs()
    with open(pid_file(), "w", encoding="utf-8") as f:
        f.write(str(os.getpid()))


def read_pid() -> Optional[int]:
    try:
        with open(pid_file(), "r", encoding="utf-8") as f:
            return int(f.read().strip())
    except Exception:
        return None


def remove_pid() -> None:
    try:
        os.remove(pid_file())
    except FileNotFoundError:
        pass


# ---- Logging / output -------------------------------------------------------


def setup_logging(level: str = "INFO", json_mode: bool = False) -> None:
    """Configure root logger. Writes to stdout and logs/cli.log."""
    ensure_dirs()
    # Avoid duplicate handlers when called multiple times
    root = logging.getLogger()
    if root.handlers:
        for h in list(root.handlers):
            root.removeHandler(h)

    logging.basicConfig(
        level=getattr(logging, level.upper(), logging.INFO),
        format="%(asctime)s %(levelname)s %(name)s: %(message)s",
        handlers=[logging.StreamHandler(sys.stdout),
                  logging.FileHandler("logs/cli.log", encoding="utf-8")],
    )
    LOG.debug("Logging configured level=%s json=%s", level, json_mode)


def print_out(data: Any, json_mode: bool) -> None:
    """Print data either as compact JSON or human-readable JSON."""
    if json_mode:
        print(json.dumps(data, ensure_ascii=False, separators=(",", ":")))
    else:
        if isinstance(data, (dict, list)):
            print(json.dumps(data, indent=2, ensure_ascii=False))
        else:
            print(str(data))


# ---- JSON persistence -------------------------------------------------------


def save_json(path: str, data: Any) -> None:
    pathlib.Path(os.path.dirname(path) or ".").mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def load_json(path: str, default: Any) -> Any:
    try:
        with open(path, "r", encoding="utf-8") as f:
            return json.load(f)
    except Exception:
        return default


# ---- Config loading ---------------------------------------------------------


def _try_load_yaml(p: str) -> Optional[Dict[str, Any]]:
    if not p or not os.path.exists(p) or not p.lower().endswith((".yaml", ".yml")):
        return None
    try:
        import yaml  # optional dependency
    except Exception:
        return None
    try:
        with open(p, "r", encoding="utf-8") as f:
            return yaml.safe_load(f) or {}
    except Exception as e:
        LOG.warning("YAML load failed for %s: %s", p, e)
        return None


def _try_load_json(p: str) -> Optional[Dict[str, Any]]:
    if not p or not os.path.exists(p) or not p.lower().endswith(".json"):
        return None
    try:
        with open(p, "r", encoding="utf-8") as f:
            return json.load(f)
    except Exception as e:
        LOG.warning("JSON load failed for %s: %s", p, e)
        return None


def load_config(explicit_path: Optional[str] = None) -> Dict[str, Any]:
    """Load the first available config from explicit path or defaults."""
    if explicit_path:
        return _try_load_yaml(explicit_path) or _try_load_json(explicit_path) or {}
    for p in DEFAULT_CONFIG_LOCATIONS:
        cfg = _try_load_yaml(p) or _try_load_json(p)
        if cfg is not None:
            return cfg
    return {}


def get_config_value(cfg: Dict[str, Any], key: str, default: Any = None) -> Any:
    """Dot-path lookup (e.g., 'mm.lt.index_dir')."""
    cur: Any = cfg
    for part in key.split("."):
        if isinstance(cur, dict) and part in cur:
            cur = cur[part]
        else:
            return default
    return cur


# ---- Misc -------------------------------------------------------------------


def env_summary() -> Dict[str, Any]:
    return {
        "python": sys.version.split()[0],
        "executable": sys.executable,
        "cwd": os.getcwd(),
        "time": time.strftime("%Y-%m-%d %H:%M:%S"),
        "path_head": sys.path[:5],
    }


def confirm(prompt: str, assume_no: bool = False) -> bool:
    """Simple y/N prompt for destructive actions."""
    default = "n" if assume_no else "y"
    answer = input(f"{prompt} [{'Y/n' if not assume_no else 'y/N'}]: ").strip().lower()
    if not answer:
        answer = default
    return answer in ("y", "yes")
