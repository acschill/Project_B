from setuptools import setup, find_packages
import pathlib

# Read the README for long_description if available
here = pathlib.Path(__file__).parent.resolve()
long_description = (here / "README.md").read_text(encoding="utf-8")

setup(
    name="project_b_cli",
    version="0.1.0",
    author="Alex Schill",
    author_email="",
    description="Command-line interface for Project B with full AEI/IM/MM debug and diagnostic capabilities",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="",  # Optionally add repo URL
    python_requires=">=3.10",
    packages=find_packages(exclude=["tests*", "docs*"]),
    install_requires=[
        "typer[all]>=0.9.0",
        "pyyaml>=6.0",       # for config loading
        "rich>=13.0.0",      # optional: better CLI output
    ],
    entry_points={
        "console_scripts": [
            # Assumes your CLI entrypoint is project_b/cli/main.py
            "pb=cli.main:main",
        ],
    },
    include_package_data=True,
    package_data={
        # Include non-Python files from the CLI package (like JSON, YAML configs)
        "project_b": ["cli/**/*", "cli/**/templates/*", "."],
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",  # Adjust if different
        "Operating System :: OS Independent",
        "Environment :: Console",
        "Intended Audience :: Developers",
        "Topic :: Software Development :: Libraries :: Application Frameworks",
        "Topic :: Utilities",
    ],
    project_urls={
        "Bug Tracker": "",    # Optionally add issue tracker URL
        "Documentation": "",  # Optionally add docs URL
        "Source Code": "",    # Optionally add repo URL
    },
)