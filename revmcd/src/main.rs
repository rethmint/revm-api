use clap::{Parser, Subcommand, ValueEnum};
use revm::primitives::{hex, Bytecode, Bytes, SpecId};
use revmffi::compiler::RuntimeAot;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        #[arg(long)]
        bin: String,

        #[arg(long, conflicts_with = "spec_id")]
        eof: bool,

        #[arg(long, value_enum, default_value = "osaka")]
        spec_id: SpecIdValueEnum,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { bin, eof, spec_id } => {
            let compiler = RuntimeAot::new(Default::default());

            let bytes = Bytes::from(hex::decode(bin).expect("Invalid hex input for bin"));
            let bytecode = Bytecode::new_raw_checked(bytes.clone()).expect("Invalid bytecode");
            let code_hash = bytecode.hash_slow();

            let spec_id = if eof { SpecId::OSAKA } else { spec_id.into() };

            let label = code_hash.to_string().leak();
            let res = compiler.compile(label, bytecode.original_byte_slice(), spec_id);

            println!("Compilation result: {:?}", res);
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
#[clap(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum SpecIdValueEnum {
    FRONTIER,
    FRONTIER_THAWING,
    HOMESTEAD,
    DAO_FORK,
    TANGERINE,
    SPURIOUS_DRAGON,
    BYZANTIUM,
    CONSTANTINOPLE,
    PETERSBURG,
    ISTANBUL,
    MUIR_GLACIER,
    BERLIN,
    LONDON,
    ARROW_GLACIER,
    GRAY_GLACIER,
    MERGE,
    SHANGHAI,
    CANCUN,
    PRAGUE,
    OSAKA,
    LATEST,
}

impl From<SpecIdValueEnum> for SpecId {
    fn from(v: SpecIdValueEnum) -> Self {
        match v {
            SpecIdValueEnum::FRONTIER => Self::FRONTIER,
            SpecIdValueEnum::FRONTIER_THAWING => Self::FRONTIER_THAWING,
            SpecIdValueEnum::HOMESTEAD => Self::HOMESTEAD,
            SpecIdValueEnum::DAO_FORK => Self::DAO_FORK,
            SpecIdValueEnum::TANGERINE => Self::TANGERINE,
            SpecIdValueEnum::SPURIOUS_DRAGON => Self::SPURIOUS_DRAGON,
            SpecIdValueEnum::BYZANTIUM => Self::BYZANTIUM,
            SpecIdValueEnum::CONSTANTINOPLE => Self::CONSTANTINOPLE,
            SpecIdValueEnum::PETERSBURG => Self::PETERSBURG,
            SpecIdValueEnum::ISTANBUL => Self::ISTANBUL,
            SpecIdValueEnum::MUIR_GLACIER => Self::MUIR_GLACIER,
            SpecIdValueEnum::BERLIN => Self::BERLIN,
            SpecIdValueEnum::LONDON => Self::LONDON,
            SpecIdValueEnum::ARROW_GLACIER => Self::ARROW_GLACIER,
            SpecIdValueEnum::GRAY_GLACIER => Self::GRAY_GLACIER,
            SpecIdValueEnum::MERGE => Self::MERGE,
            SpecIdValueEnum::SHANGHAI => Self::SHANGHAI,
            SpecIdValueEnum::CANCUN => Self::CANCUN,
            SpecIdValueEnum::PRAGUE => Self::PRAGUE,
            SpecIdValueEnum::OSAKA => Self::OSAKA,
            SpecIdValueEnum::LATEST => Self::LATEST,
        }
    }
}
