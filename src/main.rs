mod color;

use clap::{ Parser, Subcommand, Args as ClapArgs };
use anstyle::{ Color, Effects, Style};
use color::{ StkColor, ColorVariant };


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
