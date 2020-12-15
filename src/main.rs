extern crate regex;
use regex::Regex;
use enigo::*;
use clipboard_win::{formats, get_clipboard, set_clipboard};
fn main() {
  let re=Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
  println!("Did our date match? {}",re.is_match("2014-01-01"));
  let text = "my sample ><";
    
    set_clipboard(formats::Unicode, text).expect("To set clipboard");
    //Type is necessary as string can be stored in various storages
    let result: String = get_clipboard(formats::Unicode).expect("To set clipboard");
    assert_eq!(result, text);
    let mut enigo=Enigo::new();
    enigo.mouse_move_to(1244, 134);
    enigo.mouse_down(MouseButton::Right);
    enigo.mouse_up(MouseButton::Right);
}
