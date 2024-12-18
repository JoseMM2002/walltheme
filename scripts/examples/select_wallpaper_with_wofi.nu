source $"($nu.default-config-dir)/modules/imports.nu"

let image_dir = $nu.home-path | path join "Pictures" "Wallpapers"
let allowed_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp"]

let selected_image = ls $"($image_dir)" | 
get name | 
each {
    | name |
    let splitted = $name | split row "/"
    let last = $splitted | last
    return $last
} |
str join "\n" | 
wofi --show=dmenu 

if $selected_image == "" {
    echo "No image selected"
    exit
}

let wallpaper_path = $image_dir | path join $selected_image

swww img $wallpaper_path
walltheme $wallpaper_path

sh "$($cache.walltheme)/colors.sh" 

cp $"($cache.walltheme)/colors-hypr.conf" $"($config.hyprland)/themes/walltheme.conf"
cp $"($cache.walltheme)/colors-zellij.kdl" $"($config.zellij)/themes/walltheme.kdl"
cp $"($cache.walltheme)/colors-oh-my-posh.json" $"($cache.oh-my-posh)/themes/walltheme.omp.json"

try { 
    sh restart_waybar.sh 
}
try {
    pkill -USR1 -f /usr/bin/kitty
}
