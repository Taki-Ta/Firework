use crate::types::Particle;

impl Particle {
    pub fn update(&mut self) {
        // 更新位置
        self.x += self.vx;
        self.y += self.vy;
        
        // 极小的重力效果，避免粒子坠落太快
        self.vy += 0.008; 
        
        // 水平速度逐渐减小，模拟空气阻力
        self.vx *= 0.94; 
        
        // 亮度随年龄非线性衰减
        // 开始时亮度变化较慢，后期快速衰减
        let age_ratio = self.age as f32 / self.max_age as f32;
        self.brightness = (1.0 - age_ratio * age_ratio * 2.0).max(0.0);
        
        // 年龄增加
        self.age += 1;
    }
} 