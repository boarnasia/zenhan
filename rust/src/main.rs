use clap::Parser;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageA, WM_IME_CONTROL};

/// IME status controller
///
/// 0: IME ON
/// 1: IME OFF
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// IME status (0: ON, 1: OFF)
    status: Option<u8>,
}

fn main() {
    let cli = Cli::parse();

    // 定数の定義
    const IMC_GETOPENSTATUS: usize = 5;
    const IMC_SETOPENSTATUS: usize = 6;

    unsafe {
        // フォアグラウンドウィンドウの取得
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            eprintln!("No foreground window found.");
            return;
        }

        // デフォルトIMEウィンドウの取得
        let ime_hwnd = ImmGetDefaultIMEWnd(hwnd);
        if ime_hwnd.is_invalid() {
            eprintln!("No IME window found.");
            return;
        }

        let stat: isize;

        if let Some(target_stat) = cli.status {
            // Set IME status
            SendMessageA(
                ime_hwnd,
                WM_IME_CONTROL,
                WPARAM(IMC_SETOPENSTATUS),
                LPARAM(target_stat as isize),
            );
            stat = target_stat as isize;
        } else {
            // Get IME status
            stat = SendMessageA(
                ime_hwnd,
                WM_IME_CONTROL,
                WPARAM(IMC_GETOPENSTATUS),
                LPARAM(0),
            )
            .0;
        }

        println!("{}", stat);
    }
}
