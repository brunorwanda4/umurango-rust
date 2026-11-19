use std::time::Duration;

use gpui_kit::base::{
    StyledExt, Toast, ToastAdvance, ToastManager, ToastMotion, ToastOptions, ToastStack,
    ToastStackState, ToastTransitionStatus,
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::brand::ThemeColors;
use crate::icons;

/// Visual variant for the toast notification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastVariant {
    Success,
    Expense,
    Info,
    Warning,
    Error,
}

impl ToastVariant {
    /// Returns (accent_color, badge_background, icon_name).
    pub fn config(self, palette: &ThemeColors) -> (Rgba, Rgba, &'static str) {
        match self {
            Self::Success => (
                palette.success,
                palette.success.opacity(0.16),
                icons::ICON_CIRCLE_CHECK,
            ),
            Self::Expense => (
                palette.expense_text,
                palette.expense_bg,
                icons::ICON_ARROW_DOWN,
            ),
            Self::Info => (
                palette.accent_blue,
                palette.accent_blue.opacity(0.16),
                icons::ICON_CIRCLE_INFO,
            ),
            Self::Warning => (
                palette.accent_gold,
                palette.accent_gold.opacity(0.16),
                icons::ICON_TRIANGLE_EXCLAMATION,
            ),
            Self::Error => (
                palette.error,
                palette.error.opacity(0.16),
                icons::ICON_CIRCLE_XMARK,
            ),
        }
    }
}

/// Data payload for a toast item.
#[derive(Clone, Debug)]
pub struct ToastItem {
    pub id: u64,
    pub title: SharedString,
    pub message: Option<SharedString>,
    pub variant: ToastVariant,
    pub duration: Duration,
}

impl ToastItem {
    pub fn new(id: u64, title: impl Into<SharedString>, variant: ToastVariant) -> Self {
        Self {
            id,
            title: title.into(),
            message: None,
            variant,
            duration: Duration::from_secs(4),
        }
    }

    pub fn success(id: u64, title: impl Into<SharedString>) -> Self {
        Self::new(id, title, ToastVariant::Success)
    }

    pub fn expense(id: u64, title: impl Into<SharedString>) -> Self {
        Self::new(id, title, ToastVariant::Expense)
    }

    pub fn info(id: u64, title: impl Into<SharedString>) -> Self {
        Self::new(id, title, ToastVariant::Info)
    }

    pub fn warning(id: u64, title: impl Into<SharedString>) -> Self {
        Self::new(id, title, ToastVariant::Warning)
    }

    pub fn error(id: u64, title: impl Into<SharedString>) -> Self {
        Self::new(id, title, ToastVariant::Error)
    }

    pub fn with_message(mut self, message: impl Into<SharedString>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

/// Renders a single styled Toast item using the GPUI Base `Toast` primitive.
pub fn render_toast_item(
    item: &ToastItem,
    status: ToastTransitionStatus,
    palette: &ThemeColors,
    on_dismiss: impl Fn(u64, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let (accent_color, badge_bg, icon_name) = item.variant.config(palette);
    let item_id = item.id;
    let dismiss_fn = on_dismiss.clone();

    Toast::new(format!("toast-item-{}", item.id))
        .transition_status(status)
        .w_full()
        .bg(palette.bg_elevated)
        .rounded(palette.radius_input)
        .border(palette.border_width_input)
        .border_color(palette.border_color)
        .shadow(palette.card_shadows())
        .p_3()
        .child(
            div()
                .flex()
                .items_start()
                .gap_2p5()
                .w_full()
                .child(
                    // Tactile icon badge
                    div()
                        .flex_shrink_0()
                        .size(px(28.0))
                        .rounded(px(8.0))
                        .bg(badge_bg)
                        .border(px(1.5))
                        .border_color(accent_color.opacity(0.4))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            icons::icon(icon_name)
                                .size(px(14.0))
                                .text_color(accent_color),
                        ),
                )
                .child(
                    // Title and optional message
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_primary)
                                .child(item.title.clone()),
                        )
                        .when_some(item.message.clone(), |col, msg| {
                            col.child(
                                div()
                                    .text_xs()
                                    .text_color(palette.text_secondary)
                                    .child(msg),
                            )
                        }),
                )
                .child(
                    // Dismiss close button
                    div()
                        .id(format!("btn-dismiss-toast-{}", item.id))
                        .cursor_pointer()
                        .p_1()
                        .rounded(px(4.0))
                        .hover(|s| s.bg(palette.bg_input))
                        .child(
                            icons::icon(icons::ICON_XMARK)
                                .size(px(11.0))
                                .text_color(palette.text_muted),
                        )
                        .on_click(move |_e, window, cx| {
                            dismiss_fn(item_id, window, cx);
                        }),
                ),
        )
}

/// Renders the stacked toast layer using GPUI Base `ToastStack`.
pub fn render_toast_stack<'a, I>(
    stack_id: impl Into<ElementId>,
    stack_state: ToastStackState,
    items: I,
    palette: &'a ThemeColors,
    on_dismiss: impl Fn(u64, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement
where
    I: IntoIterator<Item = (&'a u64, &'a ToastItem, ToastTransitionStatus)>,
{
    let mut stack = ToastStack::new(stack_id, stack_state)
        .placement(Anchor::BottomRight)
        .motion(ToastMotion::sonner())
        .v_flex()
        .w_full()
        .max_w(px(360.0));

    for (&id, item, status) in items {
        let dismiss = on_dismiss.clone();
        stack = stack.item(
            format!("toast-entry-{}", id),
            render_toast_item(item, status, palette, dismiss),
        );
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brand::colors;
    use crate::brand::ThemeMode;
    use core::prelude::v1::test;
    use std::time::Instant;

    #[test]
    fn toast_variant_config_returns_expected_icons_and_accents() {
        let palette = colors(ThemeMode::Dark);

        let (success_color, _, success_icon) = ToastVariant::Success.config(&palette);
        assert_eq!(success_color, palette.success);
        assert_eq!(success_icon, icons::ICON_CIRCLE_CHECK);

        let (expense_color, _, expense_icon) = ToastVariant::Expense.config(&palette);
        assert_eq!(expense_color, palette.expense_text);
        assert_eq!(expense_icon, icons::ICON_ARROW_DOWN);

        let (info_color, _, info_icon) = ToastVariant::Info.config(&palette);
        assert_eq!(info_color, palette.accent_blue);
        assert_eq!(info_icon, icons::ICON_CIRCLE_INFO);

        let (warning_color, _, warning_icon) = ToastVariant::Warning.config(&palette);
        assert_eq!(warning_color, palette.accent_gold);
        assert_eq!(warning_icon, icons::ICON_TRIANGLE_EXCLAMATION);

        let (error_color, _, error_icon) = ToastVariant::Error.config(&palette);
        assert_eq!(error_color, palette.error);
        assert_eq!(error_icon, icons::ICON_CIRCLE_XMARK);
    }

    #[test]
    fn toast_item_constructors_and_builder_methods() {
        let item = ToastItem::success(1, "Income Recorded")
            .with_message("RWF 50,000 added")
            .with_duration(Duration::from_secs(5));

        assert_eq!(item.id, 1);
        assert_eq!(item.title, "Income Recorded");
        assert_eq!(item.message, Some("RWF 50,000 added".into()));
        assert_eq!(item.variant, ToastVariant::Success);
        assert_eq!(item.duration, Duration::from_secs(5));

        let expense = ToastItem::expense(2, "Expense Recorded");
        assert_eq!(expense.variant, ToastVariant::Expense);
        assert_eq!(expense.duration, Duration::from_secs(4));
    }

    #[test]
    fn toast_manager_integration_pushes_advances_and_dismisses() {
        let now = Instant::now();
        let motion = ToastMotion::sonner();
        let mut manager: ToastManager<u64, ToastItem> = ToastManager::new(motion);

        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);

        let item1 = ToastItem::success(101, "Income Recorded");
        manager.push(
            101,
            item1,
            ToastOptions {
                timeout: Some(Duration::from_secs(4)),
            },
            now,
        );

        assert_eq!(manager.len(), 1);
        assert_eq!(
            manager.get(&101).map(|t| &t.title),
            Some(&"Income Recorded".into())
        );

        // Advance to Present status
        let advance = manager.advance(now + motion.duration, false);
        assert!(advance.changed);
        assert_eq!(advance.presented, vec![101]);

        // Dismiss the toast
        let dismissed = manager.dismiss(&101, now + motion.duration);
        assert!(dismissed);

        // Advance past exit duration
        let exit_advance = manager.advance(now + motion.duration + motion.exit_duration, false);
        assert!(exit_advance.changed);
        assert_eq!(exit_advance.removed.len(), 1);
        assert_eq!(exit_advance.removed[0].0, 101);
        assert!(manager.is_empty());
    }
}
