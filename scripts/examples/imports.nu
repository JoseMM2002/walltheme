const cache_dir = $nu.home-path | path join ".cache"
const cache = {
    oh-my-posh: ($cache_dir | path join "oh-my-posh"),
    pywall: ($cache_dir | path join "wal"),
    zellij: ($cache_dir | path join "zellij"),
    walltheme: ($cache_dir | path join "walltheme"),
}
const config_dir = $nu.home-path | path join ".config"
const config = {
    nvim: ($config_dir | path join "nvim"),
    zellij: ($config_dir | path join "zellij"),
    nu: ($config_dir | path join "nushell"),
    waybar: ($config_dir | path join "waybar"),
    hyprland: ($config_dir | path join "hypr"),
    kitty: ($config_dir | path join "kitty"),
    yazi: ($config_dir | path join "yazi"),
}

