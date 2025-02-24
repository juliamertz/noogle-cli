local M = {}

---@diagnostic disable-next-line: undefined-global
local nix_store_bin_path = nix_store_bin_path or nil

--
-- Utility functions

function string:split(delimiter)
	local result = {}
	local from = 1
	local delim_from, delim_to = string.find(self, delimiter, from)
	while delim_from do
		table.insert(result, string.sub(self, from, delim_from - 1))
		from = delim_to + 1
		delim_from, delim_to = string.find(self, delimiter, from)
	end
	table.insert(result, string.sub(self, from))
	return result
end

--- TODO: can we avoid having to check for both?
---
--- Check for decoded JSON nil value
--- @param val any
--- @return boolean
local function is_nil(val)
	return val == nil or val == vim.NIL
end

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
				message = 'noogli not found in path',
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

--- Extract markdown documentation string from lambda table
--- returns nil if the function doesn't have any doc-comments
--- @param func table
--- @return string?
local function get_function_documentation(func)
	if is_nil(func.content) or is_nil(func.content.content) then
		return nil
	end

	local content = func.content.content
	if type(content) == 'string' then
		return content
	end

	return nil
end

function M.setup()
	return {
		command = 'Noogle',
		get_items = function()
			return vim.fn.systemlist(bin_path .. ' list')
		end,
		get_content = function(choice)
			local raw = vim.fn.systemlist(bin_path .. ' show --json ' .. choice)
			local json = vim.json.decode(table.concat(raw, '\n'))

			local documentation = get_function_documentation(json)
			if documentation == nil then
				return { 'No documentation for this lambda' }
			end

			return documentation:split '\n'
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
