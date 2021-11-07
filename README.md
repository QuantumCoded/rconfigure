# R(e)Configure

Tight-knit, profile-based control over any config files on your system and more.

## Mission Statement

The design purpose of `rconfigure` is to make customization of Linux systems more dynamic by providing easy ways to manage and mass edit config (or really any plain text) files.

## How it Works

`rconfigure` takes a "layered" approach to generating files. The three main components are: profiles, settings, and templates. Profiles group settings together, settings contain values to use in the templates, and templates contain the file contents and the path to overwrite. Hooks and scripts can also be used for more specialized configurations. Hooks allow running of commands or bash scripts whenever a setting or profile is enabled or disabled. Scripts (not to be confused with bash scripts) are Rhai scripts used to transform setting values in some way before templating.