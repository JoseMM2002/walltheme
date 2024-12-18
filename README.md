# Wallthemes

Wallthemes is a terminal theme generator implemented in Rust. It extracts colors from an image and applies them to predefined templates to generate themes for terminal emulators, text editors, and other applications. The tool supports all standard colors (black, red, white, etc.), and includes an extra orange color specifically for **Zellij**.

Wallthemes comes with several templates and example scripts to facilitate customization.

---

## Features

- **Extract Colors from an Image**: Automatically extracts primary colors like black, red, white, and others from a provided image.
- **Zellij Support**: Includes an additional orange color for Zellij.
- **Template-based Generation**: Uses customizable templates to generate configuration files for terminals, text editors, and more.
- **Example Scripts**: Predefined scripts and templates for quick integration.
- **Rust Implementation**: High performance and reliability.

---

## Installation

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

### Build from Source

```bash
# Clone the repository
mkdir -p ~/.cache/wallthemes/ && mkdir -p ~/.config/wallthemes/templates
git clone https://github.com/JoseMM2002/walltheme.nvim.git
cd wallthemes
# Build the project
cargo install --path .
```

---

## Usage

### Basic Command

To generate a theme:

```bash
wallthemes <image-path>
```

### Example

```bash
wallthemes ~/Pictures/wallpaper.png
```

This command extracts colors from `wallpaper.png`, generates config files using templates from `~/.config/wallthemes/templates` and saves them into `~/.cache/wallthemes/`.

---

## Templates

Wallthemes uses templates to format the generated theme files. Placeholders are replaced with extracted colors. This templates are generated by handlebars.

### Example Template

```plaintext
#{{black_hex}}
#{{red_hex}}
#{{green_hex}}
#{{blue_hex}}
rgb{{black_rgb}}
rgb{{red_rgb}}
rgb{{green_rgb}}
rgb{{blue_rgb}}
```

if the template also needs braces for some configs as oh-my-posh, you can use the following syntax:

```plaintext
{{keep "info to keep betwwen braces"}}
```

### Provided Templates

Wallthemes includes templates for:

- Kitty Terminal
- Waybar
- Oh My Posh
- Zellij
- Nushell
- wofi

Templates are located in the `templates/` directory. To use them, copy the desired template to `~/.config/wallthemes/templates/` and modify it as needed.

---

## Configuration Options

You can customize Wallthemes by creating or modifying a `config` file located at `~/.config/wallthemes/config`. This file allows you to fine-tune the behavior of the theme generation, including palette settings and thresholds.

### Example Configuration

```toml
[ConfigOpts]
mix_factor = 0.7
distance_threshold = 0.5
palette_quality = 16
palette_max_colors = 8
brighter_factor = 0.8
bright_min = 20
```

### Available Options

- `mix_factor`: Mix factor for colors not found in the palette.
- `distance_threshold`: Threshold for color distance filtering.
- `palette_quality`: Quality of the palette extraction.
- `palette_max_colors`: A number of colors in the output palette. Actual colors count can be lower depending on the image.
- `brighter_factor`: Factor to adjust bright colors.
- `bright_min`: Minimum brightness to consider a color bright.

You can set these options to suit your preferences and workflow.

---

### Examples

An example for my setup is in the `scripts/examples/` directory. It includes a script that sets the wallpaper and generates themes for Kitty, Waybar, and Zellij.

### NeoVim Integration

To integrate this with NeoVim, I had made a plugin that uses the generated theme to set the colorscheme. You can find it [walltheme.nvim](https://github.com/JoseMM2002/walltheme.nvim)

---

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/your-feature`.
3. Make changes and commit: `git commit -m "Add your feature"`.
4. Push your branch: `git push origin feature/your-feature`.
5. Submit a pull request.

---

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

---

## Acknowledgements

Special thanks to tools like [Pywal](https://github.com/dylanaraps/pywal) and inspiration from terminal theming communities.

---

## Screenshots

![Screenshot1](pictures/Screenshot1.jpeg)

![Screenshot2](pictures/Screenshot2.jpeg)

![Screenshot3](pictures/Screenshot3.jpeg)

![Screenshot4](pictures/Screenshot4.jpeg)