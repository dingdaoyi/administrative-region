use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "数据中台同步工具")]
pub struct Cli {
    #[structopt(help = "指定省导入", short = "P" ,long = "province")]
    pub province: Option<String>,
    #[structopt(help = "指定行政区划code导入", short = "C" ,long = "code" )]
    pub code: Option<String>,
    #[structopt(help = "选择省导入", short = "S" ,long = "select" )]
    pub select: bool,
}
