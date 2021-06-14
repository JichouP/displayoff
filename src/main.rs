use winapi::HWND_TOPMOST;

extern crate user32;
extern crate winapi;

fn main() {
    unsafe {
        user32::PostMessageW(HWND_TOPMOST, 0x0112, 0xf170, 2);
    }
}
