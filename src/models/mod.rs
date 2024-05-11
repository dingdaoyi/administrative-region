pub mod arg_cli;

use async_recursion::async_recursion;
use async_trait::async_trait;
use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres, query, Sqlite};
use tracing::info;

use crate::ServerError;

#[derive(Debug, FromRow, Serialize)]
pub struct Province {
    pub code: String,
    pub name: String,
}

impl Province {
    pub(crate) async fn all_list(sqlite_pool:&Pool<Sqlite>,)->Result<Vec<Province>,ServerError> {
        let res = sqlx::query_as::<_, Self>("select * from province")
            .fetch_all(sqlite_pool)
            .await?;
        Ok(res)
    }
}


#[derive(Debug, FromRow, Serialize)]
pub struct City {
    pub code: String,
    pub name: String,
    #[serde(rename = "provinceCode")]
    pub provinceCode: String,
}

impl City {
   pub async fn list_by_province_code(province_code: &String, pool: &Pool<Sqlite>) -> Result<Vec<Self>, ServerError> {
        let res = sqlx::query_as::<_, City>("select * from city where provinceCode =?")
            .bind(province_code)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
}


#[derive(Debug, FromRow, Serialize)]
pub struct Area {
    pub code: String,
    pub name: String,
    #[serde(rename = "cityCode")]
    pub cityCode: String,
    #[serde(rename = "provinceCode")]
    pub provinceCode: String,
}

impl Area {
    pub async fn list_by_city_code(city_code: &String, pool: &Pool<Sqlite>) -> Result<Vec<Self>, ServerError> {
        let res = sqlx::query_as::<_, Area>("select * from area where cityCode =?")
            .bind(&city_code)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct Street {
    pub code: String,
    pub name: String,
    #[serde(rename = "cityCode")]
    pub cityCode: String,
    #[serde(rename = "provinceCode")]
    pub provinceCode: String,
    #[serde(rename = "areaCode")]
    pub areaCode: String,
}

impl Street {
    pub async fn list_by_city_code(area_code: &String, pool: &Pool<Sqlite>) -> Result<Vec<Self>, ServerError> {
        let res = sqlx::query_as::<_, Street>("select * from street where areaCode =?")
            .bind(&area_code)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct Village {
    pub code: String,
    pub name: String,
    #[serde(rename = "cityCode")]
    pub cityCode: String,
    #[serde(rename = "provinceCode")]
    pub provinceCode: String,
    #[serde(rename = "areaCode")]
    pub areaCode: String,
    #[serde(rename = "streetCode")]
    pub streetCode: String,
}

impl Village {
    pub async fn list_by_street_code(street_code: &String, pool: &Pool<Sqlite>) -> Result<Vec<Self>, ServerError> {
        let res = sqlx::query_as::<_, Village>("select * from village where streetCode =?")
            .bind(street_code)
            .fetch_all(pool)
            .await?;
        Ok(res)
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct AdministrativeRegion {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub parent_code: Option<String>,
    pub level: i32,
    pub leaf: bool,
}

impl From<Province> for AdministrativeRegion {
    fn from(value: Province) -> Self {
        let mut code = value.code;
        code = zero_padding(code);
        let id: i64 = code.parse().unwrap();
        Self {
            id,
            code,
            name: value.name,
            parent_code: None,
            level: 1,
            leaf: false,
        }
    }
}

impl From<City> for AdministrativeRegion {
    fn from(value: City) -> Self {
        let mut code = value.code;
        code = zero_padding(code);
        let mut parent_code = value.provinceCode;
        parent_code = zero_padding(parent_code);
        let id: i64 = code.parse().unwrap();
        Self {
            id,
            code,
            name: value.name,
            parent_code: Some(parent_code),
            level: 2,
            leaf: false,
        }
    }
}

impl From<Area> for AdministrativeRegion {
    fn from(value: Area) -> Self {
        let mut code = value.code;
        code = zero_padding(code);
        let mut parent_code = value.cityCode;
        parent_code = zero_padding(parent_code);
        let id: i64 = code.parse().unwrap();
        Self {
            id,
            code,
            name: value.name,
            parent_code: Some(parent_code),
            level: 3,
            leaf: false,
        }
    }
}


impl From<Street> for AdministrativeRegion {
    fn from(value: Street) -> Self {
        let mut code = value.code;
        code = zero_padding(code);
        let mut parent_code = value.areaCode;
        parent_code = zero_padding(parent_code);
        let id: i64 = code.parse().unwrap();
        Self {
            id,
            code,
            name: value.name,
            parent_code: Some(parent_code),
            level: 4,
            leaf: false,
        }
    }
}

impl From<Village> for AdministrativeRegion {
    fn from(value: Village) -> Self {
        let mut code = value.code;
        code = zero_padding(code);
        let mut parent_code = value.streetCode;
        parent_code = zero_padding(parent_code);
        let id = code.parse().unwrap();
        Self {
            id,
            code,
            name: value.name,
            parent_code: Some(parent_code),
            level: 5,
            leaf: true,
        }
    }
}

pub trait ToAdministrativeRegion {
    fn to_administrative_region(&self) -> AdministrativeRegion;
}

impl ToAdministrativeRegion for Province {
    fn to_administrative_region(&self) -> AdministrativeRegion {
        let mut code = self.code.clone();
        code = zero_padding(code);
        let id: i64 = code.parse().unwrap();
        AdministrativeRegion {
            id,
            code,
            name: self.name.clone(),
            parent_code: None,
            level: 1,
            leaf: false,
        }
    }
}

impl ToAdministrativeRegion for Village {
    fn to_administrative_region(&self) -> AdministrativeRegion {
        let mut code = self.code.clone();
        code = zero_padding(code);
        let mut parent_code = self.streetCode.clone();
        parent_code = zero_padding(parent_code);
        let id = code.parse().unwrap();
        AdministrativeRegion {
            id,
            code,
            name: self.name.clone(),
            parent_code: Some(parent_code),
            level: 5,
            leaf: true,
        }
    }
}

impl ToAdministrativeRegion for City {
    fn to_administrative_region(&self) -> AdministrativeRegion {
        let mut code = self.code.clone();
        code = zero_padding(code);
        let mut parent_code = self.provinceCode.clone();
        parent_code = zero_padding(parent_code);
        let id = code.parse().unwrap();
        AdministrativeRegion {
            id,
            code,
            name: self.name.clone(),
            parent_code: Some(parent_code),
            level: 2,
            leaf: false,
        }
    }
}

impl ToAdministrativeRegion for Area {
    fn to_administrative_region(&self) -> AdministrativeRegion {
        let mut code = self.code.clone();
        code = zero_padding(code);
        let mut parent_code = self.cityCode.clone();
        parent_code = zero_padding(parent_code);
        let id = code.parse().unwrap();
        AdministrativeRegion {
            id,
            code,
            name: self.name.clone(),
            parent_code: Some(parent_code),
            level: 3,
            leaf: false,
        }
    }
}

impl ToAdministrativeRegion for Street {
    fn to_administrative_region(&self) -> AdministrativeRegion {
        let mut code = self.code.clone();
        code = zero_padding(code);
        let mut parent_code = self.areaCode.clone();
        parent_code = zero_padding(parent_code);
        let id: i64 = code.parse().unwrap();
        AdministrativeRegion {
            id,
            code,
            name: self.name.clone(),
            parent_code: Some(parent_code),
            level: 4,
            leaf: true,
        }
    }
}

fn zero_padding(code: String) -> String {
    let mut code = code;
    let len = code.len();
    if len < 12 {
        let zero_padding = "0".repeat(12 - len);
        code.push_str(&zero_padding);
    }
    code
}
fn zero_padding_length(code: String,length:usize) -> String {
    let mut code = code;
    let len = code.len();
    if len < length {
        let zero_padding = "0".repeat(length - len);
        code.push_str(&zero_padding);
    }
    code
}

#[derive(Debug, Clone)]
pub enum CodeType {
    PROVINCE(String),
    CITY(String),
    AREA(String),
    STREET(String),
    VILLAGE(String),
}

impl CodeType {
    pub fn parse_type(code: &String) -> Self {
        let trimmed_code = code.trim_end_matches('0');

        let len = trimmed_code.len();

        match len {
            10 | 11 | 12 => CodeType::VILLAGE(code.to_string()),
            // 7 位为区级别
            7 | 8 | 9 => CodeType::STREET(zero_padding_length(trimmed_code.to_string(),9)),
            // 4 位为市级别
            5 | 6 => CodeType::AREA(zero_padding_length(trimmed_code.to_string(),6)),
            // 2 位为省级别
            3 | 4 => CodeType::CITY(zero_padding_length(trimmed_code.to_string(),4)),
            // 其他情况，可能是不合法的区划 code
            _ => CodeType::PROVINCE(zero_padding_length(trimmed_code.to_string(),2)),
        }
    }

    #[async_recursion]
    pub async fn load_children(&self, sqlite_pool: &Pool<Sqlite>,pg_pool:&Pool<Postgres>) ->Result<(),ServerError>{
        match self {
            CodeType::PROVINCE(code) => {
                let res=  City::list_by_province_code(&code,&sqlite_pool).await?;
                AdministrativeRegion::save_batch_postgres(&res,&pg_pool)
                    .await?;
                for City{code, .. } in res {
                   let code_type= CodeType::parse_type(&code);
                    code_type.load_children(&sqlite_pool,&pg_pool)
                        .await?;
                }
            }
            CodeType::CITY(code) => {
                let res=  Area::list_by_city_code(&code, &sqlite_pool).await?;
                AdministrativeRegion::save_batch_postgres(&res,&pg_pool)
                    .await?;
                for Area{code, .. } in res {
                    let code_type= CodeType::parse_type(&code);
                    code_type.load_children(&sqlite_pool,&pg_pool)
                        .await?;
                }
            }
            CodeType::AREA(code) => {
                let res= Street::list_by_city_code(&code,&sqlite_pool).await?;
                AdministrativeRegion::save_batch_postgres(&res,&pg_pool)
                    .await?;
                for Street{code, .. } in res {
                    let code_type= CodeType::parse_type(&code);
                    code_type.load_children(&sqlite_pool,&pg_pool)
                        .await?;
                }
            }
            CodeType::STREET(code) => {
                let res= Village::list_by_street_code(&code, &sqlite_pool).await?;
                AdministrativeRegion::save_batch_postgres(&res,&pg_pool)
                    .await?;
            }
            CodeType::VILLAGE(code) => {}
        }
        Ok(())
    }
}

impl AdministrativeRegion {

    #[async_recursion]
    pub async fn save_data_to_postgres(&self, pg_pool:&Pool<Postgres>) ->Result<(),ServerError>{
        let exists = sqlx::query_as::<_, AdministrativeRegion>("SELECT * FROM tb_administrative_region WHERE code = $1")
            .bind(&self.code)
            .fetch_optional(pg_pool)
            .await?;
        if exists.is_none(){
         let res=  sqlx::query("insert into tb_administrative_region (id,code,name,parent_code,level,leaf) values ($1,$2,$3,$4,$5,$6)")
             .bind(&self.id)
             .bind(&self.code)
             .bind(&self.name)
             .bind(&self.parent_code)
             .bind(&self.level)
             .bind(&self.leaf)
             .execute(pg_pool)
             .await?;
            if res.rows_affected()>0 {
                info!("写入成功:{}",&self.id)
            }
        }else {
            info!("已存在:{}",&self.id)
        }
        Ok(())
    }
    pub async fn save_batch_postgres<T: ToAdministrativeRegion>(data: &Vec<T>, pg_pool:&Pool<Postgres>) ->Result<(),ServerError> {
        for item in data {
            let administrative_region = item.to_administrative_region();
            administrative_region.save_data_to_postgres(&pg_pool).await?;
        }
        Ok(())
    }
    }

