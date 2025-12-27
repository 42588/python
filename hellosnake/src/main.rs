use rand::Rng;
use std::collections::VecDeque;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::{Duration, Instant};

// 贪吃蛇 -- 终端版（使用 crossterm + rand）
// 将此文件保存为 `noe.rs`，在 Cargo.toml 中添加依赖：crossterm = "0.26"，rand = "0.8"
// 运行：cargo run

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, poll, read},
    execute,
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

const WIDTH: u16 = 30;
const HEIGHT: u16 = 20;
const TICK_MS: u64 = 120;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化终端
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;
    // 游戏状态
    let mut snake: VecDeque<(u16, u16)> = VecDeque::new();
    let start_x = WIDTH / 2;
    let start_y = HEIGHT / 2;
    snake.push_back((start_x, start_y));
    snake.push_back((start_x - 1, start_y));
    snake.push_back((start_x - 2, start_y));
    let mut dir = Dir::Right;
    let mut rng = rand::thread_rng();
    let mut food = gen_food(&snake, &mut rng);
    let mut score = 0;
    let mut last_tick = Instant::now();

    'game: loop {
        // 输入处理（非阻塞）
        while poll(Duration::from_millis(0))? {
            if let Event::Key(k) = read()? {
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => break 'game,
                    KeyCode::Up | KeyCode::Char('w') => {
                        if dir != Dir::Down {
                            dir = Dir::Up
                        }
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        if dir != Dir::Up {
                            dir = Dir::Down
                        }
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        if dir != Dir::Right {
                            dir = Dir::Left
                        }
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        if dir != Dir::Left {
                            dir = Dir::Right
                        }
                    }
                    _ => {}
                }
            }
        }

        // 游戏节拍
        if last_tick.elapsed() < Duration::from_millis(TICK_MS) {
            sleep(Duration::from_millis(5));
            continue;
        }
        last_tick = Instant::now();

        // 移动蛇
        let (hx, hy) = snake.front().copied().unwrap();
        let new_head = match dir {
            Dir::Up => (hx, hy.saturating_sub(1)),
            Dir::Down => (hx, hy.saturating_add(1)),
            Dir::Left => (hx.saturating_sub(1), hy),
            Dir::Right => (hx.saturating_add(1), hy),
        };

        // 撞墙判定（不穿墙）
        if new_head.0 == 0 || new_head.0 > WIDTH || new_head.1 == 0 || new_head.1 > HEIGHT {
            break;
        }
        // 撞到自己
        if snake.contains(&new_head) {
            break;
        }

        snake.push_front(new_head);
        if new_head == food {
            score += 1;
            food = gen_food(&snake, &mut rng);
        } else {
            snake.pop_back();
        }

        // 渲染
        draw(&mut stdout, &snake, food, score)?;
    }

    // 游戏结束画面
    execute!(
        stdout,
        MoveTo(0, HEIGHT + 3),
        Print(format!("Game Over! Score: {}. 按任意键退出...", score))
    )?;
    // 等待按键
    loop {
        if poll(Duration::from_millis(100))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
    }

    // 恢复终端
    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}

fn draw<W: Write>(
    w: &mut W,
    snake: &VecDeque<(u16, u16)>,
    food: (u16, u16),
    score: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // 清屏并绘制边框、食物、蛇
    let mut buf = String::new();
    buf.push_str("\x1B[H"); // 光标回到左上（兼容）
    // 顶部边框
    buf.push('+');
    for _ in 0..WIDTH {
        buf.push('-');
    }
    buf.push_str("+\r\n");
    for y in 1..=HEIGHT {
        buf.push('|');
        for x in 1..=WIDTH {
            let ch = if (x, y) == food {
                '●'
            } else if Some(&(x, y)) == snake.front() {
                '◎'
            } else if snake.contains(&(x, y)) {
                'o'
            } else {
                ' '
            };
            buf.push(ch);
        }
        buf.push('|');
        buf.push_str("\r\n");
    }
    // 底部边框
    buf.push('+');
    for _ in 0..WIDTH {
        buf.push('-');
    }
    buf.push_str("+\r\n");
    buf.push_str(&format!(
        "Score: {}  (WASD / Arrows 控制, Q 退出)\r\n",
        score
    ));

    // 输出
    execute!(w, MoveTo(0, 0), Clear(ClearType::All), Print(buf))?;
    w.flush()?;
    Ok(())
}

fn gen_food(snake: &VecDeque<(u16, u16)>, rng: &mut impl Rng) -> (u16, u16) {
    loop {
        let x = rng.gen_range(1..=WIDTH);
        let y = rng.gen_range(1..=HEIGHT);
        if !snake.contains(&(x, y)) {
            return (x, y);
        }
    }
}
