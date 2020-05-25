mod house_keeping;

pub use house_keeping::maid::clean_rooms;

pub fn clean_the_house() {
    house_keeping::maid::clean_rooms();
    house_keeping::maid::quit();
}

pub fn get_maid_uniform() -> house_keeping::maid::Uniform {
    house_keeping::maid::Uniform::create(
        house_keeping::Material::Nylon,
    )
}