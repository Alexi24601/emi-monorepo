use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
use crate::codegen::diesel_codegen::tables::directus_flows::directus_flows;
diesel::allow_tables_to_appear_in_same_query!(directus_webhooks, directus_flows);
