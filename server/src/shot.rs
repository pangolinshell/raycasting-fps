use crate::gamestate::{GameState};
use shared::{PlayerState}; //Direction
use shared::map::Map;
use std::f32;


#[derive(Debug)]
pub struct ShotResult {
    pub init_pos: (i32, i32),
    pub final_pos: (i32, i32),
    pub wall_hit: Option<(i32, i32)>,
    pub player_hit: Option<(u32, (i32, i32))>, 
}

impl GameState {
    pub fn trace_shot_angle(
        &self,
        shooter: &PlayerState,
        map: &Map,
        max_distance: f32,
    ) -> ShotResult {
        let (mut x, mut y) = (shooter.x as f32, shooter.y as f32);
        let angle_rad = shooter.angle_degrees.to_radians();
        let (dx, dy) = (angle_rad.cos(), angle_rad.sin());
    
        let init_pos = (x as i32, y as i32);
        let mut traveled = 0.0;
        let step = 0.1;
    
        while traveled < max_distance {
            x += dx * step;
            y += dy * step;
            traveled += step;
    
            let xi = x.floor() as i32;
            let yi = y.floor() as i32;
    
            if yi < 0 || xi < 0 || yi as usize >= map.layout.len() || xi as usize >= map.layout[0].len() {
                return ShotResult {
                    init_pos,
                    final_pos: (xi, yi),
                    wall_hit: Some((xi, yi)),
                    player_hit: None,
                };
            }
            if map.layout[yi as usize][xi as usize] != 0 {
                return ShotResult {
                    init_pos,
                    final_pos: (xi, yi),
                    wall_hit: Some((xi, yi)),
                    player_hit: None,
                };
            }
    
            
            for player in self.players.values() {
                if player.id != shooter.id {
                    let px = player.x as i32;
                    let py = player.y as i32;
                    if px == xi && py == yi {
                        return ShotResult {
                            init_pos,
                            final_pos: (xi, yi),
                            wall_hit: None,
                            player_hit: Some((player.id, (xi, yi))),
                        };
                    }
                }
            }
        }
        ShotResult {
            init_pos,
            final_pos: (x as i32, y as i32),
            wall_hit: None,
            player_hit: None,
        }
    }
}
