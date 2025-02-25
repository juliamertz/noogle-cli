local M = {}

local function cmd_with_output(cmd)
	local output = vim.fn.system(cmd)
	if vim.v.shell_error == 0 then
		return output
	else
		return nil
	end
end

---@diagnostic disable-next-line: undefined-global
local bin_path = nix_store_bin_path or cmd_with_output 'which noogle'

local function health()
	local checks = {}

	if not bin_path then
		table.insert(checks, {
			ok = false,
			message = 'noogle not found in path',
		})
	else
		print(bin_path .. ' --version')
		local version = vim.trim(cmd_with_output(bin_path .. ' --version'))
		table.insert(checks, {
			ok = true,
			message = 'noogle version ' .. version .. ' found at: ' .. bin_path,
		})
	end

	return checks
end

local function get_function_data(function_name)
	local raw_json = vim.fn.system(bin_path .. ' show --json "' .. function_name .. '"')
	local ok, deserialized = pcall(vim.json.decode, raw_json, {})
	if not ok then
		return nil
	end
	return deserialized
end

---@param json table
---@return GoDocDefinition?
local function extract_definition(json)
	if not json.meta or not json.meta.lambda_position then
		return nil
	end

	local pos = json.meta.lambda_position
	return {
		filepath = pos.file,
		position = { pos.line, pos.column },
	}
end

function M.setup()
	return {
		command = 'Noogle',
		health = health,
		get_items = function()
			return vim.fn.systemlist(bin_path .. ' list')
		end,
		get_content = function(choice)
			return vim.fn.systemlist(bin_path .. ' doc ' .. choice)
		end,
		get_syntax_info = function()
			return { filetype = 'markdown', language = 'markdown' }
		end,
		get_definition = function(choice)
			local json = get_function_data(choice)
			if not json then
				vim.notify('No data found for ' .. choice)
				return
			end
			return extract_definition(json)
		end,
	}
end

return M
