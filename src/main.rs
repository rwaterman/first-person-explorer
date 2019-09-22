extern crate termion;

use std::ascii::escape_default;
use std::f64::consts::PI;
use std::io::{stdin, stdout, Read, Write};
use std::str;
use termion::color;
use termion::raw::IntoRawMode;

fn main() {
    let n_screen_width = 120;
    let n_screen_height = 40;

    let mut f_player_x = 8.0;
    let mut f_player_y = 8.0;
    let mut f_player_a: f64 = 0.0;

    let n_map_height = 16;
    let n_map_width = 16;

    let f_speed = 5.0;

    let f_pov = PI / 4.0;
    let f_depth = 16.0;

    let mut screen = [b'x'; 120 * 40];
    let map = r#"
#########.......
#..............#
#.......########
#..............#
#......##......#
#......##......#
#..............#
###............#
##.............#
#......####..###
#......#.......#
#......#.......#
#..............#
#......#########
#..............#
################
"#
    .as_bytes();

    loop {
        // Initialize 'em all.
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        let stdin = stdin();
        let stdin = stdin.lock();

        let mut bytes = stdin.bytes();
        let b = bytes.next().unwrap().unwrap();
        match b {
            b'a' => {
                f_player_a -= f_speed * 0.75;
            } // * f_elapsed_time
            b'd' => {
                f_player_a += f_speed * 0.75; // * f_elapsed_time
            }
            b'w' => {
                f_player_x += f_player_a.sin() * f_speed; // f_elapsed_time
                f_player_y += f_player_a.cos() * f_speed; // f_elapsed_time

                if map[(f_player_x * n_map_width as f64 + f_player_y) as usize] == b'#' {
                    f_player_x -= f_player_a.sin() * f_speed; // f_elapsed_time
                    f_player_y -= f_player_a.cos() * f_speed; // f_elapsed_time
                }
            }
            b's' => {
                f_player_x -= f_player_a.sin() * f_speed; // * f_elapsed_time;
                f_player_y -= f_player_a.cos() * f_speed; // * f_elapsed_time;

                if map[(f_player_x * n_map_width as f64 + f_player_y) as usize] == b'#' {
                    f_player_x += f_player_a.sin() * f_speed; // * fElapsedTime
                    f_player_y += f_player_a.cos() * f_speed; // * fElapsedTime;;
                }
            }
            b'q' => return,
            // Clear the screen
            // Break up into newlines?
            _ => (),
        }

        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            std::str::from_utf8(&screen).unwrap().to_string()
        )
        .unwrap();

        stdout.flush().unwrap();

        for x in 0..n_screen_width {
            let f_ray_angle = (f_player_a - f_pov / 2.0) + (x / n_screen_width) as f64 * f_pov;

            let mut f_distance_to_wall = 0.0;
            let mut b_hit_wall = false;
            let b_boundary = false; // Set when ray hits boundary between two wall blocks

            let f_eye_x = f_ray_angle.sin();
            let f_eye_y = f_ray_angle.cos();

            while !b_hit_wall && f_distance_to_wall < f_depth {
                f_distance_to_wall += 0.1;

                let n_test_x = (f_player_x + f_eye_x * f_distance_to_wall) as i32;
                let n_test_y = (f_player_y + f_eye_y * f_distance_to_wall) as i32;

                if n_test_x < 0
                    || n_test_x >= n_map_width
                    || n_test_y < 0
                    || n_test_y >= n_map_height
                {
                    b_hit_wall = true;
                    f_distance_to_wall = f_depth;
                } else {
                    // TODO
                    //                    if map[(n_test_x * n_map_width + n_test_y) as usize] == b"#" {
                    //                        b_hit_wall = true;
                    //                        let p = vec![(0.0, 0.0); 0];
                    //
                    //                        for tx in 0..2 {
                    //                            for ty in 0..2 {
                    //                                 Angle of corner to eye
                    //                                let vy = n_test_y + ty - f_player_y as i32;
                    //                                let vx = n_test_x + tx - f_player_x as i32;
                    //                                let d = ((vx * vx + vy * vy) as f64).sqrt();
                    //                                let dot = (f_eye_x * vx as f64 / d) + (f_eye_y * vy as f64 / d);
                    //                            p.push_back(make_pair(d, dot));
                    //                            }

                    // Sort Pairs from closest to farthest
                    //                        sort(p.begin(), p.end(), [](const pair<float, float>& left, const pair<float, float>& right) {return left.first < right.first; });
                    //
                    //                         First two/three are closest (we will never see all four)
                    //                        float fBound = 0.01;
                    //                        if (acos(p.at(0).second) < fBound) b_boundary = true;
                    //                        if (acos(p.at(1).second) < fBound) b_boundary = true;
                    //                        if (acos(p.at(2).second) < fBound) b_boundary = true;
                    //                        }
                }
            }

            let n_ceiling = ((n_screen_height as f64 / 2.0)
                - n_screen_height as f64 / f_distance_to_wall as f64)
                as usize;
            let n_floor = n_screen_height - n_ceiling;

            let mut n_shade = b' ';
            if f_distance_to_wall <= f_depth / 4.0 {
                n_shade = b'1'; // Very close
            } else if f_distance_to_wall < f_depth / 3.0 {
                n_shade = b'2';
            } else if f_distance_to_wall < f_depth / 2.0 {
                n_shade = b'3';
            } else if f_distance_to_wall < f_depth {
                n_shade = b'4';
            } else {
                n_shade = b'5'; // Too far away
            }

            if b_boundary {
                n_shade = b' ';
            }

            for y in 0..n_screen_height {
                if y <= n_ceiling as usize {
                    screen[y * n_screen_width + x] = b' ';
                } else if y > n_ceiling as usize && y <= n_floor as usize {
                    screen[y * n_screen_width + x] = n_shade;
                } else
                // Floor
                {
                    // Shade floor based on distance
                    let b = 1.0 - (y - n_screen_height / 2) as f64 / (n_screen_height / 2) as f64;
                    if b < 0.25 {
                        n_shade = b'#';
                    } else if b < 0.5 {
                        n_shade = b'x';
                    } else if b < 0.75 {
                        n_shade = b'.';
                    } else if b < 0.9 {
                        n_shade = b'-';
                    } else {
                        n_shade = b' ';
                    }
                    screen[y * n_screen_width + x] = n_shade;
                }
            }
        }
    }
}
