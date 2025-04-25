use crate::effects::Effect;

pub enum Product {
    OgKush,
    SourDiesel,
    GreenCrack,
    GrandaddyPurple,
    Meth,
    Coke,
}

impl Product {
    pub const fn value(&self) -> f32 {
        match self {
            Product::Meth => 70.0,
            Product::Coke => 150.0,
            _ => 35.0,
        }
    }
    pub const fn effect(&self) -> Option<Effect> {
        match self {
            Product::OgKush => Some(Effect::Calming),
            Product::SourDiesel => Some(Effect::Refreshing),
            Product::GreenCrack => Some(Effect::Energizing),
            Product::GrandaddyPurple => Some(Effect::Sedating),
            _ => None,
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            Product::OgKush => "OG Kush",
            Product::SourDiesel => "Sour Diesel",
            Product::GreenCrack => "Green Crack",
            Product::GrandaddyPurple => "Grandaddy Purple",
            Product::Meth => "Meth",
            Product::Coke => "Coke",
        }
    }
    pub const fn from_u8(id: u8) -> Option<&'static Self> {
        match id {
            0 => Some(&Product::OgKush),
            1 => Some(&Product::SourDiesel),
            2 => Some(&Product::GreenCrack),
            3 => Some(&Product::GrandaddyPurple),
            4 => Some(&Product::Meth),
            5 => Some(&Product::Coke),
            _ => None,
        }
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
