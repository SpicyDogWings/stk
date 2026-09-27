use clap::{ Parser, ValueEnum, Subcommand, Args as ClapArgs };
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

#[derive(Clone, Debug, ValueEnum)]
enum ColorVariant {
   Log,
   Error,
   Warning,
   Success,
   Debug
}

impl From<ColorVariant> for StkColor {
    fn from(c: ColorVariant) -> Self {
        match c {
            ColorVariant::Log => StkColor::Blue.into(),
            ColorVariant::Error => StkColor::Red.into(),
            ColorVariant::Warning => StkColor::Yellow.into(),
            ColorVariant::Success => StkColor::Green.into(),
            ColorVariant::Debug => StkColor::BrightBlack.into()
        }
    }
}

impl ColorVariant {
    fn label(&self) -> &'static str {
        match self {
            ColorVariant::Log => "LOG",
            ColorVariant::Error => "ERR",
            ColorVariant::Warning => "WARN",
            ColorVariant::Success => "SUCC",
            ColorVariant::Debug => "DEBG"
        }
        
    }
}

#[derive(ClapArgs, Debug)]
struct TextArgs {
    text: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Log {
        #[command(flatten)]
        text: TextArgs,

        #[arg(short, long, value_enum, default_value_t = ColorVariant::Log)]
        variant: ColorVariant
    }
}

#[derive(Parser, Debug)]
#[command(name = "stk", version, about = "Kit de herramientas para shell scripts")]
struct Args {
    #[command(flatten)]
    text: TextArgs,

    #[arg(short, long, value_enum, default_value_t = StkColor::Green)]
    foreground: StkColor,

    #[command(subcommand)]
    command: Option<Commands>
}


fn main() {
    let args = Args::parse();
    match args.command {
        Some(Commands::Log{ text, variant }) => {
            let color: StkColor = variant.clone().into();
            let color: Color = color.into();
            let style = Style::new()  
                .fg_color(Some(StkColor::Black.into()))
                .bg_color(Some(color.into()));
            let style2 = Style::new()
                .fg_color(Some(args.foreground.into()));
            print!("{style} {} {style:#} {style2}{}{style2:#}", variant.label(), text.text.join(" "));
        }
        None => {
            let style = Style::new()
                .fg_color(Some(args.foreground.into()))
                .effects(Effects::BOLD);
            print!("{style}{}{style:#}", args.text.text.join(" "));
        }
    }
}
