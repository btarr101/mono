"""Custom Ansible Lookup Plugin for reading dotenv files. (modified)

Ansible officially supports Python 2.7 and 3.5+, but this requires Python 3.8+.

"""

from __future__ import annotations

import os
from typing import Sequence

from ansible.errors import AnsibleFileNotFound, AnsibleLookupError
from ansible.plugins.lookup import LookupBase
from re import compile


__all__ = "DOCUMENTATION", "EXAMPLES", "RETURN", "LookupModule"


# Options not documented will be ignored during processing.

DOCUMENTATION = """
  name: dotenv
  author:
    - Michael Klatt <mdklatt(at)alumni.ou.edu>
  short_description: Retrieve values from dotenv files
  requirements:
    - Ansible must be running under Python 3.8+.
  description:
    - Retrieve values from dotenv values.
  seealso:
    - name: RFC 2 - .env file
      description: Smartmob RFC for a .env file standard
      link: https://smartmob-rfc.readthedocs.io/en/latest/2-dotenv.html
  notes:
    - The RFC 2 continuation line syntax is not yet supported.
  options:
    _terms:
      description: The key(s) to look up.
      required: True
    file:
      description: Path to the dotenv file.
      default: '.env'
"""  # YAML


RETURN = """
  _raw:
    description:
      - value(s) of the search term(s) in the dotenv file
    type: list
    elements: str
"""  # YAML


EXAMPLES = """
  - name: Retrieve value from the .env file in the current working directory.
    debug:
      msg: "{{ lookup('dotenv', 'VAR') }}"
  - name: Use a non-default dotenv file.
    debug:
      msg: "{{ lookup('dotenv', 'VAR', file='path/to/.env') }}"
"""  # YAML


class LookupModule(LookupBase):  # class name is not arbitrary, DO NOT CHANGE
    """Look up values from a dotenv file."""

    def run(self, terms: Sequence[str], variables=None, **options) -> list:
        """Execute the lookup.

        :param terms: search terms
        :param variables: mapping of defined Ansible variables
        :param options: options passed directly as keyword arguments
        :return: list of found values
        """
        # Adapted from 'ansible.builtin.ini' lookup.
        # <https://github.com/ansible/ansible/blob/devel/lib/ansible/plugins/lookup/ini.py>
        # <https://docs.ansible.com/ansible/latest/dev_guide/developing_plugins.html#developing-lookup-plugins>
        self.set_options(var_options=variables, direct=options)
        params = self.get_options()
        path = self.find_file_in_search_path(variables, "files", params["file"])
        var_pattern = compile(r"^\s*([a-zA-Z_]+[a-zA-Z0-9_]*)\s*=\s*(.*)\s*$")
        dotenv_values = {}
        if path:
            with open(path, "rt") as file:
                for line in file.readlines():
                    # Parse the dotenv file permissively, accepting valid NAME=VALUE
                    # lines while ignoring everything else.
                    # TODO: Support RFC 2 line continuation syntax.
                    if match := var_pattern.match(line):
                        value = match.group(2).strip()
                        if (
                            len(value) >= 2
                            and value[0] == value[-1]
                            and value[0] in ('"', "'")
                        ):
                            value = value[1:-1]
                        dotenv_values[match.group(1)] = value

        resolved_values = []
        for key in terms:
            dotenv_value = dotenv_values.get(key, "")
            if dotenv_value:
                resolved_values.append(dotenv_value)
                continue

            env_value = os.getenv(key, "")
            if env_value:
                resolved_values.append(env_value)
                continue

            if path:
                raise AnsibleLookupError(
                    f"No value for '{key}' in {path} or process environment"
                )
            raise AnsibleFileNotFound(
                f"Could not find file {params['file']} and no process environment value for '{key}'"
            )

        return resolved_values
