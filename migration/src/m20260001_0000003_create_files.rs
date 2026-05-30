use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Files::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Files::OgFilename).string().not_null())
                    .col(
                        ColumnDef::new(Files::Filename)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Files::Mimetype).string().not_null())
                    .col(ColumnDef::new(Files::Filesize).big_integer().not_null())
                    .col(ColumnDef::new(Files::UploadedBy).big_integer().not_null())
                    .col(
                        ColumnDef::new(Files::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Files::Table, Files::UploadedBy)
                            .to(ApiKeys::Table, ApiKeys::Id)
                            .on_delete(ForeignKeyAction::Restrict), // don't delete files if key is deleted
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Files::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Files {
    Table,
    Id,
    OgFilename,
    Filename,
    Mimetype,
    Filesize,
    UploadedBy,
    CreatedAt,
}

#[derive(Iden)]
enum ApiKeys {
    Table,
    Id,
}
