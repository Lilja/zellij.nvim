local zellij = require('zellij')
local M = {}

M.check = function()
  vim.health.report_start("zellij.nvim report")
  -- make sure setup function parameters are ok
  vim.health.report_info("Using binary '" .. zellij.opts.path .. "'")
  if zellij.healthCheck() then
    vim.health.report_ok("Zellij binary found and version matches expected version number")
    vim.health.report_ok(zellij.version())
  else
    vim.health.report_error("Binary not found")
  end
end

return M
