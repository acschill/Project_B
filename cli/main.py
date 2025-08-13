import typer
import importlib
import sys
from pathlib import Path
from cli import main

app = typer.Typer(help="Project B CLI: Run, inspect, and debug Project B subsystems.")

# Dynamically load commands from the commands/ directory
def load_commands():
    commands_dir = Path(__file__).parent / "commands"
    if not commands_dir.exists():
        return
    for file in commands_dir.glob("*.py"):
        if file.name.startswith("_") or file.name == "__init__.py":
            continue
        module_name = f"commands.{file.stem}"
        try:
            module = importlib.import_module(module_name)
            if hasattr(module, "app"):
                app.add_typer(module.app, name=file.stem)
        except Exception as e:
            typer.echo(f"Warning: Failed to load command '{file.stem}': {e}", err=True)

@app.callback()
def main(
    config: Path = typer.Option(None, "--config", help="Use specific config file."),
    log_level: str = typer.Option("INFO", "--log-level", help="Override logging level."),
    json_output: bool = typer.Option(False, "--json", help="Output JSON instead of human-readable."),
    no_color: bool = typer.Option(False, "--no-color", help="Disable ANSI colors."),
    cwd: Path = typer.Option(None, "--cwd", help="Run relative to working directory."),
    profile_cpu: bool = typer.Option(False, "--profile-cpu", help="Profile CPU usage for the invoked command."),
):
    """
    Project B CLI entry point. Use --help for available commands and global flags.
    """
    # Here you could initialize logging, config, etc.
    pass

@app.command()
def hello():
    """Say hello from Project B CLI!"""
    typer.echo("Hello from Project B CLI!")

def main():
    load_commands()
    app()

if __name__ == "__main__":
    main()