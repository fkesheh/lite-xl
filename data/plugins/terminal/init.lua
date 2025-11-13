-- mod-version:4
-- Terminal Plugin - Integrated shell terminal for lite-xl

local core = require "core"
local common = require "core.common"
local command = require "core.command"
local config = require "core.config"
local keymap = require "core.keymap"
local style = require "core.style"
local View = require "core.view"
local Terminal = require "plugins.terminal.terminal"
local ansi = require "plugins.terminal.ansi"

-- Configuration
config.plugins.terminal = common.merge({
  size = 250 * SCALE,           -- Default height in pixels
  visible = false,               -- Start hidden
  max_scrollback = 1000,         -- Maximum lines to keep
  tab_width = 150 * SCALE,       -- Width of terminal tabs
}, config.plugins.terminal)

-- TerminalView class
local TerminalView = View:extend()

function TerminalView:__tostring() return "TerminalView" end

function TerminalView:new()
  TerminalView.super.new(self)
  self.scrollable = true
  self.visible = config.plugins.terminal.visible
  self.target_size = config.plugins.terminal.size
  self.terminals = {}
  self.active_terminal = 0
  self.input_focused = false

  -- Create first terminal
  self:add_terminal()
end

-- Set target size (for resizing)
function TerminalView:set_target_size(axis, value)
  if axis == "y" then
    self.target_size = value
    config.plugins.terminal.size = value
    return true
  end
end

-- Get the name for this view
function TerminalView:get_name()
  return "Terminal"
end

-- Add a new terminal
function TerminalView:add_terminal()
  local term = Terminal.new()
  term:start()
  table.insert(self.terminals, term)
  self.active_terminal = #self.terminals
  return term
end

-- Close a terminal
function TerminalView:close_terminal(index)
  if #self.terminals == 0 then return end

  index = index or self.active_terminal

  if self.terminals[index] then
    self.terminals[index]:kill()
    table.remove(self.terminals, index)
  end

  -- Adjust active terminal
  if self.active_terminal > #self.terminals then
    self.active_terminal = #self.terminals
  end

  -- Create a new terminal if none exist
  if #self.terminals == 0 then
    self:add_terminal()
  end
end

-- Switch to next terminal
function TerminalView:next_terminal()
  if #self.terminals <= 1 then return end
  self.active_terminal = (self.active_terminal % #self.terminals) + 1
end

-- Switch to previous terminal
function TerminalView:prev_terminal()
  if #self.terminals <= 1 then return end
  self.active_terminal = self.active_terminal - 1
  if self.active_terminal < 1 then
    self.active_terminal = #self.terminals
  end
end

-- Get the currently active terminal
function TerminalView:get_active_terminal()
  return self.terminals[self.active_terminal]
end

-- Calculate scrollable size
function TerminalView:get_scrollable_size()
  local term = self:get_active_terminal()
  if not term then return 0 end

  local line_height = style.code_font:get_height()
  local tab_height = line_height + style.padding.y * 2
  return #term.output_buffer * line_height + tab_height
end

-- Handle mouse press
function TerminalView:on_mouse_pressed(button, px, py, clicks)
  if TerminalView.super.on_mouse_pressed(self, button, px, py, clicks) then
    return true
  end

  local x, y = self:get_content_offset()
  local line_height = style.code_font:get_height()
  local tab_height = line_height + style.padding.y * 2

  -- Check if click is in tab bar
  if py < y + tab_height then
    local tx = x + style.padding.x
    for i, term in ipairs(self.terminals) do
      local tab_width = config.plugins.terminal.tab_width
      if px >= tx and px < tx + tab_width then
        self.active_terminal = i
        return true
      end
      tx = tx + tab_width + style.padding.x
    end

    -- Check for "+" button to add new terminal
    local add_btn_x = tx
    local add_btn_w = style.code_font:get_width("+") + style.padding.x * 2
    if px >= add_btn_x and px < add_btn_x + add_btn_w then
      self:add_terminal()
      return true
    end
  end

  self.input_focused = true
  return true
end

-- Handle mouse wheel for scrolling
function TerminalView:on_mouse_wheel(dy, dx)
  if self.scrollable then
    self.scroll.to.y = self.scroll.to.y + dy * -30
    return true
  end
end

-- Handle text input
function TerminalView:on_text_input(text)
  local term = self:get_active_terminal()
  if term and self.input_focused then
    term:send_input(text)
    return true
  end
  return false
end

-- Handle key press
function TerminalView:on_key_pressed(key, scancode)
  local term = self:get_active_terminal()
  if not term then return false end

  -- Check if this is a special key that should be sent to terminal
  if self.input_focused then
    if term:send_key(key) then
      return true
    end

    -- Handle regular keys that need special treatment
    if key == "return" then
      term:send_input("\n")
      return true
    elseif key == "backspace" then
      term:send_input("\x7f")
      return true
    elseif key == "tab" then
      term:send_input("\t")
      return true
    end
  end

  return false
end

-- Update function
function TerminalView:update()
  -- Update target size animation
  if self.visible then
    self:move_towards("size", "y", self.target_size, nil, "terminal")
  else
    self:move_towards("size", "y", 0, nil, "terminal")
  end

  TerminalView.super.update(self)
end

-- Draw the terminal view
function TerminalView:draw()
  if not self.visible or self.size.y <= 0 then return end

  self:draw_background(style.background3)

  local x, y = self:get_content_offset()
  local w, h = self.size.x, self.size.y
  local line_height = style.code_font:get_height()
  local tab_height = line_height + style.padding.y * 2

  -- Draw tab bar
  local tx = x + style.padding.x
  for i, term in ipairs(self.terminals) do
    local tab_width = config.plugins.terminal.tab_width
    local tab_color = i == self.active_terminal and style.background or style.background2
    local text_color = i == self.active_terminal and style.text or style.dim

    -- Draw tab background
    renderer.draw_rect(tx, y, tab_width, tab_height, tab_color)

    -- Draw tab text
    local title = string.format("Terminal %d", i)
    if not term:is_running() then
      title = title .. " (exited)"
    end
    common.draw_text(
      style.code_font,
      text_color,
      title,
      "left",
      tx + style.padding.x,
      y + style.padding.y,
      tab_width - style.padding.x * 2,
      line_height
    )

    -- Draw close button
    local close_x = tx + tab_width - style.padding.x - style.code_font:get_width("×")
    common.draw_text(
      style.code_font,
      text_color,
      "×",
      "left",
      close_x,
      y + style.padding.y,
      style.code_font:get_width("×"),
      line_height
    )

    tx = tx + tab_width + style.padding.x
  end

  -- Draw "+" button to add new terminal
  local add_btn_x = tx
  local add_btn_w = style.code_font:get_width("+") + style.padding.x * 2
  renderer.draw_rect(add_btn_x, y, add_btn_w, tab_height, style.background2)
  common.draw_text(
    style.code_font,
    style.text,
    "+",
    "center",
    add_btn_x,
    y + style.padding.y,
    add_btn_w,
    line_height
  )

  -- Draw separator line below tabs
  y = y + tab_height
  renderer.draw_rect(x, y, w, 1, style.divider)
  y = y + 1

  -- Draw terminal output
  local term = self:get_active_terminal()
  if term then
    local output_y = y - self.scroll.y
    local visible_start = math.max(1, math.floor(self.scroll.y / line_height))
    local visible_end = math.min(#term.output_buffer, visible_start + math.ceil(h / line_height) + 1)

    -- Set clip rect for terminal output
    core.push_clip_rect(x, y, w, h - tab_height - 1)

    for i = visible_start, visible_end do
      local line = term.output_buffer[i]
      if line then
        local segments = ansi.parse(line)
        local line_x = x + style.padding.x

        for _, segment in ipairs(segments) do
          local color = segment.color
          if color then
            color = {color[1], color[2], color[3], 255}
          else
            color = style.text
          end

          -- Apply bold by making color brighter (simple approach)
          if segment.bold and color then
            color = {
              math.min(255, color[1] + 30),
              math.min(255, color[2] + 30),
              math.min(255, color[3] + 30),
              255
            }
          end

          -- Draw background if specified
          if segment.bg_color then
            local text_width = style.code_font:get_width(segment.text)
            renderer.draw_rect(
              line_x,
              output_y,
              text_width,
              line_height,
              {segment.bg_color[1], segment.bg_color[2], segment.bg_color[3], 255}
            )
          end

          -- Draw text
          line_x = renderer.draw_text(
            style.code_font,
            segment.text,
            line_x,
            output_y,
            color
          )
        end
      end

      output_y = output_y + line_height
    end

    core.pop_clip_rect()

    -- Draw scrollbar
    self:draw_scrollbar()
  end
end

-- Toggle visibility
function TerminalView:toggle()
  self.visible = not self.visible
end

-- Show the terminal
function TerminalView:show()
  self.visible = true
  self.input_focused = true
end

-- Hide the terminal
function TerminalView:hide()
  self.visible = false
  self.input_focused = false
end

-- Create the terminal view instance
local view = TerminalView()

-- Register commands
command.add(nil, {
  ["terminal:toggle"] = function()
    view:toggle()
  end,

  ["terminal:show"] = function()
    view:show()
  end,

  ["terminal:hide"] = function()
    view:hide()
  end,

  ["terminal:new"] = function()
    view:add_terminal()
  end,

  ["terminal:next"] = function()
    view:next_terminal()
  end,

  ["terminal:previous"] = function()
    view:prev_terminal()
  end,

  ["terminal:close"] = function()
    view:close_terminal()
  end,

  ["terminal:restart"] = function()
    local term = view:get_active_terminal()
    if term then
      term:restart()
    end
  end,

  ["terminal:clear"] = function()
    local term = view:get_active_terminal()
    if term then
      term:clear()
    end
  end,
})

-- Add keybindings
keymap.add {
  ["ctrl+`"] = "terminal:toggle",
  ["ctrl+shift+`"] = "terminal:new",
  ["ctrl+pagedown"] = "terminal:next",
  ["ctrl+pageup"] = "terminal:previous",
}

-- Integrate into the UI
core.add_thread(function()
  -- Wait for core to be fully initialized
  while not core.root_view.root_node do
    coroutine.yield()
  end

  -- Split bottom for terminal
  local node = core.root_view:get_active_node_default()
  view.node = node:split("down", view, {y = true}, true)

  -- Start hidden
  view.size.y = 0
end)

return view
