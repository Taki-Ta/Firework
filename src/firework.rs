use std::io::{Write, Result};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
};
use rand::prelude::*;

use crate::types::{Config, Firework, Particle, ColorType};

impl Firework {
    pub fn new(config: &Config) -> Firework {
        let mut rng = rand::rng();
        
        let size = rng.random_range(config.min_firework_size..=config.max_firework_size);
        
        // 计算安全边距，确保烟花不会在终端边缘放置
        let horizontal_margin = size * 10;
        let vertical_margin = size * 5;
        
        let safe_horizontal_margin = horizontal_margin.min(config.width / 4);
        let safe_vertical_margin = vertical_margin.min(config.height / 4);
        
        let x = rng.random_range(
            safe_horizontal_margin..(config.width - safe_horizontal_margin)
        );
        
        let explosion_y = rng.random_range(
            safe_vertical_margin..(config.height / 2)
        );
        
        // 使用更鲜艳的颜色组合
        // 主要采用蓝色、绿色、红色、粉色和黄色这些明亮色彩
        let color_scheme = rng.random_range(0..12); // 12种鲜艳配色
        
        // 每个烟花使用2-3种相近颜色
        let (primary, secondary, tertiary) = match color_scheme {
            // 红色系列 - 更鲜艳
            0 => (Color::Red, Color::Yellow, Color::Magenta),       // 红黄粉
            1 => (Color::Red, Color::White, Color::Red),            // 红白红
            
            // 绿色系列 - 更鲜艳
            2 => (Color::Green, Color::Cyan, Color::Yellow),        // 绿青黄
            3 => (Color::Green, Color::Yellow, Color::Green),       // 绿黄绿
            
            // 蓝色系列 - 更鲜艳
            4 => (Color::Blue, Color::Cyan, Color::White),          // 蓝青白
            5 => (Color::Blue, Color::Magenta, Color::Cyan),        // 蓝粉青
            
            // 黄色系列 - 更鲜艳
            6 => (Color::Yellow, Color::White, Color::Green),       // 黄白绿
            7 => (Color::Yellow, Color::Red, Color::Cyan),          // 黄红青
            
            // 粉色(品红)系列 - 更鲜艳
            8 => (Color::Magenta, Color::Red, Color::Blue),         // 粉红蓝
            9 => (Color::Magenta, Color::White, Color::Yellow),     // 粉白黄
            
            // 青色系列 - 更鲜艳
            10 => (Color::Cyan, Color::Blue, Color::White),         // 青蓝白
            
            // 混合系列 - 更鲜艳
            _ => (Color::Yellow, Color::Magenta, Color::Cyan),      // 黄粉青
        };
        
        // 随机决定是否使用第三种颜色
        let use_tertiary = rng.random_bool(0.7); // 70%概率使用第三种颜色
        
        // 有三种颜色的情况下，决定用哪两种颜色
        let (final_secondary, final_tertiary) = if use_tertiary {
            (secondary, tertiary) // 使用全部三种颜色
        } else {
            // 只使用两种颜色，第三种颜色用第二种代替
            (secondary, secondary)
        };
        
        // 随机决定爆炸阶段数量 (1-2)
        let max_explosion_stages = rng.random_range(1..=2);
        
        Firework {
            x,
            y: config.height,
            explosion_x: x,
            explosion_y,
            size,
            primary_color: primary,
            secondary_color: final_secondary,
            tertiary_color: final_tertiary,
            exploded: false,
            particles: Vec::new(),
            age: 0,
            max_age: rng.random_range(25..45),
            trail: Vec::new(),
            explosion_stage: 0,
            max_explosion_stages,
        }
    }

    pub fn update(&mut self) {
        if !self.exploded {
            // 上升阶段
            if self.y > self.explosion_y {
                // 记录轨迹
                if self.trail.len() >= 5 {
                    self.trail.remove(0); // 保持尾迹长度有限
                }
                self.trail.push((self.x, self.y));
                
                self.y -= 1;
            } else {
                self.explode();
            }
        } else {
            // 爆炸后更新粒子
            self.age += 1;
            
            // 多阶段爆炸：当大部分粒子消失后，触发下一阶段爆炸
            if self.explosion_stage < self.max_explosion_stages - 1 && 
               self.particles.len() < self.size as usize && 
               self.age > 15 * (self.explosion_stage as u16 + 1) {
                self.explosion_stage += 1;
                self.trigger_secondary_explosion();
            }
            
            for particle in &mut self.particles {
                particle.update();
            }
            // 移除过期粒子
            self.particles.retain(|p| p.age < p.max_age);
        }
    }

    fn explode(&mut self) {
        self.exploded = true;
        let mut rng = rand::rng();
        
        // 创建更多粒子，使爆炸效果更壮观
        let particle_count = (self.size as usize) * 50; // 使用usize类型
        
        // 计算适当的缩放因子来调整为真正的圆形
        let y_scale = 0.4; // 垂直方向缩放系数，补偿字符高宽比
        
        // 创建线状爆炸效果
        // 首先确定几条主线
        let line_count = 8 + (self.size as usize);
        let main_angles: Vec<f32> = (0..line_count)
            .map(|i| (i as f32 / line_count as f32) * std::f32::consts::PI * 2.0)
            .collect();
        
        // 沿每条主线创建粒子，形成线状效果
        for &main_angle in &main_angles {
            // 主线上的粒子数量
            let line_particles = particle_count / line_count / 2;
            
            // 沿主线创建粒子
            for i in 0..line_particles {
                // 距离爆炸点的距离逐渐增加
                let distance_factor = (i as f32 / line_particles as f32).powf(0.7); // 非线性分布，使粒子更集中
                let speed_base = 0.6 + distance_factor * 0.6; // 距离越远速度越快，范围0.6-1.2
                
                // 轻微扰动角度，使线不那么规则
                let angle_variance = rng.random_range(-0.05..0.05);
                let angle = main_angle + angle_variance;
                
                // 应用缩放因子来调整为视觉上的圆形
                let vx = angle.cos() * speed_base;
                let vy = angle.sin() * speed_base * y_scale;
                
                // 计算预期移动距离
                let distance = speed_base * 0.8;
                
                // 根据距离选择字符 - 线状效果用更线性的字符
                let symbol = match i % 4 {
                    0 => '─',
                    1 => '·',
                    2 => '•',
                    _ => '∙',
                };
                
                // 颜色类型分布 - 保持颜色协调性
                let color_type = match i % 10 {
                    0..=5 => ColorType::Primary,   // 60% 主色
                    6..=8 => ColorType::Secondary, // 30% 次色
                    _ => ColorType::Tertiary,      // 10% 第三色
                };
                
                // 根据位置调整生命周期 - 线头粒子生命周期短，线尾粒子生命周期长
                let max_age = 10 + (distance_factor * 15.0) as u16; // 范围10-25
                
                self.particles.push(Particle {
                    x: self.explosion_x as f32,
                    y: self.explosion_y as f32,
                    vx,
                    vy,
                    age: 0,
                    max_age,
                    brightness: 1.0,
                    symbol,
                    color_type,
                    distance,
                });
            }
        }
        
        // 添加一些随机粒子填充空隙，使爆炸更饱满
        for _ in 0..(particle_count / 3) {
            let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
            let speed_base = rng.random_range(0.3..0.9);
            
            let vx = angle.cos() * speed_base;
            let vy = angle.sin() * speed_base * y_scale;
            
            let distance = speed_base;
            let symbol = '✦';
            
            let color_type = match rng.random_range(0..10) {
                0..=5 => ColorType::Primary,
                6..=8 => ColorType::Secondary,
                _ => ColorType::Tertiary,
            };
            
            let max_age = rng.random_range(10..20);
            
            self.particles.push(Particle {
                x: self.explosion_x as f32,
                y: self.explosion_y as f32,
                vx,
                vy,
                age: 0,
                max_age,
                brightness: 1.0,
                symbol,
                color_type,
                distance,
            });
        }
    }

    fn trigger_secondary_explosion(&mut self) {
        let mut rng = rand::rng();
        
        // 找出当前存在的粒子位置
        let existing_particles: Vec<(f32, f32)> = self.particles
            .iter()
            .filter(|p| p.brightness > 0.5) // 只选择还比较亮的粒子
            .map(|p| (p.x, p.y))
            .collect();
            
        if !existing_particles.is_empty() {
            // 随机选择某个粒子位置作为新的爆炸中心
            let idx = rng.random_range(0..existing_particles.len());
            let (center_x, center_y) = existing_particles[idx];
            
            // 计算适当的缩放因子来调整为圆形
            let y_scale = 0.4; // 保持一致的垂直缩放
            
            // 二次爆炸也使用线状效果
            let line_count = 6; // 二次爆炸线数较少
            let secondary_particles = (self.size as usize) * 20; // 使用usize类型
            
            // 创建射线方向
            for i in 0..line_count {
                let main_angle = (i as f32 / line_count as f32) * std::f32::consts::PI * 2.0;
                
                // 沿每条线创建粒子
                for j in 0..(secondary_particles / line_count) {
                    let distance_factor = (j as f32 / (secondary_particles / line_count) as f32).powf(0.7);
                    let speed = 0.3 + distance_factor * 0.3; // 二次爆炸速度较小
                    
                    // 轻微扰动角度
                    let angle_variance = rng.random_range(-0.1..0.1);
                    let angle = main_angle + angle_variance;
                    
                    // 应用缩放因子
                    let vx = angle.cos() * speed;
                    let vy = angle.sin() * speed * y_scale;
                    
                    // 计算预期移动距离
                    let distance = speed;
                    
                    // 二次爆炸用线性符号
                    let symbol = match j % 3 {
                        0 => '∙',
                        1 => '·',
                        _ => '.',
                    };
                    
                    // 二次爆炸的颜色类型 - 使用较亮的颜色
                    let color_type = match j % 10 {
                        0..=3 => ColorType::Secondary, // 40% 次色
                        4..=7 => ColorType::Primary,   // 40% 主色
                        _ => ColorType::Tertiary,      // 20% 第三色
                    };
                    
                    // 二次爆炸粒子寿命更短
                    let max_age = 5 + (distance_factor * 10.0) as u16; // 范围5-15
                    
                    self.particles.push(Particle {
                        x: center_x,
                        y: center_y,
                        vx,
                        vy,
                        age: 0,
                        max_age,
                        brightness: 1.0,
                        symbol,
                        color_type,
                        distance,
                    });
                }
            }
        }
    }

    pub fn draw(&self, stdout: &mut impl Write, config: &Config) -> Result<()> {
        if !self.exploded {
            // 绘制上升中的烟花
            execute!(
                stdout,
                MoveTo(self.x, self.y),
                SetForegroundColor(self.primary_color),
                Print("↑")
            )?;
            
            // 绘制尾迹，使用相近颜色
            for (i, (trail_x, trail_y)) in self.trail.iter().enumerate() {
                // 尾迹使用次要颜色和三级颜色，保持鲜艳
                let color = match i {
                    0..=1 => self.tertiary_color,
                    _ => self.secondary_color,
                };
                
                execute!(
                    stdout,
                    MoveTo(*trail_x, *trail_y),
                    SetForegroundColor(color),
                    Print(".")
                )?;
            }
        } else {
            // 绘制爆炸后的粒子，只使用相近颜色组合
            for particle in &self.particles {
                let x = particle.x as u16;
                let y = particle.y as u16;
                
                // 确保粒子在屏幕范围内
                if x > 0 && y > 0 && y < config.height {
                    // 根据粒子的颜色类型选择对应的颜色
                    let color = match particle.color_type {
                        ColorType::Primary => self.primary_color,
                        ColorType::Secondary => self.secondary_color,
                        ColorType::Tertiary => self.tertiary_color,
                    };
                    
                    // 保持颜色鲜艳，不进行暗色过渡
                    execute!(
                        stdout,
                        MoveTo(x, y),
                        SetForegroundColor(color),
                        Print(particle.symbol)
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn is_done(&self) -> bool {
        self.exploded && (self.particles.is_empty() || self.age >= self.max_age)
    }
} 