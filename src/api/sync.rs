use crate::api::build_api_error;
use crate::db::Database;
use crate::models::{ApiResponse, AuthUser, Item, Sync, SyncResponse};
use diesel::prelude::*;
use rocket_contrib::json::Json;

#[post("/sync", data = "<sync>")]
#[allow(unused_variables)]
pub fn sync(_user: AuthUser, conn: Database, sync: Json<Sync>) -> ApiResponse<Json<SyncResponse>> {
    use crate::schema::items::dsl::items;

    let sync = sync.into_inner();
    match diesel::insert_into(items)
        .values(&sync.items)
        .get_result::<Item>(&*conn)
    {
        Ok(item) => {
            let hi = "wat";

            // these items are new or have been modified since last sync and should be merged or created locally.

            Ok(Json(SyncResponse {
                saved_items: Some(sync.items),
                retrieved_items: None,
                unsaved: None,
                sync_token: None,
            }))
        }
        Err(_) => Err(build_api_error("Error syncing item")),
    }
}
