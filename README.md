# Lazyme [![Latest Version](https://img.shields.io/crates/v/lazyme.svg)](https://crates.io/crates/lazyme) 

Lazyme is a simple tool that helps you optimise your laziness. It displays your most often used shell commands so that you can change them into aliases and eventually type less.

## Installation
```bash
cargo install lazyme
```

## Usage
```
lazyme =>
+---------------------------------------------+-------+
|                     Lazyme                          |
+---------------------------------------------+-------+
|...                                          | ...   |
| ei                                          | 21    |
| gpstg                                       | 22    |
| gstp                                        | 23    |
| zs                                          | 28    |
| s .                                         | 30    |
| zrr                                         | 32    |
| gpshh                                       | 60    |
| rss                                         | 70    |
| c                                           | 75    |
| gd                                          | 107   |
| o .                                         | 123   |
| gst                                         | 130   |
| ls                                          | 179   |
| gl                                          | 310   |
| gp                                          | 445   |
| gds                                         | 540   |
| gaa                                         | 817   |
| g                                           | 3365  |
+---------------------------------------------+-------+
| Command                                     | Count |
+---------------------------------------------+-------+
```

By default it parses `$HOME/.bash_history` or `$HOME/.zsh_history`. Optionally you can provide a custom history file path:

```bash
lazyme ~/Users/me/.my_history
```

You can set aliases by adding following lines into your `.bashrc` or `.zshrc` files:

```
alias gr='grep --color'
alias gaa='git add . -A'
```
