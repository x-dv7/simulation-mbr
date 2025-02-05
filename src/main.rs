/// содержит только минимум кода для запуска приложения
use crate::{
    web::AxumWebServer,
    config::Config,
    model::Simulation,
    world::World,
    animal::Animal,
    food::Food,
};

use axum::{
    http::Method,
    extract::{State, Path},
    Json,
    Router,
    routing::{get, post},//delete,
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    response::{Response, IntoResponse}
};

use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock};
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json};

use lib_simulation_lifelong as sim;

mod web;     //содержит логику веб-сервера
mod config;  //конфигурация, отправляемая на Web-страницу
mod model;   //Симуляция
mod world;   //Структура мира, видимая для Web-страницы
mod animal;  //Структура птиц, видимая для Web-страницы
mod food;    //Структура еды, видимая для Web-страницы

#[tokio::main]
async fn main() {
    let conf = Config::from(&sim::Config::default());
    //веб-сервер
    let web_server = AxumWebServer::new(conf);
    //старт сервера
    web_server.start().await;
}
