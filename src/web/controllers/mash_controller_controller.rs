use axum::{
    extract::{Extension, Json},
};

use crate::web::*;
use crate::web::dtos::MashControllerDto;

pub struct MashControllerController {
}

impl MashControllerController {
    pub async fn get_mash_controller(Extension(context): Extension<WebContext>) -> Json<MashControllerDto> {
        let mash_controller_service = context.mash_controller_service.lock().await;

        let mash_controller = mash_controller_service.get_mash_controller();

        let mash_controller_dto = MashControllerDto::new(mash_controller);

        Json(mash_controller_dto)
    }

}