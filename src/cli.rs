use clap::{Args, Parser, Subcommand};

/// `whamm` instruments a Wasm application with the Probes defined in the specified Script.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct WhammCli {
    // #[clap(flatten)]
    // global_opts: GlobalOpts,
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    // /// Generate completion for shell
    // Completion {
    //     /// Shell to generate completion for
    //     #[arg(arg_enum)]
    //     shell: Shell,
    // },
    /// To provide the globals and functions available for the given probe match rule.
    /// To use this option, simply follow the command with a full or partial match rule
    /// (use pattern matching to see what would be triggered).
    Info {
        #[arg(short, long, value_parser)]
        rule: String,

        /// Show the globals in-scope when using the probe match rule.
        #[arg(long, short, action, default_value = "false")]
        globals: bool,

        /// Show the functions in-scope when using the probe match rule.
        #[arg(long, short, action, default_value = "false")]
        functions: bool,
    },

    /// To run a `wast` test.
    Wast {
        /// The path to the `wast` file to run.
        wast_path: String,
    },

    /// To instrument a Wasm application.
    Instr(InstrArgs),
}

// #[derive(Debug, Args)]
// struct GlobalOpts {
//     // (not needed yet)
// }

#[derive(Debug, Args)]
pub struct InstrArgs {
    /// The path to the application's Wasm module we want to instrument.
    #[arg(short, long, value_parser)]
    pub app: Option<String>,
    /// The path to the Script containing the instrumentation Probe definitions.
    #[arg(short, long, value_parser)]
    pub script: String,
    /// The path to the core Whamm library Wasm module.
    #[arg(short, long, value_parser)]
    pub core_lib: Option<String>,
    /// To configure user-provided libraries. These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub user_libs: Option<Vec<String>>,
    /// The path that the instrumented version of the Wasm app should be output to.
    #[arg(short, long, value_parser, default_value = "./output/output.wasm")]
    pub output_path: String,

    /// Whether to emit `mon.wasm` for instrumenting with Wizard Engine
    #[arg(short, long, action, default_value = "false")]
    pub wizard: bool,

    /// Emits metrics on time to perform certain tasks if set.
    #[arg(short, long, action, default_value = "false")]
    pub metrics: bool,
    /// Override on bundling arguments for probe bodies (can only use if no-body is true!!), does not emit if set (for gathering overhead metrics)
    #[arg(long, action, default_value = "false")]
    pub no_bundle: bool,
    /// Override on emitting probe bodies, does not emit if set (for gathering overhead metrics)
    #[arg(long, action, default_value = "false")]
    pub no_body: bool,
    /// Override on emitting dynamic probe predicates, does not emit if set (for gathering overhead metrics)
    #[arg(long, action, default_value = "false")]
    pub no_pred: bool,
    /// (only rewriting) Override on flushing report data, does not flush if set (for gathering overhead metrics)
    #[arg(long, action, default_value = "false")]
    pub no_report: bool,

    /// Whether to emit extra exported functions that are helpful during testing.
    #[arg(short, long, action, default_value = "false")]
    pub testing: bool,

    /// The strategy to take when handling the injecting references to the `whamm!` core library.
    #[arg(short, long, value_parser)]
    pub link_strategy: Option<LibraryLinkStrategyArg>,
    // /// The memory offset to use when using the `offset` library strategy.
    // #[arg(short, long, value_parser)]
    // pub mem_offset: Option<u32>
}

/// Options for handling instrumentation library.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LibraryLinkStrategyArg {
    /// Merge the library with the `app.wasm` **target VM must support multi-memory**.
    /// Will create a new memory in the `app.wasm` to be targeted by the instrumentation.
    Merged,
    /// Link the library through Wasm imports into `app.wasm` (target VM must support dynamic linking).
    /// Naturally, the instrumentation memory will reside in its own module instantiation.
    Imported,
}

// pub fn print_completion<G: Generator>(gen: G, app: &mut App) {
//     generate(gen, app, app.get_name().to_string(), &mut io::stdout());
// }
