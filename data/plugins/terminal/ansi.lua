-- ANSI escape sequence parser for terminal output
-- Handles color codes and basic formatting

local style = require "core.style"

local ansi = {}

-- ANSI color mappings (16 basic colors)
ansi.colors = {
  -- Normal colors (30-37)
  [30] = {0, 0, 0},           -- black
  [31] = {205, 49, 49},       -- red
  [32] = {13, 188, 121},      -- green
  [33] = {229, 229, 16},      -- yellow
  [34] = {36, 114, 200},      -- blue
  [35] = {188, 63, 188},      -- magenta
  [36] = {17, 168, 205},      -- cyan
  [37] = {229, 229, 229},     -- white

  -- Bright colors (90-97)
  [90] = {102, 102, 102},     -- bright black (gray)
  [91] = {241, 76, 76},       -- bright red
  [92] = {35, 209, 139},      -- bright green
  [93] = {245, 245, 67},      -- bright yellow
  [94] = {59, 142, 234},      -- bright blue
  [95] = {214, 112, 214},     -- bright magenta
  [96] = {41, 184, 219},      -- bright cyan
  [97] = {255, 255, 255},     -- bright white
}

-- Background colors (add 10 to foreground code)
ansi.bg_colors = {}
for k, v in pairs(ansi.colors) do
  ansi.bg_colors[k + 10] = v
end

-- Parse ANSI escape sequences and return styled segments
-- Returns: array of {text, color, bg_color, bold, italic, underline}
function ansi.parse(text)
  local segments = {}
  local current_color = nil
  local current_bg = nil
  local bold = false
  local italic = false
  local underline = false

  -- Start position in the string
  local pos = 1
  local buffer = ""

  while pos <= #text do
    -- Look for escape sequence: ESC[...m
    local esc_start, esc_end, codes = text:find("\27%[([%d;]*)m", pos)

    if esc_start then
      -- Add any text before the escape sequence
      if esc_start > pos then
        buffer = buffer .. text:sub(pos, esc_start - 1)
      end

      -- Parse the escape codes
      if codes and codes ~= "" then
        for code_str in (codes .. ";"):gmatch("([^;]*);") do
          local code = tonumber(code_str)
          if code then
            if code == 0 then
              -- Reset all attributes
              if #buffer > 0 then
                table.insert(segments, {
                  text = buffer,
                  color = current_color,
                  bg_color = current_bg,
                  bold = bold,
                  italic = italic,
                  underline = underline
                })
                buffer = ""
              end
              current_color = nil
              current_bg = nil
              bold = false
              italic = false
              underline = false
            elseif code == 1 then
              bold = true
            elseif code == 3 then
              italic = true
            elseif code == 4 then
              underline = true
            elseif code == 22 then
              bold = false
            elseif code == 23 then
              italic = false
            elseif code == 24 then
              underline = false
            elseif (code >= 30 and code <= 37) or (code >= 90 and code <= 97) then
              -- Foreground color
              current_color = ansi.colors[code]
            elseif (code >= 40 and code <= 47) or (code >= 100 and code <= 107) then
              -- Background color
              current_bg = ansi.bg_colors[code]
            elseif code == 39 then
              -- Default foreground
              current_color = nil
            elseif code == 49 then
              -- Default background
              current_bg = nil
            end
          end
        end
      else
        -- Empty code means reset
        if #buffer > 0 then
          table.insert(segments, {
            text = buffer,
            color = current_color,
            bg_color = current_bg,
            bold = bold,
            italic = italic,
            underline = underline
          })
          buffer = ""
        end
        current_color = nil
        current_bg = nil
        bold = false
        italic = false
        underline = false
      end

      pos = esc_end + 1
    else
      -- No more escape sequences, add remaining text
      buffer = buffer .. text:sub(pos)
      break
    end
  end

  -- Add any remaining buffered text
  if #buffer > 0 then
    table.insert(segments, {
      text = buffer,
      color = current_color,
      bg_color = current_bg,
      bold = bold,
      italic = italic,
      underline = underline
    })
  end

  -- If no segments were created, return the original text
  if #segments == 0 then
    table.insert(segments, {
      text = text,
      color = nil,
      bg_color = nil,
      bold = false,
      italic = false,
      underline = false
    })
  end

  return segments
end

-- Strip all ANSI codes from text (for length calculations)
function ansi.strip(text)
  local result = text:gsub("\27%[[%d;]*m", "")
  -- Also strip other common escape sequences
  result = result:gsub("\27%[[%d;]*[A-Za-z]", "")
  return result
end

return ansi
