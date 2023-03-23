"""
Unit Tests for `commands.py`
"""

import pytest
import typer

from ${{ carnate.project_name }} import commands


def test_what_am_i() -> None:
    """Test: Say hello to NAME"""
    assert commands.numeric_intake(2, 1) == 3
    assert commands.numeric_intake(2, 0) == 2
    assert commands.numeric_intake(1, 1) == 2


def test_version_callback():
    """
    Test error and non-error exit
    """
    assert commands.version_callback(False) is None
    with pytest.raises(typer.Exit):
        commands.version_callback(True)
