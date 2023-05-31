use axum::{
    extract::{Extension, Json},
};

use crate::web::*;
use crate::web::dtos::ThermometerWireDto;

pub struct ThermometerController {
}

impl ThermometerController {
    pub async fn get_thermometers(Extension(context): Extension<WebContext>) -> Json<ThermometerWireDto> {
        let thermometer_service = context.thermometer_service.lock().await;

        let thermometer_wire = thermometer_service.get_thermometer_wire();

        let thermometer_dto = ThermometerWireDto::new(thermometer_wire);

        Json(thermometer_dto)
    }

}