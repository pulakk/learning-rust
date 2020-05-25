pub struct Uniform {
    pub dress: super::Material,
    cap: super::Material,
}

impl Uniform {
    pub fn create(cap_material: super::Material) -> Uniform {
        Uniform {
            dress: super::Material::Cotton,
            cap: cap_material,
        }
    }
}

pub fn clean_rooms() {}
fn wash_dishes() {}
pub fn quit() {
    super::quit();
}