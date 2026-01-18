// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "WebsiteStatus"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    Region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    Website (id) {
        id -> Text,
        url -> Text,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    WebsiteTick (id) {
        id -> Text,
        response_time -> Int4,
        status -> WebsiteStatus,
        region_id -> Text,
        website_id -> Text,
    }
}

diesel::table! {
    _prisma_migrations (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
    }
}

diesel::table! {
    region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    website (id) {
        id -> Text,
        url -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    website_tick (id) {
        id -> Text,
        response_time -> Int4,
        status -> WebsiteStatus,
        region_id -> Text,
        website_id -> Text,
    }
}

diesel::joinable!(WebsiteTick -> Region (region_id));
diesel::joinable!(WebsiteTick -> Website (website_id));
diesel::joinable!(website_tick -> region (region_id));
diesel::joinable!(website_tick -> website (website_id));

diesel::allow_tables_to_appear_in_same_query!(
    Region,
    Website,
    WebsiteTick,
    _prisma_migrations,
    region,
    website,
    website_tick,
);
