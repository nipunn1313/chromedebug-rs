extern crate chromedebug;

use chromedebug::Server;

fn main() {
    println!("Open up chrome-devtools://devtools/bundled/inspector.html?ws=localhost:9223");
    Server::run();
}
