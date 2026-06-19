-- stdlib/charset.lua
-- Character class builders and definitions

local function make_charset(str)
  local set = {}
  for _, cp in utf8.codes(str) do
    set[utf8.char(cp)] = true
  end
  return set
end

local function merge_sets(...)
  local result = {}
  for _, set in ipairs({...}) do
    for k, v in pairs(set) do
      result[k] = v
    end
  end
  return result
end

-- Pre‑built character classes
local whitespace   = make_charset(' \r\t\n')
local word_chars   = make_charset("abcdefghijklmnopqrstuvwxyz")
local WORD_chars   = make_charset("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
local numbers      = make_charset("0123456789")
local underscore   = make_charset('_')

local identifier   = merge_sets(word_chars, WORD_chars, numbers, underscore)
local non_word     = merge_sets(identifier, whitespace)

return {
  make_charset = make_charset,
  merge_sets   = merge_sets,
  whitespace   = whitespace,
  identifier   = identifier,
  non_word     = non_word,
}