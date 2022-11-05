local keybindings = require('zellij.keybindings')

local zellij = {}

local directionTranslation = {
    h = "left",
    j = "down",
    k = "up",
    l = "right",
}
local ACTION = "action"
local MOVE_FOCUS_OR_TAB = "move-focus-or-tab"
local RENAME_PANE = "rename-pane"
local RENAME_TAB = "rename-tab"
local NEW_PANE = "new-pane"
local NEW_TAB = "new-tab"

function zellij.log(message)
    if zellij.opts.debug then
        vim.fn.system("mkdir /tmp/zellij.nvim")
        local formattedDate = "[" .. os.date("%T") .. "] "
        local log_file = io.open(zellij.opts.logPath, "a")
        io.output(log_file)
        local msgToWrite = formattedDate .. message
        io.write(msgToWrite .. "\n")
        io.close(log_file)
    end
end

function zellij.openDebugLog()
    if zellij.opts.debug ~= true then
        print("Not in debug mode, set debug = true in setup() to view logs.")
    else
        vim.cmd(":vsplit " .. zellij.opts.logPath)
    end
end

function zellij.version()
    return zellij.ZellijCommand("--version", true):match"^%s*(.*)":match"(.-)%s*$"
end

function zellij.healthCheck()
    return zellij.version():match("zellij %d+.%d+.%d+") ~= nil
end

function zellij.edgeDetect(direction)
    local currWin = vim.api.nvim_get_current_win()
    vim.api.nvim_command("wincmd " .. direction)
    local newWin = vim.api.nvim_get_current_win()

    -- You're at the edge when you just moved direction and the window number is the same
    return currWin == newWin
end

function zellij.mergeArgs(args)
    return table.concat(args, " ")
end

function zellij.zjNavigate(direction)
    local zellijDirection = directionTranslation[direction]
    zellij.log("Navigate " .. direction .. " aka " .. zellijDirection)
    if zellij.edgeDetect(direction) then
        zellij.ZellijCommand(zellij.mergeArgs({ ACTION, MOVE_FOCUS_OR_TAB, zellijDirection }), false)
    end
end

function zellij.renamePane(newPaneName)
    zellij.ZellijCommand(zellij.mergeArgs({ ACTION, RENAME_PANE, newPaneName }), false)
end

function zellij.renameTab(newTabName)
    zellij.ZellijCommand(zellij.mergeArgs({ ACTION, RENAME_TAB, newTabName }), false)
end

function zellij.newPane()
    zellij.ZellijCommand(zellij.mergeArgs({ ACTION, NEW_PANE }), false)
end

function zellij.newTab()
    zellij.ZellijCommand(zellij.mergeArgs({ ACTION, NEW_TAB }), false)
end

function zellij.ZellijCommand(args, shouldReadOutput)
    local cmd = zellij.opts.path .. " " .. args .. " 2>&1"
    zellij.log("ZellijCommand arg: " .. cmd)
    local o = io.popen(cmd)
    if o ~= nil then
        if shouldReadOutput or zellij.opts.debug then
            local result = o:read("*a")
            zellij.log("ZellijCommand output: " .. result)
            o:close()
            return result
        else
            o:close()
            return
        end
    end
    error("Unable to run zellij command")
end

function zellij.setup(opts)
    opts = opts or {}
    if opts.path == nil then
        opts.path = "zellij"
    end
    if opts.debug == nil then
        opts.debug = false
    end
    if opts.whichKeyEnabled == nil then
        opts.whichKeyEnabled = false
    end
    if opts.debug == true then
        print("Zellij plugin debug mode")
        local date = os.time(os.date("!*t"))
        opts.logPath = '/tmp/zellij.nvim/log-' .. date .. '.txt'
    end
    if opts.vimTmuxNavigatorKeybinds == true then
        keybindings.setupVimTmuxNavigatorBindings(opts.whichKeyEnabled)
    end
    if opts.replaceVimWindowNavigationKeybinds == true then
        keybindings.setupVimWindowBindings(opts.whichKeyEnabled)
    end
    zellij.opts = opts
end

vim.api.nvim_create_user_command('ZellijNavigateLeft', function() zellij.zjNavigate('h') end,
    { desc = "Navigate left vim window or zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNavigateRight', function() zellij.zjNavigate('l') end,
    { desc = "Navigate right vim window or zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNavigateUp', function() zellij.zjNavigate('k') end,
    { desc = "Navigate up vim window or zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNavigateDown', function() zellij.zjNavigate('j') end,
    { desc = "Navigate down vim window or zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNewPane', zellij.newPane, { desc = "New Zellij pane", nargs = 0 })
vim.api.nvim_create_user_command('ZellijNewTab', zellij.newTab, { desc = "New Zellij tab", nargs = 0 })
vim.api.nvim_create_user_command('ZellijRenamePane', function(opts) zellij.renamePane(opts.args) end,
    { desc = "Rename the current Zellij pane", nargs = 1 })
vim.api.nvim_create_user_command('ZellijRenameTab', function(opts) zellij.renameTab(opts.args) end,
    { desc = "Rename the current Zellij tab", nargs = 1 })
vim.api.nvim_create_user_command('ZellijOpenDebugLog', zellij.openDebugLog,
    { desc = "Rename the current Zellij pane", nargs = 0 })

return zellij
