use crate::player::Player;

pub(crate) struct Game {
    date: String,
    player_list: Vec<Player>,
    winners: Vec<String>
}

impl Game {
    pub(crate) fn new(date: &str) -> Game {
        Game { date: String::from(date), player_list: Vec::new(), winners: Vec::new() }
    }

    pub(crate) fn add_player(&mut self, name: &str, character: &str, souls_gained: &str, money_gained: &str, items_controlled: &str) {
        let _ = &self.player_list.push(Player::new_with_data(name, character, souls_gained, money_gained, items_controlled));
    }

    pub(crate) fn add_winner(&mut self, player: &str) {
        let _ = &self.winners.push(player.to_string());
    }

    pub(crate) fn get_date(&self) -> &String { &self.date }

    pub(crate) fn get_player_data(&self) -> String {
        let mut data = String::new();

        for player in &self.player_list {
            data += &*(player.get_name().to_string() + " (" + player.get_character() + ") gained " + player.get_souls_gained() + " souls, " + player.get_money_gained() + "¢, and " + player.get_items_controlled() + " items.\n");
        } data.pop(); data
    }

    pub(crate) fn get_player_count(&self) -> f32 {
        (self.player_list.len() + 1) as f32
    }

    pub(crate) fn get_winners(&self) -> String {
        let mut winners = String::new();

        for player in &self.winners {
            winners += &*(player.to_string() + "\n");
        } winners.pop(); winners
    }
}