use nannou::{color, image::GenericImageView, prelude::*};

use rand::Rng;

fn main() {
    nannou::app(model).run();
}

struct Model {
    finished_text: Option<String>,
    mouse: Option<(usize, usize)>,
    square_struct: SquareStruct,
    incorrect: u32,
    cells_to_fill: u32,
    space: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(900, 900)
        .title("Sudoku")
        .view(view)
        .event(event)
        .resizable(false)
        .maximized(true)
        .build()
        .unwrap();

    let temp_comp_board = generate();

    let (temp_game_board, cells_to_fill) = remove_cells(&temp_comp_board);

    let square_struct: SquareStruct = SquareStruct {
        completed_board: temp_comp_board,
        game_board: temp_game_board,
    };

    Model {
        finished_text: None,
        mouse: None,
        square_struct,
        incorrect: 0,
        cells_to_fill,
        space: false,
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    let mut key_press = KeyPressEnum::None;

    match event {
        WindowEvent::MouseMoved(mouse) => {
            let x = mouse.x + 450.0;
            let y = mouse.y + 450.0;
            if x >= 0.0 && y >= 0.0 && x < 900.0 && y < 900.0 {
                model.mouse = Some(((x * 0.01) as usize, (y * 0.01) as usize));
            } else {
                model.mouse = None;
            }
        }
        WindowEvent::MouseExited => {
            model.mouse = None;
        }
        WindowEvent::KeyPressed(Key::Space) => {
            model.space = true;
        }
        WindowEvent::KeyReleased(Key::Space) => {
            model.space = false;
        }
        WindowEvent::KeyReleased(Key::Back | Key::Delete) => {
            key_press = KeyPressEnum::Clear;
        }

        WindowEvent::KeyPressed(Key::Numpad1 | Key::Key1) => {
            key_press = KeyPressEnum::Num(NumEnum::Num1);
        }
        WindowEvent::KeyPressed(Key::Numpad2 | Key::Key2) => {
            key_press = KeyPressEnum::Num(NumEnum::Num2);
        }
        WindowEvent::KeyPressed(Key::Numpad3 | Key::Key3) => {
            key_press = KeyPressEnum::Num(NumEnum::Num3);
        }
        WindowEvent::KeyPressed(Key::Numpad4 | Key::Key4) => {
            key_press = KeyPressEnum::Num(NumEnum::Num4);
        }
        WindowEvent::KeyPressed(Key::Numpad5 | Key::Key5) => {
            key_press = KeyPressEnum::Num(NumEnum::Num5);
        }
        WindowEvent::KeyPressed(Key::Numpad6 | Key::Key6) => {
            key_press = KeyPressEnum::Num(NumEnum::Num6);
        }
        WindowEvent::KeyPressed(Key::Numpad7 | Key::Key7) => {
            key_press = KeyPressEnum::Num(NumEnum::Num7);
        }
        WindowEvent::KeyPressed(Key::Numpad8 | Key::Key8) => {
            key_press = KeyPressEnum::Num(NumEnum::Num8);
        }
        WindowEvent::KeyPressed(Key::Numpad9 | Key::Key9) => {
            key_press = KeyPressEnum::Num(NumEnum::Num9);
        }

        _ => {}
    }

    if let Some(mouse) = model.mouse {
        let x = mouse.0;
        let y = mouse.1;

        let square = &mut model.square_struct.game_board[x][y];

        match key_press {
            KeyPressEnum::None => {}
            KeyPressEnum::Num(num) => {
                match square {
                    CellEnum::StaticSingle(_) | CellEnum::CorrectSingle(_) => {}
                    CellEnum::IncorrectSingle(_) => {
                        if model.space {
                            let mask = (1 << enum_to_usize(&num)) as u32;
                            *square = CellEnum::List(mask);
                        } else {
                            if model.square_struct.completed_board[x][y] == num {
                                *square = CellEnum::CorrectSingle(num);
                                model.cells_to_fill -= 1;
                            } else {
                                *square = CellEnum::IncorrectSingle(num);
                                model.incorrect += 1;
                            }
                        }
                    }
                    CellEnum::List(mask) => {
                        if model.space {
                            *mask ^= 1 << enum_to_usize(&num); //Not the value which toggles if the mini-num is there
                        } else {
                            if model.square_struct.completed_board[x][y] == num {
                                *square = CellEnum::CorrectSingle(num);
                                model.cells_to_fill -= 1;
                            } else {
                                *square = CellEnum::IncorrectSingle(num);
                                model.incorrect += 1;
                            }
                        }
                    }
                }
            }
            KeyPressEnum::Clear => match square {
                CellEnum::StaticSingle(_) | CellEnum::CorrectSingle(_) => {}
                CellEnum::IncorrectSingle(_) | CellEnum::List(_) => {
                    *square = CellEnum::List(0u32);
                }
            },
        }
    }

    if model.cells_to_fill == 0 && model.finished_text.is_none() {
        model.finished_text = Some(
            "Well done, you finished with ".to_string()
                + &model.incorrect.to_string()
                + " mistakes!",
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(color::rgb8(0, 0, 0));

    let mut highlight = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    if let Some(mouse) = model.mouse {
        if model.finished_text.is_none() {
            let x_cell = mouse.0;
            let y_cell = mouse.1;

            for x in 0..9 {
                highlight[x][y_cell] += 1;
            }

            for y in 0..9 {
                highlight[x_cell][y] += 1;
            }

            let x_square = (x_cell / 3) * 3;
            let y_square = (y_cell / 3) * 3;

            for x in x_square..(x_square + 3) {
                for y in y_square..(y_square + 3) {
                    highlight[x][y] += 1;
                }
            }
        }
    }

    for x in 0..3 {
        for y in 0..3 {
            draw.rect()
                .color(color::rgb8(100, 100, 100))
                .x_y((x * 300 - 300) as f32, (y * 300 - 300) as f32)
                .w_h(290.0, 290.0);
        }
    }
    for x in 0..9 {
        for y in 0..9 {
            let color = match highlight[x as usize][y as usize] {
                1 => color::rgb8(200, 200, 200),
                2 => color::rgb8(225, 225, 225),
                3 => color::rgb8(250, 250, 250),
                _ => color::rgb8(175, 175, 175),
            };

            draw.rect()
                .x_y((x * 100 - 400) as f32, (y * 100 - 400) as f32)
                .w_h(90.0, 90.0)
                .color(color);

            let square = &model.square_struct.game_board[x as usize][y as usize];
            match square {
                CellEnum::StaticSingle(num) => {
                    draw.text(&(enum_to_usize(num) + 1).to_string())
                        .color(color::rgb8(0, 0, 0))
                        .font_size(75)
                        .center_justify()
                        .x_y((x * 100 - 400) as f32, (y * 100 - 400) as f32)
                        .w_h(100.0, 100.0);
                }
                CellEnum::CorrectSingle(num) => {
                    draw.text(&(enum_to_usize(num) + 1).to_string())
                        .color(color::rgb8(0, 0, 255))
                        .font_size(75)
                        .center_justify()
                        .x_y((x * 100 - 400) as f32, (y * 100 - 400) as f32)
                        .w_h(100.0, 100.0);
                }

                CellEnum::List(mask) => {
                    for x_mask in -1..2 {
                        let x_mid = (x * 100) - 400 + (x_mask * 30);
                        for y_mask in -1..2 {
                            let y_mid = (y * 100) - 400 + (y_mask * 30);
                            let num = x_mask + 1 + ((y_mask + 1) * 3);
                            if (*mask & (1 << num)) > 0 {
                                draw.text(&(num + 1).to_string())
                                    .color(color::rgb8(0, 0, 0))
                                    .font_size(25)
                                    .center_justify()
                                    .x_y(x_mid as f32, y_mid as f32)
                                    .w_h(100.0, 100.0);
                            }
                        }
                    }
                }

                CellEnum::IncorrectSingle(num) => {
                    draw.text(&(enum_to_usize(num) + 1).to_string())
                        .color(color::rgb8(255, 0, 0))
                        .font_size(75)
                        .center_justify()
                        .x_y((x * 100 - 400) as f32, (y * 100 - 400) as f32)
                        .w_h(100.0, 100.0);
                }
            };
        }
    }

    if let Some(text) = &model.finished_text {
        draw.text(&text)
            .color(color::rgb8(255, 215, 0))
            .font_size(50)
            .center_justify()
            .x_y(0.0, 0.0)
            .w_h(900.0, 900.0);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn enum_to_usize(num: &NumEnum) -> usize {
    match num {
        NumEnum::Num1 => 0,
        NumEnum::Num2 => 1,
        NumEnum::Num3 => 2,
        NumEnum::Num4 => 3,
        NumEnum::Num5 => 4,
        NumEnum::Num6 => 5,
        NumEnum::Num7 => 6,
        NumEnum::Num8 => 7,
        NumEnum::Num9 => 8,
    }
}

fn usize_to_enum(num: usize) -> NumEnum {
    match num {
        0 => NumEnum::Num1,
        1 => NumEnum::Num2,
        2 => NumEnum::Num3,
        3 => NumEnum::Num4,
        4 => NumEnum::Num5,
        5 => NumEnum::Num6,
        6 => NumEnum::Num7,
        7 => NumEnum::Num8,
        _ => NumEnum::Num9,
    }
}

struct SquareStruct {
    pub completed_board: Box<[Box<[NumEnum]>]>,
    pub game_board: Box<[Box<[CellEnum]>]>,
}

enum KeyPressEnum {
    None,
    Num(NumEnum),
    Clear,
}

#[derive(PartialEq, Clone)]
enum NumEnum {
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
}

#[derive(Clone)]
enum CellEnum {
    StaticSingle(NumEnum),
    CorrectSingle(NumEnum),
    IncorrectSingle(NumEnum),
    List(u32),
}

enum GenerateEnum {
    StaticSingle(NumEnum),
    List(Vec<NumEnum>),
}

fn generate() -> Box<[Box<[NumEnum]>]> {
    'repeat_if_invalid: loop {
        let mut temp_list1 = Vec::with_capacity(9);

        for _ in 0..9 {
            let mut temp_list2 = Vec::with_capacity(9);

            for _ in 0..9 {
                let mut temp_list3 = Vec::with_capacity(9);

                for x in 0..9 {
                    temp_list3.push(usize_to_enum(x));
                }

                let gen_enum = GenerateEnum::List(temp_list3);

                temp_list2.push(gen_enum);
            }

            let temp_array2 = temp_list2.into_boxed_slice();

            temp_list1.push(temp_array2);
        }

        let mut board = temp_list1.into_boxed_slice();

        for _ in 0..81 {
            let mut smallest = 10;
            let mut coords = (0, 0);

            for x in 0..9 {
                for y in 0..9 {
                    let temp_ref = &board[x][y];

                    match temp_ref {
                        GenerateEnum::StaticSingle(_) => {}
                        GenerateEnum::List(list) => {
                            if list.len() < smallest {
                                smallest = list.len();
                                coords = (x, y);
                            }
                        }
                    }
                }
            }

            if smallest == 0 || smallest == 10 {
                //Retry if an error appeared
                continue 'repeat_if_invalid;
            } else {
                let randenum;

                match &board[coords.0][coords.1] {
                    GenerateEnum::List(list) => {
                        let randint = rand::thread_rng().gen_range(0..smallest);
                        randenum = list[randint].clone();
                    }
                    _ => {
                        continue 'repeat_if_invalid;
                    }
                };

                //Remove colums
                for x in 0..9 {
                    let temp_ref = &mut board[x][coords.1];

                    match temp_ref {
                        GenerateEnum::List(list) => {
                            'removesame: for z in 0..list.len() {
                                if list[z] == randenum {
                                    list.remove(z);
                                    break 'removesame;
                                }
                            }
                        }
                        _ => {}
                    }
                }

                //Remove rows
                for y in 0..9 {
                    let temp_ref = &mut board[coords.0][y];

                    match temp_ref {
                        GenerateEnum::List(list) => {
                            'removesame: for z in 0..list.len() {
                                if list[z] == randenum {
                                    list.remove(z);
                                    break 'removesame;
                                }
                            }
                        }
                        _ => {}
                    }
                }

                //Remove squares
                let square = ((coords.0 / 3) * 3, (coords.1 / 3) * 3);

                for x in square.0..(square.0 + 3) {
                    for y in square.1..(square.1 + 3) {
                        let temp_ref = &mut board[x][y];

                        match temp_ref {
                            GenerateEnum::List(list) => {
                                'removesame: for z in 0..list.len() {
                                    if list[z] == randenum {
                                        list.remove(z);
                                        break 'removesame;
                                    }
                                }
                            }

                            _ => {}
                        };
                    }
                }

                board[coords.0][coords.1] = GenerateEnum::StaticSingle(randenum);
            }
        }

        if let Some(num_board) = gen_board_to_num_board(&board) {
            return num_board;
        }
    }
}

fn gen_board_to_num_board(board: &Box<[Box<[GenerateEnum]>]>) -> Option<Box<[Box<[NumEnum]>]>> {
    let mut temp_vec_super = Vec::with_capacity(9);

    for x in 0..9 {
        let mut temp_vec_sub: Vec<NumEnum> = Vec::with_capacity(9);

        for y in 0..9 {
            match &board[x][y] {
                GenerateEnum::StaticSingle(num) => {
                    temp_vec_sub.push(num.clone());
                }
                GenerateEnum::List(_) => {
                    return None;
                }
            }
        }

        let temp_array_sub: Box<[NumEnum]> = temp_vec_sub.into_boxed_slice();

        temp_vec_super.push(temp_array_sub);
    }

    let array = temp_vec_super.into_boxed_slice();

    return Some(array);
}

fn num_board_to_game_board(board: &Box<[Box<[NumEnum]>]>) -> Box<[Box<[CellEnum]>]> {
    let mut temp_super_vec = Vec::with_capacity(9);

    for x in 0..9 {
        let mut temp_sub_vec = Vec::with_capacity(9);

        for y in 0..9 {
            temp_sub_vec.push(CellEnum::StaticSingle(board[x][y].clone()));
        }

        temp_super_vec.push(temp_sub_vec.into_boxed_slice());
    }

    return temp_super_vec.into_boxed_slice();
}

fn remove_cells(cboard: &Box<[Box<[NumEnum]>]>) -> (Box<[Box<[CellEnum]>]>, u32) {
    let mut board: Box<[Box<[CellEnum]>]> = num_board_to_game_board(cboard);

    let mut coords = Vec::with_capacity(81);

    let mut removed = 0;

    for x in 0..9 {
        for y in 0..9 {
            coords.push((x, y));
        }
    }

    'removing_items: while coords.len() > 0 {
        let rand_index = rand::thread_rng().gen_range(0..coords.len());
        let coords = coords.remove(rand_index);

        let cell = match &board[coords.0][coords.1] {
            CellEnum::StaticSingle(num) => num.clone(),
            _ => continue 'removing_items,
        };

        let mut can_be_removed = false;

        let mut rows_test = true;

        for x in 0..9 {
            rows_test &= check_if_column_has_num(&board, &cell, (x, coords.1), coords);
        }

        can_be_removed |= rows_test;

        let mut colums_test = true;

        for y in 0..9 {
            colums_test &= check_if_row_has_num(&board, &cell, (coords.0, y), coords)
        }

        can_be_removed |= colums_test;

        if can_be_removed {
            board[coords.0][coords.1] = CellEnum::List(0u32);
            removed += 1;
            continue 'removing_items;
        }
    }

    (board, removed)
}

fn check_if_square_has_num(
    board: &Box<[Box<[CellEnum]>]>,
    num: &NumEnum,
    ignore: (usize, usize),
    coords: (usize, usize),
) -> bool {
    let square = ((coords.0 / 3) * 3, (coords.1 / 3) * 3);

    /* if let CellEnum::List(_) = board[coords.0][coords.1] {
        return true;
    } */

    //IF SQUARE IS FULL BUT 1, then see if that 1 is it

    for x in square.0..(square.0 + 3) {
        for y in square.1..(square.1 + 3) {
            if x != ignore.0 && y != ignore.1 {
                if let CellEnum::StaticSingle(cell_num) = &board[x][y] {
                    if *cell_num == *num {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn check_if_row_has_num(
    board: &Box<[Box<[CellEnum]>]>,
    num: &NumEnum,
    coords: (usize, usize),
    ignore: (usize, usize),
) -> bool {
    if let CellEnum::StaticSingle(_) = board[coords.0][coords.1] {
        return true;
    }

    if check_if_square_has_num(board, num, ignore, coords) {
        return true;
    }

    for x in 0..9 {
        if let CellEnum::StaticSingle(cell_num) = &board[x][coords.1] {
            if *cell_num == *num {
                return true;
            }
        }
    }

    false
}

fn check_if_column_has_num(
    board: &Box<[Box<[CellEnum]>]>,
    num: &NumEnum,
    coords: (usize, usize),
    ignore: (usize, usize),
) -> bool {
    if let CellEnum::StaticSingle(_) = board[coords.0][coords.1] {
        return true;
    }

    if check_if_square_has_num(board, num, ignore, coords) {
        return true;
    }

    for y in 0..9 {
        if let CellEnum::StaticSingle(cell_num) = &board[coords.0][y] {
            if *cell_num == *num {
                return true;
            }
        }
    }

    false
}
