extern crate regex;
extern crate keybd_event;
use regex::Regex;
use enigo::*;
use clipboard_win::{formats, get_clipboard, set_clipboard};
use std::thread::sleep;
use std::time::Duration;
use keybd_event::KeyboardKey::{KeyA,KeyZ,KeyV};
use keybd_event::KeyBondingInstance;
fn main() {

  //正则
  let re=Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
  println!("Did our date match? {}",re.is_match("2014-01-01"));
  let text = "my s...ample ><";
    
  //粘贴板
    set_clipboard(formats::Unicode, text).expect("To set clipboard");
    //Type is necessary as string can be stored in various storages
    let result: String = get_clipboard(formats::Unicode).expect("To set clipboard");
    assert_eq!(result, text);


    //鼠标
    let mut enigo=Enigo::new();
    enigo.mouse_move_to(1244, 134);
    enigo.mouse_down(MouseButton::Left);
    enigo.mouse_up(MouseButton::Left);


    //键盘
    sleep(Duration::from_secs(2));
    let mut kb=KeyBondingInstance::new().unwrap();
    // kb.has_shift(true);
    kb.has_ctrl(true);
    kb.add_keys(&[KeyV]);
    kb.launching();
}
