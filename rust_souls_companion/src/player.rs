use crate::companion::read_lines;

pub(crate) struct Player {
    name: String,
    pub(crate) character: String,
    pub(crate) souls_gained: String,
    pub(crate) money_gained: String,
    pub(crate) items_controlled: String,

    pub(crate) is_active: bool,
    pub(crate) is_winner: bool
}

impl Player {
    pub(crate) fn new(name: &str) -> Player {
        Player { name: name.to_string(), character: "".to_string(), souls_gained: "".to_string(), money_gained: "".to_string(), items_controlled: "".to_string(), is_active: false, is_winner: false }
    }

    pub(crate) fn new_with_data(name: &str, character: &str, souls_gained: &str, money_gained: &str, items_controlled: &str) -> Player {
        Player { name: name.to_string(), character: character.to_string(), souls_gained: souls_gained.to_string(), money_gained: money_gained.to_string(), items_controlled: items_controlled.to_string(), is_active: false, is_winner: false }
    }

    pub(crate) fn get_name(&self) -> &String { &self.name }
    pub(crate) fn get_character(&self) -> &String { &self.character }
    pub(crate) fn get_souls_gained(&self) -> &String { &self.souls_gained }
    pub(crate) fn get_money_gained(&self) -> &String { &self.money_gained }
    pub(crate) fn get_items_controlled(&self) -> &String { &self.items_controlled }

    pub(crate) fn get_player_list() -> Vec<Player> {
        let mut player_list = Vec::new();

        if let Ok(lines) = read_lines("player_list.txt") {
            for line in lines {
                if let Ok(line_temp) = line {
                    if line_temp.is_empty() { continue; }

                    player_list.push(Player::new(line_temp.as_str()))
                }
            }
        } player_list
    }

    pub(crate) fn get_player_names_list() -> Vec<String> {
        let list = Player::get_player_list();
        let mut list_2 = Vec::new();

        for player in list {
            list_2.push(player.get_name().to_string());
        } list_2
    }
}

pub(crate) struct PlayerSuper {
    name: String,
    total_souls_gained: i32,
    total_money_gained: i32,
    total_items_controlled: i32,
    total_games_played: i32,
    total_games_won: i32
}

impl PlayerSuper {
    pub(crate) fn new(name: String) -> PlayerSuper {
        PlayerSuper { name, total_souls_gained: 0, total_money_gained: 0, total_items_controlled: 0, total_games_played: 0, total_games_won: 0 }
    }

    pub(crate) fn add_souls_gained(&mut self, num: i32) { self.total_souls_gained += num }
    pub(crate) fn add_money_gained(&mut self, num: i32) { self.total_money_gained += num }
    pub(crate) fn add_items_controlled(&mut self, num: i32) { self.total_items_controlled += num }
    pub(crate) fn increment_games_played(&mut self) { self.total_games_played += 1 }
    pub(crate) fn increment_games_won(&mut self) { self.total_games_won += 1 }

    pub(crate) fn get_name(&self) -> &String { &self.name }
    pub(crate) fn get_total_souls_gained(&self) -> i32 { self.total_souls_gained }
    pub(crate) fn get_total_money_gained(&self) -> i32 { self.total_money_gained }
    pub(crate) fn get_total_items_controlled(&self) -> i32 { self.total_items_controlled }
    pub(crate) fn get_total_games_played(&self) -> i32 { self.total_games_played }
    pub(crate) fn get_total_games_won(&self) -> i32 { self.total_games_won }

    pub(crate) fn make_player_list() -> Vec<PlayerSuper> {
        let list = Player::get_player_list();
        let mut new_list = Vec::new();

        for player in list {
            new_list.push(PlayerSuper::new(player.get_name().to_string()));
        } new_list
    }
}