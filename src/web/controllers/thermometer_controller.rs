use axum::{
    extract::{Extension, Path, Json},
    http::StatusCode
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::web::*;
use crate::web::dtos::ThermometerDto;

pub struct ThermometerController {
}

impl ThermometerController {
    pub async fn get_thermometers(Extension(context): Extension<Arc<Mutex<WebContext>>>) -> Json<Vec<ThermometerDto>> {
        let mut context = context.lock().await;

        let thermometer_service = &mut context.thermometer_service;
        let thermometers = thermometer_service.get_all();

        let thermometer_dtos = thermometers.iter()
            .map(|thermometer| ThermometerDto::new(thermometer))
            .collect();

        Json(thermometer_dtos)
    }


    pub async fn get_thermometer(Path(id): Path<u32>,Extension(context): Extension<Arc<Mutex<WebContext>>>) -> Result<Json<ThermometerDto>, StatusCode> {

        let mut context = context.lock().await;

        let thermometer_service = &mut context.thermometer_service;
        if let Some(thermometer) = thermometer_service.get_by_id(id) {
            let thermometer_dto = ThermometerDto::new(thermometer);

            Ok(Json(thermometer_dto))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }
}