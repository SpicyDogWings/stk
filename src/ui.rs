use crate::color::StkColor;
use anstyle::{ Effects, Style};

pub fn print_text_inline(text: Vec<String>, color: StkColor) {
    let style = Style::new()
        .fg_color(Some(color.into()))
        .effects(Effects::BOLD);    
    print!("{style}{}{style:#}", text.join(" "));
}
