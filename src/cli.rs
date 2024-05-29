use clap::{Args, Parser, Subcommand};

/// `whamm` instruments a Wasm application with the Probes defined in the specified Script.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct WhammCli {
    // #[clap(flatten)]
    // global_opts: GlobalOpts,

    #[command(subcommand)]
    pub(crate) command: Cmd
}

#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    // /// Generate shell completion
    // Completion {
    //     /// Shell to generate completion for
    //     #[arg(arg_enum)]
    //     shell: Shell,
    // },
    /// To provide the globals and functions available for the given probe specification.
    /// To use this option, simply follow the command with a full or partial specification
    /// (use pattern matching to see what would be triggered).
    Info {
        #[arg(short, long, value_parser)]
        spec: String,

        /// Show the globals in-scope when using the probe specification.
        #[arg(long, short, action, default_value = "false")]
        globals: bool,

        /// Show the functions in-scope when using the probe specification.
        #[arg(long, short, action, default_value = "false")]
        functions: bool,
    },

    /// To instrument a Wasm application.
    Instr(InstrArgs),

    /// To visualize the relationship between various structures in the module and its instructions
    VisWasm {
        /// The path to the Wasm module we want to visualize.
        #[clap(short, long, value_parser)]
        wasm: String,

        /// The path to output the visualization to.
        #[clap(short, long, value_parser, default_value = "output/wasm.dot")]
        output_path: String,
    },

    /// To visualize the generated behavior tree from the specified `script`
    VisScript {
        /// The path to the `script` file we want to visualize.
        #[clap(short, long, value_parser)]
        script: String,

        /// Whether to run the verifier on the specified script
        #[clap(long, short, action, default_value = "false")] // TODO -- change this default value to true when I have this implemented
        run_verifier: bool,

        /// The path to output the visualization to.
        #[clap(short, long, value_parser, default_value = "output/vis.svg")]
        output_path: String,
    }
}

// #[derive(Debug, Args)]
// struct GlobalOpts {
//     // (not needed yet)
// }

#[derive(Debug, Args)]
pub struct InstrArgs {
    /// The path to the application's Wasm module we want to instrument.
    #[arg(short, long, value_parser)]
    pub app: String,
    /// The path to the Script containing the instrumentation Probe definitions.
    #[arg(short, long, value_parser)]
    pub script: String,
    /// The path that the instrumented version of the Wasm app should be output to.
    #[arg(short, long, value_parser, default_value = "./output/output.wasm")]
    pub output_path: String,

    /// Whether to emit Virgil code as the instrumentation code
    #[arg(short, long, action, default_value = "false")]
    pub virgil: bool,

    /// Whether to run the verifier on the specified script
    #[arg(long, short, action, default_value = "false")] // TODO -- change this default value to true when I have this implemented
    pub run_verifier: bool
}

// pub fn print_completion<G: Generator>(gen: G, app: &mut App) {
//     generate(gen, app, app.get_name().to_string(), &mut io::stdout());
// }