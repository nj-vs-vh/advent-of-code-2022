use clap::ValueEnum;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RunPart {
    Pt1,
    Pt2,
    Both,
}
