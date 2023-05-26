use axum::{
    extract::{Extension, Path, Json},
    http::StatusCode,
};

use crate::web::*;
use crate::web::dtos::ThermometerDto;
use crate::web::inputs::ThermometerInput;

pub struct ThermometerController {
}

impl ThermometerController {
    pub async fn get_thermometers(Extension(context): Extension<WebContext>) -> Json<Vec<ThermometerDto>> {
        let thermometer_service = context.thermometer_service.lock().await;

        let thermometers = thermometer_service.get_all();

        let thermometer_dtos = thermometers.iter()
            .map(|thermometer| ThermometerDto::new(thermometer))
            .collect();

        Json(thermometer_dtos)
    }

    pub async fn get_thermometer(Path(id): Path<u32>, Extension(context): Extension<WebContext>) -> Result<Json<ThermometerDto>, StatusCode> {
        let thermometer_service = context.thermometer_service.lock().await;

        if let Some(thermometer) = thermometer_service.get_by_id(id) {
            let thermometer_dto = ThermometerDto::new(thermometer);

            Ok(Json(thermometer_dto))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }


    pub async fn create_thermometer(Extension(context): Extension<WebContext>, Json(req): Json<ThermometerInput>) -> Result<Json<ThermometerDto>, StatusCode> {
        let mut thermometer_service = context.thermometer_service.lock().await;

        if let Some(thermometer) = thermometer_service.add(&req.into()) {
            let thermometer_dto = ThermometerDto::new(thermometer);

            Ok(Json(thermometer_dto))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }
}