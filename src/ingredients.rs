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

use crate::effects::Effect;
use strum_macros::EnumIter;

#[derive(Copy, Clone, EnumIter, Debug)]
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
    pub const fn price(&self) -> u8 {
        match self {
            Ingredient::Cuke => 2,
            Ingredient::Banana => 2,
            Ingredient::Paracetamol => 3,
            Ingredient::Donut => 3,
            Ingredient::Viagra => 4,
            Ingredient::MouthWash => 4,
            Ingredient::FluMedicine => 5,
            Ingredient::Gasoline => 5,
            Ingredient::EnergyDrink => 6,
            Ingredient::MotorOil => 6,
            Ingredient::MegaBean => 7,
            Ingredient::Chili => 7,
            Ingredient::Battery => 8,
            Ingredient::Iodine => 8,
            Ingredient::Addy => 9,
            Ingredient::HorseSemen => 9,
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            Ingredient::Cuke => "Cuke",
            Ingredient::Banana => "Banana",
            Ingredient::Paracetamol => "Paracetamol",
            Ingredient::Donut => "Donut",
            Ingredient::Viagra => "Viagra",
            Ingredient::MouthWash => "Mouth Wash",
            Ingredient::FluMedicine => "Flu Medicine",
            Ingredient::Gasoline => "Gasoline",
            Ingredient::EnergyDrink => "Energy Drink",
            Ingredient::MotorOil => "Motor Oil",
            Ingredient::MegaBean => "Mega Bean",
            Ingredient::Chili => "Chili",
            Ingredient::Battery => "Battery",
            Ingredient::Iodine => "Iodine",
            Ingredient::Addy => "Addy",
            Ingredient::HorseSemen => "Horse Semen",
        }
    }
    pub const fn emoji(&self) -> &'static str {
        match self {
            Ingredient::Cuke => "🥤",
            Ingredient::Banana => "🍌",
            Ingredient::Paracetamol => "⚪",
            Ingredient::Donut => "🍩",
            Ingredient::Viagra => "🔵",
            Ingredient::MouthWash => "💧",
            Ingredient::FluMedicine => "🧴",
            Ingredient::Gasoline => "⛽",
            Ingredient::EnergyDrink => "⚡",
            Ingredient::MotorOil => "🛢️",
            Ingredient::MegaBean => "🫘",
            Ingredient::Chili => "🌶️",
            Ingredient::Battery => "🔋",
            Ingredient::Iodine => "🧪",
            Ingredient::Addy => "💊",
            Ingredient::HorseSemen => "🐎",
        }
    }
    pub const fn abbreviation(&self) -> &'static str {
        match self {
            Ingredient::Cuke => "Cu",
            Ingredient::Banana => "Ba",
            Ingredient::Paracetamol => "Pa",
            Ingredient::Donut => "Dn",
            Ingredient::Viagra => "Vi",
            Ingredient::MouthWash => "MW",
            Ingredient::FluMedicine => "FM",
            Ingredient::Gasoline => "Ga",
            Ingredient::EnergyDrink => "ED",
            Ingredient::MotorOil => "MO",
            Ingredient::MegaBean => "MB",
            Ingredient::Chili => "Ch",
            Ingredient::Battery => "Bt",
            Ingredient::Iodine => "Io",
            Ingredient::Addy => "Ad",
            Ingredient::HorseSemen => "HS",
        }
    }
    // TODO: Use vector without order maybe even a hashmap
    pub fn apply(&self, effects: &[Effect]) -> Vec<Effect> {
        let mut result = Vec::new();
        match self {
            Ingredient::Cuke => {
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
                result
            }
            Ingredient::Banana => {
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
                result
            }
            Ingredient::Paracetamol => {
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
                result
            }
            Ingredient::Donut => {
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
                result
            }
            Ingredient::Viagra => {
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
                result
            }
            Ingredient::MouthWash => {
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
                result
            }
            Ingredient::FluMedicine => {
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
                result
            }
            Ingredient::Gasoline => {
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
                result
            }
            Ingredient::EnergyDrink => {
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
                result
            }
            Ingredient::MotorOil => {
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
                result
            }
            Ingredient::MegaBean => {
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
                result
            }
            Ingredient::Chili => {
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
                result
            }
            Ingredient::Battery => {
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
                result
            }
            Ingredient::Iodine => {
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
                result
            }
            Ingredient::Addy => {
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
                result
            }
            Ingredient::HorseSemen => {
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
                result
            }
        }
    }
}
