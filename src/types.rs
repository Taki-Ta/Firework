use crossterm::style::Color;

// 颜色类型枚举
pub enum ColorType {
    Primary,   // 主色调
    Secondary, // 次色调
    Tertiary,  // 第三色调
}

// 配置结构
pub struct Config {
    pub width: u16,
    pub height: u16,
    pub max_fireworks: u16,
    pub min_firework_size: u16, 
    pub max_firework_size: u16,
}

impl Config {
    pub fn new() -> Self {
        Config {
            width: 160, // 更宽的默认终端
            height: 60, // 更高的默认终端
            max_fireworks: 7, // 减少默认烟花数量
            min_firework_size: 5, // 增大默认烟花最小尺寸
            max_firework_size: 15, // 增大默认烟花最大尺寸
        }
    }
}

// 烟花结构
pub struct Firework {
    pub x: u16,
    pub y: u16,
    pub explosion_x: u16,
    pub explosion_y: u16,
    pub size: u16,
    pub primary_color: Color,     // 主色调
    pub secondary_color: Color,   // 次色调，与主色相近
    pub tertiary_color: Color,    // 第三色调，作为点缀
    pub exploded: bool,
    pub particles: Vec<Particle>,
    pub age: u16,
    pub max_age: u16,
    pub trail: Vec<(u16, u16)>,
    pub explosion_stage: u8,
    pub max_explosion_stages: u8,
}

// 粒子结构
#[allow(dead_code)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub age: u16,
    pub max_age: u16,
    pub brightness: f32,
    pub symbol: char,
    pub color_type: ColorType, // 使用颜色类型而不是具体颜色
    pub distance: f32,         // 粒子距离爆炸中心的距离
} 