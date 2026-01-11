use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub base: Color,
    pub surface0: Color,
    pub surface1: Color,
    pub surface2: Color,
    pub text: Color,
    pub subtext0: Color,
    pub subtext1: Color,
    pub overlay0: Color,
    pub overlay1: Color,
    pub overlay2: Color,
    pub blue: Color,
    pub lavender: Color,
    pub sapphire: Color,
    pub sky: Color,
    pub teal: Color,
    pub green: Color,
    pub yellow: Color,
    pub peach: Color,
    pub maroon: Color,
    pub red: Color,
    pub mauve: Color,
    pub pink: Color,
    pub flamingo: Color,
    pub rosewater: Color,
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Theme {
            name: "catppuccin-mocha".to_string(),
            base: Color::Rgb(30, 30, 46),
            surface0: Color::Rgb(49, 50, 68),
            surface1: Color::Rgb(69, 71, 90),
            surface2: Color::Rgb(88, 91, 112),
            text: Color::Rgb(205, 214, 244),
            subtext0: Color::Rgb(166, 173, 200),
            subtext1: Color::Rgb(186, 194, 222),
            overlay0: Color::Rgb(108, 112, 134),
            overlay1: Color::Rgb(127, 132, 156),
            overlay2: Color::Rgb(147, 153, 178),
            blue: Color::Rgb(137, 180, 250),
            lavender: Color::Rgb(180, 190, 254),
            sapphire: Color::Rgb(116, 199, 236),
            sky: Color::Rgb(137, 220, 235),
            teal: Color::Rgb(148, 226, 213),
            green: Color::Rgb(166, 227, 161),
            yellow: Color::Rgb(249, 226, 175),
            peach: Color::Rgb(250, 179, 135),
            maroon: Color::Rgb(238, 153, 160),
            red: Color::Rgb(243, 139, 168),
            mauve: Color::Rgb(203, 166, 247),
            pink: Color::Rgb(245, 194, 231),
            flamingo: Color::Rgb(242, 205, 205),
            rosewater: Color::Rgb(245, 224, 220),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}
