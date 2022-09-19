// Copyright 2022 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::state::ExplorerApiStateContext;
use crate::state::GeoIp;
use maxminddb::geoip2;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Request, Route, State};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::settings::OpenApiSettings;
use std::net::IpAddr;
use std::str::FromStr;

pub fn nym_terms_make_default_routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![settings: terms]
}

#[derive(Clone, Debug)]
pub struct CountryIsoCode(String);

#[derive(Debug)]
pub enum LocationError {
    NoIP,
    LocationNotFound,
    NoValidIP,
    InternalError,
}

fn find_location(request: &Request<'_>) -> Result<CountryIsoCode, (Status, LocationError)> {
    let ip = request
        .headers()
        .get_one("X-Real-IP")
        .map(|f| f.to_string())
        .ok_or_else(|| {
            error!("No IP in headers");
            (Status::Forbidden, LocationError::NoIP)
        })?;

    let db = request
        .rocket()
        .state::<GeoIp>()
        .ok_or_else(|| {
            error!("No GeoIP state found");
            (Status::InternalServerError, LocationError::InternalError)
        })?
        .db
        .as_ref()
        .ok_or_else(|| {
            error!("No registered GeoIP database");
            (Status::InternalServerError, LocationError::InternalError)
        })?;

    let ip: IpAddr = FromStr::from_str(&ip).map_err(|e| {
        error!("Fail to create IpAddr from {}: {}", &ip, e);
        (Status::InternalServerError, LocationError::NoValidIP)
    })?;
    let country: geoip2::Country = db.lookup(ip).map_err(|e| {
        error!("Fail to lookup in GeoIP database: {}", e);
        (Status::InternalServerError, LocationError::LocationNotFound)
    })?;
    Ok(CountryIsoCode(
        country
            .country
            .ok_or_else(|| {
                error!("Country data not found");
                (Status::InternalServerError, LocationError::LocationNotFound)
            })?
            .iso_code
            .ok_or_else(|| {
                error!("ISO code not found");
                (Status::InternalServerError, LocationError::LocationNotFound)
            })?
            .to_string(),
    ))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CountryIsoCode {
    type Error = LocationError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match find_location(request) {
            Ok(loc) => Outcome::Success(loc),
            Err(e) => Outcome::Failure(e),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for CountryIsoCode {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[openapi(tag = "terms")]
#[get("/")]
pub(crate) async fn terms(
    _state: &State<ExplorerApiStateContext>,
    country_code: CountryIsoCode,
) -> Result<Json<String>, status::Forbidden<String>> {
    if country_code.0 == "US" {
        return Err(status::Forbidden(Some("US government sucks".to_string())));
    }
    Ok(Json("Nym Terms & Conditions: Welcome".to_string()))
}
