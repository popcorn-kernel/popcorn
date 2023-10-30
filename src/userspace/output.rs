use crate::low_level::vga_buffer::{send_command_to_writer, Color, CommandToWriter};

#[macro_export]
macro_rules! print_with_colors {
    ( $( $x:expr ),* ) => {
        {
            $(
                $x.print_to_vga();
            )*
        }
        $crate::low_level::vga_buffer::send_command_to_writer(popcorn::low_level::vga_buffer::CommandToWriter::SetColor(Color::White, Color::Black));
    }
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::low_level::vga_buffer::send_command_to_writer($crate::low_level::vga_buffer::CommandToWriter::Print(format_args!($($arg)*))));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// log!("text") - this will get the file that called this, and say it as [file_name] <text>
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        $crate::print_with_colors!(
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::LightBlue, $crate::low_level::vga_buffer::Color::Black, "["),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::Black, $crate::low_level::vga_buffer::Color::LightGreen, core::file!()),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::LightBlue, $crate::low_level::vga_buffer::Color::Black, "] "),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::White, $crate::low_level::vga_buffer::Color::Black, $($arg)*)
        );
        $crate::println!();
    )
}

/// warn!("text") - this will get the file that called this, and say it as [file_name] <text>
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        $crate::print_with_colors!(
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::LightBlue, $crate::low_level::vga_buffer::Color::Black, "["),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::Black, $crate::low_level::vga_buffer::Color::Yellow, core::file!()),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::LightBlue, $crate::low_level::vga_buffer::Color::Black, "] "),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::Yellow, $crate::low_level::vga_buffer::Color::Black, $($arg)*)
        );
        $crate::println!();
    )
}

/// error!("text") - this will get the file that called this, and say it as [file_name] <text>
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::print_with_colors!(
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::LightBlue, $crate::low_level::vga_buffer::Color::Black, "["),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::Black, $crate::low_level::vga_buffer::Color::LightRed, core::file!()),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::LightBlue, $crate::low_level::vga_buffer::Color::Black, "] "),
            $crate::userspace::output::MessageToVga::new($crate::low_level::vga_buffer::Color::LightRed, $crate::low_level::vga_buffer::Color::Black, $($arg)*)
        );
        $crate::println!();
    )
}

pub struct MessageToVga<'a> {
    foreground: Color,
    background: Color,
    string: &'a str,
}

impl<'a> MessageToVga<'a> {
    pub fn print_to_vga(&self) {
        send_command_to_writer(CommandToWriter::SetColor(self.foreground, self.background));
        print!("{}", self.string);
    }
    pub fn new(foreground: Color, background: Color, string: &'a str) -> Self {
        MessageToVga {
            foreground,
            background,
            string,
        }
    }
}
