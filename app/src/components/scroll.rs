use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

#[derive(Default)]
struct TouchScrollState {
    scroll_handle: ScrollHandle,
    mouse_drag_origin: Option<(Pixels, Pixels)>,
    touch_drag_origin: Option<(Point<Pixels>, Pixels)>,
}

#[derive(IntoElement)]
pub struct TouchScrollable {
    id: ElementId,
    max_height: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl TouchScrollable {
    pub fn new(id: impl Into<ElementId>, max_height: Pixels) -> Self {
        Self {
            id: id.into(),
            max_height: Some(max_height),
            children: Vec::new(),
        }
    }

    pub fn fill(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            max_height: None,
            children: Vec::new(),
        }
    }
}

impl ParentElement for TouchScrollable {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TouchScrollable {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| TouchScrollState::default());
        let scroll_handle = state.read(cx).scroll_handle.clone();
        let view_id = window.current_view();

        let touch_layer = {
            let state = state.clone();
            canvas(
                |bounds, window, _| window.insert_hitbox(bounds, HitboxBehavior::Normal),
                move |_, hitbox, window, _| {
                    let state = state.clone();
                    window.on_mouse_event(move |event: &TouchDragEvent, phase, window, cx| {
                        if !phase.bubble() {
                            return;
                        }
                        state.update(cx, |this, cx| {
                            if event.phase == TouchPhase::Started {
                                if window.default_prevented() || !hitbox.is_hovered(window) {
                                    return;
                                }
                                window.prevent_default();
                                this.touch_drag_origin =
                                    Some((event.start_position, this.scroll_handle.offset().y));
                            }
                            let Some((start_pos, start_offset_y)) = this.touch_drag_origin else {
                                return;
                            };
                            if start_pos != event.start_position {
                                return;
                            }
                            cx.stop_propagation();
                            match event.phase {
                                TouchPhase::Started | TouchPhase::Moved => {
                                    let delta_y = event.position.y - start_pos.y;
                                    let max_y = this.scroll_handle.max_offset().y.max(px(0.));
                                    let mut offset = this.scroll_handle.offset();
                                    offset.y = (start_offset_y + delta_y).clamp(-max_y, px(0.));
                                    this.scroll_handle.set_offset(offset);
                                    window.refresh();
                                    cx.notify();
                                }
                                TouchPhase::Ended | TouchPhase::Cancelled => {
                                    this.touch_drag_origin = None;
                                }
                            }
                        });
                        cx.notify(view_id);
                    });
                },
            )
            .absolute()
            .size_full()
        };

        div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .when_some(self.max_height, |this, height| this.h(height))
            .when(self.max_height.is_none(), |this| this.h_full())
            .overflow_hidden()
            .cursor_grab()
            .on_mouse_down(MouseButton::Left, {
                let state = state.clone();
                move |event: &MouseDownEvent, _window, cx| {
                    state.update(cx, |this, _cx| {
                        this.mouse_drag_origin =
                            Some((event.position.y, this.scroll_handle.offset().y));
                    });
                }
            })
            .on_mouse_move({
                let state = state.clone();
                move |event: &MouseMoveEvent, window, cx| {
                    if !event.dragging() {
                        return;
                    }
                    let mut did_scroll = false;
                    state.update(cx, |this, cx| {
                        if let Some((start_y, start_offset_y)) = this.mouse_drag_origin {
                            let delta_y = event.position.y - start_y;
                            let max_y = this.scroll_handle.max_offset().y.max(px(0.));
                            let mut offset = this.scroll_handle.offset();
                            offset.y = (start_offset_y + delta_y).clamp(-max_y, px(0.));
                            this.scroll_handle.set_offset(offset);
                            did_scroll = true;
                            window.refresh();
                            cx.notify();
                        }
                    });
                    if did_scroll {
                        cx.notify(view_id);
                    }
                }
            })
            .on_mouse_up(MouseButton::Left, {
                let state = state.clone();
                move |_event: &MouseUpEvent, _window, cx| {
                    state.update(cx, |this, _cx| {
                        this.mouse_drag_origin = None;
                    });
                }
            })
            .on_mouse_up_out(MouseButton::Left, {
                let state = state.clone();
                move |_event: &MouseUpEvent, _window, cx| {
                    state.update(cx, |this, _cx| {
                        this.mouse_drag_origin = None;
                    });
                }
            })
            .child(
                div()
                    .id((self.id, "scroll-area"))
                    .size_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .track_scroll(&scroll_handle)
                    .child(
                        div()
                            .flex_none()
                            .w_full()
                            .h_auto()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .pr_3()
                            .children(self.children),
                    ),
            )
            .child(touch_layer)
            .vertical_scrollbar(&scroll_handle)
    }
}

pub fn touch_scrollable(id: impl Into<ElementId>, max_height: Pixels) -> TouchScrollable {
    TouchScrollable::new(id, max_height)
}

pub fn touch_scrollable_fill(id: impl Into<ElementId>) -> TouchScrollable {
    TouchScrollable::fill(id)
}
