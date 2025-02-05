use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub brain_neurons: usize,

    pub eye_fov_angle: f32,
    pub eye_cells: usize,

    pub food_size: f32,

    pub world_animals: usize,
    pub world_foods: usize,
}

impl Config {
    /// Генерация стандартной конфигурации
    pub(crate) fn default_config(&self) -> Config {
        Simulation::default_config()
    }
    // /// Установка конфигурации из внешнего запроса
    // pub(crate) fn set_config(&mut self, config: Config) -> String {
    //     self.world_animals = config.world_animals;
    //     self.world_foods   = config.world_foods;
    //     self.brain_neurons = config.brain_neurons;
    //     self.eye_cells     = config.eye_cells;
    //     "OK".to_string()
    // }
}
impl From<&sim::Config> for Config {
    fn from(config: &sim::Config) -> Self {
        Self {
            brain_neurons: config.brain_neurons,
            eye_fov_angle: config.eye_fov_angle,
            eye_cells: config.eye_cells,
            food_size: config.food_size,
            world_animals: config.world_animals,
            world_foods: config.world_foods,
        }
    }
}