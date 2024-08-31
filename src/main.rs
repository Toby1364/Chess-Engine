use macroquad::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Piece {
    kind: Kind,
    white: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Kind {
    Pawn = 1,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 9,
    King = 100,
}

impl Piece {
    fn wp() -> Option<Self> {
        Some(Piece {
            kind: Kind::Pawn,
            white: true,
        })
    }
    fn wn() -> Option<Self> {
        Some(Piece {
            kind: Kind::Knight,
            white: true,
        })
    }fn wb() -> Option<Self> {
        Some(Piece {
            kind: Kind::Bishop,
            white: true,
        })
    }
    fn wr() -> Option<Self> {
        Some(Piece {
            kind: Kind::Rook,
            white: true,
        })
    }
    fn wq() -> Option<Self> {
        Some(Piece {
            kind: Kind::Queen,
            white: true,
        })
    }
    fn wk() -> Option<Self> {
        Some(Piece {
            kind: Kind::King,
            white: true,
        })
    }

    fn bp() -> Option<Self> {
        Some(Piece {
            kind: Kind::Pawn,
            white: false,
        })
    }
    fn bn() -> Option<Self> {
        Some(Piece {
            kind: Kind::Knight,
            white: false,
        })
    }fn bb() -> Option<Self> {
        Some(Piece {
            kind: Kind::Bishop,
            white: false,
        })
    }
    fn br() -> Option<Self> {
        Some(Piece {
            kind: Kind::Rook,
            white: false,
        })
    }
    fn bq() -> Option<Self> {
        Some(Piece {
            kind: Kind::Queen,
            white: false,
        })
    }
    fn bk() -> Option<Self> {
        Some(Piece {
            kind: Kind::King,
            white: false,
        })
    }
}

const HEAT_MAP: [[f32; 8]; 8] = [
    [1.0, 1.0, 2.0, 2.5, 2.5, 2.0, 1.0, 1.0],
    [1.0, 2.0, 3.0, 3.0, 3.0, 3.0, 2.0, 1.0],
    [2.0, 3.0, 4.0, 3.5, 3.5, 4.0, 3.0, 2.0],
    [2.5, 3.0, 3.5, 4.0, 4.0, 3.5, 3.0, 2.5],
    [2.5, 3.0, 3.5, 4.0, 4.0, 3.5, 3.0, 2.5],
    [2.0, 3.0, 4.0, 3.5, 3.5, 4.0, 3.0, 2.0],
    [1.0, 2.0, 3.0, 3.0, 3.0, 3.0, 2.0, 1.0],
    [1.0, 1.0, 2.0, 2.5, 2.5, 2.0, 1.0, 1.0],
];

#[macroquad::main("chess")]
async fn main() {
    let mut board: [[Option<Piece>; 8]; 8] = [
        [Piece::br(),   Piece::bn(),    Piece::bb(),    Piece::bq(),    Piece::bk(),    Piece::bb(),    Piece::bn(),    Piece::br()],
        [Piece::bp(),   Piece::bp(),    Piece::bp(),    Piece::bp(),    Piece::bp(),    Piece::bp(),    Piece::bp(),    Piece::bp()],
        [None,          None,           None,           None,           None,           None,           None,           None],
        [None,          None,           None,           None,           None,           None,           None,           None],
        [None,          None,           None,           None,           None,           None,           None,           None],
        [None,          None,           None,           None,           None,           None,           None,           None],
        [Piece::wp(),   Piece::wp(),    Piece::wp(),    Piece::wp(),    Piece::wp(),    Piece::wp(),    Piece::wp(),    Piece::wp()],
        [Piece::wr(),   Piece::wn(),    Piece::wb(),    Piece::wq(),    Piece::wk(),    Piece::wb(),    Piece::wn(),    Piece::wr()],
    ];

    let pawn = Texture2D::from_file_with_format(include_bytes!("../textures/pawn.jpg"), None);
    let knight = Texture2D::from_file_with_format(include_bytes!("../textures/knight.jpg"), None);
    let bishop = Texture2D::from_file_with_format(include_bytes!("../textures/bishop.jpg"), None);
    let rook = Texture2D::from_file_with_format(include_bytes!("../textures/rook.jpg"), None);
    let queen = Texture2D::from_file_with_format(include_bytes!("../textures/queen.jpg"), None);
    let king = Texture2D::from_file_with_format(include_bytes!("../textures/king.jpg"), None);

    let mut selected = (0, 0);
    let mut promotion: Option<(usize, usize)> = None;

    let mut wmk = false;
    let mut bmk = false;

    loop {
        next_frame().await;
        clear_background(Color::new(0.3, 0.15, 0.1, 1.));

        let oy = screen_height() / 10.;
        let ox = screen_width() / 2. - oy*4.;

        draw_rectangle(ox, oy, oy*8., oy*8., Color::new(1.0, 0.8, 0.6, 1.));

        for y in 0..8 {
            for x in 0..8 {
                if (x%2!=0&&y%2!=0) || (x%2==0&&y%2==0) {
                    draw_rectangle(ox + x as f32 * oy, oy + y as f32 * oy, oy, oy, Color::new(0.15, 0.4, 0.15, 1.));
                }

                if let Some(piece) = board[y][x] {
                    let texture = match piece.kind {
                        Kind::Pawn => pawn,
                        Kind::Knight => knight,
                        Kind::Bishop => bishop,
                        Kind::Rook => rook,
                        Kind::Queen => queen,
                        Kind::King => king,
                    };
                    let mut color = Color::new(0.25, 0.25, 0.25, 1.);
                    if piece.white {color = WHITE}
                    draw_texture_ex(
                        texture, 
                        ox + x as f32 * oy, 
                        oy + y as f32 * oy, 
                        color,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(oy, oy)),
                            ..Default::default()
                        }
                    );
                }
                if mouse_position().0 > ox + x as f32 * oy && 
                    mouse_position().0 < ox + x as f32 * oy + oy && 
                    mouse_position().1 > oy + y as f32 * oy && 
                    mouse_position().1 < oy + y as f32 * oy + oy && 
                    is_mouse_button_pressed(MouseButton::Left) && 
                    promotion.is_none() {

                    if valid_moves(board, wmk, bmk).contains(&(selected.0, selected.1, x, y)) && board[selected.1][selected.0].unwrap().white {
                        board[y][x] = board[selected.1][selected.0];
                        board[selected.1][selected.0] = None;

                        if let Some(piece) = board[y][x] {
                            if piece.kind == Kind::Pawn && y == 0 {
                                promotion = Some((x, y));
                                board[y][x] = None;
                            }

                            if !wmk && piece.kind == Kind::King {
                                if x == 6 && y == 7 && selected.0 == 4 {
                                    board[7][7] = None;
                                    board[7][5] = Piece::wr();
                                } else if x == 2 && y == 7 && selected.0 == 4 {
                                    board[7][0] = None;
                                    board[7][3] = Piece::wr();
                                }

                                wmk = true;
                            }
                        }

                        play(&mut board, wmk, &mut bmk);
                    }

                    selected = (x, y);
                }
                if (x, y) == selected {
                    draw_rectangle_lines(ox + x as f32 * oy, oy + y as f32 * oy, oy, oy, oy/8., Color::new(0.3, 0.5, 0.8, 1.));
                    draw_rectangle_lines(ox + x as f32 * oy, oy + y as f32 * oy, oy, oy, oy/16., Color::new(0.4, 0.7, 1., 1.));
                } 
            }
        }

        let eval = evaluate(board);

        draw_rectangle(ox - oy * 0.7, oy, oy * 0.3, oy * 8., Color::new(0.15, 0.15, 0.15, 1.));

        draw_rectangle(ox - oy * 0.7, oy*5. - oy * eval/50., oy * 0.3, oy * (eval/50. + 4.), WHITE);

        draw_rectangle_lines(ox - oy * 0.7, oy, oy * 0.3, oy * 8., oy * 0.1, BLACK);

        if let Some((x, y)) = promotion {
            draw_texture_ex(
                queen, 
                ox + 2. * oy, 
                0., 
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(oy, oy)),
                    ..Default::default()
                }
            );

            draw_texture_ex(
                rook, 
                ox + 3. * oy, 
                0., 
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(oy, oy)),
                    ..Default::default()
                }
            );

            draw_texture_ex(
                bishop, 
                ox + 4. * oy, 
                0., 
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(oy, oy)),
                    ..Default::default()
                }
            );

            draw_texture_ex(
                knight, 
                ox + 5. * oy, 
                0., 
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(oy, oy)),
                    ..Default::default()
                }
            );

            if is_mouse_button_pressed(MouseButton::Left) && mouse_position().0 > ox + 2. * oy && mouse_position().0 < ox + 6. * oy && mouse_position().1 < oy {
                if mouse_position().0 < ox + 3. * oy {board[y][x] = Piece::wq()}
                else if mouse_position().0 < ox + 4. * oy {board[y][x] = Piece::wr()}
                else if mouse_position().0 < ox + 5. * oy {board[y][x] = Piece::wb()}
                else {board[y][x] = Piece::wn()}

                promotion = None;
            }
        }
    }
}

fn play(board: &mut [[Option<Piece>; 8]; 8], wmk: bool, bmk: &mut bool) {
    let vm = valid_moves(*board, wmk, *bmk);

    let mut options: Vec<(f32, (usize, usize, usize, usize))> = Vec::new();

    for m in vm {
        let mut ar = board.clone();

        let mut eval = 0.;

        if ar[m.1][m.0].unwrap().white {continue}

        ar[m.3][m.2] = ar[m.1][m.0];
        ar[m.1][m.0] = None;

        eval += evaluate(ar);

        let avm = valid_moves(ar, wmk, *bmk);

        let mut ao: Vec<(f32, (usize, usize, usize, usize))> = Vec::new();

        for m1 in avm {
            let mut ar1 = ar;

            if !ar1[m1.1][m1.0].unwrap().white {continue}

            ar1[m1.3][m1.2] = ar1[m1.1][m1.0];
            ar1[m1.1][m1.0] = None;

            ao.push((evaluate(ar1), m1))
        }
        ao.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        if ao.len() > 0 {
            let am = ao[0].1;

            ao.clear();

            ar[am.3][am.2] = ar[am.1][am.0];
            ar[am.1][am.0] = None;

            if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                ar[m.3][m.2] = Piece::wq();
            }
            if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 7 {
                ar[m.3][m.2] = Piece::bq();
            }

            let avm = valid_moves(ar, wmk, *bmk);

            for m1 in avm {
                let mut ar1 = ar;

                if ar1[m1.1][m1.0].unwrap().white {continue}

                ar1[m1.3][m1.2] = ar1[m1.1][m1.0];
                ar1[m1.1][m1.0] = None;

                ao.push((evaluate(ar1), m1))
            }

            ao.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

            if ao.len() != 0 {
                eval += ao[ao.len() - 1].0;

                let am = ao[0].1;

                ao.clear();

                ar[am.3][am.2] = ar[am.1][am.0];
                ar[am.1][am.0] = None;

                if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                ar[m.3][m.2] = Piece::wq();
                }
                if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 7 {
                    ar[m.3][m.2] = Piece::bq();
                }

                if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                    ar[m.3][m.2] = Piece::wq();
                }

                let avm = valid_moves(ar, wmk, *bmk);

                for m1 in avm {
                    let mut ar1 = ar;

                    if ar1[m1.1][m1.0].unwrap().white {continue}

                    ar1[m1.3][m1.2] = ar1[m1.1][m1.0];
                    ar1[m1.1][m1.0] = None;

                    ao.push((evaluate(ar1), m1))
                }

                ao.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

                if ao.len() != 0 {
                    eval += ao[ao.len() - 1].0;

                    let am = ao[0].1;

                    ao.clear();

                    ar[am.3][am.2] = ar[am.1][am.0];
                    ar[am.1][am.0] = None;

                    if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                    ar[m.3][m.2] = Piece::wq();
                    }
                    if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 7 {
                        ar[m.3][m.2] = Piece::bq();
                    }

                    if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                        ar[m.3][m.2] = Piece::wq();
                    }

                    let avm = valid_moves(ar, wmk, *bmk);

                    for m1 in avm {
                        let mut ar1 = ar;

                        if ar1[m1.1][m1.0].unwrap().white {continue}

                        ar1[m1.3][m1.2] = ar1[m1.1][m1.0];
                        ar1[m1.1][m1.0] = None;

                        if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                        ar[m.3][m.2] = Piece::wq();
                        }
                        if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 7 {
                            ar[m.3][m.2] = Piece::bq();
                        }

                        ao.push((evaluate(ar1), m1))
                    }

                    ao.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

                    if ao.len() != 0 {
                        eval += ao[ao.len() - 1].0;

                        let am = ao[0].1;

                        ao.clear();

                        ar[am.3][am.2] = ar[am.1][am.0];
                        ar[am.1][am.0] = None;

                        if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                        ar[m.3][m.2] = Piece::wq();
                        }
                        if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 7 {
                            ar[m.3][m.2] = Piece::bq();
                        }

                        if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                            ar[m.3][m.2] = Piece::wq();
                        }

                        let avm = valid_moves(ar, wmk, *bmk);

                        for m1 in avm {
                            let mut ar1 = ar;

                            if ar1[m1.1][m1.0].unwrap().white {continue}

                            ar1[m1.3][m1.2] = ar1[m1.1][m1.0];
                            ar1[m1.1][m1.0] = None;

                            if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 0 {
                            ar[m.3][m.2] = Piece::wq();
                            }
                            if ar[am.3][am.2].unwrap().kind == Kind::Pawn && m.3 == 7 {
                                ar[m.3][m.2] = Piece::bq();
                            }

                            ao.push((evaluate(ar1), m1))
                        }

                        ao.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                    }
                }
            }
        }

        if ao.len() != 0 {eval += ao[ao.len() - 1].0}

        options.push((eval, m));
    }

    options.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let m = options[options.len() - 1].1;

    board[m.3][m.2] = board[m.1][m.0];
    board[m.1][m.0] = None;

    if let Some(piece) = board[m.3][m.2] {
        if piece.kind == Kind::Pawn && m.3 == 0 {
            board[m.3][m.2] = Piece::bq();
        }

        if !*bmk && piece.kind == Kind::King {
            if m.3 == 6 && m.2 == 7 && m.0 == 4 {
                board[7][7] = None;
                board[7][5] = Piece::wr();
            } else if m.3 == 2 && m.2 == 7 && m.0 == 4 {
                board[7][0] = None;
                board[7][3] = Piece::wr();
            }

            *bmk = true;
        }
    }
}

fn evaluate(board: [[Option<Piece>; 8]; 8]) -> f32 {
    let mut score = 0.;

    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = board[y][x] {
                let mut s = 0.;
                
                match piece.kind {
                    Kind::Pawn => {
                        if piece.white {s += (piece.kind as i32) as f32 * (7-y) as f32 * 0.5}
                        else {s += (piece.kind as i32) as f32 * y as f32 * 0.5}
                    },
                    Kind::Knight => {
                        s += (piece.kind as i32) as f32 * HEAT_MAP[y][x];
                    },
                    Kind::Bishop => {
                        s += (piece.kind as i32) as f32 * HEAT_MAP[y][x];
                    },
                    Kind::Rook => {
                        s += (piece.kind as i32) as f32 * 2.5;
                    },
                    Kind::Queen => {
                        s += (piece.kind as i32) as f32 * 2.5;
                    },
                    Kind::King => {
                        s += (piece.kind as i32) as f32;
                    },
                }

                if piece.white {score += s}
                else {score -= s}
            }
        }
    }

    return score;
}

fn valid_moves(board: [[Option<Piece>; 8]; 8], wmk: bool, bmk: bool) -> Vec<(usize, usize, usize, usize)> {
    let mut valid = Vec::new();

    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = board[y][x] {
                match piece.kind {
                    Kind::Pawn => {
                        if piece.white {
                            if board[y-1][x].is_none() {
                                valid.push((x, y, x, y-1));

                                if y == 6 && board[y-2][x].is_none() {
                                    valid.push((x, y, x, y-2));
                                }
                            }
                            if x > 0 {
                                if let Some(piece) = board[y-1][x-1] {
                                    if !piece.white {valid.push((x, y, x-1, y-1))};
                                }
                            }
                            if x < 7 {
                                if let Some(piece) = board[y-1][x+1] {
                                    if !piece.white {valid.push((x, y, x+1, y-1))};
                                }
                            }
                        } else {
                            if board[y+1][x].is_none() {
                                valid.push((x, y, x, y+1));

                                if y == 1 && board[y+2][x].is_none() {
                                    valid.push((x, y, x, y+2));
                                }
                            }
                            if x > 0 {
                                if let Some(piece) = board[y+1][x-1] {
                                    if piece.white {valid.push((x, y, x-1, y+1))};
                                }
                            }
                            if x < 7 {
                                if let Some(piece) = board[y+1][x+1] {
                                    if piece.white {valid.push((x, y, x+1, y+1))};
                                }
                            }
                        }
                    },
                    Kind::Knight => {
                        if piece.white {
                            if y > 1 && x > 0 {
                                if let Some(piece) = board[y-2][x-1] {
                                    if !piece.white {valid.push((x, y, x-1, y-2))};
                                }
                                else {valid.push((x, y, x-1, y-2))}
                            }
                            if y > 1 && x < 7 {
                                if let Some(piece) = board[y-2][x+1] {
                                    if !piece.white {valid.push((x, y, x+1, y-2))};
                                }
                                else {valid.push((x, y, x+1, y-2))}
                            }
                            if y < 6 && x > 0 {
                                if let Some(piece) = board[y+2][x-1] {
                                    if !piece.white {valid.push((x, y, x-1, y+2))};
                                }
                                else {valid.push((x, y, x-1, y+2))}
                            }
                            if y < 6 && x < 7 {
                                if let Some(piece) = board[y+2][x+1] {
                                    if !piece.white {valid.push((x, y, x+1, y+2))};
                                }
                                else {valid.push((x, y, x+1, y+2))}
                            }

                            if x > 1 && y > 0 {
                                if let Some(piece) = board[y-1][x-2] {
                                    if !piece.white {valid.push((x, y, x-2, y-1))};
                                }
                                else {valid.push((x, y, x-2, y-1))}
                            }
                            if x < 6 && y > 0 {
                                if let Some(piece) = board[y-1][x+2] {
                                    if !piece.white {valid.push((x, y, x+2, y-1))};
                                }
                                else {valid.push((x, y, x+2, y-1))}
                            }
                            if x > 1 && y < 7 {
                                if let Some(piece) = board[y+1][x-2] {
                                    if !piece.white {valid.push((x, y, x-2, y+1))};
                                }
                                else {valid.push((x, y, x-2, y+1))}
                            }
                            if x < 6 && y < 7 {
                                if let Some(piece) = board[y+1][x+2] {
                                    if !piece.white {valid.push((x, y, x+2, y+1))};
                                }
                                else {valid.push((x, y, x+2, y+1))}
                            }
                        }
                        else {
                            if y > 1 && x > 0 {
                                if let Some(piece) = board[y-2][x-1] {
                                    if piece.white {valid.push((x, y, x-1, y-2))};
                                }
                            }
                            if y > 1 && x < 7 {
                                if let Some(piece) = board[y-2][x+1] {
                                    if piece.white {valid.push((x, y, x+1, y-2))};
                                }
                            }
                            if y < 6 && x > 0 {
                                if let Some(piece) = board[y+2][x-1] {
                                    if piece.white {valid.push((x, y, x-1, y+2))};
                                }
                            }
                            if y < 6 && x < 7 {
                                if let Some(piece) = board[y+2][x+1] {
                                    if piece.white {valid.push((x, y, x+1, y+2))};
                                }
                            }

                            if x > 1 && y > 0 {
                                if let Some(piece) = board[y-1][x-2] {
                                    if piece.white {valid.push((x, y, x-2, y-1))};
                                }
                            }
                            if x < 6 && y > 0 {
                                if let Some(piece) = board[y-1][x+2] {
                                    if piece.white {valid.push((x, y, x+2, y-1))};
                                }
                            }
                            if x > 1 && y < 7 {
                                if let Some(piece) = board[y+1][x-2] {
                                    if piece.white {valid.push((x, y, x-2, y+1))};
                                }
                            }
                            if x < 6 && y < 7 {
                                if let Some(piece) = board[y+1][x+2] {
                                    if piece.white {valid.push((x, y, x+2, y+1))};
                                }
                            }
                        }
                    },
                    Kind::Bishop => {
                        for xy in 1..8 {
                            if x + xy > 7 || y + xy > 7 {break}
                            if let Some(p) = board[y + xy][x + xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x + xy, y + xy))};
                                break;
                            }
                            else {valid.push((x, y, x + xy, y + xy))};
                        }

                        for xy in 1..8 {
                            if x - xy > 7 || y + xy > 7 {break}
                            if let Some(p) = board[y + xy][x - xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x - xy, y + xy))};
                                break;
                            }
                            else {valid.push((x, y, x - xy, y + xy))};
                        }

                        for xy in 1..8 {
                            if x + xy > 7 || y - xy > 7 {break}
                            if let Some(p) = board[y - xy][x + xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x + xy, y - xy))};
                                break;
                            }
                            else {valid.push((x, y, x + xy, y - xy))};
                        }

                        for xy in 1..8 {
                            if x - xy > 7 || y - xy > 7 {break}
                            if let Some(p) = board[y - xy][x - xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x - xy, y - xy))};
                                break;
                            }
                            else {valid.push((x, y, x - xy, y - xy))};
                        }
                    },
                    Kind::Rook => {
                        for xy in 1..8 {
                            if x + xy > 7 {break}
                            if let Some(p) = board[y][x + xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x + xy, y))};
                                break;
                            }
                            else {valid.push((x, y, x + xy, y))};
                        }

                        for xy in 1..8 {
                            if x - xy > 7 {break}
                            if let Some(p) = board[y][x - xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x - xy, y))};
                                break;
                            }
                            else {valid.push((x, y, x - xy, y))};
                        }

                        for xy in 1..8 {
                            if y + xy > 7 {break}
                            if let Some(p) = board[y + xy][x] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x, y + xy))};
                                break;
                            }
                            else {valid.push((x, y, x, y + xy))};
                        }

                        for xy in 1..8 {
                            if y - xy > 7 {break}
                            if let Some(p) = board[y - xy][x] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x, y - xy))};
                                break;
                            }
                            else {valid.push((x, y, x, y - xy))};
                        }
                    },
                    Kind::Queen => {
                        for xy in 1..8 {
                            if x + xy > 7 || y + xy > 7 {break}
                            if let Some(p) = board[y + xy][x + xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x + xy, y + xy))};
                                break;
                            }
                            else {valid.push((x, y, x + xy, y + xy))};
                        }

                        for xy in 1..8 {
                            if x - xy > 7 || y + xy > 7 {break}
                            if let Some(p) = board[y + xy][x - xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x - xy, y + xy))};
                                break;
                            }
                            else {valid.push((x, y, x - xy, y + xy))};
                        }

                        for xy in 1..8 {
                            if x + xy > 7 || y - xy > 7 {break}
                            if let Some(p) = board[y - xy][x + xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x + xy, y - xy))};
                                break;
                            }
                            else {valid.push((x, y, x + xy, y - xy))};
                        }

                        for xy in 1..8 {
                            if x - xy > 7 || y - xy > 7 {break}
                            if let Some(p) = board[y - xy][x - xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x - xy, y - xy))};
                                break;
                            }
                            else {valid.push((x, y, x - xy, y - xy))};
                        }

                        for xy in 1..8 {
                            if x + xy > 7 {break}
                            if let Some(p) = board[y][x + xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x + xy, y))};
                                break;
                            }
                            else {valid.push((x, y, x + xy, y))};
                        }

                        for xy in 1..8 {
                            if x - xy > 7 {break}
                            if let Some(p) = board[y][x - xy] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x - xy, y))};
                                break;
                            }
                            else {valid.push((x, y, x - xy, y))};
                        }

                        for xy in 1..8 {
                            if y + xy > 7 {break}
                            if let Some(p) = board[y + xy][x] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x, y + xy))};
                                break;
                            }
                            else {valid.push((x, y, x, y + xy))};
                        }

                        for xy in 1..8 {
                            if y - xy > 7 {break}
                            if let Some(p) = board[y - xy][x] {
                                if (piece.white && !p.white) || (!piece.white && p.white) {valid.push((x, y, x, y - xy))};
                                break;
                            }
                            else {valid.push((x, y, x, y - xy))};
                        }
                    },
                    Kind::King => {
                        if piece.white {
                            if !wmk && y == 7 && x == 4 &&
                                board[7][1].is_none() && 
                                board[7][2].is_none() &&
                                board[7][3].is_none() &&
                                board[7][0].unwrap_or_else(|| Piece::bk().unwrap()).kind == Kind::Rook {

                                valid.push((x, y, 2, 7));
                            }
                            if !wmk && y == 7 && x == 4 &&
                                board[7][5].is_none() && 
                                board[7][6].is_none() &&
                                board[7][7].unwrap_or_else(|| Piece::bk().unwrap()).kind == Kind::Rook {

                                valid.push((x, y, 6, 7));
                            }

                            if x > 0 {
                                if let Some(piece) = board[y][x-1] {
                                    if !piece.white {valid.push((x, y, x-1, y))};
                                }
                                else {valid.push((x, y, x-1, y))}
                            }
                            if x < 7 {
                                if let Some(piece) = board[y][x+1] {
                                    if !piece.white {valid.push((x, y, x+1, y))};
                                }
                                else {valid.push((x, y, x+1, y))}
                            }
                            if y > 0 {
                                if let Some(piece) = board[y-1][x] {
                                    if !piece.white {valid.push((x, y, x, y-1))};
                                }
                                else {valid.push((x, y, x, y-1))}
                            }
                            if y < 7 {
                                if let Some(piece) = board[y+1][x] {
                                    if !piece.white {valid.push((x, y, x, y+1))};
                                }
                                else {valid.push((x, y, x, y+1))}
                            }
                            if x > 0 && y > 0 {
                                if let Some(piece) = board[y-1][x-1] {
                                    if !piece.white {valid.push((x, y, x-1, y-1))};
                                }
                                else {valid.push((x, y, x-1, y-1))}
                            }
                            if x < 7 && y > 0 {
                                if let Some(piece) = board[y-1][x+1] {
                                    if !piece.white {valid.push((x, y, x+1, y-1))};
                                }
                                else {valid.push((x, y, x+1, y-1))}
                            }
                            if x > 0 && y < 7 {
                                if let Some(piece) = board[y+1][x-1] {
                                    if !piece.white {valid.push((x, y, x-1, y+1))};
                                }
                                else {valid.push((x, y, x-1, y+1))}
                            }
                            if x < 7 && y < 7 {
                                if let Some(piece) = board[y+1][x+1] {
                                    if !piece.white {valid.push((x, y, x+1, y+1))};
                                }
                                else {valid.push((x, y, x+1, y+1))}
                            }
                        } else {
                            if !bmk && y == 0 && x == 4 &&
                                board[0][1].is_none() && 
                                board[0][2].is_none() &&
                                board[0][3].is_none() &&
                                board[0][0].unwrap_or_else(|| Piece::bk().unwrap()).kind == Kind::Rook {

                                valid.push((x, y, 2, 0));
                            }
                            if !bmk && y == 0 && x == 4 &&
                                board[0][5].is_none() && 
                                board[0][6].is_none() &&
                                board[0][7].unwrap_or_else(|| Piece::bk().unwrap()).kind == Kind::Rook {

                                valid.push((x, y, 6, 0));
                            }

                            if x > 0 {
                                if let Some(piece) = board[y][x-1] {
                                    if piece.white {valid.push((x, y, x-1, y))};
                                }
                                else {valid.push((x, y, x-1, y))}
                            }
                            if x < 7 {
                                if let Some(piece) = board[y][x+1] {
                                    if piece.white {valid.push((x, y, x+1, y))};
                                }
                                else {valid.push((x, y, x+1, y))}
                            }
                            if y > 0 {
                                if let Some(piece) = board[y-1][x] {
                                    if piece.white {valid.push((x, y, x, y-1))};
                                }
                                else {valid.push((x, y, x, y-1))}
                            }
                            if y < 7 {
                                if let Some(piece) = board[y+1][x] {
                                    if piece.white {valid.push((x, y, x, y+1))};
                                }
                                else {valid.push((x, y, x, y+1))}
                            }
                            if x > 0 && y > 0 {
                                if let Some(piece) = board[y-1][x-1] {
                                    if piece.white {valid.push((x, y, x-1, y-1))};
                                }
                                else {valid.push((x, y, x-1, y-1))}
                            }
                            if x < 7 && y > 0 {
                                if let Some(piece) = board[y-1][x+1] {
                                    if piece.white {valid.push((x, y, x+1, y-1))};
                                }
                                else {valid.push((x, y, x+1, y-1))}
                            }
                            if x > 0 && y < 7 {
                                if let Some(piece) = board[y+1][x-1] {
                                    if piece.white {valid.push((x, y, x-1, y+1))};
                                }
                                else {valid.push((x, y, x-1, y+1))}
                            }
                            if x < 7 && y < 7 {
                                if let Some(piece) = board[y+1][x+1] {
                                    if piece.white {valid.push((x, y, x+1, y+1))};
                                }
                                else {valid.push((x, y, x+1, y+1))}
                            }
                        }
                    },
                }
            }
        }
    }

    valid
}