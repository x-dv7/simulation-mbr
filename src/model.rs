///модуль содержит модель данных - Симуляцию
use crate::*;

// Симуляция
pub struct Simulation {
    sim: Mutex<sim::Simulation>,
}

impl Simulation {
    pub fn new(config: sim::Config) -> Self {
        let sim = sim::Simulation::random(config);

        Self { sim: sim.into() }
    }
    pub fn default_config() -> Config {
        let conf = sim::Config::default();
        Config::from(&conf)
    }
    // pub fn config(&self) -> Config {
    //     Config::from(self.sim.config())
    // }
    pub fn set_config(&self, config: Config) {
        let mut sim = self.sim.lock().unwrap();
        let mut conf = sim::Config::default();
        conf.world_animals = config.world_animals;
        conf.world_foods   = config.world_foods;
        conf.brain_neurons = config.brain_neurons;
        conf.eye_cells     = config.eye_cells;
        sim.set_config(conf);//создание новых птиц и еды
    }
    pub fn world(&self) -> World {// так виден мир для Web-страницы (его потом оберывают в JSON.
        // отдельно еду, отдельно птиц)
        World::from(self.sim.lock().unwrap().world())
    }
    pub fn step(&self) -> Option<String> {
        self.sim.lock().unwrap().step().map(|stats| stats.to_string())
    }
    pub fn train(&self) -> String {
        self.sim.lock().unwrap().train().to_string()
    }
}