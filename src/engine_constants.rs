use std::collections::HashMap;

use log::info;
use maplit::hashmap;

use crate::common::{Direction, Rect};
use crate::player::ControlMode;
use crate::str;
use crate::text_script::TextScriptEncoding;

#[derive(Debug, Copy, Clone)]
pub struct PhysicsConsts {
    pub max_dash: isize,
    pub max_move: isize,
    pub gravity_ground: isize,
    pub gravity_air: isize,
    pub dash_ground: isize,
    pub dash_air: isize,
    pub resist: isize,
    pub jump: isize,
}


#[derive(Debug, Copy, Clone)]
pub struct BoosterConsts {
    pub fuel: usize,
    pub b2_0_up: isize,
    pub b2_0_up_nokey: isize,
    pub b2_0_down: isize,
    pub b2_0_left: isize,
    pub b2_0_right: isize,
}

#[derive(Debug, Copy, Clone)]
pub struct MyCharConsts {
    pub display_bounds: Rect<usize>,
    pub hit_bounds: Rect<usize>,
    pub life: usize,
    pub max_life: usize,
    pub control_mode: ControlMode,
    pub air_physics: PhysicsConsts,
    pub water_physics: PhysicsConsts,
    pub animations_left: [Rect<usize>; 12],
    pub animations_right: [Rect<usize>; 12],
}

#[derive(Debug)]
pub struct CaretConsts {
    pub offsets: [(isize, isize); 18],
    pub bubble_left_rects: Vec<Rect<usize>>,
    pub bubble_right_rects: Vec<Rect<usize>>,
    pub zzz_rects: Vec<Rect<usize>>,
    pub drowned_quote_left_rect: Rect<usize>,
    pub drowned_quote_right_rect: Rect<usize>,
    pub level_up_rects: Vec<Rect<usize>>,
    pub level_down_rects: Vec<Rect<usize>>,
    pub little_particles_rects: Vec<Rect<usize>>,
    pub exhaust_rects: Vec<Rect<usize>>,
    pub question_left_rect: Rect<usize>,
    pub question_right_rect: Rect<usize>,
}

impl Clone for CaretConsts {
    fn clone(&self) -> Self {
        Self {
            offsets: self.offsets,
            bubble_left_rects: self.bubble_left_rects.clone(),
            bubble_right_rects: self.bubble_right_rects.clone(),
            zzz_rects: self.zzz_rects.clone(),
            drowned_quote_left_rect: self.drowned_quote_left_rect,
            drowned_quote_right_rect: self.drowned_quote_right_rect,
            level_up_rects: self.level_up_rects.clone(),
            level_down_rects: self.level_down_rects.clone(),
            little_particles_rects: self.little_particles_rects.clone(),
            exhaust_rects: self.exhaust_rects.clone(),
            question_left_rect: self.question_left_rect,
            question_right_rect: self.question_right_rect,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WorldConsts {
    pub snack_rect: Rect<usize>,
}

#[derive(Debug, Copy, Clone)]
pub struct NPCConsts {
    pub n016_save_point: [Rect<usize>; 8],
    pub n017_health_refill: [Rect<usize>; 2],
    pub n018_door: [Rect<usize>; 2],
    pub n020_computer: [Rect<usize>; 4],
    pub n021_chest_open: Rect<usize>,
    pub n022_teleporter: [Rect<usize>; 2],
    pub n023_teleporter_lights: [Rect<usize>; 8],
    pub n027_death_trap: Rect<usize>,
    pub n029_cthulhu: [Rect<usize>; 4],
    pub n030_hermit_gunsmith: [Rect<usize>; 3],
    pub n032_life_capsule: [Rect<usize>; 2],
    pub n034_bed: [Rect<usize>; 2],
    pub n035_mannan: [Rect<usize>; 8],
    pub n037_sign: [Rect<usize>; 2],
    pub n038_fireplace: [Rect<usize>; 4],
    pub n039_save_sign: [Rect<usize>; 2],
    pub n040_santa: [Rect<usize>; 14],
    pub n041_busted_door: Rect<usize>,
    pub n042_sue: [Rect<usize>; 26],
    pub n043_chalkboard: [Rect<usize>; 2],
    pub n044_polish: [Rect<usize>; 6],
    pub n045_baby: [Rect<usize>; 3],
    pub n047_sandcroc: [Rect<usize>; 5],
    pub n048_omega_projectiles: [Rect<usize>; 4],
    pub n049_skullhead: [Rect<usize>; 6],
    pub n052_sitting_blue_robot: Rect<usize>,
    pub n055_kazuma: [Rect<usize>; 12],
    pub n059_eye_door: [Rect<usize>; 4],
    pub n060_toroko: [Rect<usize>; 16],
    pub n061_king: [Rect<usize>; 20],
    pub n062_kazuma_computer: [Rect<usize>; 3],
    pub n070_sparkle: [Rect<usize>; 4],
    pub n063_toroko_stick: [Rect<usize>; 12],
    pub n071_chinfish: [Rect<usize>; 6],
    pub n072_sprinkler: [Rect<usize>; 2],
    pub n073_water_droplet: [Rect<usize>; 5],
    pub n074_jack: [Rect<usize>; 12],
    pub n075_kanpachi: [Rect<usize>; 2],
    pub n077_yamashita: [Rect<usize>; 3],
    pub n078_pot: [Rect<usize>; 2],
    pub n079_mahin: [Rect<usize>; 6],
}

#[derive(Debug, Copy, Clone)]
pub struct TextScriptConsts {
    pub encoding: TextScriptEncoding,
    pub textbox_rect_top: Rect<usize>,
    pub textbox_rect_middle: Rect<usize>,
    pub textbox_rect_bottom: Rect<usize>,
    pub textbox_rect_yes_no: Rect<usize>,
    pub textbox_rect_cursor: Rect<usize>,
}

#[derive(Debug)]
pub struct EngineConstants {
    pub is_cs_plus: bool,
    pub my_char: MyCharConsts,
    pub booster: BoosterConsts,
    pub caret: CaretConsts,
    pub world: WorldConsts,
    pub npc: NPCConsts,
    pub tex_sizes: HashMap<String, (usize, usize)>,
    pub textscript: TextScriptConsts,
    pub font_path: String,
    pub font_scale: f32,
    pub font_space_offset: f32,
    pub organya_paths: Vec<String>,
}

impl Clone for EngineConstants {
    fn clone(&self) -> Self {
        Self {
            is_cs_plus: self.is_cs_plus,
            my_char: self.my_char,
            booster: self.booster,
            caret: self.caret.clone(),
            world: self.world.clone(),
            npc: self.npc.clone(),
            tex_sizes: self.tex_sizes.clone(),
            textscript: self.textscript.clone(),
            font_path: self.font_path.clone(),
            font_scale: self.font_scale,
            font_space_offset: self.font_space_offset,
            organya_paths: self.organya_paths.clone(),
        }
    }
}

impl EngineConstants {
    pub fn defaults() -> Self {
        EngineConstants {
            is_cs_plus: false,
            my_char: MyCharConsts {
                display_bounds: Rect { left: 8 * 0x200, top: 8 * 0x200, right: 8 * 0x200, bottom: 8 * 0x200 },
                hit_bounds: Rect { left: 5 * 0x200, top: 8 * 0x200, right: 5 * 0x200, bottom: 8 * 0x200 },
                life: 3,
                max_life: 3,
                control_mode: ControlMode::Normal,
                air_physics: PhysicsConsts {
                    max_dash: 0x32c,
                    max_move: 0x5ff,
                    gravity_air: 0x20,
                    gravity_ground: 0x50,
                    dash_air: 0x20,
                    dash_ground: 0x55,
                    resist: 0x33,
                    jump: 0x500,
                },
                water_physics: PhysicsConsts {
                    max_dash: 0x196,
                    max_move: 0x2ff,
                    gravity_air: 0x10,
                    gravity_ground: 0x28,
                    dash_air: 0x10,
                    dash_ground: 0x2a,
                    resist: 0x19,
                    jump: 0x280,
                },
                animations_left: [
                    Rect { left: 0, top: 0, right: 16, bottom: 16 },
                    Rect { left: 16, top: 0, right: 32, bottom: 16 },
                    Rect { left: 0, top: 0, right: 16, bottom: 16 },
                    Rect { left: 32, top: 0, right: 48, bottom: 16 },
                    Rect { left: 0, top: 0, right: 16, bottom: 16 },
                    Rect { left: 48, top: 0, right: 64, bottom: 16 },
                    Rect { left: 64, top: 0, right: 80, bottom: 16 },
                    Rect { left: 48, top: 0, right: 64, bottom: 16 },
                    Rect { left: 80, top: 0, right: 96, bottom: 16 },
                    Rect { left: 48, top: 0, right: 64, bottom: 16 },
                    Rect { left: 96, top: 0, right: 112, bottom: 16 },
                    Rect { left: 112, top: 0, right: 128, bottom: 16 },
                ],
                animations_right: [
                    Rect { left: 0, top: 16, right: 16, bottom: 32 },
                    Rect { left: 16, top: 16, right: 32, bottom: 32 },
                    Rect { left: 0, top: 16, right: 16, bottom: 32 },
                    Rect { left: 32, top: 16, right: 48, bottom: 32 },
                    Rect { left: 0, top: 16, right: 16, bottom: 32 },
                    Rect { left: 48, top: 16, right: 64, bottom: 32 },
                    Rect { left: 64, top: 16, right: 80, bottom: 32 },
                    Rect { left: 48, top: 16, right: 64, bottom: 32 },
                    Rect { left: 80, top: 16, right: 96, bottom: 32 },
                    Rect { left: 48, top: 16, right: 64, bottom: 32 },
                    Rect { left: 96, top: 16, right: 112, bottom: 32 },
                    Rect { left: 112, top: 16, right: 128, bottom: 32 },
                ],
            },
            booster: BoosterConsts {
                fuel: 50,
                b2_0_up: -0x5ff,
                b2_0_up_nokey: -0x5ff,
                b2_0_down: 0x5ff,
                b2_0_left: -0x5ff,
                b2_0_right: 0x5ff,
            },
            caret: CaretConsts {
                offsets: [
                    (0, 0),
                    (4 * 0x200, 4 * 0x200),
                    (8 * 0x200, 8 * 0x200),
                    (8 * 0x200, 8 * 0x200),
                    (8 * 0x200, 8 * 0x200),
                    (4 * 0x200, 4 * 0x200),
                    (8 * 0x200, 8 * 0x200),
                    (4 * 0x200, 4 * 0x200),
                    (8 * 0x200, 8 * 0x200),
                    (8 * 0x200, 8 * 0x200),
                    (28 * 0x200, 8 * 0x200),
                    (4 * 0x200, 4 * 0x200),
                    (16 * 0x200, 16 * 0x200),
                    (4 * 0x200, 4 * 0x200),
                    (20 * 0x200, 20 * 0x200),
                    (4 * 0x200, 4 * 0x200),
                    (20 * 0x200, 4 * 0x200),
                    (52 * 0x200, 4 * 0x200),
                ],
                bubble_left_rects: vec![
                    Rect { left: 0, top: 64, right: 8, bottom: 72 },
                    Rect { left: 8, top: 64, right: 16, bottom: 72 },
                    Rect { left: 16, top: 64, right: 24, bottom: 72 },
                    Rect { left: 24, top: 64, right: 32, bottom: 72 },
                ],
                bubble_right_rects: vec![
                    Rect { left: 64, top: 24, right: 72, bottom: 32 },
                    Rect { left: 72, top: 24, right: 80, bottom: 32 },
                    Rect { left: 80, top: 24, right: 88, bottom: 32 },
                    Rect { left: 88, top: 24, right: 96, bottom: 32 },
                ],
                zzz_rects: vec![
                    Rect { left: 32, top: 64, right: 40, bottom: 72 },
                    Rect { left: 32, top: 72, right: 40, bottom: 80 },
                    Rect { left: 40, top: 64, right: 48, bottom: 72 },
                    Rect { left: 40, top: 72, right: 48, bottom: 80 },
                    Rect { left: 40, top: 64, right: 48, bottom: 72 },
                    Rect { left: 40, top: 72, right: 48, bottom: 80 },
                    Rect { left: 40, top: 64, right: 48, bottom: 72 },
                ],
                drowned_quote_left_rect: Rect { left: 16, top: 80, right: 32, bottom: 96 },
                drowned_quote_right_rect: Rect { left: 32, top: 80, right: 48, bottom: 96 },
                level_up_rects: vec![
                    Rect { left: 0, top: 0, right: 56, bottom: 16 },
                    Rect { left: 0, top: 16, right: 56, bottom: 32 },
                ],
                level_down_rects: vec![
                    Rect { left: 0, top: 96, right: 56, bottom: 112 },
                    Rect { left: 0, top: 112, right: 56, bottom: 128 },
                ],
                little_particles_rects: vec![
                    Rect { left: 56, top: 24, right: 64, bottom: 32 },
                    Rect { left: 0, top: 0, right: 0, bottom: 0 },
                ],
                exhaust_rects: vec![
                    Rect { left: 56, top: 0, right: 64, bottom: 8 },
                    Rect { left: 64, top: 0, right: 72, bottom: 8 },
                    Rect { left: 72, top: 0, right: 80, bottom: 8 },
                    Rect { left: 80, top: 0, right: 88, bottom: 8 },
                    Rect { left: 88, top: 0, right: 96, bottom: 8 },
                    Rect { left: 96, top: 0, right: 104, bottom: 8 },
                    Rect { left: 104, top: 0, right: 112, bottom: 8 },
                ],
                question_left_rect: Rect { left: 0, top: 80, right: 16, bottom: 96 },
                question_right_rect: Rect { left: 48, top: 64, right: 64, bottom: 80 },
            },
            world: WorldConsts {
                snack_rect: Rect { left: 256, top: 48, right: 272, bottom: 64 },
            },
            npc: NPCConsts {
                n016_save_point: [
                    Rect { left: 96, top: 16, right: 112, bottom: 32 },
                    Rect { left: 112, top: 16, right: 128, bottom: 32 },
                    Rect { left: 128, top: 16, right: 144, bottom: 32 },
                    Rect { left: 144, top: 16, right: 160, bottom: 32 },
                    Rect { left: 160, top: 16, right: 176, bottom: 32 },
                    Rect { left: 176, top: 16, right: 192, bottom: 32 },
                    Rect { left: 192, top: 16, right: 208, bottom: 32 },
                    Rect { left: 208, top: 16, right: 224, bottom: 32 },
                ],
                n017_health_refill: [
                    Rect { left: 288, top: 0, right: 304, bottom: 16 },
                    Rect { left: 304, top: 0, right: 320, bottom: 16 },
                ],
                n018_door: [
                    Rect { left: 224, top: 16, right: 240, bottom: 40 },
                    Rect { left: 192, top: 112, right: 208, bottom: 136 },
                ],
                n020_computer: [
                    Rect { left: 288, top: 16, right: 320, bottom: 40 }, // left

                    Rect { left: 288, top: 40, right: 320, bottom: 64 }, // right
                    Rect { left: 288, top: 40, right: 320, bottom: 64 },
                    Rect { left: 288, top: 64, right: 320, bottom: 88 },
                ],
                n021_chest_open: Rect { left: 224, top: 40, right: 240, bottom: 48 },
                n022_teleporter: [
                    Rect { left: 240, top: 16, right: 264, bottom: 48 },
                    Rect { left: 248, top: 152, right: 272, bottom: 184 },
                ],
                n023_teleporter_lights: [
                    Rect { left: 264, top: 16, right: 288, bottom: 20 },
                    Rect { left: 264, top: 20, right: 288, bottom: 24 },
                    Rect { left: 264, top: 24, right: 288, bottom: 28 },
                    Rect { left: 264, top: 28, right: 288, bottom: 32 },
                    Rect { left: 264, top: 32, right: 288, bottom: 36 },
                    Rect { left: 264, top: 36, right: 288, bottom: 40 },
                    Rect { left: 264, top: 40, right: 288, bottom: 44 },
                    Rect { left: 264, top: 44, right: 288, bottom: 48 },
                ],
                n027_death_trap: Rect { left: 96, top: 64, right: 128, bottom: 88 },
                n029_cthulhu: [
                    Rect { left: 0, top: 192, right: 16, bottom: 216 }, // left
                    Rect { left: 16, top: 192, right: 32, bottom: 216 },
                    Rect { left: 0, top: 216, right: 16, bottom: 240 }, // right
                    Rect { left: 16, top: 216, right: 32, bottom: 240 },
                ],
                n030_hermit_gunsmith: [
                    Rect { left: 48, top: 0, right: 64, bottom: 16 },
                    Rect { left: 48, top: 16, right: 64, bottom: 32 },
                    Rect { left: 0, top: 32, right: 16, bottom: 48 },
                ],
                n032_life_capsule: [
                    Rect { left: 32, top: 96, right: 48, bottom: 112 },
                    Rect { left: 48, top: 96, right: 64, bottom: 112 },
                ],
                n034_bed: [
                    Rect { left: 192, top: 48, right: 224, bottom: 64 },
                    Rect { left: 192, top: 184, right: 224, bottom: 200 },
                ],
                n035_mannan: [
                    Rect { left: 96, top: 64, right: 120, bottom: 96 }, // left
                    Rect { left: 120, top: 64, right: 144, bottom: 96 },
                    Rect { left: 144, top: 64, right: 168, bottom: 96 },
                    Rect { left: 168, top: 64, right: 192, bottom: 96 },
                    Rect { left: 96, top: 96, right: 120, bottom: 128 }, // right
                    Rect { left: 120, top: 96, right: 144, bottom: 128 },
                    Rect { left: 144, top: 96, right: 168, bottom: 128 },
                    Rect { left: 168, top: 96, right: 192, bottom: 128 },
                ],
                n037_sign: [
                    Rect { left: 192, top: 64, right: 208, bottom: 80 },
                    Rect { left: 208, top: 64, right: 224, bottom: 80 },
                ],
                n038_fireplace: [
                    Rect { left: 128, top: 64, right: 144, bottom: 80 },
                    Rect { left: 144, top: 64, right: 160, bottom: 80 },
                    Rect { left: 160, top: 64, right: 176, bottom: 80 },
                    Rect { left: 176, top: 64, right: 192, bottom: 80 },
                ],
                n039_save_sign: [
                    Rect { left: 224, top: 64, right: 240, bottom: 80 },
                    Rect { left: 240, top: 64, right: 256, bottom: 80 },
                ],
                n040_santa: [
                    Rect { left: 0, top: 32, right: 16, bottom: 48 }, // left
                    Rect { left: 16, top: 32, right: 32, bottom: 48 },
                    Rect { left: 32, top: 32, right: 48, bottom: 48 },
                    Rect { left: 0, top: 32, right: 16, bottom: 48 },
                    Rect { left: 48, top: 32, right: 64, bottom: 48 },
                    Rect { left: 0, top: 32, right: 16, bottom: 48 },
                    Rect { left: 64, top: 32, right: 80, bottom: 48 },
                    Rect { left: 0, top: 48, right: 16, bottom: 64 }, // right
                    Rect { left: 16, top: 48, right: 32, bottom: 64 },
                    Rect { left: 32, top: 48, right: 48, bottom: 64 },
                    Rect { left: 0, top: 48, right: 16, bottom: 64 },
                    Rect { left: 48, top: 48, right: 64, bottom: 64 },
                    Rect { left: 0, top: 48, right: 16, bottom: 64 },
                    Rect { left: 64, top: 48, right: 80, bottom: 64 },
                ],
                n041_busted_door: Rect { left: 0, top: 80, right: 48, bottom: 112 },
                n042_sue: [
                    Rect { left: 0, top: 0, right: 16, bottom: 16 }, // left
                    Rect { left: 16, top: 0, right: 32, bottom: 16 },
                    Rect { left: 32, top: 0, right: 48, bottom: 16 },
                    Rect { left: 0, top: 0, right: 16, bottom: 16 },
                    Rect { left: 48, top: 0, right: 64, bottom: 16 },
                    Rect { left: 0, top: 0, right: 16, bottom: 16 },
                    Rect { left: 64, top: 0, right: 80, bottom: 16 },
                    Rect { left: 80, top: 32, right: 96, bottom: 48 },
                    Rect { left: 96, top: 32, right: 112, bottom: 48 },
                    Rect { left: 128, top: 32, right: 144, bottom: 48 },
                    Rect { left: 0, top: 0, right: 16, bottom: 16 },
                    Rect { left: 112, top: 32, right: 128, bottom: 48 },
                    Rect { left: 160, top: 32, right: 176, bottom: 48 },
                    Rect { left: 0, top: 16, right: 16, bottom: 32 }, // right
                    Rect { left: 16, top: 16, right: 32, bottom: 32 },
                    Rect { left: 32, top: 16, right: 48, bottom: 32 },
                    Rect { left: 0, top: 16, right: 16, bottom: 32 },
                    Rect { left: 48, top: 16, right: 64, bottom: 32 },
                    Rect { left: 0, top: 16, right: 16, bottom: 32 },
                    Rect { left: 64, top: 16, right: 80, bottom: 32 },
                    Rect { left: 80, top: 48, right: 96, bottom: 64 },
                    Rect { left: 96, top: 48, right: 112, bottom: 64 },
                    Rect { left: 128, top: 48, right: 144, bottom: 64 },
                    Rect { left: 0, top: 16, right: 16, bottom: 32 },
                    Rect { left: 112, top: 48, right: 128, bottom: 64 },
                    Rect { left: 160, top: 48, right: 176, bottom: 64 },
                ],
                n043_chalkboard: [
                    Rect { left: 128, top: 80, right: 168, bottom: 112 },
                    Rect { left: 168, top: 80, right: 208, bottom: 112 },
                ],
                n044_polish: [
                    Rect { left: 0, top: 0, right: 32, bottom: 32 }, // left
                    Rect { left: 96, top: 0, right: 128, bottom: 32 },
                    Rect { left: 128, top: 0, right: 160, bottom: 32 },
                    Rect { left: 0, top: 0, right: 32, bottom: 32 }, // right
                    Rect { left: 32, top: 0, right: 64, bottom: 32 },
                    Rect { left: 64, top: 0, right: 96, bottom: 32 },
                ],
                n045_baby: [
                    Rect { left: 0, top: 32, right: 16, bottom: 48 },
                    Rect { left: 16, top: 32, right: 32, bottom: 48 },
                    Rect { left: 32, top: 32, right: 48, bottom: 48 },
                ],
                n047_sandcroc: [
                    Rect { left: 0, top: 48, right: 48, bottom: 80 },
                    Rect { left: 48, top: 48, right: 96, bottom: 80 },
                    Rect { left: 96, top: 48, right: 144, bottom: 80 },
                    Rect { left: 144, top: 48, right: 192, bottom: 80 },
                    Rect { left: 192, top: 48, right: 240, bottom: 80 },
                ],
                n048_omega_projectiles: [
                    Rect { left: 288, top: 88, right: 304, bottom: 104 }, // left
                    Rect { left: 304, top: 88, right: 320, bottom: 104 },
                    Rect { left: 288, top: 104, right: 304, bottom: 120 }, // right
                    Rect { left: 304, top: 104, right: 320, bottom: 120 },
                ],
                n049_skullhead: [
                    Rect { left: 0, top: 80, right: 32, bottom: 104 }, // left
                    Rect { left: 32, top: 80, right: 64, bottom: 104 },
                    Rect { left: 64, top: 80, right: 96, bottom: 104 },
                    Rect { left: 0, top: 104, right: 32, bottom: 128 }, // right
                    Rect { left: 32, top: 104, right: 64, bottom: 128 },
                    Rect { left: 64, top: 104, right: 96, bottom: 128 },
                ],
                n052_sitting_blue_robot: Rect { left: 240, top: 96, right: 256, bottom: 112 },
                n055_kazuma: [
                    Rect { left: 192, top: 192, right: 208, bottom: 216 }, // left
                    Rect { left: 208, top: 192, right: 224, bottom: 216 },
                    Rect { left: 192, top: 192, right: 208, bottom: 216 },
                    Rect { left: 224, top: 192, right: 240, bottom: 216 },
                    Rect { left: 192, top: 192, right: 208, bottom: 216 },
                    Rect { left: 240, top: 192, right: 256, bottom: 216 },
                    Rect { left: 192, top: 216, right: 208, bottom: 240 }, // right
                    Rect { left: 208, top: 216, right: 224, bottom: 240 },
                    Rect { left: 192, top: 216, right: 208, bottom: 240 },
                    Rect { left: 224, top: 216, right: 240, bottom: 240 },
                    Rect { left: 192, top: 216, right: 208, bottom: 240 },
                    Rect { left: 240, top: 216, right: 256, bottom: 240 },
                ],
                n059_eye_door: [
                    Rect { left: 224, top: 16, right: 240, bottom: 40 },
                    Rect { left: 208, top: 80, right: 224, bottom: 104 },
                    Rect { left: 224, top: 80, right: 240, bottom: 104 },
                    Rect { left: 240, top: 80, right: 256, bottom: 104 },
                ],
                n060_toroko: [
                    Rect { left: 0, top: 64, right: 16, bottom: 80 }, // left
                    Rect { left: 16, top: 64, right: 32, bottom: 80 },
                    Rect { left: 32, top: 64, right: 48, bottom: 80 },
                    Rect { left: 16, top: 64, right: 32, bottom: 80 },
                    Rect { left: 48, top: 64, right: 64, bottom: 80 },
                    Rect { left: 16, top: 64, right: 32, bottom: 80 },
                    Rect { left: 112, top: 64, right: 128, bottom: 80 },
                    Rect { left: 128, top: 64, right: 144, bottom: 80 },
                    Rect { left: 0, top: 80, right: 16, bottom: 96 }, // right
                    Rect { left: 16, top: 80, right: 32, bottom: 96 },
                    Rect { left: 32, top: 80, right: 48, bottom: 96 },
                    Rect { left: 16, top: 80, right: 32, bottom: 96 },
                    Rect { left: 48, top: 80, right: 64, bottom: 96 },
                    Rect { left: 16, top: 80, right: 32, bottom: 96 },
                    Rect { left: 112, top: 80, right: 128, bottom: 96 },
                    Rect { left: 128, top: 80, right: 144, bottom: 96 },
                ],
                n061_king: [
                    Rect { left: 224, top: 32, right: 240, bottom: 48 }, // left
                    Rect { left: 240, top: 32, right: 256, bottom: 48 },
                    Rect { left: 256, top: 32, right: 272, bottom: 48 },
                    Rect { left: 272, top: 32, right: 288, bottom: 48 },
                    Rect { left: 288, top: 32, right: 304, bottom: 48 },
                    Rect { left: 224, top: 32, right: 240, bottom: 48 },
                    Rect { left: 304, top: 32, right: 320, bottom: 48 },
                    Rect { left: 224, top: 32, right: 240, bottom: 48 },
                    Rect { left: 272, top: 32, right: 288, bottom: 48 },
                    Rect { left: 0, top: 0, right: 0, bottom: 0 },
                    Rect { left: 224, top: 48, right: 240, bottom: 64 }, // right
                    Rect { left: 240, top: 48, right: 256, bottom: 64 },
                    Rect { left: 256, top: 48, right: 272, bottom: 64 },
                    Rect { left: 272, top: 48, right: 288, bottom: 64 },
                    Rect { left: 288, top: 48, right: 304, bottom: 64 },
                    Rect { left: 224, top: 48, right: 240, bottom: 64 },
                    Rect { left: 304, top: 48, right: 320, bottom: 64 },
                    Rect { left: 224, top: 48, right: 240, bottom: 64 },
                    Rect { left: 272, top: 48, right: 288, bottom: 64 },
                    Rect { left: 0, top: 0, right: 0, bottom: 0 },
                ],
                n062_kazuma_computer: [
                    Rect { left: 272, top: 192, right: 288, bottom: 216 },
                    Rect { left: 288, top: 192, right: 304, bottom: 216 },
                    Rect { left: 304, top: 192, right: 320, bottom: 216 },
                ],
                n063_toroko_stick: [
                    Rect { left: 64, top: 64, right: 80, bottom: 80 }, // left
                    Rect { left: 80, top: 64, right: 96, bottom: 80 },
                    Rect { left: 64, top: 64, right: 80, bottom: 80 },
                    Rect { left: 96, top: 64, right: 112, bottom: 80 },
                    Rect { left: 112, top: 64, right: 128, bottom: 80 },
                    Rect { left: 128, top: 64, right: 144, bottom: 80 },

                    Rect { left: 64, top: 80, right: 80, bottom: 96 }, // right
                    Rect { left: 80, top: 80, right: 96, bottom: 96 },
                    Rect { left: 64, top: 80, right: 80, bottom: 96 },
                    Rect { left: 96, top: 80, right: 112, bottom: 96 },
                    Rect { left: 112, top: 80, right: 128, bottom: 96 },
                    Rect { left: 128, top: 80, right: 144, bottom: 96 },
                ],
                n070_sparkle: [
                    Rect { left: 96, top: 48, right: 112, bottom: 64 },
                    Rect { left: 112, top: 48, right: 128, bottom: 64 },
                    Rect { left: 128, top: 48, right: 144, bottom: 64 },
                    Rect { left: 144, top: 48, right: 160, bottom: 64 },
                ],
                n071_chinfish: [
                    Rect { left: 64, top: 32, right: 80, bottom: 48 }, // left
                    Rect { left: 80, top: 32, right: 96, bottom: 48 },
                    Rect { left: 96, top: 32, right: 112, bottom: 48 },

                    Rect { left: 64, top: 48, right: 80, bottom: 64 }, // right
                    Rect { left: 80, top: 48, right: 96, bottom: 64 },
                    Rect { left: 96, top: 48, right: 112, bottom: 64 },
                ],
                n072_sprinkler: [
                    Rect { left: 224, top: 48, right: 240, bottom: 64 },
                    Rect { left: 240, top: 48, right: 256, bottom: 64 },
                ],
                n073_water_droplet: [
                    Rect { left: 72, top: 16, right: 74, bottom: 18 },
                    Rect { left: 74, top: 16, right: 76, bottom: 18 },
                    Rect { left: 76, top: 16, right: 78, bottom: 18 },
                    Rect { left: 78, top: 16, right: 80, bottom: 18 },
                    Rect { left: 80, top: 16, right: 82, bottom: 18 },
                ],
                n074_jack: [
                    Rect { left: 64, top: 0, right: 80, bottom: 16 }, // left
                    Rect { left: 80, top: 0, right: 96, bottom: 16 },
                    Rect { left: 96, top: 0, right: 112, bottom: 16 },
                    Rect { left: 64, top: 0, right: 80, bottom: 16 },
                    Rect { left: 112, top: 0, right: 128, bottom: 16 },
                    Rect { left: 64, top: 0, right: 80, bottom: 16 },

                    Rect { left: 64, top: 16, right: 80, bottom: 32 }, // right
                    Rect { left: 80, top: 16, right: 96, bottom: 32 },
                    Rect { left: 96, top: 16, right: 112, bottom: 32 },
                    Rect { left: 64, top: 16, right: 80, bottom: 32 },
                    Rect { left: 112, top: 16, right: 128, bottom: 32 },
                    Rect { left: 64, top: 16, right: 80, bottom: 32 },
                ],
                n075_kanpachi: [
                    Rect { left: 272, top: 32, right: 296, bottom: 56 },
                    Rect { left: 296, top: 32, right: 320, bottom: 56 },
                ],
                n077_yamashita: [
                    Rect { left: 0, top: 16, right: 48, bottom: 48 },
                    Rect { left: 48, top: 16, right: 96, bottom: 48 },
                    Rect { left: 96, top: 16, right: 144, bottom: 48 },
                ],
                n078_pot: [
                    Rect { left: 160, top: 48, right: 176, bottom: 64 },
                    Rect { left: 176, top: 48, right: 192, bottom: 64 },
                ],
                n079_mahin: [
                    Rect { left: 0, top: 0, right: 16, bottom: 16 }, // left
                    Rect { left: 16, top: 0, right: 32, bottom: 16 },
                    Rect { left: 32, top: 0, right: 48, bottom: 16 },

                    Rect { left: 0, top: 16, right: 16, bottom: 32 }, // right
                    Rect { left: 16, top: 16, right: 32, bottom: 32 },
                    Rect { left: 32, top: 16, right: 48, bottom: 32 },
                ],
            },
            tex_sizes: hashmap! {
                str!("ArmsImage") => (256, 16),
                str!("Arms") => (320, 200),
                str!("bk0") => (64, 64),
                str!("bkBlack") => (64, 64),
                str!("bkBlue") => (64, 64),
                str!("bkFall") => (64, 64),
                str!("bkFog") => (320, 240),
                str!("bkFog480fix") => (480, 272), // nxengine
                str!("bkGard") => (48, 64),
                str!("bkGray") => (64, 64),
                str!("bkGreen") => (64, 64),
                str!("bkHellish") => (320, 240), // nxengine
                str!("bkHellish480fix") => (480, 272), // nxengine
                str!("bkLight") => (320, 240), // nxengine
                str!("bkLight480fix") => (480, 272), // nxengine
                str!("bkMaze") => (64, 64),
                str!("bkMoon") => (320, 240),
                str!("bkMoon480fix") => (480, 272), // nxengine
                str!("bkRed") => (32, 32),
                str!("bkSunset") => (320, 240), // nxengine
                str!("bkSunset480fix") => (480, 272), // nxengine
                str!("bkWater") => (32, 48),
                str!("Bullet") => (320, 176),
                str!("Caret") => (320, 240),
                str!("casts") => (320, 240),
                str!("Face") => (288, 240),
                str!("Face_0") => (288, 240), // nxengine
                str!("Face_1") => (288, 240), // nxengine
                str!("Face_2") => (288, 240), // nxengine
                str!("Fade") => (256, 32),
                str!("ItemImage") => (256, 128),
                str!("Loading") => (64, 8),
                str!("MyChar") => (200, 64),
                str!("Npc/Npc0") => (32, 32),
                str!("Npc/NpcAlmo1") => (320, 240),
                str!("Npc/NpcAlmo2") => (320, 240),
                str!("Npc/NpcBallos") => (320, 240),
                str!("Npc/NpcBllg") => (320, 96),
                str!("Npc/NpcCemet") => (320, 112),
                str!("Npc/NpcCent") => (320, 192),
                str!("Npc/NpcCurly") => (256, 80),
                str!("Npc/NpcDark") => (160, 64),
                str!("Npc/NpcDr") => (320, 240),
                str!("Npc/NpcEggs1") => (320, 112),
                str!("Npc/NpcEggs2") => (320, 128),
                str!("Npc/NpcFrog") => (320, 240),
                str!("Npc/NpcGuest") => (320, 184),
                str!("Npc/NpcHell") => (320, 160),
                str!("Npc/NpcHeri") => (320, 128),
                str!("Npc/NpcIronH") => (320, 72),
                str!("Npc/NpcIsland") => (320, 80),
                str!("Npc/NpcKings") => (96, 48),
                str!("Npc/NpcMaze") => (320, 192),
                str!("Npc/NpcMiza") => (320, 240),
                str!("Npc/NpcMoon") => (320, 128),
                str!("Npc/NpcOmg") => (320, 120),
                str!("Npc/NpcPlant") => (320, 48),
                str!("Npc/NpcPress") => (320, 240),
                str!("Npc/NpcPriest") => (320, 240),
                str!("Npc/NpcRavil") => (320, 168),
                str!("Npc/NpcRed") => (320, 144),
                str!("Npc/NpcRegu") => (320, 240),
                str!("Npc/NpcSand") => (320, 176),
                str!("Npc/NpcStream") => (64, 32),
                str!("Npc/NpcSym") => (320, 240),
                str!("Npc/NpcToro") => (320, 144),
                str!("Npc/NpcTwinD") => (320, 144),
                str!("Npc/NpcWeed") => (320, 240),
                str!("Npc/NpcX") => (320, 240),
                str!("Resource/BITMAP/Credit01") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit02") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit03") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit04") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit05") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit06") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit07") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit08") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit09") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit10") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit11") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit12") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit14") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit15") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit16") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit17") => (160, 240), // cse2
                str!("Resource/BITMAP/Credit18") => (160, 240), // cse2
                str!("Resource/BITMAP/pixel") => (160, 16), // cse2
                str!("Resource/CURSOR/CURSOR_IKA") => (32, 32), // cse2
                str!("Resource/CURSOR/CURSOR_NORMAL") => (32, 32), // cse2
                str!("StageImage") => (256, 16),
                str!("Stage/Prt0") => (32, 32),
                str!("Stage/PrtAlmond") => (256, 96),
                str!("Stage/PrtBarr") => (256, 88),
                str!("Stage/PrtCave") => (256, 80),
                str!("Stage/PrtCent") => (256, 128),
                str!("Stage/PrtEggIn") => (256, 80),
                str!("Stage/PrtEggs") => (256, 240),
                str!("Stage/PrtEggX") => (256, 240),
                str!("Stage/PrtFall") => (256, 128),
                str!("Stage/PrtGard") => (256, 97),
                str!("Stage/PrtHell") => (256, 240),
                str!("Stage/PrtJail") => (256, 128),
                str!("Stage/PrtLabo") => (128, 64),
                str!("Stage/PrtMaze") => (256, 160),
                str!("Stage/PrtMimi") => (256, 160),
                str!("Stage/PrtOside") => (256, 64),
                str!("Stage/PrtPens") => (256, 64),
                str!("Stage/PrtRiver") => (256, 96),
                str!("Stage/PrtSand") => (256, 112),
                str!("Stage/PrtStore") => (256, 112),
                str!("Stage/PrtWeed") => (256, 128),
                str!("Stage/PrtWhite") => (256, 240),
                str!("TextBox") => (244, 144),
                str!("Title") => (320, 48),
            },
            textscript: TextScriptConsts {
                encoding: TextScriptEncoding::UTF8,
                textbox_rect_top: Rect { left: 0, top: 0, right: 244, bottom: 8 },
                textbox_rect_middle: Rect { left: 0, top: 8, right: 244, bottom: 16 },
                textbox_rect_bottom: Rect { left: 0, top: 16, right: 244, bottom: 24 },
                textbox_rect_yes_no: Rect { left: 152, top: 48, right: 244, bottom: 80 },
                textbox_rect_cursor: Rect { left: 112, top: 88, right: 128, bottom: 104 },
            },
            font_path: str!("builtin/builtin_font.fnt"),
            font_scale: 1.0,
            font_space_offset: -3.0,
            organya_paths: vec![
                str!("/org/"), // NXEngine
                str!("/base/Org/"), // CS+
                str!("/Resource/ORG/"), // CSE2E
            ],
        }
    }

    pub fn apply_csplus_patches(&mut self) {
        info!("Applying Cave Story+ constants patches...");

        self.is_cs_plus = true;
        self.tex_sizes.insert(str!("Caret"), (320, 320));
        self.tex_sizes.insert(str!("MyChar"), (200, 384));
        self.tex_sizes.insert(str!("Npc/NpcRegu"), (320, 410));
        self.font_path = str!("csfont.fnt");
        self.font_scale = 0.5;
        self.font_space_offset = 2.0;
    }


    pub fn apply_csplus_nx_patches(&mut self) {
        info!("Applying Switch-specific Cave Story+ constants patches...");
    }
}
