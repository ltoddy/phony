use std::collections::HashMap;

use super::random::Random;

pub struct Provider {
    #[allow(dead_code)]
    safe_colors: HashMap<String, String>,

    #[allow(dead_code)]
    all_colors: HashMap<String, String>,

    selector: Random,
}

impl Provider {
    //    const ColorMapper = [];

    #[allow(dead_code)]
    pub fn new() -> Self {
        let safe_colors = hash_map! {
            "black".to_string() => "#000000".to_string(),
            "maroon".to_string() => "#800000".to_string(),
            "green".to_string() => "#008000".to_string(),
            "navy".to_string() => "#000080".to_string(),
            "olive".to_string() => "#808000".to_string(),
            "purple".to_string() => "#800080".to_string(),
            "teal".to_string() => "#008080".to_string(),
            "lime".to_string() => "#00FF00".to_string(),
            "blue".to_string() => "#0000FF".to_string(),
            "silver".to_string() => "#C0C0C0".to_string(),
            "gray".to_string() => "#808080".to_string(),
            "yellow".to_string() => "#FFFF00".to_string(),
            "fuchsia".to_string() => "#FF00FF".to_string(),
            "aqua".to_string() => "#00FFFF".to_string(),
            "white".to_string() => "#FFFFFF".to_string(),
        };

        let all_colors = hash_map! {
            "AliceBlue".to_string() => "#F0F8FF".to_string(),
            "AntiqueWhite".to_string() => "#FAEBD7".to_string(),
            "Aqua".to_string() => "#00FFFF".to_string(),
            "Aquamarine".to_string() => "#7FFFD4".to_string(),
            "Azure".to_string() => "#F0FFFF".to_string(),
            "Beige".to_string() => "#F5F5DC".to_string(),
            "Bisque".to_string() => "#FFE4C4".to_string(),
            "Black".to_string() => "#000000".to_string(),
            "BlanchedAlmond".to_string() => "#FFEBCD".to_string(),
            "Blue".to_string() => "#0000FF".to_string(),
            "BlueViolet".to_string() => "#8A2BE2".to_string(),
            "Brown".to_string() => "#A52A2A".to_string(),
            "BurlyWood".to_string() => "#DEB887".to_string(),
            "CadetBlue".to_string() => "#5F9EA0".to_string(),
            "Chartreuse".to_string() => "#7FFF00".to_string(),
            "Chocolate".to_string() => "#D2691E".to_string(),
            "Coral".to_string() => "#FF7F50".to_string(),
            "CornflowerBlue".to_string() => "#6495ED".to_string(),
            "Cornsilk".to_string() => "#FFF8DC".to_string(),
            "Crimson".to_string() => "#DC143C".to_string(),
            "Cyan".to_string() => "#00FFFF".to_string(),
            "DarkBlue".to_string() => "#00008B".to_string(),
            "DarkCyan".to_string() => "#008B8B".to_string(),
            "DarkGoldenRod".to_string() => "#B8860B".to_string(),
            "DarkGray".to_string() => "#A9A9A9".to_string(),
            "DarkGreen".to_string() => "#006400".to_string(),
            "DarkKhaki".to_string() => "#BDB76B".to_string(),
            "DarkMagenta".to_string() => "#8B008B".to_string(),
            "DarkOliveGreen".to_string() => "#556B2F".to_string(),
            "DarkOrange".to_string() => "#FF8C00".to_string(),
            "DarkOrchid".to_string() => "#9932CC".to_string(),
            "DarkRed".to_string() => "#8B0000".to_string(),
            "DarkSalmon".to_string() => "#E9967A".to_string(),
            "DarkSeaGreen".to_string() => "#8FBC8F".to_string(),
            "DarkSlateBlue".to_string() => "#483D8B".to_string(),
            "DarkSlateGray".to_string() => "#2F4F4F".to_string(),
            "DarkTurquoise".to_string() => "#00CED1".to_string(),
            "DarkViolet".to_string() => "#9400D3".to_string(),
            "DeepPink".to_string() => "#FF1493".to_string(),
            "DeepSkyBlue".to_string() => "#00BFFF".to_string(),
            "DimGray".to_string() => "#696969".to_string(),
            "DodgerBlue".to_string() => "#1E90FF".to_string(),
            "FireBrick".to_string() => "#B22222".to_string(),
            "FloralWhite".to_string() => "#FFFAF0".to_string(),
            "ForestGreen".to_string() => "#228B22".to_string(),
            "Fuchsia".to_string() => "#FF00FF".to_string(),
            "Gainsboro".to_string() => "#DCDCDC".to_string(),
            "GhostWhite".to_string() => "#F8F8FF".to_string(),
            "Gold".to_string() => "#FFD700".to_string(),
            "GoldenRod".to_string() => "#DAA520".to_string(),
            "Gray".to_string() => "#808080".to_string(),
            "Green".to_string() => "#008000".to_string(),
            "GreenYellow".to_string() => "#ADFF2F".to_string(),
            "HoneyDew".to_string() => "#F0FFF0".to_string(),
            "HotPink".to_string() => "#FF69B4".to_string(),
            "IndianRed".to_string() => "#CD5C5C".to_string(),
            "Indigo".to_string() => "#4B0082".to_string(),
            "Ivory".to_string() => "#FFFFF0".to_string(),
            "Khaki".to_string() => "#F0E68C".to_string(),
            "Lavender".to_string() => "#E6E6FA".to_string(),
            "LavenderBlush".to_string() => "#FFF0F5".to_string(),
            "LawnGreen".to_string() => "#7CFC00".to_string(),
            "LemonChiffon".to_string() => "#FFFACD".to_string(),
            "LightBlue".to_string() => "#ADD8E6".to_string(),
            "LightCoral".to_string() => "#F08080".to_string(),
            "LightCyan".to_string() => "#E0FFFF".to_string(),
            "LightGoldenRodYellow".to_string() => "#FAFAD2".to_string(),
            "LightGray".to_string() => "#D3D3D3".to_string(),
            "LightGreen".to_string() => "#90EE90".to_string(),
            "LightPink".to_string() => "#FFB6C1".to_string(),
            "LightSalmon".to_string() => "#FFA07A".to_string(),
            "LightSeaGreen".to_string() => "#20B2AA".to_string(),
            "LightSkyBlue".to_string() => "#87CEFA".to_string(),
            "LightSlateGray".to_string() => "#778899".to_string(),
            "LightSteelBlue".to_string() => "#B0C4DE".to_string(),
            "LightYellow".to_string() => "#FFFFE0".to_string(),
            "Lime".to_string() => "#00FF00".to_string(),
            "LimeGreen".to_string() => "#32CD32".to_string(),
            "Linen".to_string() => "#FAF0E6".to_string(),
            "Magenta".to_string() => "#FF00FF".to_string(),
            "Maroon".to_string() => "#800000".to_string(),
            "MediumAquaMarine".to_string() => "#66CDAA".to_string(),
            "MediumBlue".to_string() => "#0000CD".to_string(),
            "MediumOrchid".to_string() => "#BA55D3".to_string(),
            "MediumPurple".to_string() => "#9370DB".to_string(),
            "MediumSeaGreen".to_string() => "#3CB371".to_string(),
            "MediumSlateBlue".to_string() => "#7B68EE".to_string(),
            "MediumSpringGreen".to_string() => "#00FA9A".to_string(),
            "MediumTurquoise".to_string() => "#48D1CC".to_string(),
            "MediumVioletRed".to_string() => "#C71585".to_string(),
            "MidnightBlue".to_string() => "#191970".to_string(),
            "MintCream".to_string() => "#F5FFFA".to_string(),
            "MistyRose".to_string() => "#FFE4E1".to_string(),
            "Moccasin".to_string() => "#FFE4B5".to_string(),
            "NavajoWhite".to_string() => "#FFDEAD".to_string(),
            "Navy".to_string() => "#000080".to_string(),
            "OldLace".to_string() => "#FDF5E6".to_string(),
            "Olive".to_string() => "#808000".to_string(),
            "OliveDrab".to_string() => "#6B8E23".to_string(),
            "Orange".to_string() => "#FFA500".to_string(),
            "OrangeRed".to_string() => "#FF4500".to_string(),
            "Orchid".to_string() => "#DA70D6".to_string(),
            "PaleGoldenRod".to_string() => "#EEE8AA".to_string(),
            "PaleGreen".to_string() => "#98FB98".to_string(),
            "PaleTurquoise".to_string() => "#AFEEEE".to_string(),
            "PaleVioletRed".to_string() => "#DB7093".to_string(),
            "PapayaWhip".to_string() => "#FFEFD5".to_string(),
            "PeachPuff".to_string() => "#FFDAB9".to_string(),
            "Peru".to_string() => "#CD853F".to_string(),
            "Pink".to_string() => "#FFC0CB".to_string(),
            "Plum".to_string() => "#DDA0DD".to_string(),
            "PowderBlue".to_string() => "#B0E0E6".to_string(),
            "Purple".to_string() => "#800080".to_string(),
            "Red".to_string() => "#FF0000".to_string(),
            "RosyBrown".to_string() => "#BC8F8F".to_string(),
            "RoyalBlue".to_string() => "#4169E1".to_string(),
            "SaddleBrown".to_string() => "#8B4513".to_string(),
            "Salmon".to_string() => "#FA8072".to_string(),
            "SandyBrown".to_string() => "#F4A460".to_string(),
            "SeaGreen".to_string() => "#2E8B57".to_string(),
            "SeaShell".to_string() => "#FFF5EE".to_string(),
            "Sienna".to_string() => "#A0522D".to_string(),
            "Silver".to_string() => "#C0C0C0".to_string(),
            "SkyBlue".to_string() => "#87CEEB".to_string(),
            "SlateBlue".to_string() => "#6A5ACD".to_string(),
            "SlateGray".to_string() => "#708090".to_string(),
            "Snow".to_string() => "#FFFAFA".to_string(),
            "SpringGreen".to_string() => "#00FF7F".to_string(),
            "SteelBlue".to_string() => "#4682B4".to_string(),
            "Tan".to_string() => "#D2B48C".to_string(),
            "Teal".to_string() => "#008080".to_string(),
            "Thistle".to_string() => "#D8BFD8".to_string(),
            "Tomato".to_string() => "#FF6347".to_string(),
            "Turquoise".to_string() => "#40E0D0".to_string(),
            "Violet".to_string() => "#EE82EE".to_string(),
            "Wheat".to_string() => "#F5DEB3".to_string(),
            "White".to_string() => "#FFFFFF".to_string(),
            "WhiteSmoke".to_string() => "#F5F5F5".to_string(),
            "Yellow".to_string() => "#FFFF00".to_string(),
            "YellowGreen".to_string() => "#9ACD32".to_string(),
        };

        Provider {
            safe_colors,
            all_colors,

            // 暂时使用37
            selector: Random::new(37),
        }
    }

    #[inline]
    pub fn color_name(&mut self) -> String {
        self.selector.choice(
            &self
                .all_colors
                .keys()
                .map(std::clone::Clone::clone)
                .collect::<Vec<_>>(),
        )
    }

    #[inline]
    pub fn safe_color_name(&mut self) -> String {
        self.selector.choice(
            &self
                .safe_colors
                .keys()
                .map(std::clone::Clone::clone)
                .collect::<Vec<_>>(),
        )
    }

    #[inline]
    pub fn hex_color(&mut self) -> String {
        self.selector.choice(
            &self
                .all_colors
                .values()
                .map(std::clone::Clone::clone)
                .collect::<Vec<_>>(),
        )
    }

    #[inline]
    pub fn safe_hex_color(&mut self) -> String {
        self.selector.choice(
            &self
                .safe_colors
                .values()
                .map(std::clone::Clone::clone)
                .collect::<Vec<_>>(),
        )
    }

    fn hex_to_rgb(hex: String) -> (u8, u8, u8) {
        let hex = hex.chars().skip(1).collect::<Vec<_>>();
        let char_to_num_mapper: HashMap<char, u8> = hash_map! {
            '0' => 0,  '1' => 1,  '2' => 2,  '3' => 3,
            '4' => 4,  '5' => 5,  '6' => 6,  '7' => 7,
            '8' => 8,  '9' => 9,  'A' => 10, 'B' => 11,
            'C' => 12, 'D' => 13, 'E' => 14, 'F' => 15,
        };
        let c0 = char_to_num_mapper.get(&hex[0]).unwrap();
        let c1 = char_to_num_mapper.get(&hex[1]).unwrap();
        let c2 = char_to_num_mapper.get(&hex[2]).unwrap();
        let c3 = char_to_num_mapper.get(&hex[3]).unwrap();
        let c4 = char_to_num_mapper.get(&hex[4]).unwrap();
        let c5 = char_to_num_mapper.get(&hex[5]).unwrap();
        (c0 * c1, c2 * c3, c4 * c5)
    }

    pub fn rgb_color(&mut self) -> String {
        let (r, g, b) = Self::hex_to_rgb(self.hex_color());
        format!("{},{},{}", r, g, b)
    }

    pub fn rgb_css_color(&mut self) -> String {
        let (r, g, b) = Self::hex_to_rgb(self.hex_color());
        format!("rgb({},{},{})", r, g, b)
    }
}

impl Default for Provider {
    fn default() -> Self {
        Provider::new()
    }
}
