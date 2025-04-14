# telescope-nucleo-sorter.nvim

`nucleo-sorter` is a Telescope extension for the [`nucleo`](https://github.com/helix-editor/nucleo) fuzzy matcher used by the [Helix](https://helix-editor.com/) editor.

## Installation

To get `telescope-nucleo-sorter` working, a [Rust installation](https://www.rust-lang.org/tools/install) is required.

### lazy.nvim

```lua
{
  'altermo/telescope-nucleo-sorter.nvim',
  build = 'cargo --release'
  -- on macos, you may need below to make build work
  -- build = 'cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup',
},
```

## Telescope Setup and Configuration

```lua
-- These are the default options so they are not required.
require('telescope').setup({
  extensions = {
    nucleo = {
      -- or 'ignore' or 'respect'
      case_mode = "smart",
      -- or 'never'
      normalize_mode = "smart",
    }
  }
})
-- To load nucleo, the `load_extension` function has to be called:
require('telescope').load_extension('nucleo')
```
