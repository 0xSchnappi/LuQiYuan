/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-10-16 10:27:27
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-10-16 10:46:54
 * @FilePath: /luqiyuan/src/migrator/m20241016_022727_country_real_interest_rate_table.rs
 * @Description: 实际利率
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use super::m20241016_010815_country_table::Country;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CountryRealInterestRate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CountryRealInterestRate::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CountryRealInterestRate::CountryId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-CountryRealInterestRate-CountryId")
                            .from(
                                CountryRealInterestRate::Table,
                                CountryRealInterestRate::CountryId,
                            )
                            .to(Country::Table, Country::Id),
                    )
                    .col(
                        ColumnDef::new(CountryRealInterestRate::Datetime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CountryRealInterestRate::RealInterestRate)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CountryRealInterestRate::NominalInterestRate)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CountryRealInterestRate::InflationRate)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CountryRealInterestRate::M2GrouthRate)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CountryRealInterestRate::GrowDomesticProduct)
                            .float()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CountryRealInterestRate::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum CountryRealInterestRate {
    Table,
    Id,
    CountryId,
    Datetime,
    RealInterestRate,    // 实际利率
    NominalInterestRate, // 名义利率
    InflationRate,       // CPI
    M2GrouthRate,        // M2增速
    GrowDomesticProduct, // GDP
}
