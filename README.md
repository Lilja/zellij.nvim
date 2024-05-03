# zellij.nvim

## Archived!

Please try to use migrate to an alternative, if you are using this plugin. I do not use zellij and this plugin is essentially a proof of concept that you could seamlessly navigate between neovim and zellij.

Read more here
- [Link 1](https://github.com/zellij-org/zellij/issues/967)
- [Link 2](https://github.com/Lilja/zellij.nvim/issues/6)

neovim integration with [zellij](https://github.com/zellij-org/zellij)

## Features

* `:ZellijNavigate<Left|Right|Down|Up>`
* `:ZellijNewPane`
* `:ZellijNewTab`
* `:ZellijRenamePane`
* `:ZellijRenameTab`

For [vim-tmux-navigator](https://github.com/christoomey/vim-tmux-navigator) compatibility, use the `vimTmuxNavigatorKeybinds` prop during setup.

```lua
use {
    'Lilja/zellij.nvim',
    -- If you want to configure the plugin
    --[[
    config = function()
        require('zellij').setup({})
    end
    ]]
}
```

## Options

```lua
{
    -- keys with designated default values.
    path = "zellij", -- Zellij binary path
    replaceVimWindowNavigationKeybinds = false, -- Will set keybinds like <C-w>h to left
    vimTmuxNavigatorKeybinds = false, -- Will set keybinds like <C-h> to left
    debug = false, -- Will log things to /tmp/zellij.nvim
}
```
