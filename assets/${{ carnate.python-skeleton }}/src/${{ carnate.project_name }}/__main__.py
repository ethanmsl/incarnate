"""
Entry point for the program.
(acts similarly to `if __name__ == '__main__': ...` in single script python modules.)
This also contains the top most logic for the program.

This file is only run if the package is run as a script.
It essentially is an inbuilt `if __name__ == '__main__': ...` statement.
"""

from .commands import app

########################################################################################
#                                      Execution
app()  # Typer (CLI framework) object
########################################################################################
