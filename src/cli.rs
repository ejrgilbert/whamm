/// `whamm` instruments a Wasm application with the Probes defined in the specified Script.
#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct WhammCli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, clap::Subcommand)]
pub enum Cmd {
    /// To provide the bound variables and functions for the given probe match rule.
    /// To use this option, simply follow the command with a full or partial match rule
    /// (use pattern matching to see what would be triggered).
    Info {
        #[arg(short, long, value_parser)]
        rule: String,

        /// The path to provider definition yaml configs.
        #[arg(short, long, value_parser)]
        defs_path: Option<String>,

        /// Show the vars in-scope when using the probe match rule.
        #[arg(long, short, action, default_value = "false")]
        vars: bool,

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

    /// To compose multiple components together.
    /// This is helpful if you've instrumented a component and now need to compose the
    /// instrumented component with the library dependencies into a single, runnable
    /// component.
    Wac(WacArgs)
}
impl std::fmt::Display for WhammCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("whamm ")?;
        match &self.command {
            Cmd::Wac(WacArgs {
                app, output_path, user_libs
            }) => {
                f.write_str("wac ")?;
                f.write_fmt(format_args!("--app {} ", app))?;
                f.write_fmt(format_args!("--output-path {} ", output_path))?;
                if !user_libs.is_empty() {
                    f.write_str("--user-libs ")?;
                    let mut is_first = true;
                    for lib in user_libs.iter() {
                        if !is_first { f.write_str(",")?; }
                        f.write_str(lib)?;
                        is_first = false
                    }
                }

                Ok(())
            }
            Cmd::Wast { .. }
            | Cmd::Instr(_)
            | Cmd::Info { .. } => Ok(())
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct InstrArgs {
    /// The path to the application's Wasm module we want to instrument.
    #[arg(short, long, value_parser)]
    pub app: Option<String>,
    /// The path to the Script containing the instrumentation Probe definitions.
    #[arg(short, long, value_parser)]
    pub script: String,
    /// The path to provider definition YAML configs.
    #[arg(short, long, value_parser)]
    pub defs_path: Option<String>,
    /// The path to the core Whamm library Wasm module.
    #[arg(short, long, value_parser)]
    pub core_lib: Option<String>,
    /// To configure user-provided libraries. These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub user_libs: Vec<String>,
    /// The path that the instrumented version of the Wasm app should be output to.
    #[arg(short, long, value_parser, default_value = "./output.wasm")]
    pub output_path: String,

    /// Whether to emit `mon.wasm` for instrumenting with the Whamm Engine Interface (wei).
    #[arg(short, long, action, default_value = "false")]
    pub wei: bool,

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

#[derive(Debug, clap::Args)]
pub struct WacArgs {
    /// The path to the application's Wasm module we want to instrument.
    #[arg(short, long, value_parser)]
    pub app: String,
    /// The path that the composed component should be output to.
    #[arg(short, long, value_parser, default_value = "./composition.wasm")]
    pub output_path: String,
    /// The component libraries to use during the composition.
    /// These should satisfy all dependencies introduced by instrumentation.
    /// These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub user_libs: Vec<String>,
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

pub const WHAMM_CORE_COMPONENT_NAME: &str = "whamm-core";
pub const DEFAULT_WHAMM_CORE_COMPONENT_PATH: &str = "whamm_core-component/target/wasm32-wasip2/release/whamm_core.wasm";
impl From<&InstrArgs> for WacArgs {
    fn from(value: &InstrArgs) -> Self {
        let mut user_libs = value.user_libs.clone();
        user_libs.insert(0, format!("{WHAMM_CORE_COMPONENT_NAME}={DEFAULT_WHAMM_CORE_COMPONENT_PATH}"));

        Self {
            app: value.output_path.clone(),
            output_path: "./composition.wasm".to_string(),
            user_libs
        }
    }
}
