#[macro_export]
macro_rules! print_with_colors {
    ( $( $x:expr ),* ) => {
        {
            $(
                $x.print_to_vga();
            )*
        }
        popcorn::low_level::vga_buffer::set_color(Color::White, Color::Black);
    };
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::low_level::vga_buffer::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
