local wezterm = require 'wezterm'

wezterm.on('gui-startup', function(cmd)
  local tab, pane, window = wezterm.mux.spawn_window(cmd or {})
  
  -- Créer le split avec PowerShell 7
  pane:split {
    direction = 'Right',
    args = { 'C:\\Program Files\\PowerShell\\7\\pwsh.exe' },
  }
end)

return {
  -- CMD avec injection automatique de Clink
  default_prog = { 'cmd.exe', '/k', 'clink inject' },
  
  initial_cols = 160,
  initial_rows = 40,
  
  keys = {
    {
      key = 'LeftArrow',
      mods = 'CTRL|SHIFT',
      action = wezterm.action.ActivatePaneDirection 'Left',
    },
    {
      key = 'RightArrow',
      mods = 'CTRL|SHIFT',
      action = wezterm.action.ActivatePaneDirection 'Right',
    },
  },
}