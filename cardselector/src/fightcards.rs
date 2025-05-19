use core::fmt;
use std::{fs};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FightCard {
    pub icon: String,
    pub health: u64,
    pub damage: u64
}

impl fmt::Display for FightCard {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[46m\x1B[93m{}\x1B[30m{}\x1B[91m{}\x1B[49m", self.damage, self.icon, self.health)
    }
}

impl FightCard {
    pub fn new(icon: &str, health: u64, damage: u64) -> Self {
        return Self {
            icon: icon.to_string(),
            health: health,
            damage: damage,
        }
    }

    pub fn read(path: &str) -> Self {
        let file = fs::File::open(path).expect("Wrong File");
        let json: serde_json::Value = serde_json::from_reader(file).expect("Not JSON file");
        let trim_match = ['\"', '\''];
        let icon = json.get("icon").unwrap().to_string().trim_start_matches(&trim_match).trim_end_matches(&trim_match).to_string();
        let health = json.get("health").unwrap().as_u64().unwrap();
        let damage = json.get("health").unwrap().as_u64().unwrap();
        return Self {
            icon,
            health,
            damage,
        }
    }
    pub fn list_mods(path: &str) -> Vec<String> {
        let paths = fs::read_dir(path).unwrap();
        let mut list = vec!();
        for p in paths {
            list.push(p.unwrap().path().display().to_string());
        }
        return list;
    }

    pub fn read_all_from_path(path: &str) -> Vec<FightCard> {
        let paths = fs::read_dir(path).unwrap();
        let mut list = vec!();

        for p in paths {
            list.push(Self::read(&p.unwrap().path().display().to_string()));
        }

        return list;
    }

    pub fn load_all(path: &str) -> Vec<FightCard> {
        let mods = Self::list_mods(path);
        let mut vec = vec!();
        for m in mods {
            println!("Loading {}", m);
            let read = Self::read_all_from_path(&m);
            for r in read {
                vec.push(r);
            }
        }
        return vec;
    }
}
