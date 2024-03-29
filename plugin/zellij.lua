vim.api.nvim_create_user_command('ZellijNavigateLeft', function()
    require("zellij").zjNavigate('h')
end, { desc = "Navigate left vim window or zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNavigateRight', function()
    require("zellij").zjNavigate('l')
end, { desc = "Navigate right vim window or zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNavigateUp', function()
    require("zellij").zjNavigate('k')
end, { desc = "Navigate up vim window or zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNavigateDown', function()
    require("zellij").zjNavigate('j')
end, { desc = "Navigate down vim window or zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNewPane', function()
    require("zellij").newPane()
end, { desc = "New Zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNewTab', function()
    require("zellij").newTab()
end, { desc = "New Zellij tab", nargs = 0 })
vim.api.nvim_create_user_command('ZellijRenamePane', function(opts)
    require("zellij").renamePane(opts.args)
end, { desc = "Rename the current Zellij pane", nargs = 1 })
vim.api.nvim_create_user_command('ZellijRenameTab', function(opts)
    require("zellij").renameTab(opts.args)
end, { desc = "Rename the current Zellij tab", nargs = 1 })
vim.api.nvim_create_user_command('ZellijOpenDebugLog', function()
    require("zellij").openDebugLog()
end, { desc = "Rename the current Zellij pane", nargs = 0 })
