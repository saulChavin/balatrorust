use strum_macros::EnumIter;
use crate::{Item, Joker};

#[derive(EnumIter, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Spectral {
    Familiar,
    Grim,
    Incantation,
    Talisman,
    Aura,
    Wraith,
    Sigil,
    Ouija,
    Ectoplasm,
    Immolate,
    Ankh,
    DejaVu,
    Hex,
    Trance,
    Medium,
    Cryptid,
    Retry,
    Retry2,
}

impl Item for Spectral {
    fn get_name(&self) -> &str {
        match self {
            Spectral::Familiar => "Familiar",
            Spectral::Grim => "Grim",
            Spectral::Incantation => "Incantation",
            Spectral::Talisman => "Talisman",
            Spectral::Aura => "Aura",
            Spectral::Wraith => "Wraith",
            Spectral::Sigil => "Sigil",
            Spectral::Ouija => "Ouija",
            Spectral::Ectoplasm => "Ectoplasm",
            Spectral::Immolate => "Immolate",
            Spectral::Ankh => "Ankh",
            Spectral::DejaVu => "Deja Vu",
            Spectral::Hex => "Hex",
            Spectral::Trance => "Trance",
            Spectral::Medium => "Medium",
            Spectral::Cryptid => "Cryptid",
            Spectral::Retry => "RETRY",
            Spectral::Retry2 => "RETRY2",
        }
    }
}


#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum UnCommonJoker {
    JokerStencil,
    FourFingers,
    Mime,
    CeremonialDagger,
    MarbleJoker,
    LoyaltyCard,
    Dusk,
    Fibonacci,
    SteelJoker,
    Hack,
    Pareidolia,
    SpaceJoker,
    Burglar,
    Blackboard,
    SixthSense,
    Constellation,
    Hiker,
    CardSharp,
    Madness,
    Seance,
    Vampire,
    Shortcut,
    Hologram,
    Cloud9,
    Rocket,
    MidasMask,
    Luchador,
    GiftCard,
    TurtleBean,
    Erosion,
    ToTheMoon,
    StoneJoker,
    LuckyCat,
    Bull,
    DietCola,
    TradingCard,
    FlashCard,
    SpareTrousers,
    Ramen,
    Seltzer,
    Castle,
    MrBones,
    Acrobat,
    SockAndBuskin,
    Troubadour,
    Certificate,
    SmearedJoker,
    Throwback,
    RoughGem,
    Bloodstone,
    Arrowhead,
    OnyxAgate,
    GlassJoker,
    Showman,
    FlowerPot,
    MerryAndy,
    OopsAll6s,
    TheIdol,
    SeeingDouble,
    Matador,
    Satellite,
    Cartomancer,
    Astronomer,
    Bootstrap,
}

impl Item for UnCommonJoker {
    fn get_name(&self) -> &str {
        match self {
            UnCommonJoker::JokerStencil => "Joker Stencil",
            UnCommonJoker::FourFingers => "Four Fingers",
            UnCommonJoker::Mime => "Mime",
            UnCommonJoker::CeremonialDagger => "Ceremonial Dagger",
            UnCommonJoker::MarbleJoker => "Marble Joker",
            UnCommonJoker::LoyaltyCard => "Loyalty Card",
            UnCommonJoker::Dusk => "Dusk",
            UnCommonJoker::Fibonacci => "Fibonacci",
            UnCommonJoker::SteelJoker => "Steel Joker",
            UnCommonJoker::Hack => "Hack",
            UnCommonJoker::Pareidolia => "Pareidolia",
            UnCommonJoker::SpaceJoker => "Space Joker",
            UnCommonJoker::Burglar => "Burglar",
            UnCommonJoker::Blackboard => "Blackboard",
            UnCommonJoker::SixthSense => "Sixth Sense",
            UnCommonJoker::Constellation => "Constellation",
            UnCommonJoker::Hiker => "Hiker",
            UnCommonJoker::CardSharp => "Card Sharp",
            UnCommonJoker::Madness => "Madness",
            UnCommonJoker::Seance => "Seance",
            UnCommonJoker::Vampire => "Vampire",
            UnCommonJoker::Shortcut => "Shortcut",
            UnCommonJoker::Hologram => "Hologram",
            UnCommonJoker::Cloud9 => "Cloud 9",
            UnCommonJoker::Rocket => "Rocket",
            UnCommonJoker::MidasMask => "Midas Mask",
            UnCommonJoker::Luchador => "Luchador",
            UnCommonJoker::GiftCard => "Gift Card",
            UnCommonJoker::TurtleBean => "Turtle Bean",
            UnCommonJoker::Erosion => "Erosion",
            UnCommonJoker::ToTheMoon => "To the Moon",
            UnCommonJoker::StoneJoker => "Stone Joker",
            UnCommonJoker::LuckyCat => "Lucky Cat",
            UnCommonJoker::Bull => "Bull",
            UnCommonJoker::DietCola => "Diet Cola",
            UnCommonJoker::TradingCard => "Trading Card",
            UnCommonJoker::FlashCard => "Flash Card",
            UnCommonJoker::SpareTrousers => "Spare Trousers",
            UnCommonJoker::Ramen => "Ramen",
            UnCommonJoker::Seltzer => "Seltzer",
            UnCommonJoker::Castle => "Castle",
            UnCommonJoker::MrBones => "Mr. Bones",
            UnCommonJoker::Acrobat => "Acrobat",
            UnCommonJoker::SockAndBuskin => "Sock and Buskin",
            UnCommonJoker::Troubadour => "Troubadour",
            UnCommonJoker::Certificate => "Certificate",
            UnCommonJoker::SmearedJoker => "Smeared Joker",
            UnCommonJoker::Throwback => "Throwback",
            UnCommonJoker::RoughGem => "Rough Gem",
            UnCommonJoker::Bloodstone => "Bloodstone",
            UnCommonJoker::Arrowhead => "Arrowhead",
            UnCommonJoker::OnyxAgate => "Onyx Agate",
            UnCommonJoker::GlassJoker => "Glass Joker",
            UnCommonJoker::Showman => "Showman",
            UnCommonJoker::FlowerPot => "Flower Pot",
            UnCommonJoker::MerryAndy => "Merry Andy",
            UnCommonJoker::OopsAll6s => "Oops! All 6s",
            UnCommonJoker::TheIdol => "The Idol",
            UnCommonJoker::SeeingDouble => "Seeing Double",
            UnCommonJoker::Matador => "Matador",
            UnCommonJoker::Satellite => "Satellite",
            UnCommonJoker::Cartomancer => "Cartomancer",
            UnCommonJoker::Astronomer => "Astronomer",
            UnCommonJoker::Bootstrap => "Bootstrap",
        }
    }
}


#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum UnCommonJoker100 {
    JokerStencil,
    FourFingers,
    Mime,
    CeremonialDagger,
    MarbleJoker,
    LoyaltyCard,
    Dusk,
    Fibonacci,
    SteelJoker,
    Hack,
    Pareidolia,
    SpaceJoker,
    Burglar,
    Blackboard,
    Constellation,
    Hiker,
    CardSharp,
    Madness,
    Vampire,
    Shortcut,
    Hologram,
    Vagabond,
    Cloud9,
    Rocket,
    MidasMask,
    Luchador,
    GiftCard,
    TurtleBean,
    Erosion,
    ReservedParking,
    ToTheMoon,
    StoneJoker,
    LuckyCat,
    Bull,
    DietCola,
    TradingCard,
    FlashCard,
    SpareTrousers,
    Ramen,
    Seltzer,
    Castle,
    MrBones,
    Acrobat,
    SockAndBuskin,
    Troubadour,
    Certificate,
    SmearedJoker,
    Throwback,
    RoughGem,
    Bloodstone,
    Arrowhead,
    OnyxAgate,
    GlassJoker,
    Showman,
    FlowerPot,
    MerryAndy,
    OopsAll6s,
    TheIdol,
    SeeingDouble,
    Matador,
    Stuntman,
    Satellite,
    Cartomancer,
    Astronomer,
    BurntJoker,
    Bootstraps,
}

impl Item for UnCommonJoker100 {
    fn get_name(&self) -> &str {
        match self {
            UnCommonJoker100::JokerStencil => "Joker Stencil",
            UnCommonJoker100::FourFingers => "Four Fingers",
            UnCommonJoker100::Mime => "Mime",
            UnCommonJoker100::CeremonialDagger => "Ceremonial Dagger",
            UnCommonJoker100::MarbleJoker => "Marble Joker",
            UnCommonJoker100::LoyaltyCard => "Loyalty Card",
            UnCommonJoker100::Dusk => "Dusk",
            UnCommonJoker100::Fibonacci => "Fibonacci",
            UnCommonJoker100::SteelJoker => "Steel Joker",
            UnCommonJoker100::Hack => "Hack",
            UnCommonJoker100::Pareidolia => "Pareidolia",
            UnCommonJoker100::SpaceJoker => "Space Joker",
            UnCommonJoker100::Burglar => "Burglar",
            UnCommonJoker100::Blackboard => "Blackboard",
            UnCommonJoker100::Constellation => "Constellation",
            UnCommonJoker100::Hiker => "Hiker",
            UnCommonJoker100::CardSharp => "Card Sharp",
            UnCommonJoker100::Madness => "Madness",
            UnCommonJoker100::Vampire => "Vampire",
            UnCommonJoker100::Shortcut => "Shortcut",
            UnCommonJoker100::Hologram => "Hologram",
            UnCommonJoker100::Vagabond => "Vagabond",
            UnCommonJoker100::Cloud9 => "Cloud 9",
            UnCommonJoker100::Rocket => "Rocket",
            UnCommonJoker100::MidasMask => "Midas Mask",
            UnCommonJoker100::Luchador => "Luchador",
            UnCommonJoker100::GiftCard => "Gift Card",
            UnCommonJoker100::TurtleBean => "Turtle Bean",
            UnCommonJoker100::Erosion => "Erosion",
            UnCommonJoker100::ReservedParking => "Reserved Parking",
            UnCommonJoker100::ToTheMoon => "To the Moon",
            UnCommonJoker100::StoneJoker => "Stone Joker",
            UnCommonJoker100::LuckyCat => "Lucky Cat",
            UnCommonJoker100::Bull => "Bull",
            UnCommonJoker100::DietCola => "Diet Cola",
            UnCommonJoker100::TradingCard => "Trading Card",
            UnCommonJoker100::FlashCard => "Flash Card",
            UnCommonJoker100::SpareTrousers => "Spare Trousers",
            UnCommonJoker100::Ramen => "Ramen",
            UnCommonJoker100::Seltzer => "Seltzer",
            UnCommonJoker100::Castle => "Castle",
            UnCommonJoker100::MrBones => "Mr. Bones",
            UnCommonJoker100::Acrobat => "Acrobat",
            UnCommonJoker100::SockAndBuskin => "Sock and Buskin",
            UnCommonJoker100::Troubadour => "Troubadour",
            UnCommonJoker100::Certificate => "Certificate",
            UnCommonJoker100::SmearedJoker => "Smeared Joker",
            UnCommonJoker100::Throwback => "Throwback",
            UnCommonJoker100::RoughGem => "Rough Gem",
            UnCommonJoker100::Bloodstone => "Bloodstone",
            UnCommonJoker100::Arrowhead => "Arrowhead",
            UnCommonJoker100::OnyxAgate => "Onyx Agate",
            UnCommonJoker100::GlassJoker => "Glass Joker",
            UnCommonJoker100::Showman => "Showman",
            UnCommonJoker100::FlowerPot => "Flower Pot",
            UnCommonJoker100::MerryAndy => "Merry Andy",
            UnCommonJoker100::OopsAll6s => "Oops! All 6s",
            UnCommonJoker100::TheIdol => "The Idol",
            UnCommonJoker100::SeeingDouble => "Seeing Double",
            UnCommonJoker100::Matador => "Matador",
            UnCommonJoker100::Stuntman => "Stuntman",
            UnCommonJoker100::Satellite => "Satellite",
            UnCommonJoker100::Cartomancer => "Cartomancer",
            UnCommonJoker100::Astronomer => "Astronomer",
            UnCommonJoker100::BurntJoker => "Burnt Joker",
            UnCommonJoker100::Bootstraps => "Bootstraps",
        }
    }
}

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum UnCommonJoker101C {
    JokerStencil,
    FourFingers,
    Mime,
    CeremonialDagger,
    MarbleJoker,
    LoyaltyCard,
    Dusk,
    Fibonacci,
    SteelJoker,
    Hack,
    Pareidolia,
    SpaceJoker,
    Burglar,
    Blackboard,
    SixthSense,
    Constellation,
    Hiker,
    CardSharp,
    Madness,
    Seance,
    Shortcut,
    Hologram,
    Cloud9,
    Rocket,
    MidasMask,
    Luchador,
    GiftCard,
    TurtleBean,
    Erosion,
    ToTheMoon,
    StoneJoker,
    LuckyCat,
    Bull,
    DietCola,
    TradingCard,
    FlashCard,
    SpareTrousers,
    Ramen,
    Seltzer,
    Castle,
    MrBones,
    Acrobat,
    SockAndBuskin,
    Troubadour,
    Certificate,
    SmearedJoker,
    Throwback,
    RoughGem,
    Bloodstone,
    Arrowhead,
    OnyxAgate,
    GlassJoker,
    Showman,
    FlowerPot,
    MerryAndy,
    OopsAll6s,
    TheIdol,
    SeeingDouble,
    Matador,
    Stuntman,
    Satellite,
    Cartomancer,
    Astronomer,
    Bootstraps,
}

impl Item for UnCommonJoker101C {
    fn get_name(&self) -> &str {
        match self {
            UnCommonJoker101C::JokerStencil => "Joker Stencil",
            UnCommonJoker101C::FourFingers => "Four Fingers",
            UnCommonJoker101C::Mime => "Mime",
            UnCommonJoker101C::CeremonialDagger => "Ceremonial Dagger",
            UnCommonJoker101C::MarbleJoker => "Marble Joker",
            UnCommonJoker101C::LoyaltyCard => "Loyalty Card",
            UnCommonJoker101C::Dusk => "Dusk",
            UnCommonJoker101C::Fibonacci => "Fibonacci",
            UnCommonJoker101C::SteelJoker => "Steel Joker",
            UnCommonJoker101C::Hack => "Hack",
            UnCommonJoker101C::Pareidolia => "Pareidolia",
            UnCommonJoker101C::SpaceJoker => "Space Joker",
            UnCommonJoker101C::Burglar => "Burglar",
            UnCommonJoker101C::Blackboard => "Blackboard",
            UnCommonJoker101C::SixthSense => "Sixth Sense",
            UnCommonJoker101C::Constellation => "Constellation",
            UnCommonJoker101C::Hiker => "Hiker",
            UnCommonJoker101C::CardSharp => "Card Sharp",
            UnCommonJoker101C::Madness => "Madness",
            UnCommonJoker101C::Seance => "Seance",
            UnCommonJoker101C::Shortcut => "Shortcut",
            UnCommonJoker101C::Hologram => "Hologram",
            UnCommonJoker101C::Cloud9 => "Cloud 9",
            UnCommonJoker101C::Rocket => "Rocket",
            UnCommonJoker101C::MidasMask => "Midas Mask",
            UnCommonJoker101C::Luchador => "Luchador",
            UnCommonJoker101C::GiftCard => "Gift Card",
            UnCommonJoker101C::TurtleBean => "Turtle Bean",
            UnCommonJoker101C::Erosion => "Erosion",
            UnCommonJoker101C::ToTheMoon => "To the Moon",
            UnCommonJoker101C::StoneJoker => "Stone Joker",
            UnCommonJoker101C::LuckyCat => "Lucky Cat",
            UnCommonJoker101C::Bull => "Bull",
            UnCommonJoker101C::DietCola => "Diet Cola",
            UnCommonJoker101C::TradingCard => "Trading Card",
            UnCommonJoker101C::FlashCard => "Flash Card",
            UnCommonJoker101C::SpareTrousers => "Spare Trousers",
            UnCommonJoker101C::Ramen => "Ramen",
            UnCommonJoker101C::Seltzer => "Seltzer",
            UnCommonJoker101C::Castle => "Castle",
            UnCommonJoker101C::MrBones => "Mr. Bones",
            UnCommonJoker101C::Acrobat => "Acrobat",
            UnCommonJoker101C::SockAndBuskin => "Sock and Buskin",
            UnCommonJoker101C::Troubadour => "Troubadour",
            UnCommonJoker101C::Certificate => "Certificate",
            UnCommonJoker101C::SmearedJoker => "Smeared Joker",
            UnCommonJoker101C::Throwback => "Throwback",
            UnCommonJoker101C::RoughGem => "Rough Gem",
            UnCommonJoker101C::Bloodstone => "Bloodstone",
            UnCommonJoker101C::Arrowhead => "Arrowhead",
            UnCommonJoker101C::OnyxAgate => "Onyx Agate",
            UnCommonJoker101C::GlassJoker => "Glass Joker",
            UnCommonJoker101C::Showman => "Showman",
            UnCommonJoker101C::FlowerPot => "Flower Pot",
            UnCommonJoker101C::MerryAndy => "Merry Andy",
            UnCommonJoker101C::OopsAll6s => "Oops! All 6s",
            UnCommonJoker101C::TheIdol => "The Idol",
            UnCommonJoker101C::SeeingDouble => "Seeing Double",
            UnCommonJoker101C::Matador => "Matador",
            UnCommonJoker101C::Stuntman => "Stuntman",
            UnCommonJoker101C::Satellite => "Satellite",
            UnCommonJoker101C::Cartomancer => "Cartomancer",
            UnCommonJoker101C::Astronomer => "Astronomer",
            UnCommonJoker101C::Bootstraps => "Bootstraps",
        }
    }
}

#[derive(EnumIter, Debug, Hash, Eq, PartialEq, Clone)]
pub enum RareJoker {
    DNA,
    Vagabond,
    Baron,
    Obelisk,
    BaseballCard,
    AncientJoker,
    Campfire,
    Blueprint,
    WeeJoker,
    HitTheRoad,
    TheDuo,
    TheTrio,
    TheFamily,
    TheOrder,
    TheTribe,
    Stuntman,
    InvisibleJoker,
    Brainstorm,
    DriversLicense,
    BurntJoke,
}

impl Item for RareJoker {
    fn get_name(&self) -> &str {
        match self {
            RareJoker::DNA => "DNA",
            RareJoker::Vagabond => "Vagabond",
            RareJoker::Baron => "Baron",
            RareJoker::Obelisk => "Obelisk",
            RareJoker::BaseballCard => "Baseball Card",
            RareJoker::AncientJoker => "Ancient Joker",
            RareJoker::Campfire => "Campfire",
            RareJoker::Blueprint => "Blueprint",
            RareJoker::WeeJoker => "Wee Joker",
            RareJoker::HitTheRoad => "Hit the Road",
            RareJoker::TheDuo => "The Duo",
            RareJoker::TheTrio => "The Trio",
            RareJoker::TheFamily => "The Family",
            RareJoker::TheOrder => "The Order",
            RareJoker::TheTribe => "The Tribe",
            RareJoker::Stuntman => "Stuntman",
            RareJoker::InvisibleJoker => "Invisible Joker",
            RareJoker::Brainstorm => "Brainstorm",
            RareJoker::DriversLicense => "Drivers License",
            RareJoker::BurntJoke => "Burnt Joker",
        }
    }
}


#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum RareJoker100 {
    DNA,
    SixthSense,
    Seance,
    Baron,
    Obelisk,
    BaseballCard,
    AncientJoker,
    Campfire,
    Blueprint,
    WeeJoker,
    HitTheRoad,
    TheDuo,
    TheTrio,
    TheFamily,
    TheOrder,
    TheTribe,
    InvisibleJoker,
    Brainstorm,
    DriversLicense,
}

impl Item for RareJoker100 {
    fn get_name(&self) -> &str {
        match self {
            RareJoker100::DNA => "DNA",
            RareJoker100::SixthSense => "Sixth Sense",
            RareJoker100::Seance => "Seance",
            RareJoker100::Baron => "Baron",
            RareJoker100::Obelisk => "Obelisk",
            RareJoker100::BaseballCard => "Baseball Card",
            RareJoker100::AncientJoker => "Ancient Joker",
            RareJoker100::Campfire => "Campfire",
            RareJoker100::Blueprint => "Blueprint",
            RareJoker100::WeeJoker => "Wee Joker",
            RareJoker100::HitTheRoad => "Hit the Road",
            RareJoker100::TheDuo => "The Duo",
            RareJoker100::TheTrio => "The Trio",
            RareJoker100::TheFamily => "The Family",
            RareJoker100::TheOrder => "The Order",
            RareJoker100::TheTribe => "The Tribe",
            RareJoker100::InvisibleJoker => "Invisible Joker",
            RareJoker100::Brainstorm => "Brainstorm",
            RareJoker100::DriversLicense => "Drivers License",
        }
    }
}

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum RareJoker101C {
    DNA,
    Vampire,
    Vagabond,
    Baron,
    Obelisk,
    BaseballCard,
    AncientJoker,
    Campfire,
    Blueprint,
    WeeJoker,
    HitTheRoad,
    TheDuo,
    TheTrio,
    TheFamily,
    TheOrder,
    TheTribe,
    InvisibleJoker,
    Brainstorm,
    DriversLicense,
    BurntJoke,
}

impl Item for RareJoker101C {
    fn get_name(&self) -> &str {
        match self {
            RareJoker101C::DNA => "DNA",
            RareJoker101C::Vampire => "Vampire",
            RareJoker101C::Vagabond => "Vagabond",
            RareJoker101C::Baron => "Baron",
            RareJoker101C::Obelisk => "Obelisk",
            RareJoker101C::BaseballCard => "Baseball Card",
            RareJoker101C::AncientJoker => "Ancient Joker",
            RareJoker101C::Campfire => "Campfire",
            RareJoker101C::Blueprint => "Blueprint",
            RareJoker101C::WeeJoker => "Wee Joker",
            RareJoker101C::HitTheRoad => "Hit the Road",
            RareJoker101C::TheDuo => "The Duo",
            RareJoker101C::TheTrio => "The Trio",
            RareJoker101C::TheFamily => "The Family",
            RareJoker101C::TheOrder => "The Order",
            RareJoker101C::TheTribe => "The Tribe",
            RareJoker101C::InvisibleJoker => "Invisible Joker",
            RareJoker101C::Brainstorm => "Brainstorm",
            RareJoker101C::DriversLicense => "Drivers License",
            RareJoker101C::BurntJoke => "Burnt Joker",
        }
    }
}

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum CommonJoker {
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker,
    HalfJoker,
    CreditCard,
    Banner,
    MysticSummit,
    Ball,
    Misprint,
    RaisedFist,
    ChaosTheClown,
    ScaryFace,
    AbstractJoker,
    DelayedGratification,
    GrosMichel,
    EvenSteven,
    OddTodd,
    Scholar,
    BusinessCard,
    Supernova,
    RideTheBus,
    Egg,
    Runner,
    IceCream,
    Splash,
    BlueJoker,
    FacelessJoker,
    GreenJoker,
    Superposition,
    ToDoList,
    Cavendish,
    RedCard,
    SquareJoker,
    Riffraff,
    Photograph,
    ReservedParking,
    MailInRebate,
    Hallucination,
    FortuneTeller,
    Juggler,
    Drunkard,
    GoldenJoker,
    Popcorn,
    WalkieTalkie,
    SmileyFace,
    GoldenTicket,
    Swashbuckler,
    HangingChad,
    ShootTheMoo,
}

impl Item for CommonJoker {
    fn get_name(&self) -> &str {
        match self {
            CommonJoker::Joker => "Joker",
            CommonJoker::GreedyJoker => "Greedy Joker",
            CommonJoker::LustyJoker => "Lusty Joker",
            CommonJoker::WrathfulJoker => "Wrathful Joker",
            CommonJoker::GluttonousJoker => "Gluttonous Joker",
            CommonJoker::JollyJoker => "Jolly Joker",
            CommonJoker::ZanyJoker => "Zany Joker",
            CommonJoker::MadJoker => "Mad Joker",
            CommonJoker::CrazyJoker => "Crazy Joker",
            CommonJoker::DrollJoker => "Droll Joker",
            CommonJoker::SlyJoker => "Sly Joker",
            CommonJoker::WilyJoker => "Wily Joker",
            CommonJoker::CleverJoker => "Clever Joker",
            CommonJoker::DeviousJoker => "Devious Joker",
            CommonJoker::CraftyJoker => "Crafty Joker",
            CommonJoker::HalfJoker => "Half Joker",
            CommonJoker::CreditCard => "Credit Card",
            CommonJoker::Banner => "Banner",
            CommonJoker::MysticSummit => "Mystic Summit",
            CommonJoker::Ball => "8 Ball",
            CommonJoker::Misprint => "Misprint",
            CommonJoker::RaisedFist => "Raised Fist",
            CommonJoker::ChaosTheClown => "Chaos the Clown",
            CommonJoker::ScaryFace => "Scary Face",
            CommonJoker::AbstractJoker => "Abstract Joker",
            CommonJoker::DelayedGratification => "Delayed Gratification",
            CommonJoker::GrosMichel => "Gros Michel",
            CommonJoker::EvenSteven => "Even Steven",
            CommonJoker::OddTodd => "Odd Todd",
            CommonJoker::Scholar => "Scholar",
            CommonJoker::BusinessCard => "Business Card",
            CommonJoker::Supernova => "Supernova",
            CommonJoker::RideTheBus => "Ride the Bus",
            CommonJoker::Egg => "Egg",
            CommonJoker::Runner => "Runner",
            CommonJoker::IceCream => "Ice Cream",
            CommonJoker::Splash => "Splash",
            CommonJoker::BlueJoker => "Blue Joker",
            CommonJoker::FacelessJoker => "Faceless Joker",
            CommonJoker::GreenJoker => "Green Joker",
            CommonJoker::Superposition => "Superposition",
            CommonJoker::ToDoList => "To Do List",
            CommonJoker::Cavendish => "Cavendish",
            CommonJoker::RedCard => "Red Card",
            CommonJoker::SquareJoker => "Square Joker",
            CommonJoker::Riffraff => "Riff-raff",
            CommonJoker::Photograph => "Photograph",
            CommonJoker::ReservedParking => "Reserved Parking",
            CommonJoker::MailInRebate => "Mail In Rebate",
            CommonJoker::Hallucination => "Hallucination",
            CommonJoker::FortuneTeller => "Fortune Teller",
            CommonJoker::Juggler => "Juggler",
            CommonJoker::Drunkard => "Drunkard",
            CommonJoker::GoldenJoker => "Golden Joker",
            CommonJoker::Popcorn => "Popcorn",
            CommonJoker::WalkieTalkie => "Walkie Talkie",
            CommonJoker::SmileyFace => "Smiley Face",
            CommonJoker::GoldenTicket => "Golden Ticket",
            CommonJoker::Swashbuckler => "Swashbuckler",
            CommonJoker::HangingChad => "Hanging Chad",
            CommonJoker::ShootTheMoo => "Shoot the Moo",
        }
    }
}

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum CommonJoker100 {
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker,
    HalfJoker,
    CreditCard,
    Banner,
    MysticSummit,
    Ball,
    Misprint,
    RaisedFist,
    ChaosTheClown,
    ScaryFace,
    AbstractJoker,
    DelayedGratification,
    GrosMichel,
    EvenSteven,
    OddTodd,
    Scholar,
    BusinessCard,
    Supernova,
    RideTheBus,
    Egg,
    Runner,
    IceCream,
    Splash,
    BlueJoker,
    FacelessJoker,
    GreenJoker,
    Superposition,
    ToDoList,
    Cavendish,
    RedCard,
    SquareJoker,
    Riffraff,
    Photograph,
    MailInRebate,
    Hallucination,
    FortuneTeller,
    Juggler,
    Drunkard,
    GoldenJoker,
    Popcorn,
    WalkieTalkie,
    SmileyFace,
    GoldenTicket,
    Swashbuckler,
    HangingChad,
    ShootTheMoon,
}

impl Item for CommonJoker100 {
    fn get_name(&self) -> &str {
        match self {
            CommonJoker100::Joker => "Joker",
            CommonJoker100::GreedyJoker => "Greedy Joker",
            CommonJoker100::LustyJoker => "Lusty Joker",
            CommonJoker100::WrathfulJoker => "Wrathful Joker",
            CommonJoker100::GluttonousJoker => "Gluttonous Joker",
            CommonJoker100::JollyJoker => "Jolly Joker",
            CommonJoker100::ZanyJoker => "Zany Joker",
            CommonJoker100::MadJoker => "Mad Joker",
            CommonJoker100::CrazyJoker => "Crazy Joker",
            CommonJoker100::DrollJoker => "Droll Joker",
            CommonJoker100::SlyJoker => "Sly Joker",
            CommonJoker100::WilyJoker => "Wily Joker",
            CommonJoker100::CleverJoker => "Clever Joker",
            CommonJoker100::DeviousJoker => "Devious Joker",
            CommonJoker100::CraftyJoker => "Crafty Joker",
            CommonJoker100::HalfJoker => "Half Joker",
            CommonJoker100::CreditCard => "Credit Card",
            CommonJoker100::Banner => "Banner",
            CommonJoker100::MysticSummit => "Mystic Summit",
            CommonJoker100::Ball => "8 Ball",
            CommonJoker100::Misprint => "Misprint",
            CommonJoker100::RaisedFist => "Raised Fist",
            CommonJoker100::ChaosTheClown => "Chaos the Clown",
            CommonJoker100::ScaryFace => "Scary Face",
            CommonJoker100::AbstractJoker => "Abstract Joker",
            CommonJoker100::DelayedGratification => "Delayed Gratification",
            CommonJoker100::GrosMichel => "Gros Michel",
            CommonJoker100::EvenSteven => "Even Steven",
            CommonJoker100::OddTodd => "Odd Todd",
            CommonJoker100::Scholar => "Scholar",
            CommonJoker100::BusinessCard => "Business Card",
            CommonJoker100::Supernova => "Supernova",
            CommonJoker100::RideTheBus => "Ride the Bus",
            CommonJoker100::Egg => "Egg",
            CommonJoker100::Runner => "Runner",
            CommonJoker100::IceCream => "Ice Cream",
            CommonJoker100::Splash => "Splash",
            CommonJoker100::BlueJoker => "Blue Joker",
            CommonJoker100::FacelessJoker => "Faceless Joker",
            CommonJoker100::GreenJoker => "Green Joker",
            CommonJoker100::Superposition => "Superposition",
            CommonJoker100::ToDoList => "To Do List",
            CommonJoker100::Cavendish => "Cavendish",
            CommonJoker100::RedCard => "Red Card",
            CommonJoker100::SquareJoker => "Square Joker",
            CommonJoker100::Riffraff => "Riff-raff",
            CommonJoker100::Photograph => "Photograph",
            CommonJoker100::MailInRebate => "Mail In Rebate",
            CommonJoker100::Hallucination => "Hallucination",
            CommonJoker100::FortuneTeller => "Fortune Teller",
            CommonJoker100::Juggler => "Juggler",
            CommonJoker100::Drunkard => "Drunkard",
            CommonJoker100::GoldenJoker => "Golden Joker",
            CommonJoker100::Popcorn => "Popcorn",
            CommonJoker100::WalkieTalkie => "Walkie Talkie",
            CommonJoker100::SmileyFace => "Smiley Face",
            CommonJoker100::GoldenTicket => "Golden Ticket",
            CommonJoker100::Swashbuckler => "Swashbuckler",
            CommonJoker100::HangingChad => "Hanging Chad",
            CommonJoker100::ShootTheMoon => "Shoot the Moon",
        }
    }
}

#[derive(EnumIter, Debug, Hash, Eq, PartialEq, Clone)]
pub enum PackType {
    Retry,
    ArcanaPack,
    JumboArcanaPack,
    MegaArcanaPack,
    CelestialPack,
    JumboCelestialPack,
    MegaCelestialPack,
    StandardPack,
    JumboStandardPack,
    MegaStandardPack,
    BuffoonPack,
    JumboBuffoonPack,
    MegaBuffoonPack,
    SpectralPack,
    JumboSpectralPack,
    MegaSpectralPack,
}

impl Joker for RareJoker {

}

impl Joker for RareJoker100 {

}

impl Joker for UnCommonJoker {

}
impl Joker for UnCommonJoker100 {

}

impl Joker for UnCommonJoker101C {

}

impl Joker for LegendaryJoker {

}

impl Joker for RareJoker101C {

}

impl Joker for CommonJoker {

}

impl Joker for CommonJoker100 {

}

impl PackType {
    pub fn get_value(&self) -> f64 {
        match self {
            PackType::Retry => 22.42,
            PackType::ArcanaPack => 4.0,
            PackType::JumboArcanaPack => 2.0,
            PackType::MegaArcanaPack => 0.5,
            PackType::CelestialPack => 4.0,
            PackType::JumboCelestialPack => 2.0,
            PackType::MegaCelestialPack => 0.5,
            PackType::StandardPack => 4.0,
            PackType::JumboStandardPack => 2.0,
            PackType::MegaStandardPack => 0.5,
            PackType::BuffoonPack => 1.2,
            PackType::JumboBuffoonPack => 0.6,
            PackType::MegaBuffoonPack => 0.15,
            PackType::SpectralPack => 0.6,
            PackType::JumboSpectralPack => 0.3,
            PackType::MegaSpectralPack => 0.07,
        }
    }

    pub fn get_kind(&self) -> PackKind {
        match self {
            PackType::ArcanaPack | PackType::JumboArcanaPack | PackType::MegaArcanaPack => {
                PackKind::Arcana
            }
            PackType::CelestialPack
            | PackType::JumboCelestialPack
            | PackType::MegaCelestialPack => PackKind::Celestial,
            PackType::StandardPack | PackType::JumboStandardPack | PackType::MegaStandardPack => {
                PackKind::Standard
            }
            PackType::BuffoonPack | PackType::JumboBuffoonPack | PackType::MegaBuffoonPack => {
                PackKind::Buffoon
            }
            PackType::SpectralPack | PackType::JumboSpectralPack | PackType::MegaSpectralPack => {
                PackKind::Spectral
            }
            _ => panic!("Invalid pack type: {:?}", self),
        }
    }

    pub fn is_mega(&self) -> bool {
        matches!(
            self,
            PackType::MegaArcanaPack
                | PackType::MegaCelestialPack
                | PackType::MegaStandardPack
                | PackType::MegaBuffoonPack
                | PackType::MegaSpectralPack
        )
    }

    pub fn is_jumbo(&self) -> bool {
        matches!(
            self,
            PackType::JumboArcanaPack
                | PackType::JumboCelestialPack
                | PackType::JumboStandardPack
                | PackType::JumboBuffoonPack
                | PackType::JumboSpectralPack
        )
    }

    pub fn is_buffoon(&self) -> bool {
        matches!(
            self,
            PackType::BuffoonPack | PackType::JumboBuffoonPack | PackType::MegaBuffoonPack
        )
    }

    pub fn is_spectral(&self) -> bool {
        matches!(
            self,
            PackType::SpectralPack | PackType::JumboSpectralPack | PackType::MegaSpectralPack
        )
    }
}

impl Item for PackType {
    fn get_name(&self) -> &str {
        match self {
            PackType::Retry => "RETRY",
            PackType::ArcanaPack => "Arcana Pack",
            PackType::JumboArcanaPack => "Jumbo Arcana Pack",
            PackType::MegaArcanaPack => "Mega Arcana Pack",
            PackType::CelestialPack => "Celestial Pack",
            PackType::JumboCelestialPack => "Jumbo Celestial Pack",
            PackType::MegaCelestialPack => "Mega Celestial Pack",
            PackType::StandardPack => "Standard Pack",
            PackType::JumboStandardPack => "Jumbo Standard Pack",
            PackType::MegaStandardPack => "Mega Standard Pack",
            PackType::BuffoonPack => "Buffoon Pack",
            PackType::JumboBuffoonPack => "Jumbo Buffoon Pack",
            PackType::MegaBuffoonPack => "Mega Buffoon Pack",
            PackType::SpectralPack => "Spectral Pack",
            PackType::JumboSpectralPack => "Jumbo Spectral Pack",
            PackType::MegaSpectralPack => "Mega Spectral Pack",
        }
    }
}

#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Stake {
    WhiteStake,
    RedStake,
    GreenStake,
    BlackStake,
    BlueStake,
    PurpleStake,
    OrangeStake,
    GoldStake,
}

impl Item for Stake {
    fn get_name(&self) -> &str {
        match self {
            Stake::WhiteStake => "White Stake",
            Stake::RedStake => "Red Stake",
            Stake::GreenStake => "Green Stake",
            Stake::BlackStake => "Black Stake",
            Stake::BlueStake => "Blue Stake",
            Stake::PurpleStake => "Purple Stake",
            Stake::OrangeStake => "Orange Stake",
            Stake::GoldStake => "Gold Stake",
        }
    }
}

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackKind {
    Arcana,
    Celestial,
    Standard,
    Buffoon,
    Spectral,
}

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Voucher {
    Overstock,
    OverstockPlus,
    ClearanceSale,
    Liquidation,
    Hone,
    GlowUp,
    RerollSurplus,
    RerollGlut,
    CrystalBall,
    OmenGlobe,
    Telescope,
    Observatory,
    Grabber,
    NachoTong,
    Wasteful,
    Recyclomancy,
    TarotMerchant,
    TarotTycoon,
    PlanetMerchant,
    PlanetTycoon,
    SeedMoney,
    MoneyTree,
    Blank,
    Antimatter,
    MagicTrick,
    Illusion,
    Hieroglyph,
    Petroglyph,
    DirectorsCut,
    Retcon,
    PaintBrush,
    Palette,
}

impl Item for Voucher {
    fn get_name(&self) -> &str {
        match self {
            Voucher::Overstock => "Overstock",
            Voucher::OverstockPlus => "Overstock Plus",
            Voucher::ClearanceSale => "Clearance Sale",
            Voucher::Liquidation => "Liquidation",
            Voucher::Hone => "Hone",
            Voucher::GlowUp => "Glow Up",
            Voucher::RerollSurplus => "Reroll Surplus",
            Voucher::RerollGlut => "Reroll Glut",
            Voucher::CrystalBall => "Crystal Ball",
            Voucher::OmenGlobe => "Omen Globe",
            Voucher::Telescope => "Telescope",
            Voucher::Observatory => "Observatory",
            Voucher::Grabber => "Grabber",
            Voucher::NachoTong => "Nacho Tong",
            Voucher::Wasteful => "Wasteful",
            Voucher::Recyclomancy => "Recyclomancy",
            Voucher::TarotMerchant => "Tarot Merchant",
            Voucher::TarotTycoon => "Tarot Tycoon",
            Voucher::PlanetMerchant => "Planet Merchant",
            Voucher::PlanetTycoon => "Planet Tycoon",
            Voucher::SeedMoney => "Seed Money",
            Voucher::MoneyTree => "Money Tree",
            Voucher::Blank => "Blank",
            Voucher::Antimatter => "Antimatter",
            Voucher::MagicTrick => "Magic Trick",
            Voucher::Illusion => "Illusion",
            Voucher::Hieroglyph => "Hieroglyph",
            Voucher::Petroglyph => "Petroglyph",
            Voucher::DirectorsCut => "Director's Cut",
            Voucher::Retcon => "Retcon",
            Voucher::PaintBrush => "Paint Brush",
            Voucher::Palette => "Palette",
        }
    }
}

#[derive(EnumIter, Debug, PartialEq, Eq, Clone)]
pub enum Deck {
    RedDeck,
    BlueDeck,
    YellowDeck,
    GreenDeck,
    BlackDeck,
    MagicDeck,
    NebulaDeck,
    GhostDeck,
    AbandonedDeck,
    CheckeredDeck,
    ZodiacDeck,
    PaintedDeck,
    AnaglyphDeck,
    PlasmaDeck,
    ErraticDeck,
}

impl Item for Deck {
    fn get_name(&self) -> &str {
        match self {
            Deck::RedDeck => "Red Deck",
            Deck::BlueDeck => "Blue Deck",
            Deck::YellowDeck => "Yellow Deck",
            Deck::GreenDeck => "Green Deck",
            Deck::BlackDeck => "Black Deck",
            Deck::MagicDeck => "Magic Deck",
            Deck::NebulaDeck => "Nebula Deck",
            Deck::GhostDeck => "Ghost Deck",
            Deck::AbandonedDeck => "Abandoned Deck",
            Deck::CheckeredDeck => "Checkered Deck",
            Deck::ZodiacDeck => "Zodiac Deck",
            Deck::PaintedDeck => "Painted Deck",
            Deck::AnaglyphDeck => "Anaglyph Deck",
            Deck::PlasmaDeck => "Plasma Deck",
            Deck::ErraticDeck => "Erratic Deck",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Boss {
    TheArm,
    TheClub,
    TheEye,
    AmberAcorn,
    CeruleanBell,
    CrimsonHeart,
    VerdantLeaf,
    VioletVessel,
    TheFish,
    TheFlint,
    TheGoad,
    TheHead,
    TheHook,
    TheHouse,
    TheManacle,
    TheMark,
    TheMouth,
    TheNeedle,
    TheOx,
    ThePillar,
    ThePlant,
    ThePsychic,
    TheSerpent,
    TheTooth,
    TheWall,
    TheWater,
    TheWheel,
    TheWindow,
}

impl Item for Boss {
    fn get_name(&self) -> &str {
        match self {
            Boss::TheArm => "The Arm",
            Boss::TheClub => "The Club",
            Boss::TheEye => "The Eye",
            Boss::AmberAcorn => "Amber Acorn",
            Boss::CeruleanBell => "Cerulean Bell",
            Boss::CrimsonHeart => "Crimson Heart",
            Boss::VerdantLeaf => "Verdant Leaf",
            Boss::VioletVessel => "Violet Vessel",
            Boss::TheFish => "The Fish",
            Boss::TheFlint => "The Flint",
            Boss::TheGoad => "The Goad",
            Boss::TheHead => "The Head",
            Boss::TheHook => "The Hook",
            Boss::TheHouse => "The House",
            Boss::TheManacle => "The Manacle",
            Boss::TheMark => "The Mark",
            Boss::TheMouth => "The Mouth",
            Boss::TheNeedle => "The Needle",
            Boss::TheOx => "The Ox",
            Boss::ThePillar => "The Pillar",
            Boss::ThePlant => "The Plant",
            Boss::ThePsychic => "The Psychic",
            Boss::TheSerpent => "The Serpent",
            Boss::TheTooth => "The Tooth",
            Boss::TheWall => "The Wall",
            Boss::TheWater => "The Water",
            Boss::TheWheel => "The Wheel",
            Boss::TheWindow => "The Window",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Specials {
    TheSoul,
    BlackHole,
}

impl Item for Specials {
    fn get_name(&self) -> &str {
        match self {
            Specials::TheSoul => "TheSoul",
            Specials::BlackHole => "BlackHole",
        }
    }
}

#[derive(EnumIter, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tarot {
    TheFool,
    TheMagician,
    TheHighPriestess,
    TheEmpress,
    TheEmperor,
    TheHierophant,
    TheLovers,
    TheChariot,
    Justice,
    TheHermit,
    TheWheelOfFortune,
    Strength,
    TheHangedMan,
    Death,
    Temperance,
    TheDevil,
    TheTower,
    TheStar,
    TheMoon,
    TheSun,
    Judgement,
    TheWorld,
}

impl Item for Tarot {
    fn get_name(&self) -> &str {
        match self {
            Tarot::TheFool => "The Fool",
            Tarot::TheMagician => "The Magician",
            Tarot::TheHighPriestess => "The High Priestess",
            Tarot::TheEmpress => "The Empress",
            Tarot::TheEmperor => "The Emperor",
            Tarot::TheHierophant => "The Hierophant",
            Tarot::TheLovers => "The Lovers",
            Tarot::TheChariot => "The Chariot",
            Tarot::Justice => "Justice",
            Tarot::TheHermit => "The Hermit",
            Tarot::TheWheelOfFortune => "The Wheel of Fortune",
            Tarot::Strength => "Strength",
            Tarot::TheHangedMan => "The Hanged Man",
            Tarot::Death => "Death",
            Tarot::Temperance => "Temperance",
            Tarot::TheDevil => "The Devil",
            Tarot::TheTower => "The Tower",
            Tarot::TheStar => "The Star",
            Tarot::TheMoon => "The Moon",
            Tarot::TheSun => "The Sun",
            Tarot::Judgement => "Judgement",
            Tarot::TheWorld => "The World",
        }
    }
}

#[derive(EnumIter, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    PlanetX,
    Ceres,
    Eris,
}

impl Item for Planet {
    fn get_name(&self) -> &str {
        match self {
            Planet::Mercury => "Mercury",
            Planet::Venus => "Venus",
            Planet::Earth => "Earth",
            Planet::Mars => "Mars",
            Planet::Jupiter => "Jupiter",
            Planet::Saturn => "Saturn",
            Planet::Uranus => "Uranus",
            Planet::Neptune => "Neptune",
            Planet::Pluto => "Pluto",
            Planet::PlanetX => "Planet X",
            Planet::Ceres => "Ceres",
            Planet::Eris => "Eris",
        }
    }
}

#[derive(EnumIter, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Joker,
    Tarot,
    Planet,
    Spectral,
    PlayingCard,
}

#[derive(EnumIter, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Edition {
    Negative,
    Polychrome,
    Holographic,
    Foil,
    NoEdition,
    Eternal,
    Perishable,
    Rental,
}

impl Item for Edition {
    fn get_name(&self) -> &str {
        match self {
            Edition::Negative => "Negative",
            Edition::Polychrome => "Polychrome",
            Edition::Holographic => "Holographic",
            Edition::Foil => "Foil",
            Edition::NoEdition => "No Edition",
            Edition::Eternal => "Eternal",
            Edition::Perishable => "Perishable",
            Edition::Rental => "Rental",
        }
    }
}

impl Item for Type {
    fn get_name(&self) -> &str {
        match self {
            Type::Joker => "Joker",
            Type::Tarot => "Tarot",
            Type::Planet => "Planet",
            Type::Spectral => "Spectral",
            Type::PlayingCard => "Playing Card",
        }
    }
}

#[derive(Debug, Clone, EnumIter, PartialEq, Eq)]
pub enum LegendaryJoker {
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}

impl Item for LegendaryJoker {
    fn get_name(&self) -> &str {
        match self {
            LegendaryJoker::Canio => "Canio",
            LegendaryJoker::Triboulet => "Triboulet",
            LegendaryJoker::Yorick => "Yorick",
            LegendaryJoker::Chicot => "Chicot",
            LegendaryJoker::Perkeo => "Perkeo",
        }
    }
}

#[derive(Debug, Clone, EnumIter, Hash, PartialEq, Eq)]
pub enum Tag {
    UncommonTag,
    RareTag,
    NegativeTag,
    FoilTag,
    HolographicTag,
    PolychromeTag,
    InvestmentTag,
    VoucherTag,
    BossTag,
    StandardTag,
    CharmTag,
    MeteorTag,
    BuffoonTag,
    HandyTag,
    GarbageTag,
    EtherealTag,
    CouponTag,
    DoubleTag,
    JuggleTag,
    D6Tag,
    TopUpTag,
    SpeedTag,
    OrbitalTag,
    EconomyTag,
}

impl Item for Tag {
    fn get_name(&self) -> &str {
        match self {
            Tag::UncommonTag => "Uncommon Tag",
            Tag::RareTag => "Rare Tag",
            Tag::NegativeTag => "Negative Tag",
            Tag::FoilTag => "Foil Tag",
            Tag::HolographicTag => "Holographic Tag",
            Tag::PolychromeTag => "Polychrome Tag",
            Tag::InvestmentTag => "Investment Tag",
            Tag::VoucherTag => "Voucher Tag",
            Tag::BossTag => "Boss Tag",
            Tag::StandardTag => "Standard Tag",
            Tag::CharmTag => "Charm Tag",
            Tag::MeteorTag => "Meteor Tag",
            Tag::BuffoonTag => "Buffoon Tag",
            Tag::HandyTag => "Handy Tag",
            Tag::GarbageTag => "Garbage Tag",
            Tag::EtherealTag => "Ethereal Tag",
            Tag::CouponTag => "Coupon Tag",
            Tag::DoubleTag => "Double Tag",
            Tag::JuggleTag => "Juggle Tag",
            Tag::D6Tag => "D6 Tag",
            Tag::TopUpTag => "Top-up Tag",
            Tag::SpeedTag => "Speed Tag",
            Tag::OrbitalTag => "Orbital Tag",
            Tag::EconomyTag => "Economy Tag",
        }
    }
}