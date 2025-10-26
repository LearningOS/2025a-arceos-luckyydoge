/// Writes a byte to the console.
pub fn putchar(c: u8) {
    #[allow(deprecated)]
    // ANSI 红色文本控制码（\033 是 ESC 字符的八进制表示）
    const RED: &[u8] = b"\x1B[31m";
    // 重置所有属性（恢复默认颜色）
    const RESET: &[u8] = b"\x1B[0m";

    unsafe {
        // 1. 先发送红色控制码（让后续字符显示为红色）
        if !SET_RED {
            SET_RED = true;
            for &byte in RED {
                sbi_rt::legacy::console_putchar(byte as usize);
            }
        }
    }

    sbi_rt::legacy::console_putchar(c as usize);

    // 3. 发送重置码（避免后续字符继续保持红色）
    // for &byte in RESET {
    //     sbi_rt::legacy::console_putchar(byte as usize);
    // }
}

/// Reads a byte from the console, or returns [`None`] if no input is available.
pub fn getchar() -> Option<u8> {
    #[allow(deprecated)]
    match sbi_rt::legacy::console_getchar() as isize {
        -1 => None,
        c => Some(c as u8),
    }
}

static mut SET_RED: bool = false;
