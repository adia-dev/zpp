use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Zli (the zpp cli)")]
#[command(author = "Abdoulaye D. <adia.dev31@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Zli is a command-line interface for the Zpp interpreter.")]
pub struct Cli {
    #[arg(short, long)]
    command: Option<String>
}

// info: Usage: zig [command] [options]
//
// Commands:
//
//   build            Build project from build.zig
//   init-exe         Initialize a `zig build` application in the cwd
//   init-lib         Initialize a `zig build` library in the cwd
//
//   ast-check        Look for simple compile errors in any set of files
//   build-exe        Create executable from source or object files
//   build-lib        Create library from source or object files
//   build-obj        Create object from source or object files
//   fmt              Reformat Zig source into canonical form
//   run              Create executable and run immediately
//   test             Create and run a test build
//   translate-c      Convert C code to Zig code
//
//   ar               Use Zig as a drop-in archiver
//   cc               Use Zig as a drop-in C compiler
//   c++              Use Zig as a drop-in C++ compiler
//   dlltool          Use Zig as a drop-in dlltool.exe
//   lib              Use Zig as a drop-in lib.exe
//   ranlib           Use Zig as a drop-in ranlib
//
//   env              Print lib path, std path, cache directory, and version
//   help             Print this help and exit
//   libc             Display native libc paths file or validate one
//   targets          List available compilation targets
//   version          Print version number and exit
//   zen              Print Zen of Zig and exit
//
// General Options:
//
//   -h, --help       Print command-specific usage


