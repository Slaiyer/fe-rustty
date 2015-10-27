use core::position::HasSize;
use core::cellbuffer::{Attr, CellAccessor};

use ui::core::{
    Alignable, 
    HorizontalAlign, 
    VerticalAlign,
    Widget,
    Painter,
    Base,
    Button,
    ButtonResult,
    find_accel_char_index
};

/// A standard button that returns some result based on
/// whether a key is pressed
/// 
/// # Examples
///
/// ```
/// use rustty::ui::core::{HorizontalAlign, VerticalAlign, ButtonResult, Widget};
/// use rustty::ui::{Dialog, StdButton};
///
/// let mut dlg = Dialog::new(60, 10);
///
/// let mut b1 = StdButton::new("Foo", 'f', ButtonResult::Ok);
/// let mut b2 = StdButton::new("Bar" ,'b', ButtonResult::Cancel);
///
/// b1.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Middle, (1,1));
/// b2.pack(&dlg, HorizontalAlign::Middle, VerticalAlign::Middle, (1,1));
///
/// dlg.add_button(b1);
/// dlg.add_button(b2);
/// ```
///
pub struct StdButton {
    window: Base,
    accel: char,
    result: ButtonResult,
    text: String
}

impl StdButton {    
    /// Constructs a new `StdButton`, asking for the text to be displayed 
    /// by the button, the key to map to, and the result returned when the
    /// key is detected
    ///
    /// # Examples
    ///
    /// ```
    /// use rustty::ui::core::ButtonResult;
    /// use rustty::ui::StdButton;
    ///
    /// let mut b1 = StdButton::new("Foo", 'f', ButtonResult::Ok);
    /// ```
    ///
    pub fn new(text: &str, accel: char, result: ButtonResult) -> StdButton {
        let s = format!("< {} >", text);
        let width = s.chars().count();
        let mut button = 
            StdButton { window: Base::new(width, 1), 
                        accel: accel.to_lowercase().next().unwrap_or(accel),
                        result: result,
                        text: s };
        button.window.printline(0, 0, &button.text[..]);
        match find_accel_char_index(text, button.accel) {
            Some(i) => {
                button.window.get_mut(i+2, 0).unwrap().set_attrs(Attr::Bold);
            },
            None    => (),
        }
        button
    }
}

impl Button for StdButton {
    fn accel(&self) -> char {
        self.accel
    }

    fn result(&self) -> ButtonResult {
        self.result
    }
}

impl Widget for StdButton {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.window.draw_into(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
                margin: (usize,usize)) {
        self.window.align(parent, halign, valign, margin);
    }

    fn window(&self) -> &Base {
        &self.window
    }

    fn window_mut(&mut self) -> &mut Base {
        &mut self.window
    }
}

