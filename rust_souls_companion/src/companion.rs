use eframe::egui::{self, Button, CentralPanel, Context, Layout, RichText, TextEdit, Window};
use std::{collections::VecDeque, io::{self, BufReader, BufRead}, fs::{self, File}, path::Path};
use egui_extras::{Column, TableBuilder};
use egui_dropdown::DropDownBox;
use {chrono, image};

use crate::character::{CardPacks, Character};
use crate::player::{Player, PlayerSuper};
use crate::game::Game;

struct Data {
    roll_result: Character,
    character_list: Vec<String>,
    player_list: Vec<Player>,
    player_names_list: Vec<String>,
    is_confirm: bool,
    player_stats_list: Vec<PlayerSuper>,
    game_list: Vec<Game>,
    is_data_loaded: bool
}

impl Data {
    pub(crate) fn new() -> Data {
        Data {
            roll_result: Character::new("unknown", "unknown"), character_list: Vec::new(),
            player_list: Player::get_player_list(), player_names_list: Player::get_player_names_list(), is_confirm: false,
            player_stats_list: PlayerSuper::make_player_list(), game_list: Vec::new(), is_data_loaded: false
        }
    }
}

pub(crate) struct Companion {
    state: i32,
    packs: CardPacks,
    data: Data
}

impl Companion {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let comp = Companion { state: 1, packs: CardPacks::new(), data: Data::new() };
        comp.configure_fonts(&cc.egui_ctx); comp
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut font_def = egui::FontDefinitions::default();
        font_def.font_data.insert("Font Souls".to_string(), egui::FontData::from_static(include_bytes!("../assets/EdmundMcMillen_v2.ttf")));
        font_def.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Font Souls".to_string());
        font_def.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("Font Souls".to_string());
        ctx.set_fonts(font_def);
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Default::default()), |ui| {
                    ui.add_space(10.);
                    ui.with_layout(Layout::top_down(Default::default()), |ui| {
                        ui.add_space(4.);
                        ui.label(RichText::new("👻").size(40.));
                    });

                    ui.with_layout(Layout::top_down(Default::default()), |ui| {
                        ui.add_space(15.);
                        ui.label(RichText::new("    FOUR SOULS COMPANION").size(30.));
                    });
                });
                ui.with_layout(Layout::top_down(Default::default()), |ui| {
                    ui.add_space(14.);
                    ui.with_layout(Layout::right_to_left(Default::default()), |ui| {
                        ui.add_space(10.);
                        if ui.add(Button::new(RichText::new("📔").size(25.))).clicked() {
                            if !self.data.is_data_loaded { self.load_data(); }
                            self.state = 4;
                        }
                        if ui.add(Button::new(RichText::new("📊").size(25.))).clicked() {
                            if !self.data.is_data_loaded { self.load_data(); }
                            self.state = 3;
                        }
                        if ui.add(Button::new(RichText::new("📝").size(25.))).clicked() {
                            self.state = 2;
                        }
                        if ui.add(Button::new(RichText::new("🎲").size(25.))).clicked() {
                            self.data.roll_result = Character::new("unknown", "unknown");
                            self.state = 1;
                        }
                    });
                });
            }); ui.add_space(1.);
        });
    }

    pub(crate) fn render_generate_character(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            Window::new("pack selection menu").title_bar(false).resizable(false).movable(false).default_pos([430., 115.]).show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Default::default()), |ui| {
                    ui.add_space(5.);
                    ui.label(RichText::new("  select card packs  ").size(30.));
                    ui.add_space(10.);
                });

                ui.with_layout(Layout::left_to_right(Default::default()), |ui| {
                    ui.add_space(10.);
                    ui.with_layout(Layout::top_down(Default::default()), |ui| {
                        ui.checkbox(&mut self.packs.base, RichText::new(" base game").size(25.));
                        ui.add_space(10.);

                        ui.checkbox(&mut self.packs.fs_plus, RichText::new(" four souls plus").size(25.));
                        ui.add_space(10.);

                        ui.checkbox(&mut self.packs.requiem, RichText::new(" requiem").size(25.));
                        ui.add_space(10.);

                        ui.checkbox(&mut self.packs.gold_box, RichText::new(" gold box expansion").size(25.));
                        ui.add_space(10.);

                        ui.checkbox(&mut self.packs.promos, RichText::new(" promo cards").size(25.));
                        ui.add_space(10.);

                        ui.checkbox(&mut self.packs.customs, RichText::new(" custom cards").size(25.));
                        ui.add_space(10.);

                        ui.checkbox(&mut self.packs.friends, RichText::new(" texpansion pack").size(25.));
                        ui.add_space(18.);

                        ui.with_layout(Layout::left_to_right(Default::default()), |ui| {
                            ui.add_space(33.);
                            if ui.add(Button::new(RichText::new(" roll ").size(35.))).clicked() {
                                self.data.roll_result = Character::generate_character(self.packs.base, self.packs.fs_plus, self.packs.requiem, self.packs.gold_box, self.packs.promos, self.packs.customs, self.packs.friends)
                            }

                            ui.add_space(20.);
                            if ui.add(Button::new(RichText::new("  ok  ").size(35.))).clicked() {
                                let mut is_duplicate = false;
                                for character in &self.data.character_list {
                                    if character == self.data.roll_result.get_name() {
                                        is_duplicate = true; break;
                                    }
                                }

                                if !is_duplicate && !self.data.roll_result.get_name().contains("unknown") && !self.data.roll_result.get_name().contains("error") && !self.data.roll_result.get_name().is_empty() {
                                    self.data.character_list.push(self.data.roll_result.get_name().to_string());
                                }
                            }
                        }); ui.add_space(15.);
                    });
                });
            });

            ui.with_layout(Layout::top_down(Default::default()), |ui| {
                ui.add_space(3.);
                ui.with_layout(Layout::left_to_right(Default::default()), |ui| {
                    ui.add_space(25.);
                    let img = ui.ctx().load_texture(
                        "character_card", get_image(("assets/cards/".to_string() + self.data.roll_result.get_image_id() + ".png").as_str()),
                        Default::default()
                    );

                    ui.add(egui::Image::new(&img, img.size_vec2()));
                });
            });
        });
    }

    pub(crate) fn render_new_game(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Default::default()), |ui| {
                ui.add_space(10.);
                ui.label(RichText::new("  new game data").size(45.));
                ui.add_space(5.);

                ui.with_layout(Layout::left_to_right(Default::default()), |ui| {
                    ui.add_space(37.);
                    ui.with_layout(Layout::top_down(Default::default()), |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .column(Column::initial(120.))
                            .column(Column::initial(180.))
                            .columns(Column::initial(102.5), 3)
                            .column(Column::initial(60.))
                            .columns(Column::remainder(), 6)
                            .header(20., |mut header| {
                                header.col(|ui| { ui.heading(RichText::new("player").size(30.)); });
                                header.col(|ui| { ui.heading(RichText::new("character").size(30.)); });
                                header.col(|ui| { ui.heading(RichText::new("souls").size(30.)); });
                                header.col(|ui| { ui.heading(RichText::new("money").size(30.)); });
                                header.col(|ui| { ui.heading(RichText::new("items").size(30.)); });
                                header.col(|ui| { ui.heading(RichText::new("win").size(30.)); });
                            })
                            .body(|mut body| {
                                let mut ind = 0;
                                for player in &mut self.data.player_list {
                                    body.row(25., |mut row| {
                                        row.col(|ui| {
                                            ui.checkbox(&mut player.is_active,
                                                        RichText::new(" ".to_string() + self.data.player_names_list.get(ind)
                                                            .expect("String error")
                                                            .as_str())
                                                            .size(20.));
                                        });
                                        row.col(|ui| {
                                            if player.is_active {
                                                ui.with_layout(Layout::top_down(Default::default()), |ui| {
                                                    ui.add_space(3.);
                                                    ui.add(DropDownBox::from_iter(&self.data.character_list,
                                                                                  self.data.player_names_list.get(ind),
                                                                                  &mut player.character, |ui, text|
                                                                                      ui.selectable_label(false, text))
                                                    );
                                                });
                                            }
                                        });
                                        row.col(|ui| {
                                            if player.is_active {
                                                ui.with_layout(Layout::top_down(Default::default()), |ui| {
                                                    ui.add_space(3.);
                                                    ui.add(TextEdit::singleline(&mut player.souls_gained)
                                                        .char_limit(1)
                                                        .hint_text(RichText::new("input souls")));
                                                });
                                            }
                                        });
                                        row.col(|ui| {
                                            if player.is_active {
                                                ui.with_layout(Layout::top_down(Default::default()), |ui| {
                                                    ui.add_space(3.);
                                                    ui.add(TextEdit::singleline(&mut player.money_gained)
                                                        .char_limit(3)
                                                        .hint_text(RichText::new("input money")));
                                                });
                                            }
                                        });
                                        row.col(|ui| {
                                            if player.is_active {
                                                ui.with_layout(Layout::top_down(Default::default()), |ui| {
                                                    ui.add_space(3.);
                                                    ui.add(TextEdit::singleline(&mut player.items_controlled)
                                                        .char_limit(2)
                                                        .hint_text(RichText::new("input items")));
                                                });
                                            }
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(Layout::top_down(Default::default()), |ui| {
                                                ui.add_space(3.);
                                                ui.with_layout(Layout::left_to_right(Default::default()), |ui| {
                                                    ui.add_space(17.);
                                                    ui.checkbox(&mut player.is_winner, "");
                                                });
                                            });
                                        }); ind += 1;
                                    });
                                }
                            });
                    });
                });

                ui.add_space(15.);
                ui.with_layout(Layout::left_to_right(Default::default()), |ui| {
                    ui.add_space(275.);
                    if ui.add(Button::new(RichText::new(" save game ").size(40.))).clicked() {
                        self.data.is_confirm = true;
                    }
                });
            });
        });
    }

    pub(crate) fn render_confirm_window(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |_ui| {
            Window::new("confirm window").title_bar(false).resizable(false).default_pos([222., 210.]).show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Default::default()), |ui| {
                    ui.add_space(20.);
                    ui.label(RichText::new(" SAVE GAME? ").size(44.));
                    ui.add_space(20.);

                    ui.with_layout(Layout::left_to_right(Default::default()), |ui| {
                        ui.add_space(36.);
                        if ui.add(Button::new(RichText::new("  ok  ").size(40.))).clicked() {
                            self.data.is_confirm = false;

                            let mut line = chrono::offset::Local::now().format("%m/%d/%Y").to_string() + &"::".to_string();

                            let mut is_empty = true;
                            for player in &self.data.player_list {
                                if player.is_active {
                                    is_empty = false;
                                    line += &*(player.get_name().to_string() + &"::".to_string());

                                    if player.get_character().is_empty() { line += "isaac::"; }
                                    else { line += &*(player.get_character().to_string() + &"::".to_string()); }

                                    if player.get_souls_gained().is_empty() { line += "0::"; }
                                    else { line += &*(player.get_souls_gained().to_string() + &"::".to_string()); }

                                    if player.get_money_gained().is_empty() { line += "0::"; }
                                    else { line += &*(player.get_money_gained().to_string() + &"::".to_string()); }

                                    if player.get_items_controlled().is_empty() { line += "0::"; }
                                    else { line += &*(player.get_items_controlled().to_string() + &"::".to_string()); }

                                    if player.is_winner { line += "T::"; }
                                    else { line += "F::"; }
                                }
                            } line += "\n";

                            if !is_empty {
                                let data = fs::read_to_string("game_data.txt").expect("Unable to read file");
                                fs::write("game_data.txt", data + &*line).expect("Unable to write to file");
                            } self.data.is_data_loaded = false;
                        }

                        ui.add_space(25.);
                        if ui.add(Button::new(RichText::new("  no  ").size(40.))).clicked() {
                            self.data.is_confirm = false;
                        } ui.add_space(28.);
                    }); ui.add_space(20.);
                });
            });
        });
    }

    pub(crate) fn render_player_stats(&self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Default::default()), |ui| {
                ui.add_space(10.);
                ui.label(RichText::new("  total player data").size(45.));
                ui.add_space(5.);

                TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::initial(120.))
                    .columns(Column::initial(102.5), 6)
                    .columns(Column::remainder(), 7)
                    .header(20., |mut header| {
                        header.col(|ui| { ui.heading(RichText::new("player").size(30.)); });
                        header.col(|ui| { ui.heading(RichText::new("souls").size(30.)); });
                        header.col(|ui| { ui.heading(RichText::new("money").size(30.)); });
                        header.col(|ui| { ui.heading(RichText::new("items").size(30.)); });
                        header.col(|ui| { ui.heading(RichText::new("games").size(30.)); });
                        header.col(|ui| { ui.heading(RichText::new("won").size(30.)); });
                        header.col(|ui| { ui.heading(RichText::new("rate").size(30.)); });
                    })
                    .body(|mut body| {
                        for player in &self.data.player_stats_list {
                            body.row(30., |mut row| {
                                row.col(|ui| { ui.label(RichText::new(player.get_name()).size(25.)); });
                                row.col(|ui| { ui.label(RichText::new(player.get_total_souls_gained().to_string()).size(22.)); });
                                row.col(|ui| { ui.label(RichText::new(player.get_total_money_gained().to_string()).size(22.)); });
                                row.col(|ui| { ui.label(RichText::new(player.get_total_items_controlled().to_string()).size(22.)); });
                                row.col(|ui| { ui.label(RichText::new(player.get_total_games_played().to_string()).size(22.)); });
                                row.col(|ui| { ui.label(RichText::new(player.get_total_games_won().to_string()).size(22.)); });
                                if player.get_total_games_played() == 0 { row.col(|ui| { ui.label(RichText::new("0%").size(22.)); }); }
                                else { row.col(|ui| {
                                    ui.label(RichText::new((((player.get_total_games_won() as f32) / (player.get_total_games_played() as f32) * 100.) as i32).to_string() + "%").size(22.)); });
                                }
                            });
                        }
                    });
            });
        });
    }

    pub(crate) fn render_game_list(&self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Default::default()), |ui| {
                ui.add_space(10.);
                ui.label(RichText::new("  total game data").size(45.));
                ui.add_space(5.);

                TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::initial(130.))
                    .column(Column::initial(495.))
                    .column(Column::initial(135.))
                    .columns(Column::remainder(), 3)
                    .header(20., |mut header| {
                        header.col(|ui| { ui.heading(RichText::new("date").size(28.)); });
                        header.col(|ui| { ui.heading(RichText::new("player data").size(28.)); });
                        header.col(|ui| { ui.heading(RichText::new("winner(s)").size(28.)); });
                    })
                    .body(|mut body| {
                        for game in &self.data.game_list {
                            body.row(game.get_player_count() * 15., |mut row| {
                                row.col(|ui| { ui.label(RichText::new(game.get_date()).size(20.)); });
                                row.col(|ui| { ui.label(RichText::new(game.get_player_data()).size(15.)); });
                                row.col(|ui| { ui.label(RichText::new(game.get_winners()).size(20.)); });
                            });
                        }
                    });
            });
        });
    }

    fn load_data(&mut self) {
        if let Ok(lines) = read_lines("game_data.txt") {
            for line in lines {
                if let Ok(line_temp) = line {
                    if line_temp.is_empty() { continue; }

                    let mut line_parse: VecDeque<&str> = line_temp.split("::").collect();

                    let mut game = Game::new(line_parse.pop_front().unwrap());
                    while line_parse.len() > 2 {
                        let name = line_parse.pop_front().unwrap();
                        let character = line_parse.pop_front().unwrap();
                        let souls_gained = line_parse.pop_front().unwrap();
                        let money_gained = line_parse.pop_front().unwrap();
                        let items_controlled = line_parse.pop_front().unwrap();
                        let winner = line_parse.pop_front().unwrap();

                        for player in &mut self.data.player_stats_list {
                            if player.get_name() == name {
                                player.add_souls_gained(souls_gained.parse::<i32>().unwrap());
                                player.add_money_gained(money_gained.parse::<i32>().unwrap());
                                player.add_items_controlled(items_controlled.parse::<i32>().unwrap());
                                player.increment_games_played();

                                if winner.contains('T') { game.add_winner(name); player.increment_games_won(); }
                            }
                        } game.add_player(name, character, souls_gained, money_gained, items_controlled);
                    } self.data.game_list.push(game);
                }
            }
        } self.data.is_data_loaded = true;
    }
}

impl eframe::App for Companion {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.render_top_panel(ctx);

        match self.state {
            1 => self.render_generate_character(ctx),
            2 => self.render_new_game(ctx),
            3 => self.render_player_stats(ctx),
            4 => self.render_game_list(ctx),
            _ => ()
        }

        if self.data.is_confirm {
            self.render_confirm_window(ctx);
        }
    }
}

pub(crate) fn get_image(filepath: &str) -> egui::ImageData {
    egui::ImageData::from(load_image_from_path(&Path::new(filepath)).unwrap())
}

fn load_image_from_path(path: &Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    Ok(egui::ColorImage::from_rgba_unmultiplied([image.width() as _, image.height() as _], image.to_rgba8().as_flat_samples().as_slice()))
}

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}