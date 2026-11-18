use crate::brand::ThemeColors;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarSize {
    Sm,
    Md,
    Lg,
    Xl,
}

impl AvatarSize {
    pub fn pixels(self) -> Pixels {
        match self {
            Self::Sm => px(24.),
            Self::Md => px(32.),
            Self::Lg => px(40.),
            Self::Xl => px(48.),
        }
    }

    pub fn font_size(self) -> Pixels {
        match self {
            Self::Sm => px(10.),
            Self::Md => px(12.),
            Self::Lg => px(14.),
            Self::Xl => px(16.),
        }
    }
}

pub fn extract_initials(name: &str) -> String {
    let initials: String = name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    if initials.is_empty() {
        "?".to_string()
    } else {
        initials
    }
}

pub fn avatar_initials(
    initials: impl Into<SharedString>,
    size: AvatarSize,
    palette: ThemeColors,
) -> impl IntoElement {
    let px_size = size.pixels();
    div()
        .w(px_size)
        .h(px_size)
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .bg(palette.primary.opacity(0.15))
        .border_1()
        .border_color(palette.primary)
        .text_size(size.font_size())
        .font_weight(FontWeight::BOLD)
        .text_color(palette.primary_light)
        .child(initials.into())
}

pub fn member_avatar(
    name: &str,
    role: Option<&str>,
    size: AvatarSize,
    palette: ThemeColors,
) -> impl IntoElement {
    let display_initials = extract_initials(name);

    div()
        .flex()
        .items_center()
        .gap_3()
        .child(avatar_initials(display_initials, size, palette))
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(palette.text_primary)
                        .child(name.to_string()),
                )
                .when_some(role, |this, r| {
                    this.child(
                        div()
                            .text_xs()
                            .text_color(palette.text_muted)
                            .child(r.to_string()),
                    )
                }),
        )
}
