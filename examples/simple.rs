extern crate winrt_notification;
use winrt_notification::{
    Duration,
    Sound,
    Toast,
};

fn main() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Look at this flip!")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .action("Flip it!", "flip_table", || { println!("°□° You flipped it!"); })
        .show()
        .expect("unable to send notification");
    
    std::thread::sleep(std::time::Duration::from_millis(10000));
}
