/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-10-16 09:04:46
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-10-16 09:08:25
 * @FilePath: /luqiyuan/src/migrator/mod.rs
 * @Description:  数据库表接口
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
pub use sea_orm_migration::prelude::*;

mod m20241016_010815_country_table;
mod m20241016_022727_country_real_interest_rate_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241016_010815_country_table::Migration),
            Box::new(m20241016_022727_country_real_interest_rate_table::Migration),
        ]
    }
}
