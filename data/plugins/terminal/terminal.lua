-- Terminal class - manages a single shell process and its I/O

local core = require "core"
local common = require "core.common"
local process = require "core.process"
local ansi = require "plugins.terminal.ansi"

local Terminal = {}
Terminal.__index = Terminal

-- Detect the default shell
local function detect_shell()
  local shell = os.getenv("SHELL")
  if shell then
    return shell
  end

  if PLATFORM == "Windows" then
    return "cmd.exe"
  else
    return "/bin/bash"
  end
end

-- Create a new terminal instance
function Terminal.new(shell, cwd, env)
  local self = setmetatable({}, Terminal)

  self.shell = shell or detect_shell()
  self.cwd = cwd or system.absolute_path(".")
  self.env = env or {}

  self.process = nil
  self.output_buffer = {}
  self.max_lines = 1000  -- Maximum scrollback
  self.active = false
  self.exit_code = nil
  self.title = "Terminal"

  return self
end

-- Start the shell process
function Terminal:start()
  if self.active then
    return false, "Terminal already running"
  end

  -- Prepare shell command with args for interactive mode
  local cmd
  if self.shell:match("bash") or self.shell:match("zsh") then
    cmd = {self.shell, "-i"}  -- Interactive shell
  else
    cmd = {self.shell}
  end

  -- Start the process
  local ok, err = pcall(function()
    self.process = process.start(cmd, {
      cwd = self.cwd,
      stdin = process.REDIRECT_PIPE,
      stdout = process.REDIRECT_PIPE,
      stderr = process.REDIRECT_STDOUT,  -- Merge stderr to stdout
      env = self.env
    })
  end)

  if not ok then
    return false, "Failed to start shell: " .. tostring(err)
  end

  self.active = true
  self.exit_code = nil

  -- Start reading thread
  self:start_reader()

  return true
end

-- Start a thread to read output from the process
function Terminal:start_reader()
  core.add_thread(function()
    while self.active and self.process:running() do
      -- Read data from stdout
      local data = self.process.stdout:read(4096)

      if data and #data > 0 then
        self:append_output(data)
      end

      -- Yield to avoid blocking the UI
      coroutine.yield(0.01)
    end

    -- Process exited
    self.active = false
    self.exit_code = self.process:returncode()

    -- Add exit message
    if self.exit_code and self.exit_code ~= 0 then
      self:append_output(string.format("\n[Process exited with code %d]\n", self.exit_code))
    else
      self:append_output("\n[Process exited]\n")
    end
  end)
end

-- Append output to the buffer
function Terminal:append_output(text)
  if not text or #text == 0 then return end

  -- Split into lines, but preserve the last incomplete line
  local lines = {}
  local current = ""

  for i = 1, #text do
    local char = text:sub(i, i)
    current = current .. char

    if char == "\n" then
      table.insert(lines, current)
      current = ""
    end
  end

  -- Handle the last incomplete line
  if #current > 0 then
    table.insert(lines, current)
  end

  -- Add lines to buffer
  for _, line in ipairs(lines) do
    table.insert(self.output_buffer, line)
  end

  -- Limit buffer size
  while #self.output_buffer > self.max_lines do
    table.remove(self.output_buffer, 1)
  end
end

-- Send input to the shell
function Terminal:send_input(text)
  if not self.active or not self.process then
    return false
  end

  -- Write to stdin
  core.add_thread(function()
    self.process.stdin:write(text)
  end)

  return true
end

-- Send a special key sequence
function Terminal:send_key(key)
  local sequences = {
    ["ctrl+c"] = "\x03",  -- ETX (interrupt)
    ["ctrl+d"] = "\x04",  -- EOT (end of transmission)
    ["ctrl+z"] = "\x1a",  -- SUB (suspend)
    ["up"] = "\x1b[A",
    ["down"] = "\x1b[B",
    ["right"] = "\x1b[C",
    ["left"] = "\x1b[D",
    ["home"] = "\x1b[H",
    ["end"] = "\x1b[F",
    ["pageup"] = "\x1b[5~",
    ["pagedown"] = "\x1b[6~",
    ["delete"] = "\x1b[3~",
    ["backspace"] = "\x7f",
    ["tab"] = "\t",
    ["return"] = "\n",
  }

  local seq = sequences[key]
  if seq then
    self:send_input(seq)
    return true
  end

  return false
end

-- Kill the process
function Terminal:kill()
  if self.process and self.active then
    self.process:terminate()
    self.active = false
  end
end

-- Restart the terminal
function Terminal:restart()
  self:kill()

  -- Wait a bit for cleanup
  core.add_thread(function()
    coroutine.yield(0.1)
    self.output_buffer = {}
    self:start()
  end)
end

-- Clear the output buffer
function Terminal:clear()
  self.output_buffer = {}
end

-- Check if the terminal is running
function Terminal:is_running()
  return self.active and self.process and self.process:running()
end

-- Get parsed output lines (with ANSI parsing)
function Terminal:get_parsed_lines()
  local parsed_lines = {}

  for _, line in ipairs(self.output_buffer) do
    table.insert(parsed_lines, ansi.parse(line))
  end

  return parsed_lines
end

return Terminal
