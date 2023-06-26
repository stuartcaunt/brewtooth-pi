use axum::{
    extract::{Extension, Json},
};

use crate::web::*;
use crate::web::dtos::{MashControllerDto, PIDDto};

pub struct MashControllerController {
}

impl MashControllerController {
    pub async fn get_mash_controller(Extension(context): Extension<WebContext>) -> Json<MashControllerDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();

        let mash_controller_dto = MashControllerDto::new(mash_controller);

        Json(mash_controller_dto)
    }

    pub async fn get_pids(Extension(context): Extension<WebContext>) -> Json<PIDDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();
        let pid = mash_controller.get_pid_params();

        Json(pid.into())
    }

    pub async fn set_pids(Extension(context): Extension<WebContext>, Json(input): Json<PIDDto>) -> Json<PIDDto> {
        let mash_controller_service = context.mash_controller_service;

        let mash_controller = mash_controller_service.get_mash_controller();
        let pid = mash_controller.set_pid_params(&input.into());

        Json(pid.into())
    }

}