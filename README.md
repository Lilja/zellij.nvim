# zellij.nvim

neovim integration with [zellij](https://github.com/zellij-org/zellij)

## Features

`:ZellijNavigate<Left|Right|Down|Up>`

For [vim-tmux-navigator](https://github.com/zellij-org/zellij) compatibility, use the `vimTmuxNavigatorKeybinds` prop during setup.

```lua
use {
    'Lilja/zellij.nvim',
    config = function()
        require('zellij').setup({})
    end
}
```

## Options

```lua
{
    path = "zellij", -- Zellij binary path
    replaceVimWindowNavigationKeybinds = false, -- Will set keybinds like <C-w>h to left
    vimTmuxNavigatorKeybinds = false, -- Will set keybinds like <C-h> to left
    debug = false, -- Will log things to /tmp/zellij.nvim
}
```
