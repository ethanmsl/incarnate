"""
Currently holding *de facto* `main()`
Later may hold equivelnt commands for various purposes.
"""

from importlib import metadata

import typer
from colorama import Fore, Style
from rich import print as rprint


# generage CLI app object
app = typer.Typer(rich_markup_mode="rich", add_completion=False)

# get version from pyproject.toml
__version__ = metadata.version(__package__)


def version_callback(version: bool):
    """
    Print app version and exit
    """
    if version:
        rprint(f"{{ carnate.project_name }} ('${{ carnate.cli_app_name }}') Version: {__version__}")
        raise typer.Exit()


@app.callback(help="[bold]${{ carnate.project_name }}[/bold] CLI App for [green]PagerDuty[/green]")
def app_options(
    _: bool = typer.Option(
        None,
        "--version",
        help="Show version of this app",
        callback=version_callback,
        is_eager=True,
    )
):
    """
    This callback is called by the **base app** itself.
    Sub-callbacks are used by the options to perform actions.
    The eager sub-callback allows us to circumvent typer's expectation that a regular
    command is still comming.

    (Side Note: Yes, I agree this is slightly awkward for something as standard as
    `--version`, but it does seem to be the best way to do it in this framework.)
    """

##################################################################################
# Regular 'ol Commands
##################################################################################

@app.command("spin")
def spinner_example(seconds: int = typer.Argument(5, min=1, max=36)) -> None:
    """Example of a progress bar"""

    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}", justify="right"),
        transient=True,
    ) as progress:
        progress.add_task("Task A...", total=seconds)
        task = progress.add_task("Task B...", total=seconds)
        for _ in range(seconds):
            time.sleep(1)
            progress.advance(task)


@app.command("progbar")
def progress_bar_example(
    seconds: int = typer.Argument(5, min=1, max=16), plain_bar: bool = False
) -> None:
    """
    Example of a progress bar.
    Default uses Rich. (colorful, but simple)
    Use `--plain-bar` to use Typer's progress bar.
    Which actually has a very nice, minimalist ascii aesthetic.
    """

    if not plain_bar:
        total_so_far: int = 0
        for _ in track(range(seconds), description="Sleeping..."):
            time.sleep(1)
            total_so_far += 1
        rprint(f"Done sleeping for {total_so_far} seconds")
    else:
        total_so_far_2 = 0
        with typer.progressbar(range(seconds), label="Sleeping...") as progress:
            for _ in progress:
                time.sleep(1)
                total_so_far_2 += 1
        rprint(f"Done sleeping for {total_so_far_2} seconds")


@app.command("nums")
def numeric_intake(
    x_int: int = typer.Argument(..., min=0, max=2),
    y_int: int = typer.Argument(..., min=-1, max=1),
) -> int:
    """testing `min` and `max` restrictions on numeric arguments"""
    print(f"X: {x_int}, Y: {y_int}")
    return x_int + y_int

