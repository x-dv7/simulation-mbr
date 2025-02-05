/// здесь изолировали логику веб-сервера
use crate::*;

///веб-сервер с брокером сообщений
pub struct AxumWebServer {
    config: Arc<RwLock<Config>>,
}

impl AxumWebServer {
    pub fn new(config: Config) -> Self {///конструктор
        let app_state = Arc::new(RwLock::new(config));
        AxumWebServer { config: app_state }
    }
    pub async fn start(&self) {///старт сервера
        //состояние для передачи в роутер
        let app_state = Arc::clone(&self.config);
        //роутер
        // set up our application with "hello world" route at "/
        let app = Router::new()
            .route("/default_config", post(Self::default_config))
            // .route("/set_config", post(Self::set_config))
            .route("/ws", get(Self::handle_websocket))
            .with_state(app_state)
            .layer(
                CorsLayer::new()
                    .allow_headers(Any)
                    // [AUTHORIZATION,
                    //         ACCEPT,
                    //         CONTENT_TYPE])
                    .max_age(Duration::from_secs(30))
                    // allow `GET` and `POST` when accessing the resource
                    .allow_methods(vec![Method::GET, Method::POST]) //
                    // allow requests from any origin
                    .allow_origin(Any)
            );
        // start the server on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
    ///Конфигурация->Роутер->Web-страница
    async fn default_config(
        State(handler): State<Arc<RwLock<Config>>>
    ) -> Json<Config>
    {
        Json(handler.read().unwrap().default_config())
    }
    // ///Установка конфигурации из Web-страницы
    // async fn set_config(
    //     State(handler): State<Arc<RwLock<Config>>>,
    //     Json(data): Json<Config>,
    // ) -> String
    // {
    //     handler.write().unwrap().set_config(data)
    // }
    /// запуск WebSocket с бесконечным циклом
    /// получения и отправки сообщений в handle_callback
    /// Сделано через веб-сокет для ускорения, т.к. постоянные POST
    /// запросы на координаты еды и птиц, а также на выполнение шага
    /// симуляции сильно тормозили
    async fn handle_websocket(
        ws: WebSocketUpgrade,
        State(config): State<Arc<RwLock<Config>>>,
    ) -> Response
    {
        ws.on_upgrade(|socket| handle_callback(socket, config))
    }
}

/// callback для обработки сообщений
async fn handle_callback(mut socket: WebSocket, config: Arc<RwLock<Config>>) {
    //делаем симуляцию внутри обработчика, чтобы не таскать его как состояние
    //между потоками. Состояние только конфигурация.
    let mut simulation = Simulation::new(sim::Config::default());
    //цикл обработки
    while let Some(msg) = socket.recv().await {
        let _ = if let Ok(msg) = msg {
            if let Message::Text(text) = msg {
                // Обработка сообщения от клиента
                // Отправка команд в Simulation через MessageBroker
                // Это именные запросы на данные
                match text.as_str() {
                    // Запрос выполнения шага симуляции
                    "step" => {
                        let resp = simulation.step();
                        let response = resp.unwrap_or("".to_string());// шаг
                        let data = serde_json::to_string(&json!({ "stat": response })).unwrap();
                        if socket.send(Message::text(data)).await.is_err() {
                            return;
                        }
                    }
                    // Запрос выполнения шага обучения
                    "train" => {
                        let response = simulation.train(); // тренировка
                        let data = serde_json::to_string(&json!({ "stat": response })).unwrap();
                        if socket.send(Message::text(data)).await.is_err() {
                            return;
                        }
                    }
                    // Запрос на данные положения еды
                    "foods" => {
                        let response = simulation.world().foods; // положение еды
                        let data = serde_json::to_string(&json!({ "foods": response })).unwrap();
                        if socket.send(Message::text(data)).await.is_err() {
                            return;
                        }
                    }
                    // Запрос на данные положения птиц
                    "animals" => {
                        let response = simulation.world().animals; // положение птиц
                        let data = serde_json::to_string(&json!({ "animals": response })).unwrap();
                        if socket.send(Message::text(data)).await.is_err() {
                            return;
                        }
                    }
                    // // Запрос на чтение State-конфигурации в модель
                    // "read_state_config" => {
                    //     simulation.set_config(config.read().unwrap().clone());
                    // }
                    _ => {
                        // Неизвестное сообщение
                        // ...
                    }
                }
            }
        } else {
            // client disconnected
            return;
        };
    }
}