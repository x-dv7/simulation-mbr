Работает через AxumWebServer.
Слушает через tokio::net::TcpListener "0.0.0.0:3000".
От Web-страница идет запрос идет по GET на соединение websocket. Обработчик handle_websocket делает запуск WebSocket с бесконечным циклом получения и отправки сообщений в handle_callback.
Симуляция запускается в обработчике handle_callback для WebSocket, чтобы не тащить состояние в разные потоки. Обработчик работает в своем потоке.
А вот конфигурация, как структура может быть как State<Arc<RwLock<Config>>> и Конфигурация->Роутер->Web-страница идет по POST. И она хранится как состояние в WebServer.
В цикле обработки WebSocket обрабатываются запросы от Web-страницы:
- Запрос выполнения шага симуляции "step"
- Запрос выполнения шага обучения "train"
- Запрос на данные положения еды "foods"
- Запрос на данные положения птиц "animals"
При этом обратно отдаются данные положения птиц и еды, а также статистика по обучению.
Объект Simulation в этом проекте содержит Mutex<sim::Simulation>. Он вызывает методы основной симуляции через блокирование мутекса. Хотя зачем так сделано, если модель крутится в одном потоке? На всякий случай.
Объект World содержит Vec<Animal> и Vec<Food>. Они содержат только координаты, поворот, скорость и некоторые характеристики, чтобы отображать на Web-странице.
