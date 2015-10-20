extern crate rustty;

use rustty::{
    Terminal,
    Event,
};

use rustty::ui::core::{
    Painter,
    Widget,
    ButtonResult,
    Alignable,
    HorizontalAlign,
    VerticalAlign,
};

use rustty::ui::{
    StdButton,
    Dialog
};

fn create_maindlg() -> Dialog {
    let mut maindlg = Dialog::new(60, 10);
    maindlg.window_mut().draw_box();
    
    let mut b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);
    b1.pack(&maindlg, HorizontalAlign::Left, VerticalAlign::Top, 6);
    let mut b2 = StdButton::new("Foo", 'f', ButtonResult::Custom(1));
    b2.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Top, 6);
    let mut b3 = StdButton::new("Bar", 'b', ButtonResult::Custom(2));
    b3.pack(&maindlg, HorizontalAlign::Right, VerticalAlign::Top, 6);
    
    maindlg.add_button(b1);
    maindlg.add_button(b2);
    maindlg.add_button(b3);
    maindlg
}

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_maindlg();
    maindlg.window_mut().align(&term, HorizontalAlign::Middle, VerticalAlign::Middle, 0);
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match maindlg.result_for_key(ch) {
                Some(ButtonResult::Ok)          => break 'main,
                Some(ButtonResult::Custom(i))   => {
                    let msg = if i == 1 { "Foo!" } else { "Bar!" };
                    let w = maindlg.window_mut();
                    let x = w.halign_line(msg, HorizontalAlign::Middle, 1);
                    let y = w.valign_line(msg, VerticalAlign::Middle, 1);
                    w.printline(x, y, msg);
                },
                _ => {},
            }
        }

        maindlg.window().draw_into(&mut term);
        term.swap_buffers().unwrap();
    }
}

