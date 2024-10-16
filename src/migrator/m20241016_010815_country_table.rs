/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-10-16 09:08:15
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-10-16 10:46:44
 * @FilePath: /luqiyuan/src/migrator/m20241016_010815_country_table.rs
 * @Description: 国家数据库表
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Country::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Country::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Country::Name).string().not_null())
                    .col(ColumnDef::new(Country::NameEn).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Country {
    Table,
    Id,
    Name,
    NameEn,
}
