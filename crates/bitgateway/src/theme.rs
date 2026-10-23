use iced::widget::{button, checkbox, container, rule, text_input, toggler};
use iced::{Border, Color, Font, Shadow, Theme, Vector};

pub const TEXT: Color = Color::from_rgb8(31, 43, 64);
pub const MUTED: Color = Color::from_rgb8(105, 120, 143);
pub const PRIMARY: Color = Color::from_rgb8(48, 101, 216);
pub const SUCCESS: Color = Color::from_rgb8(32, 128, 93);
pub const SOFT_BLUE: Color = Color::from_rgb8(237, 244, 255);

pub const HEADING: Font = Font {
    weight: iced::font::Weight::Semibold,
    ..Font::DEFAULT
};

const BORDER: Color = Color::from_rgb8(225, 232, 242);

pub fn application() -> Theme {
    Theme::custom(
        "BITGATEWAY",
        iced::theme::Palette {
            background: Color::from_rgb8(250, 251, 254),
            text: TEXT,
            primary: PRIMARY,
            success: SUCCESS,
            warning: Color::from_rgb8(133, 103, 35),
            danger: Color::from_rgb8(178, 67, 78),
        },
    )
}

pub fn divider(_theme: &Theme) -> rule::Style {
    rule::Style {
        color: BORDER,
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}

pub fn dialog_panel(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Color::WHITE.into()),
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 14.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba8(31, 43, 64, 0.12),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 18.0,
        },
        ..Default::default()
    }
}

pub fn primary_button(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::primary(theme, status);
    style.background = Some(
        match status {
            button::Status::Active => theme.palette().primary,
            button::Status::Hovered => Color::from_rgb8(44, 95, 206),
            button::Status::Pressed => Color::from_rgb8(39, 88, 196),
            button::Status::Disabled => Color::from_rgb8(170, 192, 239),
        }
        .into(),
    );
    style.text_color = Color::WHITE;
    style.border.radius = 9.0.into();
    style.shadow = Shadow::default();

    style
}

pub fn quiet_button(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: matches!(status, button::Status::Hovered | button::Status::Pressed)
            .then_some(Color::from_rgb8(239, 243, 250).into()),
        text_color: MUTED,
        border: Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn update_button(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: Some(
            if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                Color::from_rgb8(228, 238, 255)
            } else {
                SOFT_BLUE
            }
            .into(),
        ),
        text_color: theme.palette().primary,
        ..quiet_button(theme, status)
    }
}

pub fn logout_button(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: Some(
            if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                Color::from_rgb8(255, 248, 249)
            } else {
                Color::WHITE
            }
            .into(),
        ),
        text_color: if matches!(status, button::Status::Disabled) {
            MUTED
        } else {
            theme.palette().danger
        },
        border: Border {
            color: Color::from_rgb8(237, 219, 224),
            width: 1.0,
            radius: 9.0.into(),
        },
        ..Default::default()
    }
}

pub fn input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let mut style = text_input::default(theme, status);
    style.background = Color::WHITE.into();
    style.border = Border {
        color: match status {
            text_input::Status::Focused { .. } => theme.palette().primary,
            text_input::Status::Hovered => Color::from_rgb8(195, 208, 228),
            _ => BORDER,
        },
        width: 1.0,
        radius: 9.0.into(),
    };

    style
}

pub fn checkmark(theme: &Theme, status: checkbox::Status) -> checkbox::Style {
    let mut style = checkbox::primary(theme, status);
    let (checked, enabled) = match status {
        checkbox::Status::Active { is_checked } | checkbox::Status::Hovered { is_checked } => {
            (is_checked, true)
        }
        checkbox::Status::Disabled { is_checked } => (is_checked, false),
    };
    style.background = if checked {
        theme
            .palette()
            .primary
            .scale_alpha(if enabled { 1.0 } else { 0.4 })
    } else {
        Color::WHITE
    }
    .into();
    style.border = Border {
        color: if checked {
            theme.palette().primary
        } else {
            BORDER
        },
        width: 1.0,
        radius: 4.0.into(),
    };
    style.text_color = Some(MUTED);

    style
}

pub fn toggle(theme: &Theme, status: toggler::Status) -> toggler::Style {
    let (checked, enabled) = match status {
        toggler::Status::Active { is_toggled } | toggler::Status::Hovered { is_toggled } => {
            (is_toggled, true)
        }
        toggler::Status::Disabled { is_toggled } => (is_toggled, false),
    };

    toggler::Style {
        background: if checked {
            theme.palette().primary
        } else if enabled {
            Color::from_rgb8(206, 216, 232)
        } else {
            Color::from_rgb8(232, 237, 245)
        }
        .into(),
        foreground: Color::WHITE.into(),
        ..toggler::default(theme, status)
    }
}
