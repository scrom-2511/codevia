#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colors {
    Background,
    Foreground,
    Card,
    CardForeground,
    Popover,
    PopoverForeground,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Muted,
    MutedForeground,
    Accent,
    AccentForeground,
    Destructive,
    Border,
    Input,
    Ring,
    Chart1,
    Chart2,
    Chart3,
    Chart4,
    Chart5,
    Sidebar,
    SidebarForeground,
    SidebarPrimary,
    SidebarPrimaryForeground,
    SidebarAccent,
    SidebarAccentForeground,
    SidebarBorder,
    SidebarRing,
}

impl Colors {
    pub const fn rgb(self) -> (u8, u8, u8) {
        match self {
            Self::Background => (30, 30, 30),
            Self::Foreground => (250, 250, 250),
            Self::Card => (41, 41, 41),
            Self::CardForeground => (250, 250, 250),
            Self::Popover => (41, 41, 41),
            Self::PopoverForeground => (250, 250, 250),
            Self::Primary => (227, 227, 227),
            Self::PrimaryForeground => (41, 41, 41),
            Self::Secondary => (52, 52, 52),
            Self::SecondaryForeground => (250, 250, 250),
            Self::Muted => (52, 52, 52),
            Self::MutedForeground => (161, 161, 161),
            Self::Accent => (52, 52, 52),
            Self::AccentForeground => (250, 250, 250),
            Self::Destructive => (242, 95, 95),
            Self::Border => (255, 255, 255),
            Self::Input => (255, 255, 255),
            Self::Ring => (117, 117, 117),
            Self::Chart1 => (219, 219, 219),
            Self::Chart2 => (117, 117, 117),
            Self::Chart3 => (93, 93, 93),
            Self::Chart4 => (79, 79, 79),
            Self::Chart5 => (52, 52, 52),
            Self::Sidebar => (41, 41, 41),
            Self::SidebarForeground => (250, 250, 250),
            Self::SidebarPrimary => (83, 107, 207),
            Self::SidebarPrimaryForeground => (250, 250, 250),
            Self::SidebarAccent => (52, 52, 52),
            Self::SidebarAccentForeground => (250, 250, 250),
            Self::SidebarBorder => (255, 255, 255),
            Self::SidebarRing => (117, 117, 117),
        }
    }

    pub const fn rgba(self) -> (u8, u8, u8) {
        match self {
            Self::Border | Self::SidebarBorder => {
                let (fg_r, fg_g, fg_b) = Colors::Border.rgb();
                let (bg_r, bg_g, bg_b) = Colors::Background.rgb();
                let alpha = 0.50;

                let r = ((fg_r as f32 * alpha) + (bg_r as f32 * (1.0 - alpha))) as u8;
                let g = ((fg_g as f32 * alpha) + (bg_g as f32 * (1.0 - alpha))) as u8;
                let b = ((fg_b as f32 * alpha) + (bg_b as f32 * (1.0 - alpha))) as u8;

                (r, g, b)
            }
            Self::Input => {
                let (fg_r, fg_g, fg_b) = Colors::Input.rgb();
                let (bg_r, bg_g, bg_b) = Colors::Background.rgb();
                let alpha = 0.15;

                let r = ((fg_r as f32 * alpha) + (bg_r as f32 * (1.0 - alpha))) as u8;
                let g = ((fg_g as f32 * alpha) + (bg_g as f32 * (1.0 - alpha))) as u8;
                let b = ((fg_b as f32 * alpha) + (bg_b as f32 * (1.0 - alpha))) as u8;

                (r, g, b)
            }
            _ => self.rgb(),
        }
    }
}
