use crate::GameHandler;
use crossterm::event::KeyCode;
use crate::Error;
use crate::GameState;
use tui::backend::Backend;
use tui::Frame;
use tui::{
    layout::{ Rect, Layout, Direction, Constraint, Alignment },
    style::{Color, Style, Modifier},
    text::Text,
    widgets::{
        Block, BorderType, Borders, Paragraph, ListItem, List, ListState
    },
};

use crate::models::attack_options::*;

pub struct BattleView {
    menu_state: ListState,
    menu_list: Vec<String>,
}

impl BattleView {
    pub fn new() -> BattleView {
        let mut state = ListState::default();
        state.select(Some(0));

        BattleView {
            menu_state: state,
            menu_list: vec!["Attack".to_string(), "Items".to_string(), "Other".to_string()],
        }
    }
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), Error> {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [Constraint::Min(10), Constraint::Length(10)].as_ref(),
            )
            .split(rect);

        let menu_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
            )
            .split(main_chunks[1]);

        frame.render_widget(self.render_image(), main_chunks[0]);
        
        let mut menu_state = self.menu_state.clone();
        frame.render_stateful_widget(self.build_battle_menu(), menu_chunks[0], &mut menu_state);

        self.render_action_part(frame, menu_chunks[1], game_state)?;
        
        Ok(())
    }

    fn render_action_part(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), Error>  {
        match self.menu_state.selected() {
            Some(0) => {
                self.render_attack_menu(frame, rect, game_state)?;
            }
            Some(1) => {}
            Some(3) => {}
            Some(_) | None => {}
        }

        Ok(())
    }

    fn render_attack_menu(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), Error> {
        let action_rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                .split(rect);

            let action_row_0 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                .split(action_rows[0]);

            let action_row_1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                .split(action_rows[1]);

        let (one, two, three, four) = self.build_battle_options(&game_state.get_attack_options());

        frame.render_widget(one, action_row_0[0]);
        frame.render_widget(two, action_row_0[1]);

        frame.render_widget(three, action_row_1[0]);
        frame.render_widget(four, action_row_1[1]);

        Ok(())
    }


    fn render_image(&self) -> Paragraph {
        let img = Text::from("                                                                                
        ...:--+++==+===--:..                    
        ..-=+#*####+=----------===-::.               
     -++**#%##++--:.===========++++++=:-.            
   =*##%%%%#*=-..-===--+++=====++++++--==-           
    +###%%%%*+***##%%=-+**+***+***+++---=*-          
    .:-+**##*#%%##%%%#++####*#####*++==++**          
   .*++#%#*#%*==#%%%%@#**#%##%%@@%%@#--==**          
    #%%%##%*.   :##%@+..-*##%%%@+ :*--===+#.         
   :#%@%%%=     =#%#=    .+#%%%@=     :+**#+         
    .-=#+.     -%#-       :#%%@%%-     :+###-        
              -%%.        .#%%+###-      :*#%=       
             -%%:          +%% -#**.       =#%*      
             #%-           -#%=**=.         -##      
            :#*             +#%+.            +%:     
          :+*#.            .=+#              =%+     
          .::           ...:=+#=..          .==:     
                            :::.                ");
        Paragraph::new(img)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded),
            )
    }

    fn build_battle_menu(&self) -> List {
        let items: Vec<ListItem> = self.menu_list.iter().map(|text| ListItem::new(text.to_owned())).collect();

        List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC).bg(Color::Rgb(60, 60, 60)))
    }

    fn build_battle_options(&self, options: &Vec<AttackOption>) -> (Paragraph, Paragraph, Paragraph, Paragraph) {
        return (
            self.build_battle_tile(&options[0]),
            self.build_battle_tile(&options[1]),
            self.build_battle_tile(&options[2]),
            self.build_battle_tile(&options[3])
        );
    }

    fn build_battle_tile(&self, option: &AttackOption) -> Paragraph {
        let p: Paragraph = match option {
            AttackOption::None => {
                Paragraph::new("N/A")
            }
            AttackOption::Attack(description) => {
                Paragraph::new(description.title.to_owned())
            }
        };

        p.block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded),
        )
    }

    pub fn handle_input(&mut self, key_code: KeyCode, game_handler: &mut GameHandler) -> Result<bool, Error> {
        match key_code {
            KeyCode::Char('w') | KeyCode::Up => {
                self.menu_up();
            }
            KeyCode::Char('s') | KeyCode::Down => {
                self.menu_down();
            }
            KeyCode::Char('1') => {

            }
            _ => {
                match self.menu_state.selected() {
                    Some(0) => {
                        self.handle_attack_input(key_code, game_handler);
                    }
                    Some(1) => {}
                    Some(3) => {}
                    Some(_) | None => {}
                }
            }
        }

        Ok(true)
    }

    fn handle_attack_input(&self, key_code: KeyCode, game_handler: &mut GameHandler) {
        match key_code {
            KeyCode::Char('1') => {

            }
            KeyCode::Char('2') => {

            }
            KeyCode::Char('3') => {

            }
            KeyCode::Char('4') => {

            }
            _ => {}
        }
    }

    fn menu_down(&mut self) {
        if let Some(selected) = self.menu_state.selected() {
            if selected >= self.menu_list.len() - 1 {
                self.menu_state.select(Some(0));
            } else {
                self.menu_state.select(Some(selected + 1));
            }

        }
    }

    fn menu_up(&mut self) {
        if let Some(selected) = self.menu_state.selected() {
            if selected > 0 {
                self.menu_state.select(Some(selected - 1));
            } else {
                let amount = self.menu_list.len();
                self.menu_state.select(Some(amount - 1));
            }
        }    
    }
}