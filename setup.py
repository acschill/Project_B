from setuptools import setup, find_packages
import pathlib

here = pathlib.Path(__file__).parent.resolve()
readme_path = here / "README.md"
long_description = readme_path.read_text(encoding="utf-8") if readme_path.exists() else ""

setup(
    name="project_b_cli",
    version="0.1.0",
    description="Command-line interface for Project B with full AEI/IM/MM debug and diagnostic capabilities",
    long_description=long_description,
    long_description_content_type="text/markdown",
    author="Alex Schill",
    python_requires=">=3.10",
    # Include BOTH 'cli' (top-level package) and 'project_b' if present
    packages=find_packages(include=["cli", "cli.*", "project_b", "project_b.*"]),
    install_requires=[
        "typer[all]>=0.9.0",
        "pyyaml>=6.0",
        "rich>=13.0.0",
    ],
    entry_points={
        "console_scripts": [
            # Matches the import below in the pb launcher
            "pb=cli.main:main",
        ],
    },
    include_package_data=True,
    package_data={
        # Ensure non-Python assets inside cli/ are packaged (configs/templates/etc.)
        "cli": ["**/*"],
        # If you also store CLI assets under project_b/cli/, keep this too:
        "project_b": ["cli/**/*", "cli/**/templates/*"],
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Environment :: Console",
        "Intended Audience :: Developers",
        "Topic :: Utilities",
    ],
)