//! Tile types for the ASCII world. Each tile has a glyph and default colours.
//! The palette is original; nothing is copied from a commercial product.

use crate::domain::color::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TileType {
    Ground,
    Wall,
    Water,
    Tree,
    Building,
    Door,
    Loot,
    Horse,
}

impl TileType {
    /// The glyph used to render this tile.
    pub fn glyph(self) -> char {
        match self {
            TileType::Ground => '.',
            TileType::Wall => '#',
            TileType::Water => '~',
            TileType::Tree => 'T',
            TileType::Building => 'B',
            TileType::Door => '+',
            TileType::Loot => '$',
            TileType::Horse => '&',
        }
    }

    /// Default foreground colour.
    pub fn fg(self) -> Rgb {
        match self {
            TileType::Ground => Rgb::GRAY,
            TileType::Wall => Rgb::DARK_GRAY,
            TileType::Water => Rgb::BLUE,
            TileType::Tree => Rgb::GREEN,
            TileType::Building => Rgb::BROWN,
            TileType::Door => Rgb::ORANGE,
            TileType::Loot => Rgb::CYAN,
            TileType::Horse => Rgb::BROWN,
        }
    }

    /// Default background colour.
    pub fn bg(self) -> Rgb {
        match self {
            TileType::Water => Rgb::new(30, 60, 100),
            _ => Rgb::BLACK,
        }
    }

    /// Whether the player can walk onto this tile.
    pub fn walkable(self) -> bool {
        matches!(
            self,
            TileType::Ground | TileType::Door | TileType::Loot | TileType::Horse
        )
    }

    /// Create a flat sprite for this tile (gui feature only).
    #[cfg(feature = "gui")]
    pub fn to_sprite(&self, camera: &crate::render::Camera) -> crate::render::Sprite {
        use crate::render::Sprite;
        use crate::domain::color::Rgb;

        let (ch, fg, bg) = match self {
            TileType::Ground => ('.', Rgb::GRAY, Rgb::BLACK),
            TileType::Wall => ('#', Rgb::DARK_GRAY, Rgb::BLACK),
            TileType::Water => ('~', Rgb::BLUE, Rgb::new(30, 60, 100)),
            TileType::Tree => ('T', Rgb::GREEN, Rgb::BLACK),
            TileType::Building => ('B', Rgb::BROWN, Rgb::BLACK),
            TileType::Door => ('+', Rgb::ORANGE, Rgb::BLACK),
            TileType::Loot => ('$', Rgb::CYAN, Rgb::BLACK),
            TileType::Horse => ('&', Rgb::BROWN, Rgb::BLACK),
        };

        Sprite {
            key: "tile",
            width: 1,
            height: 1,
            cells: vec![vec![(ch, fg, bg)]],
            depth: vec![vec![0.0]],
            anchor: (0, 0),
        }
    }
}
