use std::{
    io::{stdout, Write, Result},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
};
use rand::prelude::*;

use crate::types::{Config, Firework};

pub fn run() -> Result<()> {
    // 初始配置
    let config = Config::new();

    // 终端设置
    let mut stdout = stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        Hide,
        SetSize(config.width, config.height)
    )?;
    terminal::enable_raw_mode()?;

    let mut fireworks: Vec<Firework> = Vec::new();
    let mut rng = rand::rng();
    let mut last_spawn = Instant::now();
    let mut running = true;

    // 主循环
    while running {
        // 检查是否有键盘输入
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    running = false;
                }
            }
        }

        // 随机创建新烟花
        if fireworks.len() < config.max_fireworks as usize && 
           last_spawn.elapsed() > Duration::from_millis(rng.random_range(100..500)) {
            fireworks.push(Firework::new(&config));
            last_spawn = Instant::now();
        }

        // 清屏
        execute!(stdout, Clear(ClearType::All))?;

        // 显示控制指令
        execute!(
            stdout, 
            MoveTo(0, 0),
            SetForegroundColor(Color::White),
            Print("按键: q:退出")
        )?;

        // 更新并绘制所有烟花
        for firework in &mut fireworks {
            firework.update();
            firework.draw(&mut stdout, &config)?;
        }

        // 移除完成的烟花
        fireworks.retain(|firework| !firework.is_done());

        // 刷新输出
        stdout.flush()?;
        thread::sleep(Duration::from_millis(50));
    }

    // 恢复终端设置
    execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
} 