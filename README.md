# Noogle CLI

A command-line interface for searching [Noogle](https://noogle.dev/) 

## Features

- **Fuzzy finding** through Noogle's dataset
- **Neovim integration** via included plugin

## Build

### CLI Tool

```bash
nix build .#noogle-cli
```

### Neovim plugin

This will build the latest CLI and bundle it with the neovim plugin

```bash
nix build .#noogle-nvim
```

## Usage

### CLI

```bash
# Basic search
noogle search "callPackage"

# Show metadata for function
noogle show "lib.strings.optionalString"

# Show JSON data for first result
noogle search "optionalString" | head -n 1 | noogle show --json
```

### Neovim Plugin

Make sure you have [godoc.nvim](https://github.com/fredrikaverpil/godoc.nvim) installed as this plugin only provides an adapter

Add to your Neovim configuration (using your preferred plugin manager):

With [nixvim]()
```nix
TODO:
```

With [lazy.nvim](https://github.com/folke/lazy.nvim):
```lua
{
  {
    'fredrikaverpil/godoc.nvim',
    dependencies = {
      {
        'juliamertz/noogle-cli',
        -- optionally build from source, this is only necessary if you don't have noogle in your PATH
        build = 'nix build .#noogle-nvim && cp -vrTL result/lua lua',
      },
    },
    opts = {
      adapters = {
        { setup = function() return require('noogle').setup() end },
      },
    },
  },
}
```

The adapter provides the following command:

```vim
:Noogle  " Open picker
```
