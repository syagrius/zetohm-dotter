local wezterm = require 'wezterm'

wezterm.on('gui-startup', function(cmd)
  local tab, pane, window = wezterm.mux.spawn_window(cmd or {})

  -- Créer le split avec zsh ou bash
  pane:split {
    direction = 'Right',
    args = { '/bin/zsh' },  -- ou '/bin/bash' selon votre préférence
  }
end)

return {
  -- Shell par défaut pour Linux
  default_prog = { '/bin/zsh' },  -- ou { '/bin/bash' }

  initial_cols = 160,
  initial_rows = 40,

  -- Configuration de la police (identique sur Linux)
  font = wezterm.font('JetBrains Mono'),
  font_size = 10.0,

  -- Thème (identique entre OS)
  color_scheme = 'Dracula',

  -- Raccourcis clavier (identiques)
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
    -- Split horizontal
    {
      key = 'd',
      mods = 'CTRL|SHIFT',
      action = wezterm.action.SplitHorizontal { domain = 'CurrentPaneDomain' },
    },
    -- Split vertical
    {
      key = 's',
      mods = 'CTRL|SHIFT',
      action = wezterm.action.SplitVertical { domain = 'CurrentPaneDomain' },
    },
  },
}