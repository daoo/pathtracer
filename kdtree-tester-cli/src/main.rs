use clap::{Parser, Subcommand};
use kdtree::sah::SahCost;
use ray_tester::kdtree_ray_tester;
use reducer::kdtree_reduce;
use size::Size;

mod checked_intersection;
mod ray_bouncer;
mod ray_tester;
mod reducer;
mod size;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compare kdtree intersection with naive intersection
    Test {
        /// Wavefront OBJ input path
        #[arg(short = 'i', long, required = true)]
        input: std::path::PathBuf,

        /// Output ray fail binary data path
        #[arg(short = 'o', long)]
        output: Option<std::path::PathBuf>,
        /// Image size in pixels
        #[arg(short, long, default_value_t = Size::new(512, 512))]
        size: Size,
        /// Max number of bounces to test
        #[arg(short, long, default_value_t = 10)]
        bounces: u32,

        /// SAH kd-tree traverse cost
        #[arg(long, default_value_t = SahCost::default().traverse_cost)]
        traverse_cost: f32,
        /// SAH kd-tree intersect cost
        #[arg(long, default_value_t = SahCost::default().intersect_cost)]
        intersect_cost: f32,
        /// SAH kd-tree empty factor
        #[arg(long, default_value_t = SahCost::default().empty_factor)]
        empty_factor: f32,
    },
    /// Reduce tree size for a specific intersection error
    Reduce {
        /// Wavefront OBJ input path
        #[arg(short = 'i', long, required = true)]
        input: std::path::PathBuf,

        /// Output reduced kd-tree JSON data path
        #[arg(short = 'o', long, required = true)]
        output: std::path::PathBuf,

        /// Output ray fail binary data path
        #[arg(short = 'f', long)]
        fail: Option<std::path::PathBuf>,

        /// Seed for random generator used to shuffle input geometry
        #[arg(short = 's', long, required = true)]
        seed: u64,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Test {
            input,
            output,
            size,
            bounces,
            traverse_cost,
            intersect_cost,
            empty_factor,
        } => kdtree_ray_tester(
            input,
            output,
            size,
            bounces,
            SahCost {
                traverse_cost,
                intersect_cost,
                empty_factor,
            },
        ),
        Commands::Reduce {
            input,
            output,
            fail,
            seed,
        } => kdtree_reduce(input, output, fail, seed),
    }
}
