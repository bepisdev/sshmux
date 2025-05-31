use colored::*;

pub fn get_colored_prefix(host: &str, index: usize) -> ColoredString {
    let colors = ["red", "green", "yellow", "blue", "magenta", "cyan", "white"];
    let color = colors[index % colors.len()];
    format!("[{}]", host).color(color)
}
