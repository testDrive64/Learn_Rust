trait ShowStatus {
    fn showStats(&self) -> String;
}
struct termi_status {
    hungry: i64,
    boring: i64,
    health: i64,
    mana: i64,
    level: i64,
}

struct termi {
    name: String,
    status: termi_status,
}

impl ShowStatus for termi {
    fn showStats(&self) -> String {
        let returnStr = format!("Hungry: {}\nBored: {}\nHealth: {}\nMana: {}\nLevel:{}", self.status.hungry.to_string(), self.status.bored.to_string(), self.status.health.to_string(), self.status.mana.to_string(), self.status.level.to_string());
        return returnStr;
    }
}

fn main() {
    let status = termi_status{hungry:10, boring:5, health:9, mana:10, level:2};
    let termi = termi{name: "Jorge".to_string(), status: status};

    println!("{}", termi.showStats());
}
