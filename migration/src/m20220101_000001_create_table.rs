use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(integer("uid").primary_key().auto_increment())
                    .col(string("username").not_null().unique_key())
                    .col(string("psd").not_null())
                    .col(string("spool_key").not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("nodes")
                    .if_not_exists()
                    .col(integer("node_id").primary_key().auto_increment())
                    .col(string("node_name").not_null())
                    .col(string("ip_addr").not_null())
                    .col(string("region").not_null())
                    .col(string("status").not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("files")
                    .if_not_exists()
                    .col(integer("id").primary_key().auto_increment())
                    .col(string("orig_filename").not_null())
                    .col(string("uploaded_filename").not_null())
                    .col(integer("file_size").not_null())
                    .col(string("mime_type").not_null())
                    .col(timestamp("timestamp").not_null())
                    .col(integer("uploaded_by").not_null())
                    .col(integer("assigned_node").not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-files-uploaded_by")
                            .from("files", "uploaded_by")
                            .to("users", "uid"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("u_settings")
                    .if_not_exists()
                    .col(integer("id").primary_key().auto_increment())
                    .col(integer("uid").not_null())
                    .col(integer("cache_ttl").not_null())
                    .col(string("allowed_filetypes").not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-u_settings-uid")
                            .from("u_settings", "uid")
                            .to("users", "uid"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("u_settings").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("files").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("nodes").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table("users").to_owned())
            .await
    }
}
