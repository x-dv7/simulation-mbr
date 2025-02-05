/// Мир с птицами и едой, видимые для Web-страницы
use crate::*;

#[derive(Clone, Debug)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();

        Self { animals, foods }
    }
}