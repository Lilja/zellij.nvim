local Bindings = {}

function Bindings.setupVimTmuxNavigatorBindings(shouldUseWhichKey)
    if shouldUseWhichKey then
        local wk = require("which-key")
        wk.register({
            ["<C-h>"] = { "<cmd>ZellijNavigateLeft<cr>", "Navigate left vim window/zellij  pane" },
            ["<C-j>"] = { "<cmd>ZellijNavigateDown<cr>", "Navigate down vim window/zellij  pane" },
            ["<C-k>"] = { "<cmd>ZellijNavigateUp<cr>", "Navigate up vim window/zellij pane" },
            ["<C-l>"] = { "<cmd>ZellijNavigateRight<cr>", "Navigate right vim window/zellij pane" },
        })
    else
        vim.keymap.set('n', '<C-h>', "<cmd>ZellijNavigateLeft<cr>")
        vim.keymap.set('n', '<C-j>', "<cmd>ZellijNavigateDown<cr>")
        vim.keymap.set('n', '<C-k>', "<cmd>ZellijNavigateUp<cr>")
        vim.keymap.set('n', '<C-l>', "<cmd>ZellijNavigateRight<cr>")
    end
end

function Bindings.setupVimWindowBindings(shouldUseWhichKey)
    if shouldUseWhichKey then
        local wk = require("which-key")
        wk.register({
            ["<C-w>h"] = { "<cmd>ZellijNavigateLeft<cr>", "Navigate left vim window/zellij  pane" },
            ["<C-w>j"] = { "<cmd>ZellijNavigateDown<cr>", "Navigate down vim window/zellij  pane" },
            ["<C-w>k"] = { "<cmd>ZellijNavigateUp<cr>", "Navigate up vim window/zellij pane" },
            ["<C-w>l"] = { "<cmd>ZellijNavigateRight<cr>", "Navigate right vim window/zellij pane" },
        })
    else
        vim.keymap.set('n', '<C-w>h', "<cmd>ZellijNavigateLeft<cr>")
        vim.keymap.set('n', '<C-w>j', "<cmd>ZellijNavigateDown<cr>")
        vim.keymap.set('n', '<C-w>k', "<cmd>ZellijNavigateUp<cr>")
        vim.keymap.set('n', '<C-w>l', "<cmd>ZellijNavigateRight<cr>")
    end
end

return Bindings
