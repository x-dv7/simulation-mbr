/// Еда, видимая для Web-страницы
use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}