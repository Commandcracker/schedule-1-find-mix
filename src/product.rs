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
            Self::Meth => 70.,
            Self::Coke => 150.,
            _ => 35.,
        }
    }
    pub const fn effect(&self) -> Option<Effect> {
        match self {
            Self::OgKush => Some(Effect::Calming),
            Self::SourDiesel => Some(Effect::Refreshing),
            Self::GreenCrack => Some(Effect::Energizing),
            Self::GrandaddyPurple => Some(Effect::Sedating),
            _ => None,
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            Self::OgKush => "OG Kush",
            Self::SourDiesel => "Sour Diesel",
            Self::GreenCrack => "Green Crack",
            Self::GrandaddyPurple => "Grandaddy Purple",
            Self::Meth => "Meth",
            Self::Coke => "Coke",
        }
    }
    pub const fn from_u8(id: u8) -> Option<&'static Self> {
        match id {
            0 => Some(&Self::OgKush),
            1 => Some(&Self::SourDiesel),
            2 => Some(&Self::GreenCrack),
            3 => Some(&Self::GrandaddyPurple),
            4 => Some(&Self::Meth),
            5 => Some(&Self::Coke),
            _ => None,
        }
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
