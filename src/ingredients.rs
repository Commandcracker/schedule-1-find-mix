pub trait VecExtensions<T> {
    fn push_if_not_exists(&mut self, item: T);
}

impl<T: PartialEq> VecExtensions<T> for Vec<T> {
    fn push_if_not_exists(&mut self, item: T) {
        if !self.contains(&item) {
            self.push(item);
        }
    }
}

use strum_macros::EnumIter;

use crate::effects::Effect;

#[derive(Clone, EnumIter, PartialEq)]
pub enum Ingredient {
    Cuke,
    Banana,
    Paracetamol,
    Donut,
    Viagra,
    MouthWash,
    FluMedicine,
    Gasoline,
    EnergyDrink,
    MotorOil,
    MegaBean,
    Chili,
    Battery,
    Iodine,
    Addy,
    HorseSemen,
}
impl Ingredient {
    pub const fn from_u8(id: u8) -> Option<&'static Self> {
        match id {
            0 => Some(&Self::Cuke),
            1 => Some(&Self::Banana),
            2 => Some(&Self::Paracetamol),
            3 => Some(&Self::Donut),
            4 => Some(&Self::Viagra),
            5 => Some(&Self::MouthWash),
            6 => Some(&Self::FluMedicine),
            7 => Some(&Self::Gasoline),
            8 => Some(&Self::EnergyDrink),
            9 => Some(&Self::MotorOil),
            10 => Some(&Self::MegaBean),
            11 => Some(&Self::Chili),
            12 => Some(&Self::Battery),
            13 => Some(&Self::Iodine),
            14 => Some(&Self::Addy),
            15 => Some(&Self::HorseSemen),
            _ => None,
        }
    }

    pub const fn price(&self) -> u8 {
        match self {
            Self::Cuke => 2,
            Self::Banana => 2,
            Self::Paracetamol => 3,
            Self::Donut => 3,
            Self::Viagra => 4,
            Self::MouthWash => 4,
            Self::FluMedicine => 5,
            Self::Gasoline => 5,
            Self::EnergyDrink => 6,
            Self::MotorOil => 6,
            Self::MegaBean => 7,
            Self::Chili => 7,
            Self::Battery => 8,
            Self::Iodine => 8,
            Self::Addy => 9,
            Self::HorseSemen => 9,
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Cuke => "Cuke",
            Self::Banana => "Banana",
            Self::Paracetamol => "Paracetamol",
            Self::Donut => "Donut",
            Self::Viagra => "Viagra",
            Self::MouthWash => "Mouth Wash",
            Self::FluMedicine => "Flu Medicine",
            Self::Gasoline => "Gasoline",
            Self::EnergyDrink => "Energy Drink",
            Self::MotorOil => "Motor Oil",
            Self::MegaBean => "Mega Bean",
            Self::Chili => "Chili",
            Self::Battery => "Battery",
            Self::Iodine => "Iodine",
            Self::Addy => "Addy",
            Self::HorseSemen => "Horse Semen",
        }
    }
    pub const fn emoji(&self) -> &'static str {
        match self {
            Self::Cuke => "🥤",
            Self::Banana => "🍌",
            Self::Paracetamol => "⚪",
            Self::Donut => "🍩",
            Self::Viagra => "🔵",
            Self::MouthWash => "💧",
            Self::FluMedicine => "🧴",
            Self::Gasoline => "⛽",
            Self::EnergyDrink => "⚡",
            Self::MotorOil => "🛢️",
            Self::MegaBean => "🫘",
            Self::Chili => "🌶️",
            Self::Battery => "🔋",
            Self::Iodine => "🧪",
            Self::Addy => "💊",
            Self::HorseSemen => "🐎",
        }
    }
    pub const fn abbreviation(&self) -> &'static str {
        match self {
            Self::Cuke => "Cu",
            Self::Banana => "Ba",
            Self::Paracetamol => "Pa",
            Self::Donut => "Dn",
            Self::Viagra => "Vi",
            Self::MouthWash => "MW",
            Self::FluMedicine => "FM",
            Self::Gasoline => "Ga",
            Self::EnergyDrink => "ED",
            Self::MotorOil => "MO",
            Self::MegaBean => "MB",
            Self::Chili => "Ch",
            Self::Battery => "Bt",
            Self::Iodine => "Io",
            Self::Addy => "Ad",
            Self::HorseSemen => "HS",
        }
    }
    // TODO: Use vector without order maybe even a hashmap
    pub fn apply(&self, effects: &[Effect]) -> Vec<Effect> {
        let mut result = Vec::new();
        match self {
            Self::Cuke => {
                for effect in effects {
                    match effect {
                        Effect::Euphoric => result.push_if_not_exists(Effect::Laxative),
                        Effect::Foggy => result.push_if_not_exists(Effect::Cyclopean),
                        Effect::Gingeritis => result.push_if_not_exists(Effect::ThoughtProvoking),
                        Effect::Munchies => result.push_if_not_exists(Effect::Athletic),
                        Effect::Slippery => result.push_if_not_exists(Effect::Munchies),
                        Effect::Sneaky => result.push_if_not_exists(Effect::Paranoia),
                        Effect::Toxic => result.push_if_not_exists(Effect::Euphoric),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Energizing);
            }
            Self::Banana => {
                for effect in effects {
                    match effect {
                        Effect::Calming => result.push_if_not_exists(Effect::Sneaky),
                        Effect::Cyclopean => result.push_if_not_exists(Effect::Energizing),
                        Effect::Disorienting => result.push_if_not_exists(Effect::Focused),
                        Effect::Energizing => result.push_if_not_exists(Effect::ThoughtProvoking),
                        Effect::Focused => result.push_if_not_exists(Effect::SeizureInducing),
                        Effect::LongFaced => result.push_if_not_exists(Effect::Refreshing),
                        Effect::Paranoia => result.push_if_not_exists(Effect::Jennerising),
                        Effect::Smelly => result.push_if_not_exists(Effect::AntiGravity),
                        Effect::Toxic => result.push_if_not_exists(Effect::Smelly),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Gingeritis);
            }
            Self::Paracetamol => {
                for effect in effects {
                    match effect {
                        Effect::Calming => result.push_if_not_exists(Effect::Slippery),
                        Effect::Electrifying => result.push_if_not_exists(Effect::Athletic),
                        Effect::Energizing => result.push_if_not_exists(Effect::Paranoia),
                        Effect::Focused => result.push_if_not_exists(Effect::Gingeritis),
                        Effect::Foggy => result.push_if_not_exists(Effect::Calming),
                        Effect::Glowing => result.push_if_not_exists(Effect::Toxic),
                        Effect::Munchies => result.push_if_not_exists(Effect::AntiGravity),
                        Effect::Paranoia => result.push_if_not_exists(Effect::Balding),
                        Effect::Spicy => result.push_if_not_exists(Effect::BrightEyed),
                        Effect::Toxic => result.push_if_not_exists(Effect::TropicThunder),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Sneaky);
            }
            Self::Donut => {
                for effect in effects {
                    match effect {
                        Effect::AntiGravity => result.push_if_not_exists(Effect::Slippery),
                        Effect::Balding => result.push_if_not_exists(Effect::Sneaky),
                        Effect::CalorieDense => result.push_if_not_exists(Effect::Explosive),
                        Effect::Focused => result.push_if_not_exists(Effect::Euphoric),
                        Effect::Jennerising => result.push_if_not_exists(Effect::Gingeritis),
                        Effect::Munchies => result.push_if_not_exists(Effect::Calming),
                        Effect::Shrinking => result.push_if_not_exists(Effect::Energizing),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::CalorieDense);
            }
            Self::Viagra => {
                for effect in effects {
                    match effect {
                        Effect::Athletic => result.push_if_not_exists(Effect::Sneaky),
                        Effect::Disorienting => result.push_if_not_exists(Effect::Toxic),
                        Effect::Euphoric => result.push_if_not_exists(Effect::BrightEyed),
                        Effect::Laxative => result.push_if_not_exists(Effect::Calming),
                        Effect::Shrinking => result.push_if_not_exists(Effect::Gingeritis),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::TropicThunder);
            }
            Self::MouthWash => {
                for effect in effects {
                    match effect {
                        Effect::Calming => result.push_if_not_exists(Effect::AntiGravity),
                        Effect::CalorieDense => result.push_if_not_exists(Effect::Sneaky),
                        Effect::Explosive => result.push_if_not_exists(Effect::Sedating),
                        Effect::Focused => result.push_if_not_exists(Effect::Jennerising),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Balding);
            }
            Self::FluMedicine => {
                for effect in effects {
                    match effect {
                        Effect::Athletic => result.push_if_not_exists(Effect::Munchies),
                        Effect::Calming => result.push_if_not_exists(Effect::BrightEyed),
                        Effect::Cyclopean => result.push_if_not_exists(Effect::Foggy),
                        Effect::Electrifying => result.push_if_not_exists(Effect::Refreshing),
                        Effect::Euphoric => result.push_if_not_exists(Effect::Toxic),
                        Effect::Focused => result.push_if_not_exists(Effect::Calming),
                        Effect::Laxative => result.push_if_not_exists(Effect::Euphoric),
                        Effect::Munchies => result.push_if_not_exists(Effect::Slippery),
                        Effect::Shrinking => result.push_if_not_exists(Effect::Paranoia),
                        Effect::ThoughtProvoking => result.push_if_not_exists(Effect::Gingeritis),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Sedating);
            }
            Self::Gasoline => {
                for effect in effects {
                    match effect {
                        Effect::Disorienting => result.push_if_not_exists(Effect::Glowing),
                        Effect::Electrifying => result.push_if_not_exists(Effect::Disorienting),
                        Effect::Energizing => result.push_if_not_exists(Effect::Euphoric),
                        Effect::Euphoric => result.push_if_not_exists(Effect::Spicy),
                        Effect::Gingeritis => result.push_if_not_exists(Effect::Smelly),
                        Effect::Jennerising => result.push_if_not_exists(Effect::Sneaky),
                        Effect::Laxative => result.push_if_not_exists(Effect::Foggy),
                        Effect::Munchies => result.push_if_not_exists(Effect::Sedating),
                        Effect::Paranoia => result.push_if_not_exists(Effect::Calming),
                        Effect::Shrinking => result.push_if_not_exists(Effect::Focused),
                        Effect::Sneaky => result.push_if_not_exists(Effect::TropicThunder),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Toxic);
            }
            Self::EnergyDrink => {
                for effect in effects {
                    match effect {
                        Effect::Disorienting => result.push_if_not_exists(Effect::Electrifying),
                        Effect::Euphoric => result.push_if_not_exists(Effect::Energizing),
                        Effect::Focused => result.push_if_not_exists(Effect::Shrinking),
                        Effect::Foggy => result.push_if_not_exists(Effect::Laxative),
                        Effect::Glowing => result.push_if_not_exists(Effect::Disorienting),
                        Effect::Schizophrenic => result.push_if_not_exists(Effect::Balding),
                        Effect::Sedating => result.push_if_not_exists(Effect::Munchies),
                        Effect::Spicy => result.push_if_not_exists(Effect::Euphoric),
                        Effect::TropicThunder => result.push_if_not_exists(Effect::Sneaky),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Athletic);
            }
            Self::MotorOil => {
                for effect in effects {
                    match effect {
                        Effect::Energizing => result.push_if_not_exists(Effect::Munchies),
                        Effect::Euphoric => result.push_if_not_exists(Effect::Sedating),
                        Effect::Foggy => result.push_if_not_exists(Effect::Toxic),
                        Effect::Munchies => result.push_if_not_exists(Effect::Schizophrenic),
                        Effect::Paranoia => result.push_if_not_exists(Effect::AntiGravity),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Slippery);
            }
            Self::MegaBean => {
                for effect in effects {
                    match effect {
                        Effect::Athletic => result.push_if_not_exists(Effect::Laxative),
                        Effect::Calming => result.push_if_not_exists(Effect::Glowing),
                        Effect::Energizing => result.push_if_not_exists(Effect::Cyclopean),
                        Effect::Focused => result.push_if_not_exists(Effect::Disorienting),
                        Effect::Jennerising => result.push_if_not_exists(Effect::Paranoia),
                        Effect::SeizureInducing => result.push_if_not_exists(Effect::Focused),
                        Effect::Shrinking => result.push_if_not_exists(Effect::Electrifying),
                        Effect::Slippery => result.push_if_not_exists(Effect::Toxic),
                        Effect::Sneaky => result.push_if_not_exists(Effect::Calming),
                        Effect::ThoughtProvoking => result.push_if_not_exists(Effect::Energizing),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Foggy);
            }
            Self::Chili => {
                for effect in effects {
                    match effect {
                        Effect::AntiGravity => result.push_if_not_exists(Effect::TropicThunder),
                        Effect::Athletic => result.push_if_not_exists(Effect::Euphoric),
                        Effect::Laxative => result.push_if_not_exists(Effect::LongFaced),
                        Effect::Munchies => result.push_if_not_exists(Effect::Toxic),
                        Effect::Shrinking => result.push_if_not_exists(Effect::Refreshing),
                        Effect::Sneaky => result.push_if_not_exists(Effect::BrightEyed),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Spicy);
            }
            Self::Battery => {
                for effect in effects {
                    match effect {
                        Effect::Cyclopean => result.push_if_not_exists(Effect::Glowing),
                        Effect::Electrifying => result.push_if_not_exists(Effect::Euphoric),
                        Effect::Euphoric => result.push_if_not_exists(Effect::Zombifying),
                        Effect::Laxative => result.push_if_not_exists(Effect::CalorieDense),
                        Effect::Munchies => result.push_if_not_exists(Effect::TropicThunder),
                        Effect::Shrinking => result.push_if_not_exists(Effect::Munchies),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::BrightEyed);
            }
            Self::Iodine => {
                for effect in effects {
                    match effect {
                        Effect::Calming => result.push_if_not_exists(Effect::Balding),
                        Effect::CalorieDense => result.push_if_not_exists(Effect::Gingeritis),
                        Effect::Euphoric => result.push_if_not_exists(Effect::SeizureInducing),
                        Effect::Foggy => result.push_if_not_exists(Effect::Paranoia),
                        Effect::Refreshing => result.push_if_not_exists(Effect::ThoughtProvoking),
                        Effect::Toxic => result.push_if_not_exists(Effect::Sneaky),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::Jennerising);
            }
            Self::Addy => {
                for effect in effects {
                    match effect {
                        Effect::Explosive => result.push_if_not_exists(Effect::Euphoric),
                        Effect::Foggy => result.push_if_not_exists(Effect::Energizing),
                        Effect::Glowing => result.push_if_not_exists(Effect::Refreshing),
                        Effect::LongFaced => result.push_if_not_exists(Effect::Electrifying),
                        Effect::Sedating => result.push_if_not_exists(Effect::Gingeritis),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::ThoughtProvoking);
            }
            Self::HorseSemen => {
                for effect in effects {
                    match effect {
                        Effect::AntiGravity => result.push_if_not_exists(Effect::Calming),
                        Effect::Gingeritis => result.push_if_not_exists(Effect::Refreshing),
                        Effect::SeizureInducing => result.push_if_not_exists(Effect::Energizing),
                        Effect::ThoughtProvoking => result.push_if_not_exists(Effect::Electrifying),
                        a => result.push_if_not_exists(a.clone()),
                    }
                }
                result.push_if_not_exists(Effect::LongFaced);
            }
        }
        result
    }
}
