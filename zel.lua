
function edgeDetect(direction)
	local currWin = vim.api.nvim_get_current_win()
	vim.api.nvim_command("wincmd " .. direction)
	local newWin = vim.api.nvim_get_current_win()

	-- You're at the edge when you just moved direction and the window number is the same
	print("ol winN ")
	print(currWin)
	print(" new ")
	print(newWin)
	print(" same? ")
	print(currWin == newWin)
	return currWin == newWin
end

function zjCall(direction)
	local directionTranslation = {
		h = "left",
		j = "down",
		k = "up",
		l = "right",
	}
	-- local cmd  = "zellij action move-focus-or-tab " .. directionTranslation[direction]
	local cmd  = "zellij action move-focus-or-tab " .. directionTranslation[direction]
	local cmd2 = "zellij --help"
	print("cmd")
	print(cmd)
	local c = vim.fn.system(cmd)
	print(c)
	local c2 = vim.fn.system("ls -l")
	print(c2)
end

function zjNavigate(direction)
	if edgeDetect(direction) then
		zjCall(direction)
	end
end

vim.keymap.set('n', ",,h", function() zjNavigate('h') end)
vim.keymap.set('n', ",,j", function() zjNavigate('j') end)
vim.keymap.set('n', ",,k", function() zjNavigate('k') end)
vim.keymap.set('n', ",,l", function() zjNavigate('l') end)
