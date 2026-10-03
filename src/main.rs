mod color;
mod ui;

use clap::{ Parser, Subcommand, Args as ClapArgs };
use anstyle::{ Color, Style};
use color::{ StkColor, ColorVariant };
use time::{
    OffsetDateTime,
    macros::format_description
};
use ui::print_text_inline;

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
            let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
            let time_fmt = format_description!("[hour]:[minute]:[second]");
            let style = Style::new()
                .fg_color(Some(StkColor::Black.into()))
                .bg_color(Some(color.into()));
            let style2 = Style::new()
                .fg_color(Some(StkColor::White.into()));
            let style3 = Style::new()
                .fg_color(Some(StkColor::BrightBlack.into()));
            print!("{style} {} {style:#} {style3}{}{style3:#} {style2}{}{style2:#}", variant.label(), now.format(time_fmt).unwrap(), text.text.join(" "));
        }
        None => {
            print_text_inline(args.text.text, args.foreground);
        }
    }
}
