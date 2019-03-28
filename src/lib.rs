pub mod game;
pub mod player;
pub mod zombie;
pub mod combo;

#[macro_export]
macro_rules! color {
    (Black, $s: expr) => {
        concat!("\x1b[30m", $s, "\x1b[0m")
    };
    (Red, $s: expr) => {
        concat!("\x1b[31m", $s, "\x1b[0m")
    };
    (Green, $s: expr) => {
        concat!("\x1b[32m", $s, "\x1b[0m")
    };
    (Yellow, $s: expr) => {
        concat!("\x1b[33m", $s, "\x1b[0m")
    };
    (Blue, $s: expr) => {
        concat!("\x1b[34m", $s, "\x1b[0m")
    };
    (Magenta, $s: expr) => {
        concat!("\x1b[35m", $s, "\x1b[0m")
    };
    (Cyan, $s: expr) => {
        concat!("\x1b[36m", $s, "\x1b[0m")
    };
    (White, $s: expr) => {
        concat!("\x1b[37m", $s, "\x1b[0m")
    };
}
