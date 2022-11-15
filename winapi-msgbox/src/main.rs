extern crate msgbox;

use msgbox::IconType;

fn main() {
    msgbox::create("Your Title", "Your Message", IconType::Info);
}
