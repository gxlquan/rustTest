extern crate regex;
extern crate keybd_event;
use regex::Regex;
use enigo::*;
use clipboard_win::{formats, get_clipboard, set_clipboard};
use std::thread::sleep;
use std::time::Duration;
use keybd_event::KeyboardKey::{KeyA,KeyZ,KeyV,KeyC};
use keybd_event::KeyBondingInstance;
use rdev::{listen, Event};
use std::process;



fn run_app() {
  loop {
    println!("Running...");
    sleep(Duration::from_millis(500));
  }
}


fn main() {

  
  


  //正则
  // let re=Regex::new(r"的几率额外掉落一叠通货碎片").unwrap();
  // println!("Did our date match? {}",re.is_match("2014-01-01"));
  // let text = "my s...ample ><";
    
  //粘贴板
    // set_clipboard(formats::Unicode, text).expect("To set clipboard");
    //Type is necessary as string can be stored in various storages
    // let result: String = get_clipboard(formats::Unicode).expect("To set clipboard");
    // assert_eq!(result, text); 
    // println!("{}",result);

  
    //鼠标
    // let mut enigo=Enigo::new();
    // enigo.mouse_move_to(1244, 134);
    // enigo.mouse_down(MouseButton::Left);
    // enigo.mouse_up(MouseButton::Left);


    //键盘
    // sleep(Duration::from_secs(2));
    // let mut kb=KeyBondingInstance::new().unwrap();
    // // kb.has_shift(true);
    // // kb.has_ctrl(true);
    // // kb.add_keys(&[KeyV]);
    // // kb.launching();
    // // kb.launching();
    // // kb.clear();
    // // sleep(Duration::from_secs(3));
    // kb.has_ctrl(true);
    // kb.add_keys(&[KeyA]);
    // kb.launching();
    // kb.clear();
    // sleep(Duration::from_secs(3));
    // kb.has_ctrl(true);
    // kb.add_keys(&[KeyC]);
    // kb.launching();
    // kb.clear();
    // sleep(Duration::from_secs(1));
    // let result: String = get_clipboard(formats::Unicode).expect("To set clipboard");
    // // assert_eq!(result, text); 
    // println!("{}",result);
    // println!("Did our date match? {}",re.is_match(&result));


  //键盘监听
    if let Err(error) = listen(callback) {
      println!("Error: {:?}", error)
  }
  
  fn callback(event: Event) {
      // println!("My callback {:?}", event);
      // match event.name {
      //     Some(string) => println!("User wrote {:?}", string),
      //     None => (),
      // }
      match event.event_type{
        rdev::EventType::KeyRelease(rdev::Key::Home) => {
          println!("home 1");
        },
        rdev::EventType::KeyRelease(rdev::Key::End) => {
          println!("end");
        },
          _ => {},
      }

      // if let rdev::EventType::KeyRelease(rdev::Key::Home)  = event.event_type {
      //   println!("end")
      // }
  }
  // run_app();
  // sleep(Duration::from_secs(3));
  // println!("stop.");

  //   std::process::exit( run_app);

}
