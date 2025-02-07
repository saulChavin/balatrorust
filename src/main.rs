mod enums;
mod lua_random;
mod util;

use crate::enums::{
    Boss, CommonJoker, CommonJoker100, Deck, Edition, LegendaryJoker, PackType, Planet, RareJoker,
    RareJoker100, RareJoker101C, Specials, Spectral, Stake, Tag, Tarot, Type, UnCommonJoker,
    UnCommonJoker100, UnCommonJoker101C, Voucher,
};
use crate::lua_random::{randint, random};
use crate::util::{pseudohash, round13};
use float_next_after::NextAfter;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator; // Trait for iterating over enum variants
use std::time::{SystemTime, UNIX_EPOCH};

fn current_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis()
}

fn main() {
    let mut found = 0;
    let init = current_millis();
    let count = 100000;

    for x in 0..count {
        let mut functions = Functions::new(
            format!("alex{}", x),
            InstanceParams {
                deck: Deck::RedDeck,
                stake: Stake::WhiteStake,
                showman: false,
                sixes_factor: 1,
                voucher: HashSet::new(),
                version: 10106,
            },
        );

        functions.first_locks();
        functions.init_locks(1);

        for i in 0..2 {
            // println!("Ante {:?}", i);
            // println!("Boss {:?}", functions.next_boss(i));
            // println!("Vocher: {:?}", functions.next_voucher(i));
            // println!("Tag: {:?}", functions.next_tag(i));
            // println!("Tag: {:?}", functions.next_tag(i));

            if i == 0 {
                continue
            }

            if i == 1 {
                for _ in 0..15 {
                    let shop_item = functions.next_shop_item(i);
                    //println!("Shop Item: {:?}", shop_item.item);
                    if shop_item.item == "Blueprint" {
                        found += 1
                    }
                }
            } else {
                for _ in 0..50 {
                    let shop_item = functions.next_shop_item(i);
                   // println!("Shop Item: {:?}", shop_item.item);
                    if shop_item.item == "Blueprint" {
                        found += 1
                    }
                }
            }
        }
    }

    println!("---------------------------------------------------------------------------------------------------");
    println!("Found: {:?} seeds in {} ms, {} ops/s", found, current_millis() - init, count / ((current_millis() - init) / 1000) );
    println!("---------------------------------------------------------------------------------------------------");
}

trait Item: Any {
    fn get_name(&self) -> &str;
}

trait Joker: Item {}

impl Item for &'static dyn Item {
    fn get_name(&self) -> &str {
        (*self).get_name()
    }
}

#[derive(Debug, Clone)]
struct JokerStickers {
    eternal: bool,
    perishable: bool,
    rental: bool,
}

struct JokerData {
    joker: String,
    rarity: i32,
    edition: Edition,
    stickers: JokerStickers,
}

impl JokerData {
    fn new(joker: String, rarity: i32, edition: Edition, stickers: JokerStickers) -> Self {
        Self {
            joker,
            rarity,
            edition,
            stickers,
        }
    }
}

struct ShopItem {
    item_type: Type,
    item: String,
    joker_data: Option<JokerData>,
}

struct ItemOption {
    name: Box<dyn Item>,
    sticker: Box<dyn Item>,
}

struct PackInfo {
    pack_type: PackType,
    size: i32,
    choices: i32,
    options: HashSet<ItemOption>,
}

impl PackInfo {
    fn new(pack_type: PackType, size: i32, choices: i32) -> Self {
        Self {
            pack_type,
            size,
            choices,
            options: HashSet::new(),
        }
    }

    fn contains_option(&self, name: &String) -> bool {
        for i in &self.options {
            if i.name.get_name() == *name {
                return true;
            }
        }
        false
    }
}

impl ShopItem {
    fn new(item_type: Type, item: String, joker_data: Option<JokerData>) -> Self {
        Self {
            item_type,
            item,
            joker_data,
        }
    }
}

struct InstanceParams {
    deck: Deck,
    stake: Stake,
    showman: bool,
    sixes_factor: i32,
    voucher: HashSet<Voucher>,
    version: i64,
}

struct Cache {
    nodes: HashMap<String, f64>,
    generated_first_pack: bool,
}

struct Functions {
    cache: Cache,
    seed: String,
    hashed_seed: f64,
    instance_params: InstanceParams,
    locked: HashSet<String>,
}

struct ShopInstance {
    joker_rate: f64,
    tarot_rate: f64,
    planet_rate: f64,
    playing_card_rate: f64,
    spectral_rate: f64,
}

impl ShopInstance {
    fn total_rate(&self) -> f64 {
        self.joker_rate
            + self.tarot_rate
            + self.planet_rate
            + self.playing_card_rate
            + self.spectral_rate
    }
}

impl Functions {
    fn first_locks(&mut self) {
        self.lock(&"Overstock Plus".to_string());
        self.lock(&"Liquidation".to_string());
        self.lock(&"Glow Up".to_string());
        self.lock(&"Reroll Glut".to_string());
        self.lock(&"Omen Globe".to_string());
        self.lock(&"Observatory".to_string());
        self.lock(&"Nacho Tong".to_string());
        self.lock(&"Recyclomancy".to_string());
        self.lock(&"Tarot Tycoon".to_string());
        self.lock(&"Planet Tycoon".to_string());
        self.lock(&"Money Tree".to_string());
        self.lock(&"Antimatter".to_string());
        self.lock(&"Illusion".to_string());
        self.lock(&"Petroglyph".to_string());
        self.lock(&"Retcon".to_string());
        self.lock(&"Palette".to_string());
    }

    fn init_locks(&mut self, ante: i32) {
        if ante < 2 {
            self.lock(&"The Mouth".to_string());
            self.lock(&"The Fish".to_string());
            self.lock(&"The Wall".to_string());
            self.lock(&"The House".to_string());
            self.lock(&"The Mark".to_string());
            self.lock(&"The Wheel".to_string());
            self.lock(&"The Arm".to_string());
            self.lock(&"The Water".to_string());
            self.lock(&"The Needle".to_string());
            self.lock(&"The Flint".to_string());
            self.lock(&"Negative Tag".to_string());
            self.lock(&"Standard Tag".to_string());
            self.lock(&"Meteor Tag".to_string());
            self.lock(&"Buffoon Tag".to_string());
            self.lock(&"Handy Tag".to_string());
            self.lock(&"Garbage Tag".to_string());
            self.lock(&"Ethereal Tag".to_string());
            self.lock(&"Top-up Tag".to_string());
            self.lock(&"Orbital Tag".to_string());
        }

        if ante < 3 {
            self.lock(&"The Tooth".to_string());
            self.lock(&"The Eye".to_string());
        }

        if ante < 4 {
            self.lock(&"The Plant".to_string());
        }
        if ante < 5 {
            self.lock(&"The Serpent".to_string());
        }
        if ante < 6 {
            self.lock(&"The Ox".to_string());
        }
    }

    fn new(seed: String, instance_params: InstanceParams) -> Self {
        let hashed_seed = pseudohash(&seed);
        let cache = Cache {
            nodes: HashMap::new(),
            generated_first_pack: false,
        };
        Self {
            cache,
            seed,
            hashed_seed,
            instance_params,
            locked: HashSet::new(),
        }
    }

    fn get_node(&mut self, id: String) -> f64 {
        let c = {
            if let Some(&value) = self.cache.nodes.get(&id) {
                value
            } else {
                let nc = pseudohash(&format!("{}{}", id, self.seed));
                self.cache.nodes.insert(id.clone(), nc);
                nc
            }
        };

        let value = round13((c * 1.72431234 + 2.134453429141) % 1.0);
        self.cache.nodes.insert(id, value);

        (value + self.hashed_seed) / 2.0
    }

    fn random(&mut self, id: String) -> f64 {
        unsafe { random(self.get_node(id)) }
    }

    fn randint(&mut self, id: String, max: i64) -> i64 {
        unsafe { randint(self.get_node(id), 0, max) }
    }

    fn lock(&mut self, id: &str) {
        self.locked.insert(id.to_string());
    }

    fn unlock(&mut self, id: &str) {
        self.locked.remove(id);
    }

    fn is_locked<T: Item>(&self, item: &T) -> bool {
        self.locked.contains(item.get_name())
    }

    fn is_locked2(&self, item_name: &str) -> bool {
        self.locked.contains(item_name)
    }

    fn rand_weighted_choice(&mut self, id: String, items: &Vec<PackType>) -> PackType {
        let poll = self.random(id) * 22.42;
        let mut idx = 1;
        let mut weight = 0.0;
        while weight < poll {
            weight += items[idx].get_value();
            idx += 1;
        }
        items[idx - 1].clone()
    }

    fn rand_choice<'a, T>(
        &mut self,
        id: String,
        mapper: fn(&T) -> &str,
        items: &'a Vec<T>,
    ) -> &'a T {
        let mut item = &items[self.randint(id.clone(), items.len() as i64 - 1) as usize];
        let mut name = mapper(item);

        if !self.instance_params.showman && self.is_locked2(&name) || name == "RETRY" {
            let mut resample = 2;
            loop {
                item = &items[self.randint(
                    format!("{}_resample{}", id, resample),
                    items.len() as i64 - 1,
                ) as usize];

                resample += 1;
                name = mapper(item);

                if (!self.is_locked2(&name) && name != "RETRY") || resample > 1000 {
                    return item;
                }
            }
        }

        item
    }

    fn next_tarot(&mut self, source: String, ante: i32, soulable: bool) -> String {
        if soulable
            && (self.instance_params.showman || !self.is_locked(&Specials::TheSoul))
            && self.random(format!("soul_Tarot{}", ante)) > 0.997
        {
            return Specials::TheSoul.get_name().to_string();
        }
        let tarot_items: Vec<Tarot> = Tarot::iter().collect();

        let choice= self.rand_choice(
            format!("Tarot{}{}", source, ante),
            Tarot::get_name,
            &tarot_items
        );

         choice.get_name().to_string()
    }

    fn next_planet(&mut self, source: String, ante: i32, soulable: bool) -> String {
        if soulable
            && (self.instance_params.showman || !self.is_locked(&Specials::BlackHole))
            && self.random(format!("soul_Planet{}", ante)) > 0.997
        {
            return Specials::BlackHole.get_name().to_string();
        }
        self.rand_choice(
            format!("Planet{}{}", source, ante),
            Planet::get_name,
            &Planet::iter().collect::<Vec<_>>(),
        )
        .get_name().to_string()
    }

    fn next_spectral(&mut self, source: String, ante: i32, soulable: bool) -> String {
        if soulable {
            if self.random(format!("soul_Spectral{}", ante)) < 0.997 {
                return self
                    .rand_choice(
                        format!("Spectral{}{}", source, ante),
                        Spectral::get_name,
                        &Spectral::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string();
            }

            if self.instance_params.showman || !self.is_locked(&Specials::TheSoul) {
                return Specials::TheSoul.get_name().to_string();
            }

            if self.instance_params.showman || !self.is_locked(&Specials::BlackHole) {
                return Specials::BlackHole.get_name().to_string();
            }
        }

        self.rand_choice(
            format!("Spectral{}{}", source, ante),
            Spectral::get_name,
            &Spectral::iter().collect::<Vec<_>>(),
        )
        .get_name().to_string()
    }

    fn is_voucher_active(&self, voucher: &Voucher) -> bool {
        self.instance_params.voucher.contains(voucher)
    }

    fn get_shop_instance(&self) -> ShopInstance {
        let mut tarot_rate: f64 = 4.0;
        let mut planet_rate: f64 = 4.0;
        let mut playing_card_rate: f64 = 0.0;
        let mut spectral_rate: f64 = 0.0;

        if self.instance_params.deck == Deck::GhostDeck {
            spectral_rate = 2.0;
        }

        if self.is_voucher_active(&Voucher::TarotTycoon) {
            tarot_rate = 32.0;
        } else if self.is_voucher_active(&Voucher::TarotMerchant) {
            tarot_rate = 9.6;
        }

        if self.is_voucher_active(&Voucher::PlanetTycoon) {
            planet_rate = 32.0;
        } else if self.is_voucher_active(&Voucher::PlanetMerchant) {
            planet_rate = 9.6;
        }

        if self.is_voucher_active(&Voucher::MagicTrick) {
            playing_card_rate = 4.0;
        }

        ShopInstance {
            joker_rate: 20.0,
            tarot_rate,
            planet_rate,
            playing_card_rate,
            spectral_rate,
        }
    }

    fn next_pack(&mut self, ante: i32) -> PackType {
        if ante <= 2 && !self.cache.generated_first_pack && self.instance_params.version > 10099 {
            self.cache.generated_first_pack = true;
            return PackType::BuffoonPack;
        }

        self.rand_weighted_choice(
            format!("shop_pack{}", ante),
            &PackType::iter().collect::<Vec<_>>(),
        )
    }

    fn next_shop_item(&mut self, ante: i32) -> ShopItem {
        let shop = self.get_shop_instance();

        let mut cdt_poll = self.random(format!("cdt{}", ante)) * shop.total_rate();

        let item_type = if cdt_poll < shop.joker_rate {
            Type::Joker
        } else {
            cdt_poll -= shop.joker_rate;
            if cdt_poll < shop.tarot_rate {
                Type::Tarot
            } else {
                cdt_poll -= shop.tarot_rate;
                if cdt_poll < shop.planet_rate {
                    Type::Planet
                } else {
                    cdt_poll -= shop.planet_rate;
                    if cdt_poll < shop.playing_card_rate {
                        Type::PlayingCard
                    } else {
                        Type::Spectral
                    }
                }
            }
        };

        match item_type {
            Type::Joker => {
                let jkr = self.next_joker(&*"sho".to_string(), ante, true);
                ShopItem::new(item_type, jkr.joker.clone(), Some(jkr))
            }
            Type::Tarot => ShopItem::new(
                item_type,
                self.next_tarot("sho".to_string(), ante, false).to_string(),
                None,
            ),
            Type::Planet => ShopItem::new(
                item_type,
                self.next_planet("sho".to_string(), ante, false).to_string(),
                None,
            ),
            Type::Spectral => ShopItem::new(
                item_type,
                self.next_spectral("sho".to_string(), ante, false).to_string(),
                None,
            ),
            Type::PlayingCard => ShopItem {
                item_type,
                item: "Fuck".to_string(),
                joker_data: None,
            },
        }
    }

    fn next_arcana_pack(&mut self, size: usize, ante: i32) -> Vec<String> {
        let mut pack: Vec<String> = Vec::with_capacity(size);

        for _ in 0..size {
            if self.is_voucher_active(&Voucher::OmenGlobe)
                && self.random("omen_globe".to_string()) > 0.8
            {
                pack.push(self.next_spectral("ar2".to_string(), ante, true).to_string());
            } else {
                pack.push(self.next_tarot("ar1".to_string(), ante, true).to_string());
            }
            if !self.instance_params.showman {
                self.lock(pack.last().unwrap());
            }
        }

        for item in &pack {
            self.unlock(item);
        }

        pack
    }

    fn next_celestial_pack(&mut self, size: usize, ante: i32) -> Vec<String> {
        let mut pack: Vec<String> = Vec::with_capacity(size);

        for _ in 0..size {
            let planet = self.next_planet("pl1".to_string(), ante, true);
            pack.push(planet.to_string());
            if !self.instance_params.showman {
                self.lock(pack.last().unwrap());
            }
        }

        for item in &pack {
            self.unlock(item);
        }

        pack
    }

    fn next_spectral_pack(&mut self, size: usize, ante: i32) -> Vec<String> {
        let mut pack: Vec<String> = Vec::with_capacity(size);

        for _ in 0..size {
            pack.push(self.next_spectral("spe".to_string(), ante, true).to_string());

            if !self.instance_params.showman {
                self.lock(pack.last().unwrap());
            }
        }

        for item in &pack {
            self.unlock(item);
        }

        pack
    }

    fn next_buffoon_pack(&mut self, size: usize, ante: i32) -> Vec<JokerData> {
        let mut pack: Vec<JokerData> = Vec::with_capacity(size);

        for _ in 0..size {
            let joker_data = self.next_joker("buf", ante, true);
            let joker_name = joker_data.joker.clone();
            pack.push(joker_data);
            if !self.instance_params.showman {
                self.lock(&joker_name);
            }
        }

        for joker_data in &pack {
            self.unlock(joker_data.joker.as_str());
        }

        pack
    }

    fn set_deck(&mut self, deck: Deck) {
        self.instance_params.deck = deck.clone();
        match deck {
            Deck::MagicDeck => self.activate_voucher(Voucher::CrystalBall),
            Deck::NebulaDeck => self.activate_voucher(Voucher::Telescope),
            Deck::ZodiacDeck => {
                self.activate_voucher(Voucher::TarotMerchant);
                self.activate_voucher(Voucher::PlanetMerchant);
                self.activate_voucher(Voucher::Overstock);
            }
            _ => {}
        }
    }

    fn create_sets() -> (HashSet<&'static str>, HashSet<&'static str>) {
        let set_a: HashSet<&'static str> = vec![
            "Gros Michel",
            "Ice Cream",
            "Cavendish",
            "Luchador",
            "Turtle Bean",
            "Diet Cola",
            "Popcorn",
            "Ramen",
            "Seltzer",
            "Mr. Bones",
            "Invisible Joker",
        ]
        .into_iter()
        .collect();

        let set_b: HashSet<&'static str> = vec![
            "Ceremonial Dagger",
            "Ride the Bus",
            "Runner",
            "Constellation",
            "Green Joker",
            "Red Card",
            "Madness",
            "Square Joker",
            "Vampire",
            "Rocket",
            "Obelisk",
            "Lucky Cat",
            "Flash Card",
            "Spare Trousers",
            "Castle",
            "Wee Joker",
        ]
        .into_iter()
        .collect();

        (set_a, set_b)
    }

    fn next_joker(&mut self, source: &str, ante: i32, has_stickers: bool) -> JokerData {
        let (set_a, set_b) = Self::create_sets();

        // Get rarity
        let rarity = match source {
            "sou" => 4,
            "wra" | "rta" => 3,
            "uta" => 2,
            _ => {
                let rarity_poll = self.random(format!("rarity{}{}", ante, source));
                if rarity_poll > 0.95 {
                    3
                } else if rarity_poll > 0.7 {
                    2
                } else {
                    1
                }
            }
        };

        // Get edition
        let edition_rate = if self.is_voucher_active(&Voucher::GlowUp) {
            4
        } else if self.is_voucher_active(&Voucher::Hone) {
            2
        } else {
            1
        };

        let edition_poll = self.random(format!("edi{}{}", source, ante));
        let edition = if edition_poll > 0.997 {
            Edition::Negative
        } else if edition_poll > 1.0 - 0.006 * edition_rate as f64 {
            Edition::Polychrome
        } else if edition_poll > 1.0 - 0.02 * edition_rate as f64 {
            Edition::Holographic
        } else if edition_poll > 1.0 - 0.04 * edition_rate as f64 {
            Edition::Foil
        } else {
            Edition::NoEdition
        };

        // Get next joker
        let joker: String = match rarity {
            4 => {
                if self.instance_params.version > 10099 {
                    self.rand_choice(
                        "Joker4".to_string(),
                        LegendaryJoker::get_name,
                        &LegendaryJoker::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                } else {
                    self.rand_choice(
                        format!("Joker4{}{}", source, ante),
                        LegendaryJoker::get_name,
                        &LegendaryJoker::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                }
            }
            3 => {
                if self.instance_params.version > 10103 {
                    self.rand_choice(
                        format!("Joker3{}{}", source, ante),
                        RareJoker::get_name,
                        &RareJoker::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                } else if self.instance_params.version > 10099 {
                    self.rand_choice(
                        format!("Joker3{}{}", source, ante),
                        RareJoker101C::get_name,
                        &RareJoker101C::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                } else {
                    self.rand_choice(
                        format!("Joker3{}{}", source, ante),
                        RareJoker100::get_name,
                        &RareJoker100::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                }
            }
            2 => {
                if self.instance_params.version > 10103 {
                    self.rand_choice(
                        format!("Joker2{}{}", source, ante),
                        UnCommonJoker::get_name,
                        &UnCommonJoker::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                } else if self.instance_params.version > 10099 {
                    self.rand_choice(
                        format!("Joker2{}{}", source, ante),
                        UnCommonJoker101C::get_name,
                        &UnCommonJoker101C::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                } else {
                    self.rand_choice(
                        format!("Joker2{}{}", source, ante),
                        UnCommonJoker100::get_name,
                        &UnCommonJoker100::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                }
            }
            _ => {
                if self.instance_params.version > 10099 {
                    self.rand_choice(
                        format!("Joker1{}{}", source, ante),
                        CommonJoker::get_name,
                        &CommonJoker::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                } else {
                    self.rand_choice(
                        format!("Joker1{}{}", source, ante),
                        CommonJoker100::get_name,
                        &CommonJoker100::iter().collect::<Vec<_>>(),
                    )
                    .get_name().to_string()
                }
            }
        };

        // Get next joker stickers
        let mut stickers = JokerStickers {
            eternal: false,
            perishable: false,
            rental: false,
        };

        if has_stickers {
            if self.instance_params.version > 10103 {
                let search_for_sticker = matches!(
                    self.instance_params.stake,
                    Stake::BlackStake
                        | Stake::BlueStake
                        | Stake::PurpleStake
                        | Stake::OrangeStake
                        | Stake::GoldStake
                );

                let mut sticker_poll = 0.0;
                if search_for_sticker {
                    sticker_poll = self.random(format!(
                        "{}{}",
                        if source == "buf" {
                            "packetper"
                        } else {
                            "etperpoll"
                        },
                        ante
                    ));
                }

                if sticker_poll > 0.7 && !set_a.contains(joker.as_str()) {
                    stickers.eternal = true;
                }

                if (sticker_poll > 0.4 && sticker_poll <= 0.7)
                    && matches!(
                        self.instance_params.stake,
                        Stake::OrangeStake | Stake::GoldStake
                    )
                    && !set_b.contains(joker.as_str())
                {
                    stickers.perishable = true;
                }

                if self.instance_params.stake == Stake::GoldStake {
                    stickers.rental = self.random(format!(
                        "{}{}",
                        if source == "buf" { "packssjr" } else { "ssjr" },
                        ante
                    )) > 0.7;
                }
            } else {
                if matches!(
                    self.instance_params.stake,
                    Stake::BlackStake
                        | Stake::BlueStake
                        | Stake::PurpleStake
                        | Stake::OrangeStake
                        | Stake::GoldStake
                ) && !set_a.contains(joker.as_str())
                {
                    stickers.eternal =
                        self.random(format!("stake_shop_joker_eternal{}", ante)) > 0.7;
                }

                if self.instance_params.version > 10099 {
                    if matches!(
                        self.instance_params.stake,
                        Stake::OrangeStake | Stake::GoldStake
                    ) && !stickers.eternal
                    {
                        stickers.perishable = self.random(format!("ssjp{}", ante)) > 0.49;
                    }
                    if self.instance_params.stake == Stake::GoldStake {
                        stickers.rental = self.random(format!("ssjr{}", ante)) > 0.7;
                    }
                }
            }
        }

        JokerData::new(joker.to_string(), rarity, edition, stickers)
    }

    fn activate_voucher(&mut self, voucher: Voucher) {
        self.instance_params.voucher.insert(voucher.clone());
        self.lock(&voucher.get_name());

        let vouchers: Vec<Voucher> = Voucher::iter().collect();
        for i in (0..vouchers.len()).step_by(2) {
            if vouchers[i] == voucher {
                self.unlock(vouchers[i + 1].get_name());
            }
        }
    }

    fn next_boss(&mut self, ante: i32) -> Boss {
        let mut boss_pool: Vec<Boss> = Vec::new();
        let mut num_bosses = 0;

        // First pass: Try to find unlocked bosses
        for boss in Boss::iter() {
            if !self.is_locked(&boss) {
                if (ante % 8 == 0 && !boss.get_name().starts_with('T'))
                    || (ante % 8 != 0 && boss.get_name().starts_with('T'))
                {
                    boss_pool.push(boss.clone());
                    num_bosses += 1;
                }
            }
        }

        // If no bosses found, unlock appropriate bosses and try again
        if num_bosses == 0 {
            for boss in Boss::iter() {
                if (ante % 8 == 0 && !boss.get_name().starts_with('T'))
                    || (ante % 8 != 0 && boss.get_name().starts_with('T'))
                {
                    self.unlock(boss.get_name());
                }
            }
            return self.next_boss(ante);
        }

        let chosen_boss = self.rand_choice("boss".to_string(), Boss::get_name, &boss_pool);
        self.lock(&chosen_boss.get_name());

        chosen_boss.clone()
    }

    fn next_tag(&mut self, ante: i32) -> Tag {
        self.rand_choice(
            format!("Tag{}", ante),
            Tag::get_name,
            &Tag::iter().collect::<Vec<_>>(),
        )
        .clone()
    }

    fn next_voucher(&mut self, ante: i32) -> Voucher {
        self.rand_choice(
            format!("Voucher{}", ante),
            Voucher::get_name,
            &Voucher::iter().collect::<Vec<_>>(),
        )
        .clone()
    }
}
