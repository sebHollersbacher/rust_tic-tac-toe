use slint::Image;
use std::path::Path;

slint::include_modules!();

#[derive(Copy, Clone, PartialEq)]
enum Player {
    Empty,
    P1,
    P2,
}

struct GameField {
    current_player: Player,
    field: [Player; 9],
}

impl GameField {
    pub fn new() -> Self {
        Self {current_player: Player::P1, field: [Player::Empty;9]}
    }

    pub fn get_current_image(&self) -> Image {
        if self.current_player == Player::P1 {
            return Image::load_from_path(Path::new("./ui/o.png")).unwrap();
        } else {
            return Image::load_from_path(Path::new("./ui/x.png")).unwrap();
        }     
    }

    pub fn play(&mut self, idx: i32) {
        let idx = idx as usize;
        self.field[idx] = self.current_player;
        self.change_player();
    }

    fn change_player(&mut self) {
        if self.current_player == Player::P1 {
            self.current_player = Player::P2;
        } else if self.current_player == Player::P2 {
            self.current_player = Player::P1;
        }
    }

    fn check_win(&self, idx: i32) -> bool {
        let row = (idx / 3) as usize;
        let col = (idx % 3) as usize;
        let mut win = false;

        // row
        if self.field[row*3] == self.field[(row*3)+1] && self.field[row*3] == self.field[(row*3)+2]{
            win = true;
        }

        // col
        if self.field[col] == self.field[col+3] && self.field[col] == self.field[col+6]{
            win = true;
        }

        // main diagonal
        if col == row && self.field[0] == self.field[4] && self.field[0] == self.field[8]{
            win = true;
        }

        // anti-diagonal
        if (idx == 2 || idx == 4 || idx == 6) && self.field[2] == self.field[4] && self.field[2] == self.field[6]{
            win = true;
        }

        win
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let mut field = GameField::new();
    
    let ui = Game::new().unwrap();
    let _weak = ui.as_weak();
    ui.global::<ImageLogic>().on_image_pressed(move | idx | {
        field.play(idx);
        field.check_win(idx);
        field.get_current_image()
    });

    ui.run()
}
