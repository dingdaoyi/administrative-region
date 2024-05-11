use std::io;
use std::io::Write;
use sqlx::{Pool, Postgres, Sqlite};
use structopt::StructOpt;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tracing::info;
pub use common::ServerError;

use crate::config::init_data_pool;
use crate::models::{CodeType, Province};
use crate::models::arg_cli::Cli;

pub mod models;
pub mod config;
pub mod common;
pub mod utils;

#[tokio::main]
 async fn main() ->Result<(),ServerError>{
    dotenv::dotenv().ok();
    setup_logger().await?;
    let (sqlite_pool, pg_pool) = init_data_pool().await?;
    let cli = Cli::from_args();
    let code= select_code(&sqlite_pool,cli).await?;
    info!("code is : {:?}",code);
    // transform_data(&sqlite_pool,&pg_pool).await?;
    Ok(())
}

pub async fn select_code(sqlite_pool:&Pool<Sqlite>,cli: Cli)->Result<Option<String>,ServerError> {
    if let Some(code) = cli.code {
        return Ok(Some(code));
    }
    // 创建一个标准输入的句柄
    let stdin = io::stdin();
    // 创建一个标准输出的句柄
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // 初始化选择索引
    let mut selected_index = 0;

   let provinces:Vec<Province>= Province::all_list(sqlite_pool)
        .await?;

    if let Some(province) = cli.province {
        let res = provinces.iter()
            .find(|item| {
                item.name.eq(&province)
            })
            .map(|item|item.code.clone());
        return Ok(res);
    }
    // 输出选项列表
    print_options(&mut stdout, &provinces, selected_index);
    let mut res = None;
    // 循环读取输入事件
    for c in stdin.keys() {
        match c.unwrap() {
            // 上箭头键
            Key::Up => {
                if selected_index > 0 {
                    selected_index -= 1;
                }
            }
            // 下箭头键
            Key::Down => {
                if selected_index < provinces.len() - 1 {
                    selected_index += 1;
                }
            }
            // 回车键
            Key::Char('\n') => {
                // 输出选中的选项
                println!("Selected: {}", provinces[selected_index].name);
                res = Some(provinces[selected_index].code.clone());
                break;
            }
            _ => {}
        }

        // 清空当前行并重新输出选项列表
        print_options(&mut stdout, &provinces, selected_index);
    }
    Ok(res)
}
// 输出选项列表函数
fn print_options(stdout: &mut io::Stdout, options: &Vec<Province>, selected_index: usize) {
    // 清空当前行
    write!(stdout, "{}{}", termion::clear::CurrentLine, termion::cursor::Goto(1, 1)).unwrap();
    // 输出选项列表
    for (index, option) in options.iter().enumerate() {
        if index == selected_index {
            // 如果是选中的选项，则加上前缀 "* "
            write!(stdout, "* {}\r\n", option.name).unwrap();
        } else {
            write!(stdout, "  {}\r\n", option.name).unwrap();
        }
    }
    // 刷新标准输出
    stdout.flush().unwrap();
}
async fn transform_data(sqlite_pool:&Pool<Sqlite>, pg_pool:&Pool<Postgres>) -> Result<(), ServerError> {

    // 指定节点查询,
    let code = "510000000000".to_string();
    // 全部节点查询添加或修改
    let res = CodeType::parse_type(&code);
    res.load_children(sqlite_pool, pg_pool)
        .await?;
    Ok(())
}

async fn setup_logger() -> Result<(), ServerError> {
    // 设置用户缓存
    tracing_subscriber::fmt::init();
    Ok(())
}