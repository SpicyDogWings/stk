use clap::{ Parser, ValueEnum };
use anstyle::{ AnsiColor, Color, Effects, Style};

#[derive(Clone, Debug, ValueEnum)]
enum StkColor {
    Red,
    Green,
    Blue,
    Cyan,
    Black,
    Yellow,
    Magenta,
    White,
    BrightBlue,
    BrightGreen,
    BrightRed,
    BrightCyan,
    BrightBlack,
    BrightYellow,
    BrightMagenta,
    BrightWhite
}

impl From<StkColor> for Color {
    fn from(c: StkColor) -> Self {
        match c {
            StkColor::Blue => AnsiColor::Blue.into(),
            StkColor::Red => AnsiColor::Red.into(),
            StkColor::Green => AnsiColor::Green.into(),
            StkColor::Cyan => AnsiColor::Cyan.into(),
            StkColor::Black => AnsiColor::Black.into(),
            StkColor::Yellow => AnsiColor::Yellow.into(),
            StkColor::Magenta => AnsiColor::Magenta.into(),
            StkColor::White => AnsiColor::White.into(),
            StkColor::BrightBlue => AnsiColor::BrightBlue.into(),
            StkColor::BrightRed => AnsiColor::BrightRed.into(),
            StkColor::BrightGreen => AnsiColor::BrightGreen.into(),
            StkColor::BrightCyan => AnsiColor::BrightCyan.into(),
            StkColor::BrightBlack => AnsiColor::BrightBlack.into(),
            StkColor::BrightYellow => AnsiColor::BrightYellow.into(),
            StkColor::BrightMagenta => AnsiColor::BrightMagenta.into(),
            StkColor::BrightWhite => AnsiColor::BrightWhite.into(),
        }

    }
    
}

#[derive(Parser, Debug)]
#[command(name = "stk", version, about = "Pinta texto en la terminal")]
struct Args {
    #[arg(required = true)]
    text: Vec<String>,

    #[arg(short, long, value_enum, default_value_t = StkColor::Green)]
    foreground: StkColor,
}

fn main() {
    let args = Args::parse();
    let style = Style::new()
        .fg_color(Some(args.foreground.into()))
        .effects(Effects::BOLD);
    print!("{style}{}{style:#}", args.text.join(" "));
}
