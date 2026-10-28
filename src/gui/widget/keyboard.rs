// SPDX-FileCopyrightText: 2026 The ReGreet Authors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! A built-in on-screen keyboard.
//!
//! The keys never take the keyboard focus, so whatever text field is currently focused (username,
//! password, etc.) keeps it and receives the typed characters. If nothing editable is focused, the
//! text is sent to the first visible fallback field.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use relm4::gtk::{self, prelude::*};

const KEY_WIDTH: i32 = 56;
const WIDE_KEY_WIDTH: i32 = 90;
const SPACE_WIDTH: i32 = 320;
const KEY_HEIGHT: i32 = 48;
const KEY_SPACING: i32 = 6;

/// Number of keys per row is the same in every layer, so the layers can be zipped together.
const LOWER: [&str; 4] = ["1234567890", "qwertyuiop", "asdfghjklñ", "zxcvbnm,.-"];
const UPPER: [&str; 4] = ["!\"#$%&/()=", "QWERTYUIOP", "ASDFGHJKLÑ", "ZXCVBNM;:_"];
const SYMBOLS: [&str; 4] = ["!@#$%^&*()", "[]{}<>\\|~`", "-_=+;:'\"?/", "áéíóú¿¡,.€"];

/// Style class given to the active Shift/symbols buttons.
const ACTIVE_CLASS: &str = "suggested-action";

/// A function that finds the text field that should receive the typed text.
type TargetFinder = Rc<dyn Fn() -> Option<gtk::Editable>>;

/// The state of the keyboard, along with the widgets that depend on it.
struct Osk {
    /// Each character key, along with its characters for: normal, shift and symbols layers.
    keys: RefCell<Vec<(gtk::Button, [char; 3])>>,
    /// Whether shift is active (applies only to the next typed character).
    shift: Cell<bool>,
    /// Whether the symbols layer is active.
    symbols: Cell<bool>,
    shift_button: gtk::Button,
    symbols_button: gtk::Button,
}

impl Osk {
    /// Get the character a key produces in the current layer.
    fn pick(&self, chars: &[char; 3]) -> char {
        if self.symbols.get() {
            chars[2]
        } else if self.shift.get() {
            chars[1]
        } else {
            chars[0]
        }
    }

    /// Update all key labels and the Shift/symbols buttons to match the current state.
    fn refresh(&self) {
        for (button, chars) in self.keys.borrow().iter() {
            button.set_label(&self.pick(chars).to_string());
        }

        set_active_style(&self.shift_button, self.shift.get() && !self.symbols.get());
        set_active_style(&self.symbols_button, self.symbols.get());
        self.symbols_button
            .set_label(if self.symbols.get() { "ABC" } else { "?123" });
    }
}

fn set_active_style(button: &gtk::Button, active: bool) {
    if active {
        button.add_css_class(ACTIVE_CLASS);
    } else {
        button.remove_css_class(ACTIVE_CLASS);
    }
}

/// Create a key that doesn't steal the focus from the text field.
fn make_key(label: &str, width: i32) -> gtk::Button {
    let button = gtk::Button::with_label(label);
    button.set_focusable(false);
    button.set_focus_on_click(false);
    button.set_width_request(width);
    button.set_height_request(KEY_HEIGHT);
    button.add_css_class("osk-key");
    button
}

/// Create a horizontal row of keys.
fn make_row() -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, KEY_SPACING);
    row.set_halign(gtk::Align::Center);
    row
}

/// Find the text field that should receive the typed text.
fn find_target(window: &gtk::ApplicationWindow, fallbacks: &[gtk::Widget]) -> Option<gtk::Editable> {
    if let Some(focused) = GtkWindowExt::focus(window) {
        if let Ok(editable) = focused.dynamic_cast::<gtk::Editable>() {
            return Some(editable);
        }
    }

    fallbacks
        .iter()
        .find(|widget| WidgetExt::is_visible(*widget))
        .and_then(|widget| widget.clone().dynamic_cast::<gtk::Editable>().ok())
}

/// Insert the text at the cursor, replacing any selected text.
fn insert_text(editable: &gtk::Editable, text: &str) {
    if editable.selection_bounds().is_some() {
        editable.delete_selection();
    }
    let mut position = editable.position();
    editable.insert_text(text, &mut position);
    editable.set_position(position);
}

/// Delete the selected text, or else the character before the cursor.
fn backspace(editable: &gtk::Editable) {
    if editable.selection_bounds().is_some() {
        editable.delete_selection();
        return;
    }
    let position = editable.position();
    if position > 0 {
        editable.delete_text(position - 1, position);
    }
}

/// Build the on-screen keyboard.
///
/// - `window`: the main window; its focused widget receives the typed text.
/// - `login_button`: the button that is clicked by the Enter key.
/// - `fallbacks`: text fields used when no text field is focused; the first visible one is used.
///
/// The returned revealer is hidden at first. Add it as an overlay, and show it with
/// `set_reveal_child`.
pub fn build(
    window: &gtk::ApplicationWindow,
    login_button: &gtk::Button,
    fallbacks: Vec<gtk::Widget>,
) -> gtk::Revealer {
    // Use weak references, so that the keys' closures don't keep the window alive.
    let weak_window = window.downgrade();
    let weak_login = login_button.downgrade();
    let target: TargetFinder = Rc::new(move || {
        let window = weak_window.upgrade()?;
        find_target(&window, &fallbacks)
    });

    let shift_button = make_key("⇧", WIDE_KEY_WIDTH);
    let symbols_button = make_key("?123", WIDE_KEY_WIDTH);
    let osk = Rc::new(Osk {
        keys: RefCell::new(Vec::new()),
        shift: Cell::new(false),
        symbols: Cell::new(false),
        shift_button: shift_button.clone(),
        symbols_button: symbols_button.clone(),
    });

    let rows = gtk::Box::new(gtk::Orientation::Vertical, KEY_SPACING);
    rows.set_margin_top(12);
    rows.set_margin_bottom(12);
    rows.set_margin_start(12);
    rows.set_margin_end(12);

    // The rows with the character keys.
    for row_idx in 0..LOWER.len() {
        let row = make_row();

        if row_idx == LOWER.len() - 1 {
            row.append(&shift_button);
        }

        let layers = LOWER[row_idx]
            .chars()
            .zip(UPPER[row_idx].chars())
            .zip(SYMBOLS[row_idx].chars());
        for ((lower, upper), symbol) in layers {
            let chars = [lower, upper, symbol];
            let key = make_key(&lower.to_string(), KEY_WIDTH);

            let osk_key = Rc::clone(&osk);
            let target = Rc::clone(&target);
            key.connect_clicked(move |_| {
                if let Some(editable) = target() {
                    insert_text(&editable, &osk_key.pick(&chars).to_string());
                }
                // Shift only applies to a single character.
                if osk_key.shift.get() {
                    osk_key.shift.set(false);
                    osk_key.refresh();
                }
            });

            osk.keys.borrow_mut().push((key.clone(), chars));
            row.append(&key);
        }

        if row_idx == 0 {
            let backspace_key = make_key("⌫", WIDE_KEY_WIDTH);
            let target = Rc::clone(&target);
            backspace_key.connect_clicked(move |_| {
                if let Some(editable) = target() {
                    backspace(&editable);
                }
            });
            row.append(&backspace_key);
        }

        rows.append(&row);
    }

    // Shift and symbols toggles.
    {
        let osk_clone = Rc::clone(&osk);
        shift_button.connect_clicked(move |_| {
            osk_clone.shift.set(!osk_clone.shift.get());
            osk_clone.refresh();
        });

        let osk_clone = Rc::clone(&osk);
        symbols_button.connect_clicked(move |_| {
            osk_clone.symbols.set(!osk_clone.symbols.get());
            osk_clone.shift.set(false);
            osk_clone.refresh();
        });
    }

    // The bottom row: symbols toggle, space and enter.
    let bottom_row = make_row();
    bottom_row.append(&symbols_button);

    let space_key = make_key("", SPACE_WIDTH);
    {
        let target = Rc::clone(&target);
        space_key.connect_clicked(move |_| {
            if let Some(editable) = target() {
                insert_text(&editable, " ");
            }
        });
    }
    bottom_row.append(&space_key);

    let enter_key = make_key("⏎", WIDE_KEY_WIDTH);
    enter_key.connect_clicked(move |_| {
        if let Some(login_button) = weak_login.upgrade() {
            login_button.emit_clicked();
        }
    });
    bottom_row.append(&enter_key);
    rows.append(&bottom_row);

    let frame = gtk::Frame::new(None);
    frame.add_css_class("background");
    frame.add_css_class("osk");
    frame.set_child(Some(&rows));

    let revealer = gtk::Revealer::new();
    revealer.set_transition_type(gtk::RevealerTransitionType::SlideUp);
    revealer.set_halign(gtk::Align::Center);
    revealer.set_valign(gtk::Align::End);
    // Leave room for the buttons and notifications at the bottom of the screen.
    revealer.set_margin_bottom(90);
    revealer.set_child(Some(&frame));
    revealer.set_reveal_child(false);

    osk.refresh();
    revealer
}
