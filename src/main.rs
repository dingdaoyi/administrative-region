use std::io;
use std::io::Write;
use sqlx::{Pool, Postgres, Sqlite};
use structopt::StructOpt;
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
    if let Some(code) = code {
        transform_data(code,&sqlite_pool,&pg_pool).await?;
    }
    Ok(())
}

pub async fn select_code(sqlite_pool:&Pool<Sqlite>,cli: Cli)->Result<Option<String>,ServerError> {
    if let Some(code) = cli.code {
        return Ok(Some(code));
    }
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
    if  cli.list {
        provinces.iter().for_each(|item| {
            println!("省份:{},code:{}",item.name,item.code)
        });
    }
    Ok(None)
}
async fn transform_data(code:String,sqlite_pool:&Pool<Sqlite>, pg_pool:&Pool<Postgres>) -> Result<(), ServerError> {
    // 指定节点查询,
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