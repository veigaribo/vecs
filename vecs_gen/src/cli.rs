use clap::Parser;

/// ECS component and main loop generator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
  /// Path to Vecs source file
  pub source: String,

  /// Path of generated C file
  #[arg(short = 'C', long = "c-out", default_value = "vecs.c")]
  pub c_output: String,

  /// Path of generated C header file
  #[arg(short = 'H', long = "h-out", default_value = "vecs.h")]
  pub h_output: String,
}
