local M = {}

--- Get path on filesystem to executable in users PATH
--- @return string?
--- @param name string
local function which(name)
	local path = vim.fn.system 'which ' .. name
	if vim.v.shell_error == 0 then
		return path
	else
		return nil
	end
end

---@diagnostic disable-next-line: undefined-global
local nix_store_bin_path = nix_store_bin_path or nil

local function health()
	local checks = {}

	if not nix_store_bin_path then
		table.insert(checks, {
			ok = false,
			message = 'plugin was built without nix, expecting noogle to be available in PATH',
		})

		local path = which 'noogle'
		if not path then
			table.insert(checks, {
				ok = false,
				message = 'noogle not found in path',
			})
		else
			table.insert(checks, {
				ok = true,
				message = 'noogle found at: ' .. path,
			})
		end
	else
		table.insert(checks, {
			ok = true,
			message = 'noogle found at: ' .. nix_store_bin_path,
		})
	end

	return checks
end

-- check if binary path has been patched in as part of the build process
-- if not we just fall back on whatever can be found in our PATH
local bin_path = nix_store_bin_path or which 'noogle'

function M.setup()
	return {
		command = 'Noogle',
		get_items = function()
			return vim.fn.systemlist(bin_path .. ' list')
		end,
		get_content = function(choice)
			return vim.fn.systemlist(bin_path .. ' doc ' .. choice)
		end,
		get_syntax_info = function()
			return {
				filetype = 'markdown', -- filetype for buffer that is opened
				language = 'markdown', -- tree-sitter parser
			}
		end,
		health = health,
	}
end

return M
