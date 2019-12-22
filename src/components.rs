use tcod::colors;

pub enum Component {
    Position(PositionComponent),
    Render(RenderComponent),
    Action(ActionComponent),
}

pub struct PositionComponent {
    pub x: i32,
    pub y: i32,
    pub map: usize,
}

pub struct RenderComponent {
    pub glyph: char,
    pub fg: colors::Color,
    pub bg: Option<colors::Color>,
}

pub enum Action {
    Move(i32, i32),
}

pub struct ActionComponent {
    pub actions: Vec<Action>,
}
