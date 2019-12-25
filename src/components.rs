use crate::coord::Coord;
use tcod::colors;

pub enum Component {
    Position(PositionComponent),
    Render(RenderComponent),
}

pub struct PositionComponent {
    pub coord: Coord,
    pub map: usize,
}

pub struct RenderComponent {
    pub glyph: char,
    pub fg: colors::Color,
    pub bg: Option<colors::Color>,
}
