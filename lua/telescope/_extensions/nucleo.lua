local lib = require('nucleo.lib')
local sorters = require('telescope.sorters')

local function create_sorter(matcher)
  return sorters.Sorter:new({
    start = function(_, prompt)
      matcher:set_pattern(vim.trim(prompt))
    end,
    discard = true,
    scoring_function = function(_, _, line)
      local score = matcher:match(line)
      if score == 0 then
        return -1
      end
      return 1 / score
    end,
    highlighter = function(_, _, display)
      local _, hlcols = matcher:match(display)
      return hlcols
    end,
  })
end

local defaults = {
  case_mode = 'smart',
  normalize_mode = 'smart',
}

return require('telescope').register_extension({
  setup = function(user_configuration, configuration)
    local merged = vim.tbl_deep_extend('force', defaults, user_configuration)
    local matcher = lib.create_matcher(merged)

    configuration.file_sorter = function()
      return create_sorter(matcher)
    end
    configuration.generic_sorter = function()
      return create_sorter(matcher)
    end
  end,
})
